#[cfg(test)]
mod test {
    use ntest::assert_about_eq;

    use crate::assignments::assignment06::symbolic_differentiation::*;

    // Constant rationals to use
    const TWO: Rational = Rational::new(2, 1);
    const FOUR: Rational = Rational::new(4, 1);
    const THIRD: Rational = Rational::new(1, 3);
    const FIVE_THIRD: Rational = Rational::new(5, 3);
    const TWO_SEVENTH: Rational = Rational::new(2, 7);

    #[test]
    fn test_rational_simpl() {
        assert_eq!(format!("{}", Rational::new(1, 2)), "1/2".to_string());

        assert_eq!(format!("{}", Rational::new(0, 0)), "0".to_string());

        assert_eq!(format!("{}", Rational::new(1, 1)), "1".to_string());

        assert_eq!(format!("{}", Rational::new(-3, 7)), "-3/7".to_string());
    }

    #[test]
    fn test_rational_arithmetic() {
        assert_eq!(
            format!("{}", Rational::new(1, 4) + Rational::new(3, 4)),
            "1".to_string()
        );

        assert_eq!(
            format!("{}", Rational::new(1, 3) + Rational::new(1, 6)),
            "1/2".to_string()
        );

        assert_eq!(
            format!("{}", Rational::new(1, 5) - Rational::new(1, 2)),
            "-3/10".to_string()
        );

        assert_eq!(
            format!("{}", Rational::new(-5, 12) * Rational::new(6, 125)),
            "-1/50".to_string()
        );

        assert_eq!(
            format!("{}", Rational::new(-3, 4) / Rational::new(-7, 13)),
            "39/28".to_string()
        );
    }

    #[test]
    fn test_rational_arithmetic_long() {
        assert_eq!(
            format!(
                "{}",
                Rational::new(1, 2) + Rational::new(1, 2) + Rational::new(1, 2)
            ),
            "3/2".to_string()
        );

        assert_eq!(
            format!(
                "{}",
                Rational::new(1, 2) - Rational::new(1, 4) + Rational::new(1, 5)
            ),
            "9/20".to_string()
        );

        assert_eq!(
            format!(
                "{}",
                Rational::new(1, 2)
                    * Rational::new(1, 4)
                    * Rational::new(1, 8)
                    * Rational::new(1, 16)
                    / Rational::new(1, 1024)
            ),
            "1".to_string()
        );

        assert_eq!(
            format!(
                "{}",
                Rational::new(123, 798)
                    + Rational::new(684, 32) / (Rational::new(13, 44) - Rational::new(123, 4472))
                        * Rational::new(1237, 2)
            ),
            "12356494070/250439".to_string()
        );
    }

    #[test]
    fn test_differentiate_simple() {
        // Constant
        assert_eq!(format!("{}", Rational::new(3, 2).diff()), "0".to_string());

        // Polynomials
        assert_eq!(
            format!("{}", SingletonPolynomial::new_c(Rational::new(3, 1)).diff()),
            "0".to_string()
        );
        assert_eq!(
            format!("{}", SingletonPolynomial::new_poly(TWO, FOUR).diff()),
            "(8)x^(3)".to_string()
        );
        assert_eq!(
            format!(
                "{}",
                SingletonPolynomial::new_poly(FIVE_THIRD, THIRD).diff()
            ),
            "(5/9)x^(-2/3)".to_string()
        );

        // Exponential
        assert_eq!(format!("{}", Exp::new().diff()), "exp(x)".to_string());

        // Trigonometric
        assert_eq!(
            format!("{}", Trignometric::new_sine(ONE).diff()),
            "cos(x)".to_string()
        );
        assert_eq!(
            format!("{}", Trignometric::new_cosine(ONE).diff()),
            "-sin(x)".to_string()
        );
        assert_eq!(
            format!("{}", Trignometric::new_sine(FIVE_THIRD).diff()),
            "(5/3)cos(x)".to_string()
        );
        assert_eq!(
            format!("{}", Trignometric::new_cosine(TWO_SEVENTH).diff()),
            "(-2/7)sin(x)".to_string()
        );
    }

    #[test]
    fn test_differentiate_complex() {
        type BF = BaseFuncs;
        type CF = ComplexFuncs<BF>;

        // Unlike the above simple test, it is hard to state a canonical
        // form for derivative of more complex functions. Thus, we only test that the
        // derivative is correct at certain points.

        // Add
        //
        // d/dx (2x^4 + exp(x)) = 8x^3 + exp(x)
        let f1 = SingletonPolynomial::new_poly(TWO, FOUR);
        let f2 = Exp::new();
        let deriv = CF::Add(
            Box::new(CF::Func(BF::Poly(f1))),
            Box::new(CF::Func(BF::Exp(f2))),
        )
        .diff();
        assert_about_eq!(deriv.evaluate(2.2), 94.2090134994f64);
        assert_about_eq!(deriv.evaluate(4.5), 819.017131301);

        // Sub
        //
        // d/dx ((5/3)cos(x) - sin(x)) = (-5/3)sin(x) - cos(x)
        let f1 = Trignometric::new_cosine(FIVE_THIRD);
        let f2 = Trignometric::new_sine(ONE);
        let deriv = CF::Sub(
            Box::new(CF::Func(BF::Trig(f1))),
            Box::new(CF::Func(BF::Trig(f2))),
        )
        .diff();
        assert_about_eq!(deriv.evaluate(2.7), 0.191772341627);
        assert_about_eq!(deriv.evaluate(0.01), -1.01661638931);

        // Mult
        //
        // d/dx (2x^4 * cos(x) * exp(x)) =
        // 8x^2 * cos(x) * exp(x) - 2x^4 * sin(x) * exp(x) + 2x^4 * cos(x) * exp(x)
        let f1 = SingletonPolynomial::new_poly(TWO, FOUR);
        let f2 = Trignometric::new_cosine(ONE);
        let f3 = Exp::new();
        let deriv = CF::Mul(
            Box::new(CF::Func(BF::Poly(f1))),
            Box::new(CF::Mul(
                Box::new(CF::Func(BF::Trig(f2))),
                Box::new(CF::Func(BF::Exp(f3))),
            )),
        )
        .diff();
        assert_about_eq!(deriv.evaluate(3.4), -14804.9016757);
        assert_about_eq!(deriv.evaluate(0.07), 0.00298352866);

        // Div
        //
        // (d/dx) (sin(x)/cos(x)) = (cos(x)*cos(x) + sin(x)*sin(x)) / cos(x)*cos(x)
        let f1 = Trignometric::new_sine(ONE);
        let f2 = Trignometric::new_cosine(ONE);
        let deriv = CF::Div(
            Box::new(CF::Func(BF::Trig(f1))),
            Box::new(CF::Func(BF::Trig(f2))),
        )
        .diff();
        assert_about_eq!(deriv.evaluate(core::f64::consts::PI), 1f64);
        assert_about_eq!(deriv.evaluate(0f64), 1f64);

        // Comp
        //
        // d/dx (cos(x^2)) = -2x * sin(x^2)
        let f1 = Trignometric::new_cosine(ONE);
        let f2 = SingletonPolynomial::new_poly(ONE, TWO);
        let deriv = CF::Comp(
            Box::new(CF::Func(BF::Trig(f1))),
            Box::new(CF::Func(BF::Poly(f2))),
        )
        .diff();
        assert_about_eq!(deriv.evaluate(2.714), -4.79392977);
        assert_about_eq!(deriv.evaluate(3.9), -3.72556973);
    }
}
