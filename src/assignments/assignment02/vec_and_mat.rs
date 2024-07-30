//! Vector and matrices.
//!
//! You will implement simple operations on vectors and matrices.

use std::cmp::PartialEq;
use std::ops::Mul;

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

    /// Consult <https://www.mathsisfun.com/algebra/matrix-multiplying.html>
    fn mul(self, rhs: Mat2) -> Self::Output {
        todo!()
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    /// Multiplies the matrix by the vector.
    ///
    /// Consult <https://www.mathsisfun.com/algebra/matrix-multiplying.html>
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

/// Calculates the Fibonacci number. (We assume the absence of integer overflow.)
///
/// Consult <https://web.media.mit.edu/~holbrow/post/calculating-fibonacci-numbers-with-matrices-and-linear-algebra/> for matrix computation of Fibonacci numbers.
pub fn fibonacci(n: u64) -> u64 {
    (FIBONACCI_MAT.power(n) * FIBONACCI_VEC).get_upper()
}

/// 2x2 floating-point matrix of the following configuration:
///
/// a, b
/// c, d
#[derive(Debug, Clone, Copy)]
pub struct FMat2 {
    /// row 1, column 1
    pub a: f64,
    /// row 1, column 2
    pub b: f64,
    /// row 2, column 1
    pub c: f64,
    /// row 2, column 2
    pub d: f64,
}

impl FMat2 {
    /// Returns the inverse of the given matrix. (We assume the given matrix is always invertible.)
    /// HINT: <https://www.mathcentre.ac.uk/resources/uploaded/sigma-matrices7-2009-1.pdf>
    ///
    /// # Example
    ///
    /// ```ignore
    /// assert_eq!(
    ///     FMat2 { a: 1.0, b: 1.0, c: 2.0, d: 3.0 }.inverse(),
    ///     FMat2 { a: 3.0, b: -1.0, c: -2.0, d: 1.0 }
    /// );
    /// ```
    pub fn inverse(self) -> Self {
        todo!()
    }
}

// Equivalence between two floating-point matrices, as element-wise equivalence
impl PartialEq for FMat2 {
    fn eq(&self, other: &FMat2) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d
    }
}
