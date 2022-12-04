#[cfg(test)]
mod test {
    use super::super::assignment13::*;
    use rayon::prelude::IntoParallelIterator;

    #[test]
    fn test_sigma() {
        assert_eq!(sigma([].into_par_iter(), |x: i64| x * 2), 0);
        assert_eq!(sigma([1].into_par_iter(), |x| x * 3), 3);
        assert_eq!(sigma([1, 2].into_par_iter(), |x| x + 2), 7);
        assert_eq!(sigma([1, 2].into_par_iter(), |x| x * 4), 12);
        assert_eq!(sigma([1, 2, 3].into_par_iter(), |x| x * 5), 30);

        assert_eq!(
            sigma([-1.2, 3.0, 4.2, 5.8].into_par_iter(), |x: f64| x.floor()
                as i64),
            10
        );
        assert_eq!(
            sigma([-1.2, 3.0, 4.2, 5.8].into_par_iter(), |x: f64| x.ceil()
                as i64),
            13
        );
        assert_eq!(
            sigma([-1.2, 3.0, 4.2, 5.8].into_par_iter(), |x: f64| x.round()
                as i64),
            12
        );

        assert_eq!(
            sigma(["Hello,", "World!"].into_par_iter(), |x| x.len() as i64),
            12
        );
    }

    #[test]
    fn test_interleave3() {
        assert_eq!(
            interleave3(
                [1, 2].into_par_iter(),
                [3, 4].into_par_iter(),
                [5, 6].into_par_iter()
            ),
            vec![1, 3, 5, 2, 4, 6]
        );

        assert_eq!(
            interleave3(
                [1, 2, 3].into_par_iter(),
                [4, 5, 6].into_par_iter(),
                [7, 8, 9].into_par_iter()
            ),
            vec![1, 4, 7, 2, 5, 8, 3, 6, 9]
        );

        assert_eq!(
            interleave3(
                ["a", "b", "c"].into_par_iter(),
                ["d", "e", "f"].into_par_iter(),
                ["g", "h", "i"].into_par_iter()
            )
            .into_iter()
            .collect::<String>(),
            "adgbehcfi"
        );
    }
}
