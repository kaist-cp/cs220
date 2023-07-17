#[cfg(test)]
mod test {
    use super::super::assignment10::*;

    #[test]
    fn test_pythagorean() {
        let pythagoreans = [
            (3, 4, 5),
            (5, 12, 13),
            (8, 15, 17),
            (7, 24, 25),
            (20, 21, 29),
            (12, 35, 37),
            (9, 40, 41),
            (28, 45, 53),
            (11, 60, 61),
            (16, 63, 65),
            (33, 56, 65),
            (48, 55, 73),
            (13, 84, 85),
            (36, 77, 85),
            (39, 80, 89),
            (65, 72, 97),
            (20, 99, 101),
            (60, 91, 109),
            (15, 112, 113),
            (44, 117, 125),
            (88, 105, 137),
            (17, 144, 145),
            (24, 143, 145),
            (51, 140, 149),
            (85, 132, 157),
            (119, 120, 169),
            (52, 165, 173),
            (19, 180, 181),
            (57, 176, 185),
            (104, 153, 185),
            (95, 168, 193),
            (28, 195, 197),
            (84, 187, 205),
            (133, 156, 205),
            (21, 220, 221),
            (140, 171, 221),
            (60, 221, 229),
            (105, 208, 233),
            (120, 209, 241),
            (32, 255, 257),
            (23, 264, 265),
            (96, 247, 265),
            (69, 260, 269),
            (115, 252, 277),
            (160, 231, 281),
            (161, 240, 289),
            (68, 285, 293),
        ];

        for (i, t) in pythagorean().enumerate().take(1000) {
            if i < pythagoreans.len() {
                assert_eq!(pythagoreans[i], t)
            }
            let (a, b, c) = t;
            assert_eq!(a * a + b * b, c * c);
        }
    }

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
