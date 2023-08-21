use std::vec;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    /// The underlining buffer.
    array: Vec<Option<T>>,
    /// The index in the array where the oldest element resides.
    first: usize,
    /// The index in the array where the newest element resides.
    last: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

/// Clone is necessary in order to be able to easily fill with `None`.
impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            // Fill in order to enforce valid indices and allocate only once.
            array: vec::from_elem(None, capacity),
            first: 0,
            last: 0,
        }
    }

    /// Checked version of `overwrite`.
    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.array[self.last].is_some() {
            Err(Error::FullBuffer)
        } else {
            self.overwrite(element);
            Ok(())
        }
    }

    /// Reads when possible and takes the element out.
    pub fn read(&mut self) -> Result<T, Error> {
        if self.array[self.first].is_none() {
            Err(Error::EmptyBuffer)
        } else {
            let element = self.array[self.first].take().unwrap();
            self.first = (self.first + 1) % self.array.len();
            Ok(element)
        }
    }

    /// Clears the buffer withour re-allocating, hence the necessary unstable feature.
    pub fn clear(&mut self) {
        self.first = 0;
        self.last = 0;
        self.array.fill(None);
    }

    /// Adds a new element to the buffer by possibly replacing some previous element.
    pub fn overwrite(&mut self, element: T) {
        let next_first = (self.first + 1) % self.array.len();

        // Update the oldest element's index when having looped back.
        if self.last == self.first && self.array[next_first].is_some() {
            self.first = next_first;
        }

        self.array[self.last] = Some(element);
        self.last = (self.last + 1) % self.array.len();
    }
}
