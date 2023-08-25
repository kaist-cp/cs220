#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::assignments::assignment08::church::*;

    #[test]
    fn you_must_pass_these_examples() {
        let c_zero = zero::<usize>();
        assert_eq!(to_usize(c_zero.clone()), 0);

        let c_one = succ(c_zero.clone());
        assert_eq!(to_usize(c_one.clone()), to_usize(one::<()>()));

        let c_two = add(c_one.clone(), c_one.clone());
        let c_three = add(c_one.clone(), c_two.clone());
        assert_eq!(to_usize(c_three.clone()), 3);

        let c_product = mult(c_three.clone(), c_two.clone());
        assert_eq!(to_usize(c_product.clone()), 6);

        let c_exponent = exp::<()>(6, 3);
        assert_eq!(to_usize(c_exponent), 216);

        let c_exponent2 = exp::<()>(3, 0);
        assert_eq!(to_usize(c_exponent2), 1);
    }

    fn id(n: usize) -> usize {
        to_usize(from_usize::<()>(n))
    }

    fn c_id(n: Church<usize>) -> Church<usize> {
        from_usize(to_usize(n))
    }

    #[test]
    fn engineering_isnt_just_mathematics() {
        const N: usize = 77777;
        assert_eq!(N, id(N));
    }

    /// This test case is an optional challenge.
    /// While it's not necessary to pass this test,
    /// successfully doing so could provide a sense of satisfaction and achievement.
    // #[test]
    // fn i_said_engineering_isnt_just_mathematics() {
    //     const N: usize = 777777777777777;
    //     assert_eq!(N, id(N));
    // }

    #[test]
    fn be_honest() {
        let mut rng = rand::thread_rng();

        for _ in 0..77 {
            let x = rng.gen_range(0..=7);
            let y = rng.gen_range(0..=7);

            let c_x = from_usize(x);
            let c_y = from_usize(y);

            let c_sum = add(c_x.clone(), c_y.clone());
            assert_eq!(to_usize(c_id(c_sum)), x + y);

            let c_prod = mult(c_x.clone(), c_y.clone());
            assert_eq!(to_usize(c_id(c_prod)), x * y);

            let c_exp = exp(x, y);
            assert_eq!(to_usize(c_id(c_exp)), x.pow(y as u32));
        }
    }
}
