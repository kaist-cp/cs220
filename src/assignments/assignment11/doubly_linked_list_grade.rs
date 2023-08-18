//! Test cases for assignment11/doubly_linked_list.rs

#[cfg(test)]
mod test_doubly_linked_list {
    use super::super::doubly_linked_list::*;

    #[test]
    fn test_works() {
        let mut list = DoublyLinkedList::new();

        list.push_back(3);
        list.push_back(4);
        assert_eq!(list.pop_front(), Some(3));

        list.push_front(5);
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_can_push_back() {
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.pop_back(), None);

        list.push_back(3);
        list.push_back(4);
        list.push_back(5);
        assert_eq!(list.pop_back(), Some(5));

        list.push_back(6);
        list.push_back(7);
        assert_eq!(list.pop_back(), Some(7));
        assert_eq!(list.pop_back(), Some(6));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(3));

        list.push_back(2);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), None);
    }
}
