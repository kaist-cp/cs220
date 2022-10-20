#[cfg(test)]
mod test {
    use super::super::assignment08::*;

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
}
