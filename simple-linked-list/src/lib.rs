use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    node: Option<Box<Node<T>>>,
    len: usize,
}

struct Node<T> {
    head: T,
    tail: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { node: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, element: T) {
        self.len = self.len.saturating_add(1);
        self.node = Some(Box::new(Node {
            head: element,
            tail: self.node.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.len = self.len.saturating_sub(1);
        self.node.take().map(|node| {
            self.node = node.tail;
            node.head
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.node.as_ref().map(|node| &node.head)
    }

    pub fn rev(mut self) -> Self {
        let mut ll = Self::new();

        while !self.is_empty() {
            ll.push(self.pop().unwrap());
        }

        ll
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ll = Self::new();

        for x in iter {
            ll.push(x);
        }

        ll
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = Vec::<T>::with_capacity(self.len);
        let mut ll = self.rev();

        while !ll.is_empty() {
            vec.push(ll.pop().unwrap());
        }

        vec
    }
}
