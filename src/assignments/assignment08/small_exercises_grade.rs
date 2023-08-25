#[cfg(test)]
mod test {
    use crate::assignments::assignment08::small_exercises::*;

    #[test]
    fn test_repeat() {
        for i in 0..10 {
            assert_eq!(42 + 2 * i, repeat(i, |x| x + 2)(42));
        }
    }

    #[test]
    fn test_funny_map() {
        assert_eq!(
            vec![0, 3, 6, 9, 12],
            funny_map(|x| x + 2, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_either2_map() {
        let v1 = Either2::<u32, f32>::Case1 { inner: 42 };
        let u1 = Either2::<u32, f32>::Case1 { inner: 43 };
        assert_eq!(u1, v1.map(|i| i + 1, |f| f + 1.0));

        let v2 = Either2::<u32, f32>::Case2 { inner: 42.0 };
        let u2 = Either2::<u32, f32>::Case2 { inner: 43.0 };
        assert_eq!(u2, v2.map(|i| i + 1, |f| f + 1.0));
    }

    #[test]
    fn test_count_repeat() {
        let inc_mod_100 = |x| (x + 1) % 100;
        assert_eq!(count_repeat(inc_mod_100, 10), 100);
        assert_eq!(count_repeat(inc_mod_100, 12345), 101);

        let p_lookup = |n| vec![1, 0, 2, 4, 5, 6, 7, 3][n];
        assert_eq!(count_repeat(p_lookup, 0), 2);
        assert_eq!(count_repeat(p_lookup, 1), 2);
        assert_eq!(count_repeat(p_lookup, 2), 1);
        assert_eq!(count_repeat(p_lookup, 3), 5);
        assert_eq!(count_repeat(p_lookup, 4), 5);
        assert_eq!(count_repeat(p_lookup, 5), 5);
        assert_eq!(count_repeat(p_lookup, 6), 5);
        assert_eq!(count_repeat(p_lookup, 7), 5);
    }
}
