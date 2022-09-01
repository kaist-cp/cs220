#[cfg(test)]
mod test {
    use super::super::assignment01::*;

    #[test]
    fn test_add_7_3() {
        assert_eq!(add(7, 3), 10);
    }

    #[test]
    fn test_add_overflow() {
        assert_eq!(add(usize::MAX, 1), usize::MIN);
    }

    #[test]
    fn test_sub_7_3() {
        assert_eq!(sub(7, 3), 4);
    }

    #[test]
    fn test_sub_overflow() {
        assert_eq!(sub(usize::MIN, 1), usize::MAX);
    }
}
