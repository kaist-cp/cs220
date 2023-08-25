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

    /// Create a new list from the given vector `vec`.
    pub fn from_vec(vec: Vec<T>) -> Self {
        todo!()
    }

    /// Convert the current list into a vector.
    pub fn as_vec(&self) -> Vec<T> {
        todo!()
    }

    /// Return the length (i.e., number of nodes) of the list.
    pub fn length(&self) -> usize {
        todo!()
    }

    /// Apply function `f` on every element of the list.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2]`, `f`: `|x| x + 1` ==> `[2, 3]`
    pub fn map<F: Fn(T) -> T>(&mut self, f: F) {
        todo!()
    }

    /// Insert given list `another` at the specified index `idx`.
    /// If `idx` is out-of-bound of `self`, append `another` at the end of `self`.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2]`, `another`: `[3, 4]`, `idx`: `1` ==> `[1, 3, 4, 2]`
    /// `self`: `[1, 2]`, `another`: `[3, 4]`, `idx`: `5` ==> `[1, 2, 3, 4]`
    pub fn insert(&mut self, another: &Self, idx: usize) {
        todo!()
    }

    /// Reverse the list in a chunk of size `n`.
    /// If `n == 0`, do nothing.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2, 3, 4, 5, 6, 7, 8, 9]`, `n`: `3`
    /// // each chunk of size `3`: `[1, 2, 3]`, `[4, 5, 6]`, `[7, 8, 9]`
    /// // reversed sequence of chunks: `[7, 8, 9]`, `[4, 5, 6]`, `[1, 2, 3]`
    /// ==> `[7, 8, 9, 4, 5, 6, 1, 2, 3]`,
    ///
    /// `self`: `[1, 2, 3, 4, 5, 6, 7, 8, 9]`, `n`: `4`
    /// // each chunk of size `4`: `[1, 2, 3, 4]`, `[5, 6, 7, 8]`, `[9]`
    /// // reversed sequence of chunks: `[9]`, `[5, 6, 7, 8]`, `[1, 2, 3, 4]`
    /// ==> `[9, 5, 6, 7, 8, 1, 2, 3, 4]`
    pub fn chunk_reverse(&mut self, n: usize) {
        todo!()
    }

    /// Apply given function `f` for each adjacent pair of elements in the list.
    /// If `self.length() < 2`, do nothing.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2, 3, 4]`, `f`: `|x, y| x + y`
    /// // each adjacent pair of elements: `(1, 2)`, `(2, 3)`, `(3, 4)`
    /// // apply `f` to each pair: `f(1, 2) == 3`, `f(2, 3) == 5`, `f(3, 4) == 7`
    /// ==> `[3, 5, 7]`
    pub fn pair_map<F: Fn(T, T) -> T>(&mut self, f: F) {
        todo!()
    }
}

// A list of lists.
impl<T: Debug> SinglyLinkedList<SinglyLinkedList<T>> {
    /// Flatten the list of lists into a single list.
    ///
    /// # Examples
    /// `self`: `[[1, 2, 3], [4, 5, 6], [7, 8]]`
    /// ==> `[1, 2, 3, 4, 5, 6, 7, 8]`
    pub fn flatten(self) -> SinglyLinkedList<T> {
        todo!()
    }
}
