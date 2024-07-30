#[cfg(test)]
mod test {

    use ntest::{assert_false, assert_true};

    use crate::assignments::assignment09::bigint::*;

    #[test]
    fn test_inf_prec_simple() {
        // Basic
        assert_eq!("00000000", format!("{}", BigInt::new(0)));
        assert_eq!("ffffffff", format!("{}", BigInt::new(u32::MAX)));
        assert_eq!("00bc4fdc", format!("{}", BigInt::new(12_341_212)));
        assert_eq!("fffffed8", format!("{}", BigInt::new(4_294_967_000u32)));

        // Add Basic
        assert_eq!("00000001", format!("{}", BigInt::new(0) + BigInt::new(1)));

        assert_eq!(
            "0df655df",
            format!("{}", BigInt::new(13_413) + BigInt::new(234_234_234))
        );

        assert_eq!(
            "ffffff03",
            format!("{}", BigInt::new(4_294_967_000u32) + BigInt::new(43))
        );

        // Sub Basic
        assert_eq!("ffffffff", format!("{}", BigInt::new(0) - BigInt::new(1)));

        assert_eq!(
            "f20a12eb",
            format!("{}", BigInt::new(13_413) - BigInt::new(234_234_234))
        );

        assert_eq!(
            "fffffead",
            format!("{}", BigInt::new(4_294_967_000u32) - BigInt::new(43))
        );
    }

    #[test]
    #[should_panic]
    fn test_inf_prec_panic() {
        let _unused = BigInt::new_large(vec![]);
    }

    #[test]
    fn test_inf_prec_complex() {
        // Positive overflow
        assert_eq!(
            "0000000080000000",
            format!("{}", BigInt::new(i32::MAX as u32) + BigInt::new(1))
        );

        // Negative overflow
        assert_eq!(
            "ffffffff7fffffff",
            format!("{}", BigInt::new(i32::MIN as u32) - BigInt::new(1))
        );

        // Larger positive overflow
        assert_eq!(
            "00000000fffffffe00000000",
            format!(
                "{}",
                BigInt::new_large(vec![i32::MAX as u32, 0])
                    + BigInt::new_large(vec![i32::MAX as u32, 0])
            )
        );

        // Smaller negative overflow
        assert_eq!(
            "ffffffff000000000119464a",
            format!(
                "{}",
                BigInt::new_large(vec![i32::MIN as u32, 2_871_572])
                    + BigInt::new_large(vec![i32::MIN as u32, 15_562_038])
            )
        );

        // Truncate
        assert_eq!(
            "00000000",
            format!(
                "{}",
                BigInt::new_large(vec![i32::MIN as u32, 2_871_572, 123_456])
                    - BigInt::new_large(vec![i32::MIN as u32, 2_871_572, 123_456])
            )
        );

        assert_eq!(
            "ffffffff",
            format!(
                "{}",
                BigInt::new_large(vec![i32::MIN as u32, 2_871_572, 123_456])
                    - BigInt::new_large(vec![i32::MIN as u32, 2_871_572, 123_457])
            )
        );

        // TODO: add a test case testing sign extension.
    }
}
