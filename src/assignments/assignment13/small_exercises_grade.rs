#[cfg(test)]
mod test {
    use std::hint;
    use std::time::Instant;

    use approx::*;
    use itertools::Itertools;
    use ndarray::prelude::*;
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;
    use ntest::assert_about_eq;
    use rayon::prelude::IntoParallelIterator;

    use crate::assignments::assignment09::matmul::*;
    use crate::assignments::assignment13::small_exercises::*;

    #[test]
    fn test_sigma_par() {
        assert_eq!(sigma_par([].into_par_iter(), |x: i64| x * 2), 0);
        assert_eq!(sigma_par([1].into_par_iter(), |x| x * 3), 3);
        assert_eq!(sigma_par([1, 2].into_par_iter(), |x| x + 2), 7);
        assert_eq!(sigma_par([1, 2].into_par_iter(), |x| x * 4), 12);
        assert_eq!(sigma_par([1, 2, 3].into_par_iter(), |x| x * 5), 30);

        assert_eq!(
            sigma_par([-1.2, 3.0, 4.2, 5.8].into_par_iter(), |x: f64| x.floor()
                as i64),
            10
        );
        assert_eq!(
            sigma_par([-1.2, 3.0, 4.2, 5.8].into_par_iter(), |x: f64| x.ceil()
                as i64),
            13
        );
        assert_eq!(
            sigma_par([-1.2, 3.0, 4.2, 5.8].into_par_iter(), |x: f64| x.round()
                as i64),
            12
        );

        assert_eq!(
            sigma_par(["Hello,", "World!"].into_par_iter(), |x| x.len() as i64),
            12
        );
    }

    #[test]
    fn test_interleave3_par() {
        assert_eq!(
            interleave3_par(
                [1, 2].into_par_iter(),
                [3, 4].into_par_iter(),
                [5, 6].into_par_iter()
            ),
            vec![1, 3, 5, 2, 4, 6]
        );

        assert_eq!(
            interleave3_par(
                [1, 2, 3].into_par_iter(),
                [4, 5, 6].into_par_iter(),
                [7, 8, 9].into_par_iter()
            ),
            vec![1, 4, 7, 2, 5, 8, 3, 6, 9]
        );

        assert_eq!(
            interleave3_par(
                ["a", "b", "c"].into_par_iter(),
                ["d", "e", "f"].into_par_iter(),
                ["g", "h", "i"].into_par_iter()
            )
            .into_iter()
            .collect::<String>(),
            "adgbehcfi"
        );
    }

    #[test]
    fn vec_add_test() {
        let vec1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let vec2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let res = vec_add(&vec1, &vec2);
        assert_eq!(res, vec![2.0, 4.0, 6.0, 8.0, 10.0]);

        for _ in 0..5 {
            let vec1 = hint::black_box(Array::random(5_000_000, Uniform::new(0., 10.)));
            let vec2 = hint::black_box(Array::random(5_000_000, Uniform::new(0., 10.)));

            let now_seq = Instant::now();
            let res_seq = hint::black_box(vec_add(
                hint::black_box(vec1.as_slice().unwrap()),
                hint::black_box(vec2.as_slice().unwrap()),
            ));
            let elapsed_seq = now_seq.elapsed();

            let now_par = Instant::now();
            let res_par = hint::black_box(vec_add_par(
                hint::black_box(vec1.as_slice().unwrap()),
                hint::black_box(vec2.as_slice().unwrap()),
            ));
            let elapsed_par = now_par.elapsed();

            let ans = vec1 + vec2;
            assert_eq!(Array::from_vec(res_seq), ans);
            assert_eq!(Array::from_vec(res_par), ans);
            assert!(elapsed_par < elapsed_seq);
        }
    }

    #[test]
    fn dot_product_test() {
        let vec1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let vec2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let res_seq = dot_product(&vec1, &vec2);
        let res_par = dot_product_par(&vec1, &vec2);
        assert_eq!(res_seq, 55.0);
        assert_eq!(res_par, 55.0);

        for _ in 0..5 {
            let vec1 = Array::random(10_000_000, Uniform::new(0., 10.));
            let vec2 = Array::random(10_000_000, Uniform::new(0., 10.));

            let now_seq = Instant::now();
            let res_seq = hint::black_box(dot_product(
                hint::black_box(vec1.as_slice().unwrap()),
                hint::black_box(vec2.as_slice().unwrap()),
            ));
            let elapsed_seq = now_seq.elapsed();

            let now_par = Instant::now();
            let res_par = hint::black_box(dot_product_par(
                hint::black_box(vec1.as_slice().unwrap()),
                hint::black_box(vec2.as_slice().unwrap()),
            ));
            let elapsed_par = now_par.elapsed();

            let correct = vec1.dot(&vec2);
            assert_about_eq!(res_seq, correct, 1.0e-4);
            assert_about_eq!(res_par, correct, 1.0e-4);

            assert!(
                elapsed_par < elapsed_seq,
                "Sequential: {elapsed_seq:?}, Parallel: {elapsed_par:?}"
            );
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
        let res_seq = matmul(&mat1, &mat2);
        let res_par = matmul_par(&mat1, &mat2);
        assert_eq!(ans, res_seq);
        assert_eq!(ans, res_par);

        for _ in 0..5 {
            let mat1 = Array::random((500, 500), Uniform::new(0., 10.));
            let mat2 = Array::random((500, 500), Uniform::new(0., 10.));
            let ans = mat1.dot(&mat2);
            let mat2_transposed = mat2.t();

            // Run sequential matrix multiplication
            let now_seq = Instant::now();
            let res_seq = hint::black_box(matmul(
                hint::black_box(
                    mat1.axis_iter(Axis(0))
                        .map(|row| row.to_vec())
                        .collect::<Vec<_>>()
                        .as_slice(),
                ),
                hint::black_box(
                    mat2_transposed
                        .axis_iter(Axis(0))
                        .map(|row| row.to_vec())
                        .collect::<Vec<_>>()
                        .as_slice(),
                ),
            ));
            let elapsed_seq = now_seq.elapsed();

            // Run parallel matrix multiplication
            let now_par = Instant::now();
            let res_par = hint::black_box(matmul_par(
                hint::black_box(
                    mat1.axis_iter(Axis(0))
                        .map(|row| row.to_vec())
                        .collect::<Vec<_>>()
                        .as_slice(),
                ),
                hint::black_box(
                    mat2_transposed
                        .axis_iter(Axis(0))
                        .map(|row| row.to_vec())
                        .collect::<Vec<_>>()
                        .as_slice(),
                ),
            ));
            let elapsed_par = now_par.elapsed();

            // Check answer
            for it in ans.iter().zip(vec_to_array(res_seq).iter()) {
                let (ans, seq) = it;
                let _res = relative_eq!(ans, seq);
            }
            for it in ans.iter().zip(vec_to_array(res_par).iter()) {
                let (ans, par) = it;
                let _res = relative_eq!(ans, par);
            }

            // Check time
            // println!("Sequential: {:?}", elapsed_seq);
            // println!("Parallel: {:?}", elapsed_par);
            assert!(elapsed_par < elapsed_seq);
        }
    }
}
