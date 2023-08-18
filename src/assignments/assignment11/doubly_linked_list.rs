//! Doubly Linked List.
//!
//! Refer `doubly_linked_list_grade.rs` for test cases.

use std::{cell::RefCell, fmt::Debug, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// Node of a doubly-linked list.
#[derive(Debug)]
pub struct Node<T: Debug + Clone> {
    /// Value of current node.
    value: RefCell<T>,

    /// Pointer to the next node. If it is `None`, there is no next node.
    next: Link<T>,

    /// Pointer to the previous node. If it is `None`, there is no previous node.
    prev: Link<T>,
}

impl<T: Debug + Clone> Node<T> {
    /// Creates a new node.
    fn new(value: T) -> Self {
        Self {
            value: RefCell::new(value),
            next: None,
            prev: None,
        }
    }

    /// Fetch the value contained in node
    pub fn get(&self) -> T {
        self.value.borrow().clone()
    }

    /// Replace the data contained in the node
    pub fn replace(&self, new_value: T) -> T {
        self.value.replace(new_value)
    }

    /// Fetch previous node
    pub fn prev(&self) -> Link<T> {
        self.prev.clone()
    }

    /// Fetch next node
    pub fn next(&self) -> Link<T> {
        self.next.clone()
    }
}

/// A doubly-linked list.
#[derive(Default, Debug)]
pub struct DoublyLinkedList<T: Debug + Clone> {
    /// Head node of the list. If it is `None`, the list is empty.
    head: Link<T>,

    /// Tail node of the list. If it is `None`, the list is empty.
    tail: Link<T>,
}

impl<T: Debug + Clone> DoublyLinkedList<T> {
    /// Creates a new list.
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
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

impl<T: Debug + Clone> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.borrow_mut().prev.take();
            self.head = node.borrow_mut().next.take();
        }
        let _unused = self.tail.take();
    }
}
