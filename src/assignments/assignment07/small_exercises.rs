//! Implement functions using `Iterator` trait

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

/// Implement generic fibonacci iterator
struct FibIter<T> {
    // TODO: remove `_marker` and add necessary fields as you want
    _marker: std::marker::PhantomData<T>,
}

impl<T: std::ops::Add<Output = T> + Copy> FibIter<T> {
    fn new(first: T, second: T) -> Self {
        todo!()
    }
}

impl<T> Iterator for FibIter<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Returns and iterator over the generic fibonacci sequence starting from `first` and `second`.
/// This is a generic version of `fibonacci` function, which works for any types that implements
/// `std::ops::Add` trait.
pub fn fib<T>(first: T, second: T) -> impl Iterator<Item = T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    todo!("replace `std::iter::empty() with your own implementation`");
    std::iter::empty()
}

/// Endpoint of range, inclusive or exclusive.
#[derive(Debug)]
pub enum Endpoint {
    /// Inclusive endpoint
    Inclusive(isize),

    /// Exclusive endpoint
    Exclusive(isize),
}

struct RangeIter {
    // TODO: add necessary fields as you want
}

impl RangeIter {
    fn new(endpoints: (Endpoint, Endpoint), step: isize) -> Self {
        todo!()
    }
}

impl Iterator for RangeIter {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Returns an iterator over the range [left, right) with the given step.
pub fn range(left: Endpoint, right: Endpoint, step: isize) -> impl Iterator<Item = isize> {
    todo!("replace `std::iter::empty() with your own implementation`");
    std::iter::empty()
}

/// Write an iterator that returns all divisors of n in increasing order.
/// Assume n > 0.
///
/// Hint: trying all candidates from 1 to n will most likely time out!
/// To optimize it, make use of the following fact:
/// if x is a divisor of n that is greater than sqrt(n),
/// then n/x is a divisor of n that is smaller than sqrt(n).
struct Divisors {
    n: u64,
    // TODO: you may define additional fields here
}

impl Iterator for Divisors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Returns an iterator over the divisors of n.
pub fn divisors(n: u64) -> impl Iterator<Item = u64> {
    Divisors {
        n,
        // TODO: you may define additional fields here
    }
}
