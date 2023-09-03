use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

/// Shortcut for link representation.
///
/// Using [`NonNull`] enables easy covariance over `T`.
type Ptr<T> = Option<NonNull<T>>;

/// Doubly-linked list.
pub struct LinkedList<T> {
    head: Ptr<Node<T>>,
    tail: Ptr<Node<T>>,
    len: usize,
}

/// Node of a linked list holding one element of data.
struct Node<T> {
    data: T,
    next: Ptr<Self>,
    prev: Ptr<Self>,
}

/// Some helpers.
impl<T> Node<T> {
    /// Builds a new unlinked node holding `data` as its element.
    fn new(data: T) -> Self {
        Self {
            data,
            next: Ptr::None,
            prev: Ptr::None,
        }
    }

    /// Builds a new unlinked node holding `data` as its element and allocates
    /// the result on the heap, returning a somewhat raw pointer to it.
    ///
    /// `None` cannot actually occur. Although this would be a bad API choice,
    /// it is fine here as the return type is thus more often compatible with
    /// the encountered nodes in the cursor operations implementation.
    fn alloc_new(data: T) -> Ptr<Self> {
        NonNull::new(Box::leak(Box::new(Self::new(data))))
    }
}

impl<T> LinkedList<T> {
    /// Builds an empty list.
    #[must_use]
    pub fn new() -> Self {
        Self {
            head: Ptr::None,
            tail: Ptr::None,
            len: 0,
        }
    }

