#[cfg(test)]
mod test {
    use crate::assignments::assignment03::custom_operators::MyOption::*;
    use crate::assignments::assignment03::custom_operators::*;

    #[test]
    fn test_my_map() {
        fn len(s: &str) -> usize {
            s.len()
        }

        fn plus_one(x: isize) -> isize {
            x + 1
        }

        fn is_positive(x: f64) -> bool {
            x > 0.0f64
        }

        assert_eq!(my_map(MySome("Hello, World!"), len), MySome(13));
        assert_eq!(my_map(MyNone, len), MyNone);

        assert_eq!(my_map(MySome(1), plus_one), MySome(2));
        assert_eq!(my_map(MyNone, plus_one), MyNone);

        assert_eq!(my_map(MySome(5.0f64), is_positive), MySome(true));
        assert_eq!(my_map(MySome(-3.0f64), is_positive), MySome(false));
        assert_eq!(my_map(MyNone::<f64>, is_positive), MyNone);
    }

    #[test]
    fn test_my_and_then() {
        fn plus_one(x: isize) -> MyOption<isize> {
            MySome(x + 1)
        }

        fn none(_: isize) -> MyOption<isize> {
            MyNone
        }

        assert_eq!(my_and_then(MySome(1), plus_one), MySome(2));
        assert_eq!(my_and_then(MySome(1), none), MyNone);

        assert_eq!(my_and_then(MyNone, plus_one), MyNone);
        assert_eq!(my_and_then(MyNone, none), MyNone);
    }

    fn product(a: i32, b: i32) -> i32 {
        a * b
    }

    #[test]
    fn test_my_option_op_or() {
        assert_eq!(my_option_op_or(MyNone, MyNone, product), MyNone);
        assert_eq!(my_option_op_or(MySome(3), MyNone, product), MySome(3));
        assert_eq!(my_option_op_or(MyNone, MySome(5), product), MySome(5));
        assert_eq!(my_option_op_or(MySome(3), MySome(5), product), MySome(15));
    }
}
