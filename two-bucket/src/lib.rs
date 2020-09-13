use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solves the bucket problem.
///
/// It is actually the `explore` function that does the solving, but as it is recursive, the
/// initialization has to be done here, which includes enforcing some of this problem's rules.
pub fn solve(capacity1: u8, capacity2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
    // The (0, 0) state.
    let zero_state = State::new(BucketState::new(capacity1), BucketState::new(capacity2));
    explore(
        match start_bucket {        // Fill the start bucket,
            Bucket::One => zero_state.fill_one_up(),
            Bucket::Two => zero_state.fill_two_up(),
        },
        1,
        goal,
        vec![                       // and reject illegal states:
            zero_state.clone(),     // the void state,
            match start_bucket {    // and the opposite branch.
                Bucket::One => zero_state.fill_two_up(),
                Bucket::Two => zero_state.fill_one_up(),
            },
        ]
        .into_iter()
        .collect(),
    )
}

/// Represents the state of a bucket in terms of liquids.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct BucketState {
    /// The current amount of liquids in the bucket.
    fill: u8,
    /// The maximum capacity of the bucket.
    capacity: u8,
}

impl BucketState {
    /// Returns an empty bucket with the given `capacity`.
    fn new(capacity: u8) -> Self {
        Self { fill: 0, capacity }
    }

    /// Clones the bucket with a `fill` of the given one.
    fn with_fill(&self, fill: u8) -> Self {
        Self {
            fill,
            capacity: self.capacity,
        }
    }

    /// Clones the bucket and fills it up to its `capacity`.
    fn fill_up(&self) -> Self {
        self.with_fill(self.capacity)
    }

    /// Clones the bucket and empties its `fill`.
    fn empty(&self) -> Self {
        self.with_fill(0)
    }

    /// Clones the bucket and adds the given `amount` to its `fill` without going over the
    /// `capacity`.
    fn pour_in(&self, amount: u8) -> Self {
        self.with_fill(self.capacity.min(self.fill.saturating_add(amount)))
    }

    /// Clones the bucket and removes the given `amount` from its `fill`.
    fn pour_out(&self, amount: u8) -> Self {
        self.with_fill(self.fill.saturating_sub(amount))
    }

    /// Pours the maximum amount from `self`'s clone into `other`'s returning a couple in the same
    /// order respectively.
    fn pour_into(&self, other: &BucketState) -> (Self, Self) {
        let amount = self.fill.min(other.capacity - other.fill);
        (self.pour_out(amount), other.pour_in(amount))
    }
}

/// Represents the state of the system: two `BucketState`s.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    /// The first bucket's state.
    bucket1: BucketState,
    /// The second bucket's state.
    bucket2: BucketState,
}

impl State {
    /// Returns a new state from the two given bucket states.
    fn new(bucket1: BucketState, bucket2: BucketState) -> Self {
        Self { bucket1, bucket2 }
    }

    /// Clones the state and fills up the first bucket.
    fn fill_one_up(&self) -> Self {
        Self::new(self.bucket1.fill_up(), self.bucket2.clone())
    }

    /// Clones the state and fills up the second bucket.
    fn fill_two_up(&self) -> Self {
        Self::new(self.bucket1.clone(), self.bucket2.fill_up())
    }

    /// Clones the state and empties the first bucket
    fn empty_one(&self) -> Self {
        Self::new(self.bucket1.empty(), self.bucket2.clone())
    }

    /// Clones the state and empties the second bucket.
    fn empty_two(&self) -> Self {
        Self::new(self.bucket1.clone(), self.bucket2.empty())
    }

    /// Clones the state and pours the first bucket into the second one, without spilling.
    fn pour_one_into_two(&self) -> Self {
        let (new_one, new_two) = self.bucket1.pour_into(&self.bucket2);
        Self::new(new_one, new_two)
    }

    /// Clones the state and pours the second bucket into the first one, without spilling.
    fn pour_two_into_one(&self) -> Self {
        let (new_two, new_one) = self.bucket2.pour_into(&self.bucket1);
        Self::new(new_one, new_two)
    }
}

/// Depth-first search as a solution to the problem, directly returning the stats.
///
/// `state` is where to start the search but also where it is currently, as this function is
/// recursive. `moves` is the current counter for those. `explored` is a set containing all the
/// previously visited system states.
///
/// Note that a depth-first search is equivalent to a breadth-first search, they are both tree
/// exploration algorithms. Practical efficency can also be equivalent, as long as the already
/// visited states are rejected as soon as possible, which is what is done here: look at the first
/// two lines of the definition. Depth-first search methods are usually easier to implement though:
/// simply using a recursive function.
fn explore(state: State, moves: u8, goal: u8, mut explored: HashSet<State>) -> Option<BucketStats> {
    if explored.contains(&state) {                  // If the exploration came back on its steps,
        None                                        // end this branch right now;
    } else if state.bucket1.fill == goal {          // if it reached one
        Some(BucketStats {
            moves,
            goal_bucket: Bucket::One,
            other_bucket: state.bucket2.fill,
        })
    } else if state.bucket2.fill == goal {          // or the other case of success,
        Some(BucketStats {                          // return the result stats;
            moves,
            goal_bucket: Bucket::Two,
            other_bucket: state.bucket1.fill,
        })
    } else {
        explored.insert(state.clone());             // otherwise mark the state as visited,

        [
            state.fill_one_up(),
            state.fill_two_up(),
            state.empty_one(),
            state.empty_two(),
            state.pour_one_into_two(),
            state.pour_two_into_one(),
        ]
        .iter()                                     // and explore the rest recursively:
        .map(|state| explore(state.clone(), moves + 1, goal, explored.clone()))
        .filter_map(|opt| opt.clone())              // the rejected paths are filtered out,
        .min_by_key(|stats| stats.clone().moves)    // and the move-minimum path is selected.
    }
}
