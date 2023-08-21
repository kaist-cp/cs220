#[cfg(test)]
mod test_peano_nat {
    use crate::assignments::assignment11::peano_nat::*;

    #[test]
    fn test_from_as_usize() {
        assert_eq!(Nat::from_usize(0), Nat::O);
        assert_eq!(
            Nat::from_usize(2),
            Nat::S(Box::new(Nat::S(Box::new(Nat::O))))
        );

        for n in 0..100 {
            assert_eq!(Nat::from_usize(n).as_usize(), n);
        }
    }

    fn safe_sub(i: usize, j: usize) -> usize {
        if i > j {
            i - j
        } else {
            0
        }
    }

    #[test]
    fn test_add_sub_mul() {
        for i in 0..30 {
            let n = Nat::from_usize(i);
            for j in 0..30 {
                let m = Nat::from_usize(j);
                assert_eq!((n.clone() + m.clone()).as_usize(), i + j);
                assert_eq!((n.clone() - m.clone()).as_usize(), safe_sub(i, j));
                assert_eq!((n.clone() * m.clone()).as_usize(), i * j);
            }
        }
    }
}
