//! Small problems.

use std::iter;

const FAHRENHEIT_OFFSET: f64 = 32.0;
const FAHRENHEIT_SCALE: f64 = 5.0 / 9.0;

/// Converts Fahrenheit to Celsius temperature degree.
pub fn fahrenheit_to_celsius(degree: f64) -> f64 {
    todo!()
}

/// Capitalizes English alphabets (leaving the other characters intact).
pub fn capitalize(input: String) -> String {
    todo!()
}

/// Returns the sum of the given array. (We assume the absence of integer overflow.)
pub fn sum_array(input: &[u64]) -> u64 {
    todo!()
}

/// Given a non-negative integer, say `n`, return the smallest integer of the form `3^m` that's
/// greater than or equal to `n`.
///
/// For instance, up3(6) = 9, up3(9) = 9, up3(10) = 27. (We assume the absence of integer overflow.)
pub fn up3(n: u64) -> u64 {
    todo!()
}

/// Returns the greatest common divisor (GCD) of two non-negative integers. (We assume the absence
/// of integer overflow.)
pub fn gcd(lhs: u64, rhs: u64) -> u64 {
    todo!()
}

/// Returns the array of nC0, nC1, nC2, ..., nCn, where nCk = n! / (k! * (n-k)!). (We assume the
/// absence of integer overflow.)
///
/// Consult <https://en.wikipedia.org/wiki/Pascal%27s_triangle> for computation of binomial
/// coefficients without integer overflow.
pub fn chooses(n: u64) -> Vec<u64> {
    todo!()
}

/// Returns the "zip" of two vectors.
///
/// For instance, `zip(vec![1, 2, 3], vec![4, 5])` equals to `vec![(1, 4), (2, 5)]`. Here, `3` is
/// ignored because it doesn't have a partner.
pub fn zip(lhs: Vec<u64>, rhs: Vec<u64>) -> Vec<(u64, u64)> {
    todo!()
}
