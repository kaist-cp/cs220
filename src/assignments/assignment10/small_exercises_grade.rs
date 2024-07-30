#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::assignments::assignment10::small_exercises::*;

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

    #[test]
    fn test_remove_even() {
        let mut vec = vec![1, 2, 3, 4, 5];
        remove_even(&mut vec);
        assert_eq!(*vec, vec![1, 3, 5]);

        let mut vec = vec![11, 1000, 12, 101, 105, 104, 200];
        remove_even(&mut vec);
        assert_eq!(*vec, vec![11, 101, 105]);
    }

    #[test]
    fn test_remove_duplicate() {
        let mut vec = vec![1, 2, 1, 1, 3, 7, 5, 7];
        remove_duplicate(&mut vec);

        let set1: HashSet<i64> = HashSet::from_iter(vec);
        let set2: HashSet<i64> = HashSet::from_iter(vec![1, 2, 3, 7, 5]);
        assert_eq!(set1, set2);
    }

    #[test]
    fn test_natural_join() {
        let row1: Vec<String> = ["20230001", "Jack"].iter().map(|s| s.to_string()).collect();
        let row2: Vec<String> = ["20231234", "Mike"].iter().map(|s| s.to_string()).collect();
        let table1 = vec![row1, row2];
        let row1: Vec<String> = ["20230001", "CS"].iter().map(|s| s.to_string()).collect();
        let row2: Vec<String> = ["20230001", "EE"].iter().map(|s| s.to_string()).collect();
        let row3: Vec<String> = ["20231234", "ME"].iter().map(|s| s.to_string()).collect();
        let table2 = vec![row1, row2, row3];
        let row1: Vec<String> = ["20230001", "Jack", "CS"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let row2: Vec<String> = ["20230001", "Jack", "EE"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let row3: Vec<String> = ["20231234", "Mike", "ME"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let table3 = vec![row1, row2, row3];

        assert_eq!(
            HashSet::<Vec<String>>::from_iter(natural_join(table1, table2)),
            HashSet::<Vec<String>>::from_iter(table3)
        );

        let row1: Vec<String> = ["20230001", "Alice"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let row2: Vec<String> = ["20230002", "Bob"].iter().map(|s| s.to_string()).collect();
        let row3: Vec<String> = ["20230003", "Charlie"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let row4: Vec<String> = ["20230004", "David"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let table1 = vec![row1, row2, row3, row4];
        let row1: Vec<String> = ["20230001", "Apple"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let row2: Vec<String> = ["20230001", "Avocado"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let row3: Vec<String> = ["20230002", "Banana"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let row4: Vec<String> = ["20230002", "Berries"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let row5: Vec<String> = ["20230004", "Durian"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let table2 = vec![row1, row2, row3, row4, row5];
        let row1: Vec<String> = ["20230001", "Alice", "Apple"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let row2: Vec<String> = ["20230001", "Alice", "Avocado"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let row3: Vec<String> = ["20230002", "Bob", "Banana"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let row4: Vec<String> = ["20230002", "Bob", "Berries"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let row5: Vec<String> = ["20230004", "David", "Durian"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let table3 = vec![row1, row2, row3, row4, row5];

        assert_eq!(
            HashSet::<Vec<String>>::from_iter(natural_join(table1, table2)),
            HashSet::<Vec<String>>::from_iter(table3)
        );
    }

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

        for (i, (a, b, c)) in pythagorean().enumerate().take(1000) {
            if i < pythagoreans.len() {
                assert_eq!(pythagoreans[i], (a, b, c))
            }
            assert_eq!(a * a + b * b, c * c);
        }
    }
}
