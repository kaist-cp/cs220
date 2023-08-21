#[cfg(test)]
mod test_linked_list {
    use crate::assignments::assignment11::linked_list::*;

    #[derive(Debug, PartialEq, Eq)]
    struct V(usize);

    #[test]
    fn test_push_pop() {
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

    #[test]
    fn test_from_as_vec() {
        assert_eq!(SinglyLinkedList::<i32>::new().as_vec(), vec![]);
        assert_eq!(
            SinglyLinkedList::from_vec(vec![1, 2, 3]).as_vec(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_length() {
        let list = SinglyLinkedList::from_vec(vec![1, 2, 3]);
        assert_eq!(list.length(), 3);
    }

    #[test]
    fn test_map() {
        let mut list = SinglyLinkedList::from_vec(vec![1, 2, 3]);
        let incr = |x: i32| x + 1;
        list.map(incr);
        assert_eq!(list.as_vec(), vec![2, 3, 4]);
    }

    #[test]
    fn test_insert() {
        let mut list1 = SinglyLinkedList::from_vec(vec![1, 2, 3]);
        let mut list2 = SinglyLinkedList::from_vec(vec![1, 2, 3]);
        let mut list3 = SinglyLinkedList::from_vec(vec![1, 2, 3]);
        let list4 = SinglyLinkedList::from_vec(vec![4, 5, 6]);

        list1.insert(&list4, 0);
        assert_eq!(list1.as_vec(), vec![4, 5, 6, 1, 2, 3]);

        list2.insert(&list4, 1);
        assert_eq!(list2.as_vec(), vec![1, 4, 5, 6, 2, 3]);

        list3.insert(&list4, 4);
        assert_eq!(list3.as_vec(), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_chunk_reverse() {
        let mut list1 = SinglyLinkedList::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        list1.chunk_reverse(3);
        assert_eq!(list1.as_vec(), vec![7, 8, 9, 3, 4, 5, 1, 2, 3]);

        let mut list2 = SinglyLinkedList::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        list2.chunk_reverse(3);
        assert_eq!(list2.as_vec(), vec![7, 8, 4, 5, 6, 1, 2, 3]);

        let mut list3 = SinglyLinkedList::from_vec(vec![1, 2, 3]);
        list3.chunk_reverse(4);
        assert_eq!(list3.as_vec(), vec![1, 2, 3]);

        let mut list4 = SinglyLinkedList::from_vec(vec![1, 2, 3, 4]);
        list4.chunk_reverse(1);
        assert_eq!(list4.as_vec(), vec![4, 3, 2, 1]);

        let mut list5 = SinglyLinkedList::from_vec(vec![1, 2, 3, 4]);
        list4.chunk_reverse(0);
        assert_eq!(list4.as_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_pair_map() {
        let mut list = SinglyLinkedList::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let add = |x: i32, y: i32| x + y;

        list.pair_map(add);
        assert_eq!(list.as_vec(), vec![3, 5, 7, 9, 11, 13, 15, 17]);

        list.pair_map(add);
        assert_eq!(list.as_vec(), vec![8, 12, 16, 20, 24, 28, 32]);

        list.pair_map(add);
        assert_eq!(list.as_vec(), vec![20, 28, 36, 44, 52, 60]);

        list.pair_map(add);
        assert_eq!(list.as_vec(), vec![48, 64, 80, 96, 112]);
    }

    #[test]
    fn test_flatten() {
        let list1 = SinglyLinkedList::from_vec(vec![1, 2]);
        let list2 = SinglyLinkedList::from_vec(vec![3]);
        let list3 = SinglyLinkedList::from_vec(vec![4, 5, 6, 7]);
        let list4 = SinglyLinkedList::<i32>::new();
        let list5 = SinglyLinkedList::from_vec(vec![8, 9, 10]);

        let list_list = SinglyLinkedList::from_vec(vec![list1, list2, list3, list4, list5]);

        assert_eq!(
            list_list.flatten().as_vec(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
    }
}
