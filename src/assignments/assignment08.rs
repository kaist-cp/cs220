//! Assignment 8: First-class functions.
//!
//! The primary goal of this assignment is to get used to first-class functions.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-08.sh` works fine.
//! See `assignment08_grade.rs` and `/scripts/grade-08.sh` for the test script.

/// Returns an anonymous function that applies the given function `f` for `n` times.
///
/// For instance, `repeat(3, f)(x)` roughly translates to `f(f(f(x)))`.
pub fn repeat<T, F: FnMut(T) -> T>(n: usize, mut f: F) -> impl FnMut(T) -> T {
    todo!()
}

/// Applies the given function `f` for `i` times for the `i`-th element of the given vector.
///
/// For instance, `funny_map(f, [v0, v1, v2, v3])` roughly translates to `[v0, f(v1), f(f(v2)), f(f(f(v3)))]`.
pub fn funny_map<T, F: Fn(T) -> T>(f: F, vs: Vec<T>) -> Vec<T> {
    todo!()
}

/// Either `T1`, or `T2`.
#[derive(Debug, PartialEq, Eq)]
pub enum Either2<T1, T2> {
    /// Case 1.
    Case1 {
        /// The inner value.
        inner: T1,
    },
    /// Case 1.
    Case2 {
        /// The inner value.
        inner: T2,
    },
}

impl<T1, T2> Either2<T1, T2> {
    /// Maps the inner value.
    ///
    /// If the inner value is case 1, apply `f1`, and if it is case 2, apply `f2`.
    pub fn map<U1, U2, F1, F2>(self, f1: F1, f2: F2) -> Either2<U1, U2>
    where
        F1: FnOnce(T1) -> U1,
        F2: FnOnce(T2) -> U2,
    {
        todo!()
    }
}
