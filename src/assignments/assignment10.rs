//! Assignment 10: Iterators (2/2).
//!
//! The primary goal of this assignment is to get used to iterators.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-10.sh` works fine.
//! See `assignment10_grade.rs` and `/scripts/grade-10.sh` for the test script.

use itertools::*;

/// Returns the pairs of `(i, j)` where `i < j` and `inner[i] > inner[j]` in increasing order.
///
/// For example, the inversions of `[3, 5, 1, 2, 4]` is `[(0, 2), (0, 3), (1, 2), (1, 3), (1, 4)]` because as follows:
///
/// - `0 < 2`, `inner[0] = 3 > 1 = inner[2]`
/// - `0 < 3`, `inner[0] = 3 > 2 = inner[3]`
/// - `1 < 2`, `inner[1] = 5 > 1 = inner[2]`
/// - `1 < 3`, `inner[1] = 5 > 2 = inner[3]`
/// - `1 < 4`, `inner[1] = 5 > 4 = inner[4]`
///
/// Consult <https://en.wikipedia.org/wiki/Inversion_(discrete_mathematics)> for more details of inversion.
pub fn inversion<T: Ord>(inner: Vec<T>) -> Vec<(usize, usize)> {
    todo!()
}

/// Represents a node of tree data structure.
///
/// Consult <https://en.wikipedia.org/wiki/Tree_(data_structure)> for more details on tree data structure.
#[derive(Debug)]
pub enum Node<T> {
    /// Non-leaf node
    ///
    /// It contains `(the name of node, list of child nodes)`.
    NonLeaf((T, Vec<Node<T>>)),
    /// Leaf node
    ///
    /// It contains the name of node.
    Leaf(T),
}

/// Traverses the tree in preorder.
///
/// The algorithm for preorder traversal is as follows:
///
/// 1. Visit the root.
/// 2. If the root is a leaf node, end the traverse.
/// 3. If the root is a non-leaf node, traverse each subtree from the child nodes.
///
/// For example, the result of preorder traversal for the following tree
///
/// ```text
///     1
///    /|\
///   2 3 4
///  /|  /|\
/// 5 6 7 8 9
/// ```
///
/// which can be represented as
///
/// ```ignore
/// Node::NonLeaf((
///     1,
///     vec![
///         Node::NonLeaf((2, vec![Node::Leaf(5), Node::Leaf(6)])),
///         Node::Leaf(3),
///         Node::NonLeaf((4, vec![Node::Leaf(7), Node::Leaf(8), Node::Leaf(9)])),
///     ]
/// ))
/// ```
///
/// is `1 -> 2 -> 5 -> 6 -> 3 -> 4 -> 7 -> 8 -> 9`.
pub fn traverse_preorder<T>(root: Node<T>) -> Vec<T> {
    todo!()
}

/// File
#[derive(Debug)]
pub enum File {
    /// Directory
    ///
    /// It contains `(name of directory, list of files under the directory)`
    ///
    /// The size of a directory is the sum of the sizes of its sub-files.
    Directory(String, Vec<File>),

    /// Data
    ///
    /// It contains `(name of data, size of data)`
    Data(String, usize),
}

/// Given a file, summarize all subfiles and sizes in ascending order of size.
///
/// - Its behaviour is the same as the `du | sort -h` command on Linux.
/// - If the file size is the same, sort it by name.
/// - Assume that there are no duplicate file names.
///
/// # Example
///
/// Input:
///
/// ```txt
/// root (Directory)
/// |
/// |__a (Directory)
/// |  |__a1 (Data, size: 1)
/// |  |__a2 (Data, size: 3)
/// |
/// |__b (Directory)
/// |  |__b1 (Data, size: 3)
/// |  |__b2 (Data, size: 15)
/// |
/// |__c (Data, size: 8)
/// ```
///
/// Output: `[("a1", 1), ("a2", 3), ("b1", 3), ("a", 4), ("c", 8), ("b2", 15), ("b", 18), ("root", 30)]`
pub fn du_sort(root: &File) -> Vec<(&str, usize)> {
    todo!()
}
