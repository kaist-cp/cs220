#[cfg(test)]
mod test {
    use approx::*;
    use itertools::Itertools;
    use ndarray::prelude::*;
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;

    use crate::assignments::assignment09::matmul::*;

    #[test]
    fn vec_add_test() {
        let vec1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let vec2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let res = vec_add(&vec1, &vec2);
        assert_eq!(res, vec![2.0, 4.0, 6.0, 8.0, 10.0]);

        for _ in 0..5 {
            let vec1 = Array::random(500000, Uniform::new(0., 10.));
            let vec2 = Array::random(500000, Uniform::new(0., 10.));

            let res = vec_add(vec1.as_slice().unwrap(), vec2.as_slice().unwrap());

            let ans = vec1 + vec2;
            assert_eq!(Array::from_vec(res), ans);
        }
    }

    #[test]
    fn dot_product_test() {
        let vec1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let vec2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let res = dot_product(&vec1, &vec2);
        assert_eq!(res, 55.0);

        for _ in 0..5 {
            let vec1 = Array::random(1000000, Uniform::new(0., 10.));
            let vec2 = Array::random(1000000, Uniform::new(0., 10.));

            let res = dot_product(vec1.as_slice().unwrap(), vec2.as_slice().unwrap());
            let _res = relative_eq!(res, vec1.dot(&vec2), epsilon = f64::EPSILON);
        }
    }

    /// Reference: <https://github.com/rust-ndarray/ndarray/issues/590>
    /// Converts nested `Vec`s to a 2-D array by cloning the elements.
    ///
    /// **Panics** if the length of any axis overflows `isize`, if the
    /// size in bytes of all the data overflows `isize`, or if not all the
    /// rows have the same length.
    fn vec_to_array<T: Clone>(v: Vec<Vec<T>>) -> Array2<T> {
        if v.is_empty() {
            return Array2::from_shape_vec((0, 0), Vec::new()).unwrap();
        }
        let nrows = v.len();
        let ncols = v[0].len();
        let mut data = Vec::with_capacity(nrows * ncols);
        for row in &v {
            assert_eq!(row.len(), ncols);
            data.extend_from_slice(row);
        }
        Array2::from_shape_vec((nrows, ncols), data).unwrap()
    }

    #[test]
    fn matmul_test() {
        let mat1 = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let mat2 = vec![
            vec![7.0, 8.0, 9.0],
            vec![10.0, 11.0, 12.0],
            vec![13.0, 14.0, 15.0],
            vec![16.0, 17.0, 18.0],
        ];
        let ans = vec![
            vec![50.0, 68.0, 86.0, 104.0],
            vec![122.0, 167.0, 212.0, 257.0],
        ];
        let res = matmul(&mat1, &mat2);
        assert_eq!(ans, res);

        for _ in 0..5 {
            let mat1 = Array::random((500, 500), Uniform::new(0., 10.));
            let mat2 = Array::random((500, 500), Uniform::new(0., 10.));
            let ans = mat1.dot(&mat2);
            let mat2_transposed = mat2.t();

            // Run sequential matrix multiplication
            let res = matmul(
                mat1.axis_iter(Axis(0))
                    .map(|row| row.to_vec())
                    .collect::<Vec<_>>()
                    .as_slice(),
                mat2_transposed
                    .axis_iter(Axis(0))
                    .map(|row| row.to_vec())
                    .collect::<Vec<_>>()
                    .as_slice(),
            );

            // Check answer
            for it in ans.iter().zip(vec_to_array(res).iter()) {
                let (ans, res) = it;
                let _res = relative_eq!(ans, res);
            }
        }
    }
}
