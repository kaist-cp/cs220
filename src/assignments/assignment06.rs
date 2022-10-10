//! Assignment 6: Mastering advanced types (1/2).
//!
//! The primary goal of this assignment is to understand generics, traits, and lifetimes.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-06.sh` works fine.
//! See `assignment06_grade.rs` and `/scripts/grade-06.sh` for the test script.

use std::{collections::HashMap, fmt::Debug};

/// Semiring.
///
/// Consult <https://en.wikipedia.org/wiki/Semiring>.
pub trait Semiring: Debug + Clone + PartialEq {
    /// Additive identity.
    fn zero() -> Self;
    /// Multiplicative identity.
    fn one() -> Self;
    /// Addition operation.
    fn add(&self, rhs: &Self) -> Self;
    /// Multiplication operation.
    fn mul(&self, rhs: &Self) -> Self;
}

/// Converts integer to semiring value.
pub fn from_usize<T: Semiring>(value: usize) -> T {
    let mut result = T::zero();
    let one = T::one();

    for _ in 0..value {
        result = T::add(&result, &one);
    }

    result
}

impl Semiring for u64 {
    fn zero() -> Self {
        todo!()
    }

    fn one() -> Self {
        todo!()
    }

    fn add(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn mul(&self, rhs: &Self) -> Self {
        todo!()
    }
}

impl Semiring for i64 {
    fn zero() -> Self {
        todo!()
    }

    fn one() -> Self {
        todo!()
    }

    fn add(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn mul(&self, rhs: &Self) -> Self {
        todo!()
    }
}

impl Semiring for f64 {
    fn zero() -> Self {
        todo!()
    }

    fn one() -> Self {
        todo!()
    }

    fn add(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn mul(&self, rhs: &Self) -> Self {
        todo!()
    }
}

/// Polynomials with coefficient in `C`.
///
/// For example, polynomial `x^2 + 5x + 6` is represented in `Polynomial<u64>` as follows:
///
/// ```
/// Polynomial {
///     coefficients: {
///         2: 1,
///         1: 5,
///         0: 6,
///     },
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial<C: Semiring> {
    coefficients: HashMap<u64, C>,
}

impl<C: Semiring> Semiring for Polynomial<C> {
    fn zero() -> Self {
        todo!()
    }

    fn one() -> Self {
        todo!()
    }

    fn add(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn mul(&self, rhs: &Self) -> Self {
        todo!()
    }
}

impl<C: Semiring> From<C> for Polynomial<C> {
    fn from(value: C) -> Self {
        todo!()
    }
}

impl<C: Semiring> Polynomial<C> {
    /// Constructs polynomial `x`.
    pub fn x() -> Self {
        todo!()
    }

    /// Evaluates the polynomial with the given value.
    pub fn eval(&self, value: C) -> C {
        todo!()
    }
}
