//! Assignment 7: Mastering advanced types (2/2).
//!
//! The primary goal of this assignment is to understand generics, traits, and lifetimes.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-07.sh` works fine.
//! See `assignment07_grade.rs` and `/scripts/grade-07.sh` for the test script.

use std::marker::PhantomData;

struct FindIter<'s, T: Eq> {
    query: &'s [T],
    base: &'s [T],
    curr: usize,
}

impl<T: Eq> Iterator for FindIter<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Returns an iterator over substring query indexes in the base.
pub fn find<'s, T: Eq>(query: &'s [T], base: &'s [T]) -> impl 's + Iterator<Item = usize> {
    FindIter {
        query,
        base,
        curr: 0,
    }
}

/// Represents transformation of value with type `T`.
pub trait Transform<T> {
    /// Transforms value.
    fn transform(&self, value: T) -> T;
}

/// Identity transformation.
#[derive(Debug, Clone, Copy)]
pub struct Identity;

impl<T> Transform<T> for Identity {
    fn transform(&self, value: T) -> T {
        // Just return the value.
        todo!()
    }
}

impl<T1, T2, Tr1: Transform<T1>, Tr2: Transform<T2>> Transform<(T1, T2)> for (Tr1, Tr2) {
    fn transform(&self, value: (T1, T2)) -> (T1, T2) {
        // Apply `Tr1` to the first element, apply `Tr2` to the second element.
        todo!()
    }
}

/// Custom transformation.
#[derive(Debug, Clone, Copy)]
pub struct Custom<T, F: Fn(T) -> T> {
    f: F,
    _marker: PhantomData<T>,
}

impl<T, F: Fn(T) -> T> From<F> for Custom<T, F> {
    fn from(f: F) -> Self {
        Self {
            f,
            _marker: PhantomData,
        }
    }
}

impl<T, F: Fn(T) -> T> Transform<T> for Custom<T, F> {
    fn transform(&self, value: T) -> T {
        // Apply `f` to the value.
        todo!()
    }
}

/// Repeats transformation for `n` times.
#[derive(Debug, Clone, Copy)]
pub struct Repeat<T, Tr: Transform<T>> {
    inner: Tr,
    n: u32,
    _marker: PhantomData<T>,
}

impl<T, Tr: Transform<T>> Repeat<T, Tr> {
    /// Creates a new repeat transformation.
    pub fn new(inner: Tr, n: u32) -> Self {
        Repeat {
            inner,
            n,
            _marker: PhantomData,
        }
    }
}

impl<T, Tr: Transform<T>> Transform<T> for Repeat<T, Tr> {
    fn transform(&self, mut value: T) -> T {
        // Apply `Tr` transformation `n` times.
        todo!()
    }
}

/// Repeats transformation until converges.
#[derive(Debug, Clone, Copy)]
pub struct RepeatUntilConverge<T: Eq, Tr: Transform<T>> {
    inner: Tr,
    _marker: PhantomData<T>,
}

impl<T: Clone + Eq, Tr: Transform<T>> RepeatUntilConverge<T, Tr> {
    /// Creates a new repeat transformation.
    pub fn new(inner: Tr) -> Self {
        RepeatUntilConverge {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<T: Clone + Eq, Tr: Transform<T>> Transform<T> for RepeatUntilConverge<T, Tr> {
    fn transform(&self, mut value: T) -> T {
        // Apply `Tr` transformation until there is no change.
        todo!()
    }
}
