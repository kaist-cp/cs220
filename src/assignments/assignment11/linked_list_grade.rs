//! Test cases for assignment11/linked_list.rs

#[cfg(test)]
mod test_linked_list {
    use super::super::linked_list::*;

    #[derive(Debug, PartialEq, Eq)]
    struct V(usize);

    #[test]
    fn test_linked_list() {
        let mut list = SinglyLinkedList::new();
        list.push_back(V(3));
        list.push_front(V(2));
        list.push_back(V(4));
        list.push_front(V(1));
        list.push_back(V(5));

        assert_eq!(list.pop_front(), Some(V(1)));
        assert_eq!(list.pop_back(), Some(V(5)));
        assert_eq!(list.pop_front(), Some(V(2)));
        assert_eq!(list.pop_back(), Some(V(4)));
        assert_eq!(list.pop_front(), Some(V(3)));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
    }
}
