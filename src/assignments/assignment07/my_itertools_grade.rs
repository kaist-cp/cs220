#[cfg(test)]
mod test {
    use itertools::Itertools;
    use ntest::assert_about_eq;

    use crate::assignments::assignment07::my_itertools::*;

    #[test]
    fn test_itertools() {
        assert_eq!(
            [10, 1, 1, 1, 2, 3, 4, 1, 3, 2]
                .into_iter()
                .my_chain(std::iter::repeat(100))
                .my_unique()
                .take(4)
                .collect::<Vec<_>>(),
            vec![10, 1, 2, 3]
        );

        assert_eq!(
            std::iter::repeat(5)
                .my_enumerate()
                .map(|(i, e)| { i * e })
                .take(5)
                .collect::<Vec<_>>(),
            vec![0, 5, 10, 15, 20]
        );

        assert_eq!(
            vec![0, 1, 10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,],
            [0, 1, 10].into_iter().my_chain(0..10).collect::<Vec<_>>(),
        );

        let it = || (1..=5).cycle().my_zip((1..=3).cycle()).map(|(x, y)| x * y);
        let take15 = vec![
            1,  // 1 * 1,
            4,  // 2 * 2,
            9,  // 3 * 3,
            4,  // 4 * 1,
            10, // 5 * 2,
            3,  // 1 * 3,
            2,  // 2 * 1,
            6,  // 3 * 2,
            12, // 4 * 3,
            5,  // 5 * 1,
            2,  // 1 * 2,
            6,  // 2 * 3,
            3,  // 3 * 1,
            8,  // 4 * 2,
            15, // 5 * 3,
        ];

        assert_eq!(
            // 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5
            // 1 2 3 1 2 3 1 2 3 1 2 3 1 2 3
            it().take(15).collect::<Vec<_>>(),
            take15
        );

        assert_eq!(
            it().take(15).my_fold(0, |elt, acc| elt + acc),
            take15.iter().sum()
        );
    }
}
