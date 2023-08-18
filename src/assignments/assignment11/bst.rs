//! Binary Search Tree
//!
//! Refer `bst_grade.rs` for test cases.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::{Rc, Weak};

/// Node struct of tree
#[derive(Debug, Clone)]
struct Node<T>
where
    T: Ord,
{
    value: T,
    parent: Option<Weak<RefCell<Node<T>>>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            parent: None,
            left: None,
            right: None,
        }))
    }

    fn with_parent(value: T, parent: Weak<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            parent: Some(parent),
            left: None,
            right: None,
        }))
    }

    /// Minimum node starting from cursor
    fn min_node(mut cursor: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        todo!();
    }

    /// Upgraded parent node.
    /// `None` if the node has no parent.
    fn parent(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.parent.as_ref().and_then(|p| p.upgrade())
    }
}

/// Binary Search Tree
#[derive(Debug)]
pub struct Tree<T>
where
    T: Ord,
{
    root: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T: Ord> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Tree<T>
where
    T: Ord,
{
    /// New tree
    pub fn new() -> Tree<T> {
        Tree { root: None, len: 0 }
    }

    /// Length of the tree
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Check if the tree contains the value.
    pub fn contains(&self, value: &T) -> bool {
        if let Some(mut cursor) = self.root.clone() {
            todo!();
        } else {
            false
        }
    }

    /// Insert the value into the tree.
    /// If there was no equal value in the tree, it returns `true`.
    /// Otherwise, it returns `false`.
    pub fn insert(&mut self, value: T) -> bool {
        self.len += 1;
        if let Some(mut cursor) = self.root.clone() {
            todo!();
        } else {
            self.root = Some(Node::new(value));
            true
        }
    }

    /// Remove the node from the tree.
    /// Returns the value of the removed node.
    fn remove_node(&mut self, mut node: Rc<RefCell<Node<T>>>) -> T {
        todo!();
    }

    /// Remove the value from the tree.
    /// If there is an equal value in the tree, it returns `true`.
    /// Otherwise, it returns `false`.
    pub fn remove(&mut self, value: &T) -> Option<T> {
        let res = if let Some(root) = self.root.clone() {
            let mut cursor = root;

            loop {
                let mut cursor_ref = cursor.deref().borrow_mut();
                let child = match value.cmp(&cursor_ref.value) {
                    Ordering::Less => cursor_ref.left.clone(),
                    Ordering::Greater => cursor_ref.right.clone(),
                    Ordering::Equal => {
                        drop(cursor_ref);
                        break Some(self.remove_node(cursor));
                    }
                };

                if let Some(child) = child {
                    drop(cursor_ref);
                    cursor = child;
                } else {
                    break None;
                }
            }
        } else {
            None
        };

        self.len -= res.is_some() as usize;
        res
    }
}

impl<T> Clone for Tree<T>
where
    T: Ord + Clone,
{
    fn clone(&self) -> Self {
        Tree {
            root: self.root.clone(),
            len: self.len,
        }
    }
}

/// IntoIterator for Tree
#[derive(Debug)]
pub struct IntoIter<T>
where
    T: Ord,
{
    tree: Tree<T>,
    len: usize,
}

impl<T> IntoIterator for Tree<T>
where
    T: Ord,
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.len;
        IntoIter { tree: self, len }
    }
}

impl<T> Iterator for IntoIter<T>
where
    T: Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}
