//! Assignment 2: Mastering common programming concepts (1/2).
//!
//! The primary goal of this assignment is to re-learn the common programming concepts in Rust, especially those in the Rust Book chapters 3 and 5.
//! Please make sure you're comfortable with the concepts to proceed on to the next assignments.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-02.sh` works fine.
//! See `assignment02_grade.rs` and `/scripts/grade-02.sh` for the test script.

use std::ops::Mul;

const FAHRENHEIT_OFFSET: f64 = 32.0;
const FAHRENHEIT_SCALE: f64 = 5.0 / 9.0;

/// Converts Fahrenheit to Celsius temperature degree.
pub(crate) fn fahrenheit_to_celsius(degree: f64) -> f64 {
    todo!()
}

/// Captializes English alphabets (leaving the other characters intact).
pub(crate) fn capitalize(input: String) -> String {
    todo!()
}

/// Returns the sume of the given array. (We assume the absence of integer overflow.)
pub(crate) fn sum_array(input: &[u64]) -> u64 {
    todo!()
}

/// Given a non-negative integer, say `n`, return the smallest integer of the form `3^m` that's greater than or equal to `n`.
///
/// For instance, up3(6) = 9, up3(9) = 9, up3(10) = 27. (We assume the absence of integer overflow.)
pub(crate) fn up3(n: u64) -> u64 {
    todo!()
}

/// Returns the greatest common divisor (GCD) of two non-negative integers. (We assume the absence of integer overflow.)
pub(crate) fn gcd(lhs: u64, rhs: u64) -> u64 {
    todo!()
}

/// Returns the array of nC0, nC1, nC2, ..., nCn, where nCk = n! / (k! * (n-k)!).
pub(crate) fn chooses(n: u64) -> Vec<u64> {
    todo!()
}

/// Returns the "zip" of two vectors.
///
/// For instance, `zip(vec![1, 2, 3], vec![4, 5])` equals to `vec![(1, 4), (2, 5)]`.
/// Here, `3` is ignored because it doesn't have a partner.
pub(crate) fn zip(lhs: Vec<u64>, rhs: Vec<u64>) -> Vec<(u64, u64)> {
    todo!()
}

/// 2x2 matrix of the following configuration:
///
/// a, b
/// c, d
#[derive(Debug, Clone, Copy)]
struct Mat2 {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

/// 2x1 matrix of the following configuration:
///
/// a
/// b
#[derive(Debug, Clone, Copy)]
struct Vec2 {
    a: u64,
    b: u64,
}

impl Mat2 {
    /// Creates an identity matrix.
    fn new() -> Self {
        Self {
            a: 1,
            b: 0,
            c: 0,
            d: 1,
        }
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Mat2;

    fn mul(self, rhs: Mat2) -> Self::Output {
        todo!()
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        todo!()
    }
}

impl Mat2 {
    /// Calculates the power of matrix.
    fn power(self, power: u64) -> Mat2 {
        todo!()
    }
}

impl Vec2 {
    /// Gets the upper value of vector.
    fn get_upper(self) -> u64 {
        todo!()
    }
}

/// The matrix used for calculating Fibonacci numbers.
const FIBONACCI_MAT: Mat2 = Mat2 {
    a: 1,
    b: 1,
    c: 1,
    d: 0,
};

/// The vector used for calculating Fibonacci numbers.
const FIBONACCI_VEC: Vec2 = Vec2 { a: 1, b: 0 };

/// Calculates the Fibonacci number.
///
/// Consult <https://web.media.mit.edu/~holbrow/post/calculating-fibonacci-numbers-with-matrices-and-linear-algebra/> for matrix computation of Fibonacci numbers.
pub(crate) fn fibonacci(n: u64) -> u64 {
    (FIBONACCI_MAT.power(n) * FIBONACCI_VEC).get_upper()
}

/// Writes down the lyrics of "twelve days of christmas".
///
/// Hint: Google the song title for lyrics and look at the test code for the expected result.
pub(crate) fn twelve_days_of_christmas_lyrics() -> String {
    todo!()
}
