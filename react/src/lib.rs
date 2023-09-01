use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

// A bunch of proc-macros to ease up enum handling.
#[macro_use]
extern crate derive_where;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate enum_assoc;

/// Unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);

/// Unique identifier for a compute cell.
///
/// Values of type `InputCellId` and `ComputeCellId` are not mutually
/// assignable, demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r
///     .create_compute(&[react::CellId::Input(input)], |_| 222)
///     .unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);

/// Unique identifier for a compute cell callback.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

/// Unique identifier for a generic cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, From, Unwrap)]
#[unwrap(owned, ref, ref_mut)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

/// Generic reactor cell.
#[derive_where(Debug; T: Debug)]
#[derive(From, IsVariant, Unwrap, Assoc)]
#[unwrap(owned, ref, ref_mut)]
#[func(pub fn id(&self) -> CellId { _0.id.into() })]
#[func(pub fn value(&self) -> &T { &_0.value })]
#[func(pub fn dependents(&self) -> &[ComputeCellId] { _0.dependents.as_slice() })]
#[func(pub fn dependents_mut(&mut self) -> &mut Vec<ComputeCellId> { &mut _0.dependents })]
enum Cell<'cb, T> {
    Input(InputCell<T>),
    Compute(ComputeCell<'cb, T>),
}

/// Reactor input cell.
#[derive_where(Debug; T: Debug)]
struct InputCell<T> {
    id: InputCellId,
    value: T,
    /// Links to compute cells that depend on the current one.
    dependents: Vec<ComputeCellId>,
}

impl<T> InputCell<T> {
    /// Builds a new input cell from the given `id` and initial `value`.
    pub fn new(id: InputCellId, value: T) -> Self {
        Self {
            id,
            value,
            dependents: Vec::new(),
        }
    }
}

/// Reactor compute cell.
#[derive_where(Debug; T: Debug)]
struct ComputeCell<'cb, T> {
    id: ComputeCellId,
    value: T,
    /// Links to compute cells that depend on the current one.
    dependents: Vec<ComputeCellId>,
    dependencies: Vec<CellId>,
    #[derive_where(skip)]
    comp_fn: Box<dyn Fn(&[T]) -> T>,
    #[derive_where(skip)]
    callbacks: HashMap<CallbackId, Box<dyn FnMut(T) + 'cb>>,
    /// Last generated callback ID.
    max_callback_id: usize,
}

impl<'cb, T: Copy + PartialEq> ComputeCell<'cb, T> {
    /// Builds a new compute cell from the given ID, computation function,
    /// dependencies and dependency values. All must respect the same order.
    pub fn new(
        id: ComputeCellId,
        comp_fn: impl Fn(&[T]) -> T + 'static,
        dependencies: &[CellId],
        dep_vals: &[T],
    ) -> Self {
        Self {
            id,
            value: comp_fn(&dep_vals),
            dependents: Vec::new(),
            dependencies: dependencies.into(),
            comp_fn: Box::new(comp_fn),
            callbacks: HashMap::new(),
            max_callback_id: 0,
        }
    }

    /// Generates a new callback id.
    ///
    /// Does not re-use IDs from previously-removed callbacks.
    fn gen_callback_id(&mut self) -> CallbackId {
        let res = self.max_callback_id;
        self.max_callback_id += 1;
        CallbackId(res)
    }

    /// Updates the cell value from the given dependency values to be fed into
    /// the cell's computation function.
    pub fn update_value(&mut self, dep_vals: &[T]) {
        let new_val = (self.comp_fn)(dep_vals);

        if new_val != self.value {
            for cb in self.callbacks.values_mut() {
                cb(new_val)
            }
        }

        self.value = new_val;
    }
}

/// Possible failures when trying to remove a callback from a compute cell.
#[derive(Debug, PartialEq, Eq, Display, Error)]
pub enum RemoveCallbackError {
    /// The targeted compute cell ID does not exist.
    NonexistentCell,
    /// The targeted callback ID does not exist in the compute cell.
    NonexistentCallback,
}

/// Basic reactive system.
#[derive_where(Debug; T: Debug)]
pub struct Reactor<'cb, T> {
    cells: HashMap<CellId, Cell<'cb, T>>,
    /// Last generated input cell ID.
    max_input_id: usize,
    /// Last generated compute cell ID.
    max_comp_id: usize,
}

