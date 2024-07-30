//! Semiring

use std::collections::HashMap;
use std::fmt::Debug;

use itertools::Itertools;

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
/// ```ignore
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

impl<C: Semiring> Polynomial<C> {
    /// Constructs polynomial `x`.
    pub fn x() -> Self {
        todo!()
    }

    /// Evaluates the polynomial with the given value.
    pub fn eval(&self, value: C) -> C {
        todo!()
    }

    /// Constructs polynomial `ax^n`.
    pub fn term(a: C, n: u64) -> Self {
        todo!()
    }
}

impl<C: Semiring> From<C> for Polynomial<C> {
    fn from(value: C) -> Self {
        todo!()
    }
}

/// Given a string `s`, parse it into a `Polynomial<C>`.
/// You may assume that `s` follows the criteria below.
/// Therefore, you do not have to return `Err`.
///
/// Assumptions:
/// - Each term is separated by ` + `.
/// - Each term is one of the following form: `a`, `x`, `ax`, `x^n`, and `ax^n`, where `a` is a
///   `usize` number and `n` is a `u64` number. This `a` should then be converted to a `C` type.
/// - In `a`, it is guaranteed that `a >= 1`.
/// - In `ax` and `ax^n`, it is guaranteed that `a >= 2`.
/// - In `x^n` and `ax^n`, it is guaranteed that `n >= 2`.
/// - All terms have unique degrees.
///
/// Consult `assignment06/grade.rs` for example valid strings.
///
/// Hint: `.split`, `.parse`, and `Polynomial::term`
impl<C: Semiring> std::str::FromStr for Polynomial<C> {
    type Err = (); // Ignore this for now...

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
