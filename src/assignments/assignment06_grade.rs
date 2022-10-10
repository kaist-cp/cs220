#[cfg(test)]
mod test {
    use super::super::assignment06::*;

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
