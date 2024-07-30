#[cfg(test)]
mod test {
    use ntest::assert_about_eq;

    use crate::assignments::assignment06::semiring::*;

    fn test_from_str(s: &str, f: impl Fn(i64) -> i64) {
        let poly = s.parse::<Polynomial<i64>>().unwrap();
        for i in 0..10 {
            assert_eq!(poly.eval(i), f(i));
        }
    }

    fn test_polynomial<T: Semiring>() {
        // x^2 + 5x + 6
        let poly = Polynomial::add(
            &Polynomial::add(
                &Polynomial::mul(
                    &Polynomial::from(from_usize::<T>(1)),
                    &Polynomial::mul(&Polynomial::x(), &Polynomial::x()),
                ),
                &Polynomial::mul(&Polynomial::from(from_usize::<T>(5)), &Polynomial::x()),
            ),
            &Polynomial::from(from_usize::<T>(6)),
        );

        // 13^2 + 5*13 + 6
        let value = poly.eval(from_usize(13));

        assert_eq!(value, from_usize(13 * 13 + 5 * 13 + 6));
    }

    #[test]
    fn test_123() {
        test_from_str("123", |x| 123);
    }

    #[test]
    fn test_x() {
        test_from_str("x", |x| x);
    }

    #[test]
    fn test_24x() {
        test_from_str("24x", |x| 24 * x);
    }

    #[test]
    fn test_2x_3() {
        test_from_str("2x + 3", |x| 2 * x + 3);
    }

    #[test]
    fn test_x3() {
        test_from_str("x^3", |x| x * x * x);
    }

    #[test]
    fn test_2x3_3x2_5x_12() {
        test_from_str("2x^3 + 3x^2 + 5x + 12", |x| {
            2 * x * x * x + 3 * x * x + 5 * x + 12
        });
    }

    #[test]
    fn test_x5_1() {
        test_from_str("x^5 + 1", |x| x * x * x * x * x + 1);
    }

    #[test]
    fn test_polynomial_u64() {
        test_polynomial::<u64>();
    }

    #[test]
    fn test_polynomial_f64() {
        test_polynomial::<f64>();
    }

    #[test]
    fn test_polynomial_p_u64() {
        test_polynomial::<Polynomial<u64>>();
    }

    #[test]
    fn test_polynomial_xy() {
        // (x+1)(y+2)
        let poly: Polynomial<Polynomial<u64>> = Polynomial::mul(
            &Polynomial::from(Polynomial::add(
                &Polynomial::x(),
                &Polynomial::from(from_usize::<u64>(1)),
            )),
            &(Polynomial::add(
                &Polynomial::x(),
                &Polynomial::from(Polynomial::from(from_usize::<u64>(2))),
            )),
        );

        // poly with y = x+3
        let value = poly.eval(Polynomial::add(
            &Polynomial::x(),
            &Polynomial::from(from_usize::<u64>(3)),
        ));

        // x^2 + 6x + 5
        let expected = Polynomial::add(
            &Polynomial::add(
                &Polynomial::mul(
                    &Polynomial::from(from_usize::<u64>(1)),
                    &Polynomial::mul(&Polynomial::x(), &Polynomial::x()),
                ),
                &Polynomial::mul(&Polynomial::from(from_usize::<u64>(6)), &Polynomial::x()),
            ),
            &Polynomial::from(from_usize::<u64>(5)),
        );

        assert_eq!(value, expected);
    }

    #[test]
    fn test_zero_remove() {
        // (x-1)(x+1)
        let poly: Polynomial<i64> = Polynomial::mul(
            &Polynomial::add(&Polynomial::x(), &Polynomial::from(-1)),
            &Polynomial::add(&Polynomial::x(), &Polynomial::from(1)),
        );

        // (x-1)(x+1) == x^2 - 1
        assert_eq!(
            poly,
            Polynomial::add(
                &Polynomial::mul(&Polynomial::x(), &Polynomial::x()),
                &Polynomial::from(-1)
            )
        );
    }
}
