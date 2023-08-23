//! Test cases for assignment11/graph.rs

#[cfg(test)]
mod test_graph {
    use crate::assignments::assignment11::graph::*;

    #[test]
    fn test_graph() {
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
        (0..6).for_each(|n| {
            assert!(graph1.add_node(nodes[n].clone()));
        });
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
