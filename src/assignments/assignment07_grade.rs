#[cfg(test)]
mod test {
    use super::super::assignment07::*;

    #[test]
    fn test_find() {
        assert_eq!(
            find("abc".as_bytes(), "abcdabcd".as_bytes()).collect::<Vec<usize>>(),
            vec![0, 4]
        );

        assert_eq!(
            find("aaba".as_bytes(), "aabaacaadaabaaba".as_bytes()).collect::<Vec<usize>>(),
            vec![0, 9, 12]
        );

        assert_eq!(
            find("ababac".as_bytes(), "abababcabababcabababc".as_bytes()).collect::<Vec<usize>>(),
            vec![]
        );

        assert_eq!(
            find("ababc".as_bytes(), "abc".as_bytes()).collect::<Vec<usize>>(),
            vec![]
        );
    }

    #[test]
    fn test_find_usize() {
        assert_eq!(
            find(&[1, 2, 3], &[1, 2, 3, 4, 1, 2, 3, 4]).collect::<Vec<usize>>(),
            vec![0, 4]
        );

        assert_eq!(
            find(
                &[5, 5, 7, 5],
                &[5, 5, 7, 5, 5, 8, 5, 5, 9, 5, 5, 7, 5, 5, 7, 5]
            )
            .collect::<Vec<usize>>(),
            vec![0, 9, 12]
        );
    }

    #[test]
    fn test_transform_identity() {
        let tr = Identity;

        assert_eq!(tr.transform(3), 3);
        assert_eq!(tr.transform(3.0), 3.0);
        assert_eq!(tr.transform("abc"), "abc");
    }

    #[test]
    fn test_transform_tuple() {
        let f1 = |x: u32| x + 1;
        let f2 = |x: String| x.clone() + &x;

        let tr1: Custom<_, _> = f1.into();
        let tr2: Custom<_, _> = f2.into();

        let list1 = 0u32..10u32;
        let list2 = ["a".to_string(), "bb".to_string(), "ccc".to_string()];

        for v1 in list1 {
            for v2 in list2.clone() {
                let tr = (tr1.clone(), tr2.clone());
                let input = (v1, v2.clone());
                let expected = (f1(v1), f2(v2));
                assert_eq!(tr.transform(input), expected);
            }
        }
    }

    #[test]
    fn test_transform_repeat() {
        let inc = Custom::from(|x: i32| x + 1);
        let dec = Custom::from(|x: i32| x - 1);

        for i in 0..10 {
            for j in -10..10 {
                assert_eq!(Repeat::new(inc, i as u32).transform(j), j + i);
                assert_eq!(Repeat::new(dec, i as u32).transform(j), j - i);
            }
        }
    }

    #[test]
    fn test_transform_repeat_until_converge() {
        let inc = Custom::from(|x: i32| if x < 50 { x + 1 } else { x });
        let dec = Custom::from(|x: i32| if x > 50 { x - 1 } else { x });

        assert_eq!(RepeatUntilConverge::new(inc).transform(40), 50);
        assert_eq!(RepeatUntilConverge::new(inc).transform(60), 60);

        assert_eq!(RepeatUntilConverge::new(dec).transform(40), 40);
        assert_eq!(RepeatUntilConverge::new(dec).transform(60), 50);
    }
}