/// Reactor is only tested against types that are `Copy + PartialEq`.
impl<'cb, T: Copy + PartialEq> Reactor<'cb, T> {
    /// Builds a new reactor.
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            max_comp_id: 0,
            max_input_id: 0,
        }
    }

    /// Generates a new input cell ID.
    fn gen_input_id(&mut self) -> InputCellId {
        let res = self.max_input_id;
        self.max_input_id += 1;
        InputCellId(res)
    }

    /// Generates a new compute cell ID.
    fn gen_comp_id(&mut self) -> ComputeCellId {
        let res = self.max_comp_id;
        self.max_comp_id += 1;
        ComputeCellId(res)
    }

    /// Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, value: T) -> InputCellId {
        let id = self.gen_input_id();
        self.cells
            .insert(id.into(), InputCell::new(id, value).into());
        id
    }

    /// Creates a compute cell with the specified dependencies and compute
    /// function, returning the cell ID. The compute function is expected to
    /// take in its arguments in the same order as specified in `dependencies`.
    ///
    /// If any dependency doesn't exist, returns an `Err` with that nonexistent
    /// dependency. If multiple dependencies do not exist, exactly which one
    /// is returned is not defined and will not be tested.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        comp_fn: F,
    ) -> Result<ComputeCellId, CellId> {
        let id = self.gen_comp_id();
        self.cells.insert(
            id.into(),
            ComputeCell::new(
                id,
                comp_fn,
                dependencies,
                &dependencies
                    .iter()
                    .copied()
                    .map(|id| self.value(id).ok_or(id))
                    .collect::<Result<Vec<_>, CellId>>()?,
            )
            .into(),
        );

        // Add the new cell to all dependencies.
        for dep_id in dependencies.iter() {
            self.cells
                .get_mut(dep_id)
                .unwrap()
                .dependents_mut()
                .push(id);
        }

        Ok(id)
    }

    /// Retrieves the current value of the cell, or `None` if the cell does not
    /// exist.
    pub fn value(&self, id: CellId) -> Option<T> {
        self.cells.get(&id).map(Cell::value).copied()
    }

    /// Sets the value of the specified input cell.
    ///
    /// Returns `false` if the cell does not exist.
    pub fn set_value(&mut self, id: InputCellId, new_val: T) -> bool {
        // The next compute cells that need a refresh.
        let mut to_compute = HashSet::new();
        // The compute cells that have already been visited and updated.
        let mut computed = HashSet::new();

        // Update initial input cell and start the rest with its dependents.
        match self.cells.get_mut(&id.into()) {
            Some(Cell::Input(ic)) => {
                ic.value = new_val;
                to_compute.extend(ic.dependents.iter().copied());
            }
            _ => return false,
        };

        // Visit every indirect dependent in dependency-availability order.
        while !to_compute.is_empty() {
            // Temporary holders with identical semantics.
            let mut to_compute_next = HashSet::new();
            let mut just_computed = HashSet::new();

            // Update each compute cell that has all its dependenices already
            // updated.
            for cc_id in to_compute.iter().copied() {
                let cc = self.cells.get(&cc_id.into()).unwrap().unwrap_compute_ref();

                // Input cells are not part of the propagation: their value are
                // always already computed; computed cells are done; other
                // cells neither computed or to compute are not part of the
                // propagation either: compute cells with up-to-date values.
                if cc.dependencies.iter().all(|dep_id| {
                    self.cells.get(dep_id).unwrap().is_input()
                        || computed.contains(dep_id.unwrap_compute_ref())
                        || !to_compute.contains(dep_id.unwrap_compute_ref())
                }) {
                    // Propagate to next cells.
                    to_compute_next.extend(cc.dependents.iter().copied());
                    let dep_vals = cc
                        .dependencies
                        .iter()
                        .map(|dep_id| self.value(*dep_id).unwrap())
                        .collect::<Vec<_>>();
                    self.cells
                        .get_mut(&cc_id.into())
                        .unwrap()
                        .unwrap_compute_mut()
                        .update_value(&dep_vals);
                    just_computed.insert(cc_id);
                }
            }

            // Update propagation markers from temporaries.
            to_compute.retain(|cc_id| !just_computed.contains(cc_id));
            to_compute.extend(to_compute_next.into_iter());
            computed.extend(just_computed.into_iter());
        }

        true
    }

    /// Adds a callback to the specified compute cell.
    ///
    /// Returns the ID of the just-added callback, or `None` if the cell
    /// doesn't exist.
    ///
    /// For a single `set_value` call, each compute cell's callbacks is called:
    /// * Zero times if the compute cell's value did not change as a result of
    ///   the `set_value` call.
    /// * Exactly once if the compute cell's value changed as a result of the
    ///   `set_value` call.
    ///   The value passed to the callback is the final value of the compute
    ///   cell after the `set_value` call.
    pub fn add_callback<F: FnMut(T) + 'cb>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        self.cells.get_mut(&id.into()).map(move |cell| {
            let cc = cell.unwrap_compute_mut();
            let id = cc.gen_callback_id();
            cc.callbacks.insert(id, Box::new(callback));
            id
        })
    }

    /// Removes the specified callback, using an ID returned from `add_callback`.
    ///
    /// Returns an `Err` if either the cell or callback does not exist.
    ///
    /// A removed callback is no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        self.cells
            .get_mut(&cell.into())
            .ok_or(RemoveCallbackError::NonexistentCell)
            .and_then(|cell| {
                cell.unwrap_compute_mut()
                    .callbacks
                    .remove(&callback)
                    .ok_or(RemoveCallbackError::NonexistentCallback)
                    .map(|_| ())
            })
    }
}
