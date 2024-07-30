//! Assignment 13: Parallelism.
//!
//! If you did well on assignment 09, you will do well on this assignment. Take it easy!

use rayon::prelude::*;

/// Returns the sum of `f(v)` for all element `v` the given array.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment13::small_exercises::*;
/// use rayon::iter::IntoParallelIterator;
///
/// assert_eq!(sigma_par([1, 2].into_par_iter(), |x| x + 2), 7);
/// assert_eq!(sigma_par([1, 2].into_par_iter(), |x| x * 4), 12);
/// ```
pub fn sigma_par<T, F: Fn(T) -> i64 + Sync + Send>(
    inner: impl ParallelIterator<Item = T>,
    f: F,
) -> i64 {
    todo!()
}

/// Alternate elements from three iterators until they have run out.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment13::small_exercises::*;
/// use rayon::iter::IntoParallelIterator;
///
/// assert_eq!(
///     interleave3_par([1, 2].into_par_iter(), [3, 4].into_par_iter(), [5, 6].into_par_iter()),
///     vec![1, 3, 5, 2, 4, 6]
/// );
/// ```
pub fn interleave3_par<T: Send>(
    list1: impl IndexedParallelIterator<Item = T>,
    list2: impl IndexedParallelIterator<Item = T>,
    list3: impl IndexedParallelIterator<Item = T>,
) -> Vec<T> {
    todo!()
}

/// Parallel vector addition
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment13::small_exercises::*;
///
/// let vec1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let vec2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let res = vec_add_par(&vec1, &vec2);
/// assert_eq!(res, vec![2.0, 4.0, 6.0, 8.0, 10.0]);
/// ```
pub fn vec_add_par(lhs: &[f64], rhs: &[f64]) -> Vec<f64> {
    todo!()
}

/// Parallel dot product of two arrays
///
/// You don't know how to calculate dot product?
/// See <https://mathinsight.org/dot_product_examples>
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment13::small_exercises::*;
///
/// let vec1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let vec2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let res = dot_product_par(&vec1, &vec2);
///
/// assert_eq!(res, 55.0);
/// ```
pub fn dot_product_par(lhs: &[f64], rhs: &[f64]) -> f64 {
    todo!()
}

/// Parallel Matrix multiplication
///
/// You don't know how to multiply matrix?
/// Quite simple! See <https://www.mathsisfun.com/algebra/matrix-multiplying.html>
///
/// Assume rhs is transposed
/// - lhs: (m, n)
/// - rhs: (p, n)
/// - output: (m, p)
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment13::small_exercises::*;
///
/// let mat1 = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
/// let mat2 = vec![
///     vec![7.0, 8.0, 9.0],
///     vec![10.0, 11.0, 12.0],
///     vec![13.0, 14.0, 15.0],
///     vec![16.0, 17.0, 18.0],
/// ];
/// let ans = vec![
///     vec![50.0, 68.0, 86.0, 104.0],
///     vec![122.0, 167.0, 212.0, 257.0],
/// ];
/// let res = matmul_par(&mat1, &mat2);
/// assert_eq!(ans, res);
/// ```
pub fn matmul_par(lhs: &[Vec<f64>], rhs: &[Vec<f64>]) -> Vec<Vec<f64>> {
    todo!()
}
