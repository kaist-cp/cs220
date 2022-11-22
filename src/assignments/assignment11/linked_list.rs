//! Singly linked list.
//!
//! Consult <https://doc.rust-lang.org/book/ch15-01-box.html>.

use std::fmt::Debug;

/// Node of the list.
#[derive(Debug)]
pub struct Node<T: Debug> {
    /// Value of current node.
    pub value: T,

    /// Pointer to the next node. If it is `None`, there is no next node.
    pub next: Option<Box<Node<T>>>,
}

impl<T: Debug> Node<T> {
    /// Creates a new node.
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

/// A singly-linked list.
#[derive(Debug)]
pub struct SinglyLinkedList<T: Debug> {
    /// Head node of the list. If it is `None`, the list is empty.
    head: Option<Node<T>>,
}

impl<T: Debug> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> SinglyLinkedList<T> {
    /// Creates a new list.
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Adds the given node to the front of the list.
    pub fn push_front(&mut self, value: T) {
        todo!()
    }

    /// Adds the given node to the back of the list.
    pub fn push_back(&mut self, value: T) {
        todo!()
    }

    /// Removes and returns the node at the front of the list.
    pub fn pop_front(&mut self) -> Option<T> {
        todo!()
    }

    /// Removes and returns the node at the back of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        todo!()
    }
}
