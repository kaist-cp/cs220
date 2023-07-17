#[cfg(test)]
mod test {

    #[test]
    fn test_tv_room() {
        use crate::assignments::assignment11::tv_room::*;

        let tv_room = TVRoom::new();
        assert!(!tv_room.is_opened());

        // Turn on and add new guests.
        let manager = tv_room.open().unwrap();
        assert!(tv_room.is_opened());
        let guest1 = manager.new_guest();
        let guest2 = manager.new_guest();
        drop(manager);
        drop(guest1);
        assert!(tv_room.open().is_none());
        drop(guest2);
        assert!(!tv_room.is_opened());

        // Turn on and add new guests.
        let manager = tv_room.open().unwrap();
        assert!(tv_room.is_opened());
        let guest3 = manager.new_guest();
        drop(guest3);
        assert!(tv_room.is_opened());
        drop(manager);
        assert!(!tv_room.is_opened());
    }

    #[test]
    fn test_mock_storage() {
        use crate::assignments::assignment11::mock_storage::*;

        let mock_storage = MockStorage::new(100);

        let uploader1 = FileUploader::new(&mock_storage);
        let uploader2 = FileUploader::new(&mock_storage);

        let usage_analyzer = UsageAnalyzer::new(&mock_storage, 0.75);

        assert!(uploader1.upload("file1.txt", 20).is_ok());
        assert!(usage_analyzer.is_usage_under_bound());

        assert!(uploader2.upload("file2.txt", 30).is_ok());
        assert!(usage_analyzer.is_usage_under_bound());

        assert!(uploader1.upload("file3.txt", 40).is_ok());
        assert!(!usage_analyzer.is_usage_under_bound());

        assert_eq!(uploader2.upload("file4.txt", 50), Err(40));
        assert!(!usage_analyzer.is_usage_under_bound());

        assert!(uploader1.upload("file3.txt", 10).is_ok());
        assert!(usage_analyzer.is_usage_under_bound());
    }

    #[derive(Debug, PartialEq, Eq)]
    struct V(usize);

    #[test]
    fn test_linked_list() {
        use crate::assignments::assignment11::linked_list::*;

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
    fn test_graph() {
        use crate::assignments::assignment11::graph::*;

        let mut nodes = (0..6).map(NodeHandle::new).collect::<Vec<_>>();
        let edges = [
            (0, 1),
            (0, 3),
            (1, 4),
            (2, 4),
            (2, 5),
            (3, 1),
            (4, 3),
            (5, 5),
        ];

        for (from, to) in edges {
            assert!(nodes[from].add_edge(nodes[to].clone()).unwrap());
        }

        let mut graph1 = SubGraph::new();
        for n in 0..6 {
            assert!(graph1.add_node(nodes[n].clone()));
        }
        assert!(graph1.detect_cycle());
        assert!(!graph1.add_node(nodes[0].clone()));

        let mut graph2 = SubGraph::new();
        for n in [0, 1, 3] {
            assert!(graph2.add_node(nodes[n].clone()));
        }
        assert!(!graph2.detect_cycle());

        assert!(graph2.add_node(nodes[4].clone()));
        assert!(graph2.detect_cycle());

        assert!(nodes[4].remove_edge(&nodes[3]).unwrap());
        assert!(!graph2.detect_cycle());

        let mut graph3 = SubGraph::new();
        for n in [0, 1, 2, 3] {
            assert!(graph3.add_node(nodes[n].clone()));
        }
        assert!(!graph3.detect_cycle());

        let more_edges = [(1, 2), (2, 3)];
        for (from, to) in more_edges {
            assert!(nodes[from].add_edge(nodes[to].clone()).unwrap());
        }
        assert!(graph3.detect_cycle());

        assert!(graph3.remove_node(&nodes[2]));
        assert!(!graph3.detect_cycle());


        for n in nodes {
            n.clear_edges().unwrap();
        }
    }
}
