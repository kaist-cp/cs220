#[cfg(test)]
mod test {
    use super::super::assignment10::*;

    #[test]
    fn test_inversion() {
        assert_eq!(inversion(vec![3, 5, 4]), vec![(1, 2)]);
        assert_eq!(inversion(vec!["c", "a", "b", "d"]), vec![(0, 1), (0, 2)]);
        assert_eq!(
            inversion(vec![2, 5, 4, 6, 3, 1]),
            vec![
                (0, 5),
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 4),
                (2, 5),
                (3, 4),
                (3, 5),
                (4, 5)
            ]
        );
    }

    #[test]
    fn test_traverse_preorder() {
        let root = Node::NonLeaf((
            1,
            vec![
                Node::NonLeaf((2, vec![Node::Leaf(5), Node::Leaf(6)])),
                Node::Leaf(3),
                Node::NonLeaf((4, vec![Node::Leaf(7), Node::Leaf(8), Node::Leaf(9)])),
            ],
        ));

        assert_eq!(traverse_preorder(root), vec![1, 2, 5, 6, 3, 4, 7, 8, 9]);
    }

    #[test]
    fn test_du_sort() {
        let rootfile = File::Directory(
            "root".to_string(),
            vec![
                File::Directory(
                    "a".to_string(),
                    vec![
                        File::Data("a1".to_string(), 1),
                        File::Data("a2".to_string(), 3),
                    ],
                ),
                File::Directory(
                    "b".to_string(),
                    vec![
                        File::Data("b1".to_string(), 3),
                        File::Data("b2".to_string(), 15),
                    ],
                ),
                File::Data("c".to_string(), 8),
            ],
        );

        assert_eq!(
            du_sort(&rootfile),
            vec![
                ("a1", 1),
                ("a2", 3),
                ("b1", 3),
                ("a", 4),
                ("c", 8),
                ("b2", 15),
                ("b", 18),
                ("root", 1 + 3 + 3 + 15 + 8)
            ]
        );

        let rootfile = File::Directory(
            "root".to_string(),
            vec![
                File::Directory(
                    "b".to_string(),
                    vec![
                        File::Data("b1".to_string(), 3),
                        File::Data("b2".to_string(), 15),
                    ],
                ),
                File::Data("c".to_string(), 8),
                File::Directory(
                    "a".to_string(),
                    vec![
                        File::Data("a1".to_string(), 1),
                        File::Data("a2".to_string(), 3),
                    ],
                ),
            ],
        );

        assert_eq!(
            du_sort(&rootfile),
            vec![
                ("a1", 1),
                ("a2", 3),
                ("b1", 3),
                ("a", 4),
                ("c", 8),
                ("b2", 15),
                ("b", 18),
                ("root", 1 + 3 + 3 + 15 + 8)
            ]
        );

        let rootfile = File::Directory(
            "root".to_string(),
            vec![
                File::Directory(
                    "a".to_string(),
                    vec![
                        File::Data("a1".to_string(), 1),
                        File::Data("a2".to_string(), 3),
                        File::Directory(
                            "a3".to_string(),
                            vec![
                                File::Data("a31".to_string(), 1),
                                File::Data("a32".to_string(), 3),
                                File::Data("a33".to_string(), 6),
                            ],
                        ),
                    ],
                ),
                File::Directory(
                    "b".to_string(),
                    vec![
                        File::Data("b1".to_string(), 3),
                        File::Data("b2".to_string(), 15),
                    ],
                ),
                File::Data("c".to_string(), 16),
            ],
        );

        assert_eq!(
            du_sort(&rootfile),
            vec![
                ("a1", 1),
                ("a31", 1),
                ("a2", 3),
                ("a32", 3),
                ("b1", 3),
                ("a33", 6),
                ("a3", 10),
                ("a", 14),
                ("b2", 15),
                ("c", 16),
                ("b", 18),
                ("root", 48)
            ]
        );
    }
}
