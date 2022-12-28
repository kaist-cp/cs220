#[cfg(test)]
mod test {
    use super::super::assignment09::*;

    #[test]
    fn test_is_fibonacci() {
        assert_eq!(is_fibonacci([1, 1, 2, 3, 5, 8, 13].into_iter()), true);
        assert_eq!(is_fibonacci([1, 1, 2, 3, 5, 8, 14].into_iter()), false);
        assert_eq!(is_fibonacci([2, 4, 6, 10, 16, 26].into_iter()), true);
        assert_eq!(is_fibonacci([4, 9, 13, 22, 35].into_iter()), true);
        assert_eq!(is_fibonacci([0, 0, 0, 0, 0].into_iter()), true);
        assert_eq!(is_fibonacci([1, 1].into_iter()), true);
        assert_eq!(is_fibonacci([1].into_iter()), true);
        assert_eq!(is_fibonacci([].into_iter()), true);

        assert_eq!(is_fibonacci([1, 1, 2, 2, 3, 3].into_iter()), false);
        assert_eq!(is_fibonacci([0, 0, 0, 0, 1].into_iter()), false);
        assert_eq!(is_fibonacci([1, 1, 1, 1].into_iter()), false);
        assert_eq!(is_fibonacci([4, 3, 2, 1].into_iter()), false);
    }

    #[test]
    fn test_sigma() {
        assert_eq!(sigma([].into_iter(), |x: i64| x * 2), 0);
        assert_eq!(sigma([1].into_iter(), |x| x * 3), 3);
        assert_eq!(sigma([1, 2].into_iter(), |x| x + 2), 7);
        assert_eq!(sigma([1, 2].into_iter(), |x| x * 4), 12);
        assert_eq!(sigma([1, 2, 3].into_iter(), |x| x * 5), 30);

        assert_eq!(
            sigma([-1.2, 3.0, 4.2, 5.8].into_iter(), |x: f64| x.floor() as i64),
            10
        );
        assert_eq!(
            sigma([-1.2, 3.0, 4.2, 5.8].into_iter(), |x: f64| x.ceil() as i64),
            13
        );
        assert_eq!(
            sigma([-1.2, 3.0, 4.2, 5.8].into_iter(), |x: f64| x.round() as i64),
            12
        );

        assert_eq!(
            sigma(["Hello,", "World!"].into_iter(), |x| x.len() as i64),
            12
        );
    }

    #[test]
    fn test_interleave3() {
        assert_eq!(
            interleave3([1, 2].into_iter(), [3, 4].into_iter(), [5, 6].into_iter()),
            vec![1, 3, 5, 2, 4, 6]
        );

        assert_eq!(
            interleave3(
                [1, 2, 3].into_iter(),
                [4, 5, 6].into_iter(),
                [7, 8, 9].into_iter()
            ),
            vec![1, 4, 7, 2, 5, 8, 3, 6, 9]
        );

        assert_eq!(
            interleave3(
                ["a", "b", "c"].into_iter(),
                ["d", "e", "f"].into_iter(),
                ["g", "h", "i"].into_iter()
            )
            .into_iter()
            .collect::<String>(),
            "adgbehcfi"
        );
    }

