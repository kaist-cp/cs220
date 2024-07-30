#[cfg(test)]
mod test {
    use crate::assignments::assignment02::vec_and_mat::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 1);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 2);
        assert_eq!(fibonacci(3), 3);
        assert_eq!(fibonacci(4), 5);
        assert_eq!(fibonacci(5), 8);
        assert_eq!(fibonacci(6), 13);
        assert_eq!(fibonacci(7), 21);
        assert_eq!(fibonacci(50), 20365011074);
        assert_eq!(fibonacci(92), 12200160415121876738);
    }

    #[test]
    fn test_inverse() {
        assert_eq!(
            FMat2 {
                a: 1.0,
                b: 1.0,
                c: 2.0,
                d: 3.0
            }
            .inverse(),
            FMat2 {
                a: 3.0,
                b: -1.0,
                c: -2.0,
                d: 1.0
            }
        );
        assert_eq!(
            FMat2 {
                a: 2.0,
                b: 3.0,
                c: 5.0,
                d: 7.0
            }
            .inverse(),
            FMat2 {
                a: -7.0,
                b: 3.0,
                c: 5.0,
                d: -2.0
            }
        );
    }
}
