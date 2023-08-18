//! Test cases for assignment11/bst.rs

#[cfg(test)]
mod test_bst {
    use super::super::bst::*;

    #[test]
    fn bst_insert_test() {
        let mut tree = Tree::new();

        let _ = tree.insert(1);
        let _ = tree.insert(5);
        let _ = tree.insert(3);
        let _ = tree.insert(7);

        assert_eq!(tree.into_iter().collect::<Vec<_>>(), vec![1, 3, 5, 7]);
    }

    #[test]
    fn bst_remove_test() {
        let mut tree = Tree::new();

        let _ = tree.insert(1);
        let _ = tree.insert(5);
        let _ = tree.insert(3);
        let _ = tree.insert(7);
        let _ = tree.remove(&7);

        assert_eq!(tree.into_iter().collect::<Vec<_>>(), vec![1, 3, 5]);
    }

    #[test]
    fn bst_complex_test() {
        let mut tree = Tree::new();

        let _ = tree.insert(1);
        let _ = tree.insert(5);
        let _ = tree.insert(3);
        let _ = tree.insert(7);
        let _ = tree.remove(&7);
        let _ = tree.insert(7);
        let _ = tree.insert(6);
        let _ = tree.insert(8);
        let _ = tree.remove(&5);
        let _ = tree.remove(&1);
        let _ = tree.remove(&3);
        let _ = tree.remove(&7);
        let _ = tree.remove(&6);
        let _ = tree.remove(&8);

        assert_eq!(tree.into_iter().collect::<Vec<_>>(), vec![]);
    }
}
