#[cfg(test)]
mod test {
    use itertools::Itertools;
    use ntest::assert_about_eq;

    use crate::assignments::assignment07::small_exercises::*;

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
    fn test_fib_iter() {
        assert_eq!(
            fib(0, 1).take(10).collect::<Vec<_>>(),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        );

        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        struct Rgb(u8, u8, u8);

        impl std::ops::Add for Rgb {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(
                    ((self.0 as u16 + rhs.0 as u16) / 2) as u8,
                    ((self.1 as u16 + rhs.1 as u16) / 2) as u8,
                    ((self.2 as u16 + rhs.2 as u16) / 2) as u8,
                )
            }
        }

        assert_eq!(
            fib(Rgb(255, 0, 100), Rgb(1, 128, 0))
                .take(20)
                .collect::<Vec<_>>(),
            vec![
                Rgb(255, 0, 100),
                Rgb(1, 128, 0),
                Rgb(128, 64, 50),
                Rgb(64, 96, 25),
                Rgb(96, 80, 37),
                Rgb(80, 88, 31),
                Rgb(88, 84, 34),
                Rgb(84, 86, 32),
                Rgb(86, 85, 33),
                Rgb(85, 85, 32),
                Rgb(85, 85, 32),
                Rgb(85, 85, 32),
                Rgb(85, 85, 32),
                Rgb(85, 85, 32),
                Rgb(85, 85, 32),
                Rgb(85, 85, 32),
                Rgb(85, 85, 32),
                Rgb(85, 85, 32),
                Rgb(85, 85, 32),
                Rgb(85, 85, 32)
            ]
        );
    }

    #[test]
    fn test_range_iter() {
        let one_to_tens = [
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            range(Endpoint::Inclusive(1), Endpoint::Inclusive(10), 1).collect(),
            range(Endpoint::Exclusive(0), Endpoint::Inclusive(10), 1).collect(),
            range(Endpoint::Inclusive(1), Endpoint::Exclusive(11), 1).collect(),
            range(Endpoint::Exclusive(0), Endpoint::Exclusive(11), 1).collect(),
        ];
        assert!(one_to_tens.iter().all_equal());

        let ten_to_ones = [
            vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
            range(Endpoint::Inclusive(10), Endpoint::Inclusive(1), -1).collect(),
            range(Endpoint::Exclusive(11), Endpoint::Inclusive(1), -1).collect(),
            range(Endpoint::Inclusive(10), Endpoint::Exclusive(0), -1).collect(),
            range(Endpoint::Exclusive(11), Endpoint::Exclusive(0), -1).collect(),
        ];
        assert!(ten_to_ones.iter().all_equal());

        let five_evens = vec![
            vec![2, 4, 6, 8, 10],
            range(Endpoint::Inclusive(2), Endpoint::Inclusive(10), 2).collect(),
            range(Endpoint::Inclusive(2), Endpoint::Inclusive(11), 2).collect(),
            range(Endpoint::Exclusive(1), Endpoint::Inclusive(10), 2).collect(),
            range(Endpoint::Exclusive(1), Endpoint::Inclusive(11), 2).collect(),
            range(Endpoint::Inclusive(2), Endpoint::Exclusive(11), 2).collect(),
            range(Endpoint::Inclusive(2), Endpoint::Exclusive(12), 2).collect(),
            range(Endpoint::Exclusive(1), Endpoint::Exclusive(11), 2).collect(),
            range(Endpoint::Exclusive(1), Endpoint::Exclusive(12), 2).collect(),
        ];
        assert!(five_evens.iter().all_equal());

        let emptys = [
            vec![],
            range(Endpoint::Inclusive(2), Endpoint::Inclusive(10), -1).collect(),
            range(Endpoint::Inclusive(10), Endpoint::Inclusive(-100), 1).collect(),
            range(Endpoint::Inclusive(1), Endpoint::Exclusive(1), 1).collect(),
        ];
        assert!(emptys.iter().all_equal());
    }

    #[test]
    fn test_small() {
        assert_eq!(divisors(10).collect::<Vec<u64>>(), vec![1, 2, 5, 10]);

        assert_eq!(divisors(17).collect::<Vec<u64>>(), vec![1, 17]);

        assert_eq!(divisors(49).collect::<Vec<u64>>(), vec![1, 7, 49]);

        assert_eq!(
            divisors(120).collect::<Vec<u64>>(),
            vec![1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 20, 24, 30, 40, 60, 120]
        );

        assert_eq!(divisors(1).collect::<Vec<u64>>(), vec![1]);

        assert_eq!(divisors(2).collect::<Vec<u64>>(), vec![1, 2]);

        assert_eq!(divisors(3).collect::<Vec<u64>>(), vec![1, 3]);
    }

    #[test]
    fn test_large() {
        assert_eq!(
            divisors(1_000_000_000_000_037).collect::<Vec<u64>>(),
            vec![1, 1_000_000_000_000_037]
        );

        assert_eq!(
            divisors(99_999_820_000_081).collect::<Vec<u64>>(),
            vec![1, 9_999_991, 99_999_820_000_081]
        );

        assert_eq!(
            divisors(1_234_567_890_123).collect::<Vec<u64>>(),
            vec![
                1,
                3,
                3_541,
                10_623,
                116_216_501,
                348_649_503,
                411_522_630_041,
                1_234_567_890_123
            ]
        );

        assert_eq!(divisors(97_821_761_637_600).count(), 17280);
    }
}
