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
    fn test_from_into_vec() {
        assert_eq!(SinglyLinkedList::<i32>::new().into_vec(), vec![]);
        assert_eq!(
            SinglyLinkedList::from_vec(vec![1, 2, 3]).into_vec(),
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
        let list = SinglyLinkedList::from_vec(vec![1, 2, 3]);
        let list_ = list.map(|x: i32| x + 1);
        assert_eq!(list_.into_vec(), vec![2, 3, 4]);
    }

    #[test]
    fn test_pair_map() {
        let add = |x: i32, y: i32| x + y;

        let list1 = SinglyLinkedList::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).pair_map(add);
        let vec1 = list1.into_vec();
        assert_eq!(vec1.clone(), vec![3, 5, 7, 9, 11, 13, 15, 17]);

        let list2 = SinglyLinkedList::from_vec(vec1).pair_map(add);
        let vec2 = list2.into_vec();
        assert_eq!(vec2.clone(), vec![8, 12, 16, 20, 24, 28, 32]);

        let list3 = SinglyLinkedList::from_vec(vec2).pair_map(add);
        let vec3 = list3.into_vec();
        assert_eq!(vec3.clone(), vec![20, 28, 36, 44, 52, 60]);

        let list4 = SinglyLinkedList::from_vec(vec3).pair_map(add);
        assert_eq!(list4.into_vec(), vec![48, 64, 80, 96, 112]);
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
            list_list.flatten().into_vec(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
    }
}
