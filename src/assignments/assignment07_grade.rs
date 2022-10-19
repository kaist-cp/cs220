#[cfg(test)]
mod test {
    use super::super::assignment07::*;

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
}