    #[test]
    fn test_k_smallest_man() {
        assert_eq!(
            k_smallest_mean(vec![1, 3, 2].into_iter(), 2),
            ((1 + 2) as f64 / 2.0)
        );
        assert_eq!(
            k_smallest_mean(vec![5, 3, 7, 7].into_iter(), 2),
            ((3 + 5) as f64 / 2.0)
        );
        assert_eq!(
            k_smallest_mean(vec![7, 5, 3, 6].into_iter(), 3),
            ((3 + 5 + 6) as f64 / 3.0)
        );
        assert_eq!(
            k_smallest_mean(vec![1, 3, 2, 4, 4, 5, 6].into_iter(), 3),
            ((1 + 2 + 3) as f64 / 3.0)
        );
        assert_eq!(k_smallest_mean(vec![].into_iter(), 3), (0 as f64 / 3.0));
        assert_eq!(
            k_smallest_mean(
                vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5].into_iter(),
                5
            ),
            ((0 + 1 + 2 + 3 + 4) as f64 / 5.0)
        );
    }

    #[test]
    fn test_calculate_mean() {
        assert_eq!(
            calculate_mean(
                [
                    ("CS100".to_string(), 60),
                    ("CS200".to_string(), 60),
                    ("CS200".to_string(), 80),
                    ("CS300".to_string(), 100),
                ]
                .into_iter()
            ),
            [
                ("CS100".to_string(), 60.0),
                ("CS200".to_string(), 70.0),
                ("CS300".to_string(), 100.0)
            ]
            .into_iter()
            .collect()
        );

        assert_eq!(
            calculate_mean(
                [
                    ("CS220".to_string(), 60),
                    ("CS420".to_string(), 60),
                    ("CS220".to_string(), 80),
                    ("CS431".to_string(), 60),
                    ("CS420".to_string(), 80),
                    ("CS220".to_string(), 100)
                ]
                .into_iter()
            ),
            [
                ("CS220".to_string(), 80.0),
                ("CS420".to_string(), 70.0),
                ("CS431".to_string(), 60.0)
            ]
            .into_iter()
            .collect()
        )
    }

    #[test]
    fn test_sum_is_n() {
        assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 3), 1);
        assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 4), 2);
        assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 5), 2);
        assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 6), 1);
        assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 2), 0);

        assert_eq!(sum_is_n(vec![(1..100).collect()], 50), 1);

        assert_eq!(
            sum_is_n(vec![(1..10).collect(), (1..10).rev().collect()], 10),
            9
        );

        assert_eq!(
            sum_is_n(
                vec![
                    (0..10).map(|x| x * 2 + 1).collect(),
                    (0..20).map(|x| x * 3).collect(),
                    (0..30).map(|x| x * 5 + 2).collect()
                ],
                53
            ),
            30
        );
    }

    // find_count_n
    #[test]
    fn test_find_count_n() {
        assert_eq!(find_count_n(vec![], 1), vec![]);
        assert_eq!(find_count_n(vec![1, 2], 1), vec![1, 2]);
        assert_eq!(find_count_n(vec![1, 3, 3], 1), vec![1]);
        assert_eq!(find_count_n(vec![1, 3, 3], 2), vec![3]);
        assert_eq!(find_count_n(vec![1, 2, 3, 4, 4], 1), vec![1, 2, 3]);
        assert_eq!(find_count_n(vec![1, 3, 2, 3, 2, 3], 3), vec![3]);
        assert_eq!(find_count_n(vec![1, 2, 2, 3, 3, 4], 2), vec![2, 3]);
        assert_eq!(find_count_n(vec![1, 3, 2, 2, 3], 2), vec![2, 3]);
        assert_eq!(find_count_n(vec![0, 2, 2, 4, 3], 0), vec![]);
        assert_eq!(find_count_n(vec![1, 1, 1, 2, 2], 1), vec![]);
    }

    #[test]
    fn test_position_median() {
        assert_eq!(position_median(Vec::<usize>::new()), None);
        assert_eq!(position_median(vec![3]), Some(0));
        assert_eq!(position_median(vec![3, 3]), Some(0));
        assert_eq!(position_median(vec![3, 3, 3]), Some(0));
        assert_eq!(position_median(vec![1, 3, 3, 3]), Some(1));
        assert_eq!(position_median(vec![3, 1, 3, 3]), Some(0));
        assert_eq!(position_median(vec![3, 1, 5, 3]), Some(0));
        assert_eq!(position_median(vec![1, 3, 3, 6, 7, 8, 9]), Some(3));
        assert_eq!(position_median(vec![1, 2, 3, 4, 5, 6, 8, 9]), Some(4));
    }
}