    /// Returns whether the list is empty, i.e. does not contain any element.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.head.is_none() && self.tail.is_none()
    }

    /// Returns the number of elements the list currently contains.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns a cursor positioned on the front element.
    #[must_use]
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            node: self.head,
            list: self,
        }
    }

    /// Returns a cursor positioned on the back element.
    #[must_use]
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            node: self.tail,
            list: self,
        }
    }

    /// Returns an iterator that moves from front to back.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.head)
    }

    /// Adds a new element at the back of the list, making it the new back.
    pub fn push_back(&mut self, element: T) {
        self.cursor_back().insert_after(element);
    }

    /// Adds a new element at the front of the list, making it the new front.
    pub fn push_front(&mut self, element: T) {
        self.cursor_front().insert_before(element);
    }

    /// Removes and returns the last element of the list, shifting the back
    /// element once towards the front.
    pub fn pop_back(&mut self) -> Option<T> {
        self.cursor_back().take()
    }

    /// Removes and returns the first element of the list, shifting the front
    /// element once towards the back.
    pub fn pop_front(&mut self) -> Option<T> {
        self.cursor_front().take()
    }

    /// Returns a reference to the front element if present, `None` otherwise.
    #[must_use]
    pub fn front(&self) -> Option<&T> {
        // SAFETY: the head points to a valid node at all times or is `None`.
        self.head.map(|head| unsafe { &head.as_ref().data })
    }

    /// Returns a reference to the back element if present, `None` otherwise.
    #[must_use]
    pub fn back(&self) -> Option<&T> {
        // SAFETY: the tail points to a valid node at all times or is `None`.
        self.tail.map(|tail| unsafe { &tail.as_ref().data })
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::iter::FromIterator<T> for LinkedList<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut list = Self::new();

        for elem in iter {
            list.push_back(elem);
        }

        list
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // `pop_front` would have worked here, but implies more machinery than
        // required. Following the links and deallocating is enough here.
        while let Ptr::Some(head) = self.head {
            // SAFETY: the head is valid at all times or is `None`.
            unsafe {
                self.head = head.as_ref().next;
                mem::drop(Box::from_raw(head.as_ptr()));
            }
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

/// Movable cursor over a [`LinkedList`] and enabling its modification,
/// therefore mutably borrowing it.
pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    node: Ptr<Node<T>>,
}

impl<T> Cursor<'_, T> {
    /// Takes a mutable reference to the current element.
    #[must_use]
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // SAFETY: the node is valid or is `None`.
        self.node.map(|mut node| unsafe { &mut node.as_mut().data })
    }

    /// Moves one position forward (towards the back) and returns a reference
    /// to the element at the new position.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        // SAFETY: `node` and `next` are valid or `None`.
        self.node.and_then(|node| unsafe {
            self.node = node.as_ref().next;
            self.node.map(|mut next| &mut next.as_mut().data)
        })
    }

    /// Moves one position backward (towards the front) and returns a reference
    /// to the element at the new position.
    pub fn prev(&mut self) -> Option<&mut T> {
        // SAFETY: `node` and `prev` are valid or `None`.
        self.node.and_then(|node| unsafe {
            self.node = node.as_ref().prev;
            self.node.map(|mut prev| &mut prev.as_mut().data)
        })
    }

    /// Advances the cursor `n` positions forwards, stopping if it reaches the
    /// back and returning whether it has done so or not.
    pub fn seek_forward(&mut self, n: usize) -> bool {
        (0..n).all(|_| self.next().is_some())
    }

    /// Advances the cursor `n` positions backwards, stopping if it reaches the
    /// front and returning whether it has done so or not.
    pub fn seek_backward(&mut self, n: usize) -> bool {
        (0..n).all(|_| self.prev().is_some())
    }

    /// Removes and returns the element at the current position and moves the
    /// cursor to the neighboring element that's closest to the back. This can
    /// be either the next (when there is one) or previous position (when not).
    pub fn take(&mut self) -> Option<T> {
        // SAFETY: nodes are valid or `None`; links are correctly placed.
        self.node.map(|node| unsafe {
            let node_ptr = node.as_ptr();

            match (*node_ptr).prev {
                // No previous: update the head.
                Ptr::None => {
                    self.list.head = (*node_ptr).next;
                }
                Ptr::Some(prev) => {
                    // Update link: jump one.
                    (*prev.as_ptr()).next = (*node_ptr).next;
                    // Go back once, only if no next: see below.
                    self.node = Ptr::Some(prev);
                }
            }

            match (*node_ptr).next {
                // No next: update the tail.
                Ptr::None => {
                    self.list.tail = (*node_ptr).prev;
                }
                Ptr::Some(next) => {
                    // Update link: jump one.
                    (*next.as_ptr()).prev = (*node_ptr).prev;
                    // Go forward once, prioritizing over going back.
                    self.node = Ptr::Some(next);
                }
            }

            // Reinstate invariant when only one last node.
            if (*node_ptr).prev.is_none() && (*node_ptr).next.is_none() {
                self.list.head = Ptr::None;
                self.list.tail = Ptr::None;
            }

            // Deallocation.
            self.list.len -= 1;
            Box::from_raw(node_ptr).data
        })
    }

    /// Adds a new element of `data` after the current position, making it the
    /// new back element if it must.
    #[allow(clippy::missing_panics_doc)]
    pub fn insert_after(&mut self, data: T) {
        let new_node = Node::alloc_new(data);

        match self.node {
            // Empty list: spray the new node everywhere.
            Ptr::None => {
                self.list.head = new_node;
                self.list.tail = new_node;
                self.node = new_node;
            }
            // SAFETY: `node` is valid because not `None` and links are OK.
            Ptr::Some(node) => unsafe {
                // `new_node` is never `None`: see `Node::alloc_new`.
                let new_node_ptr = new_node.unwrap().as_ptr();
                let node_ptr = node.as_ptr();

                match (*node_ptr).next {
                    // No next: update tail.
                    Ptr::None => {
                        self.list.tail = new_node;
                    }
                    // Update links to it when there is one.
                    Ptr::Some(next) => {
                        (*new_node_ptr).next = Ptr::Some(next);
                        (*next.as_ptr()).prev = new_node;
                    }
                }

                // Update links to current node.
                (*new_node_ptr).prev = self.node;
                (*node_ptr).next = new_node;
            },
        }

        self.list.len += 1;
    }

    /// Adds a new element of `data` before the current position, making it the
    /// new front element if it must.
    #[allow(clippy::missing_panics_doc)]
    pub fn insert_before(&mut self, data: T) {
        let new_node = Node::alloc_new(data);

        match self.node {
            // Empty list: spray the new node everywhere.
            Ptr::None => {
                self.list.head = new_node;
                self.list.tail = new_node;
                self.node = new_node;
            }
            // SAFETY: `node` is valid because not `None` and links are OK.
            Ptr::Some(node) => unsafe {
                // `new_node` is never `None`: see `Node::alloc_new`.
                let new_node_ptr = new_node.unwrap().as_ptr();
                let node_ptr = node.as_ptr();

                match (*node_ptr).prev {
                    // No next: update head.
                    Ptr::None => {
                        self.list.head = new_node;
                    }
                    // Update links to it when there is one.
                    Ptr::Some(prev) => {
                        (*new_node_ptr).prev = Ptr::Some(prev);
                        (*prev.as_ptr()).next = new_node;
                    }
                }

                // Update links to current node.
                (*new_node_ptr).next = self.node;
                (*node_ptr).prev = new_node;
            },
        }

        self.list.len += 1;
    }
}

/// Pure front-to-back borrowing iterator over a [`LinkedList`].
#[must_use]
pub struct Iter<'a, T> {
    node: Ptr<Node<T>>,
    _marker: PhantomData<&'a LinkedList<T>>,
}

impl<'a, T> Iter<'a, T> {
    /// Builds a new iterator starting at the given `node`.
    fn new(node: Ptr<Node<T>>) -> Self {
        Self {
            node,
            _marker: PhantomData {},
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        // SAFETY: `node` is valid or `None`.
        self.node.map(|node| unsafe { node.as_ref() }).map(|node| {
            self.node = node.next;
            &node.data
        })
    }
}

// These are tests for code that must not compile. They need to be here (or in lib.rs)
// because only doctests can use `compile_fail` without additional dependencies
// and doctests are ignored inside tests/doubly-linked-list.rs.

#[allow(unused)]
#[cfg(feature = "advanced")]
/// ```compile_fail
/// use doubly_linked_list::LinkedList;
/// trait AssertSend: Send {}
/// impl<T> AssertSend for LinkedList<T> {}
/// ```
pub struct IllegalSend;

#[allow(unused)]
#[cfg(feature = "advanced")]
/// ```compile_fail
/// use doubly_linked_list::LinkedList;
/// trait AssertSync: Sync {}
/// impl<T> AssertSync for LinkedList<T> {}
/// ```
pub struct IllegalSync;
