use aequa::{
    XffValue,
    graph::{Graph, GraphError},
};

#[test]
fn test_graph_add_remove_nodes() {
    let mut graph = Graph::new();

    let n0 = graph.add_node(XffValue::from("root"), XffValue::Null);
    let n1 = graph.add_node(XffValue::from("node1"), XffValue::Null);
    let n2 = graph.add_node(XffValue::from("node2"), XffValue::Null);

    assert_eq!(n0, 0);
    assert_eq!(n1, 1);
    assert_eq!(n2, 2);

    assert!(graph.get_node(n1).is_some());

    graph.remove_node(n1);
    assert!(graph.get_node(n1).is_none());

    // New node should reuse index 1
    let n3 = graph.add_node(XffValue::from("node3"), XffValue::Null);
    assert_eq!(n3, 1);
    assert_eq!(
        graph.get_node(n3).unwrap().payload.as_string().unwrap(),
        "node3"
    );
}

#[test]
fn test_graph_root_protection() {
    let mut graph = Graph::new();
    let n0 = graph.add_node(XffValue::from("root"), XffValue::Null);
    assert_eq!(n0, 0);

    // Try to remove root
    graph.remove_node(0);
    assert!(graph.has_node(0));
    assert_eq!(
        graph.get_node(0).unwrap().payload.as_string().unwrap(),
        "root"
    );
}

#[test]
fn test_graph_add_connection_validation() {
    let mut graph = Graph::new();
    let n0 = graph.add_node(XffValue::from("node0"), XffValue::Null);

    // Valid connection
    let n1 = graph.add_node(XffValue::from("node1"), XffValue::Null);
    let c0 = graph.add_connection(n0, n1, XffValue::Null).unwrap();
    assert_eq!(c0, 0);

    // Invalid from node
    let res = graph.add_connection(99, n1, XffValue::Null);
    assert_eq!(res, Err(GraphError::NodeNotFound(99)));

    // Invalid to node
    let res = graph.add_connection(n0, 99, XffValue::Null);
    assert_eq!(res, Err(GraphError::NodeNotFound(99)));
}

#[test]
fn test_graph_connections_cleanup() {
    let mut graph = Graph::new();

    let n0 = graph.add_node(XffValue::from("node0"), XffValue::Null);
    let n1 = graph.add_node(XffValue::from("node1"), XffValue::Null);

    let c0 = graph.add_connection(n0, n1, XffValue::Null).unwrap();
    assert!(graph.get_connection(c0).is_some());

    // Check tracking
    assert_eq!(graph.get_node(n0).unwrap().outbound_connections, vec![c0]);
    assert_eq!(graph.get_node(n1).unwrap().inbound_connections, vec![c0]);

    // Removing a node should remove its connections
    graph.remove_node(n1);
    assert!(graph.get_connection(c0).is_none());

    // Check tracking update in n0
    assert!(graph.get_node(n0).unwrap().outbound_connections.is_empty());

    // Re-adding n1 (should get index 1 again)
    let n1_new = graph.add_node(XffValue::from("node1_new"), XffValue::Null);
    assert_eq!(n1_new, 1);

    // c0 index should be free now
    let c1 = graph.add_connection(n0, n1_new, XffValue::Null).unwrap();
    assert_eq!(c1, 0);
}

#[test]
fn test_graph_bfs() {
    let mut graph = Graph::new();

    // Create a simple diamond: 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3
    let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    let n1 = graph.add_node(XffValue::from("1"), XffValue::Null);
    let n2 = graph.add_node(XffValue::from("2"), XffValue::Null);
    let n3 = graph.add_node(XffValue::from("3"), XffValue::Null);

    graph.add_connection(n0, n1, XffValue::Null).unwrap();
    graph.add_connection(n0, n2, XffValue::Null).unwrap();
    graph.add_connection(n1, n3, XffValue::Null).unwrap();
    graph.add_connection(n2, n3, XffValue::Null).unwrap();

    let bfs = graph.traverse_bfs(n0);
    assert_eq!(bfs.len(), 4);
    assert_eq!(bfs[0], n0);
    // BFS layer 1
    assert!(bfs[1] == n1 || bfs[1] == n2);
    assert!(bfs[2] == n1 || bfs[2] == n2);
    // BFS layer 2
    assert_eq!(bfs[3], n3);
}

#[test]
fn test_graph_dfs_and_reachability() {
    let mut graph = Graph::new();

    // 0 -> 1 -> 2 -> 3
    let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    let n1 = graph.add_node(XffValue::from("1"), XffValue::Null);
    let n2 = graph.add_node(XffValue::from("2"), XffValue::Null);
    let n3 = graph.add_node(XffValue::from("3"), XffValue::Null);

    graph.add_connection(n0, n1, XffValue::Null).unwrap();
    graph.add_connection(n1, n2, XffValue::Null).unwrap();
    graph.add_connection(n2, n3, XffValue::Null).unwrap();

    let dfs = graph.traverse_dfs(n0);
    assert_eq!(dfs, vec![0, 1, 2, 3]);

    assert!(graph.is_reachable(n0, n3));
    assert!(graph.is_reachable(n1, n3));
    assert!(!graph.is_reachable(n3, n0));
}

#[test]
fn test_graph_cycle_detection() {
    let mut graph = Graph::new();

    let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    let n1 = graph.add_node(XffValue::from("1"), XffValue::Null);
    let n2 = graph.add_node(XffValue::from("2"), XffValue::Null);

    graph.add_connection(n0, n1, XffValue::Null).unwrap();
    graph.add_connection(n1, n2, XffValue::Null).unwrap();

    assert!(!graph.has_cycle());

    // Create cycle: 2 -> 0
    graph.add_connection(n2, n0, XffValue::Null).unwrap();
    assert!(graph.has_cycle());
}

#[test]
fn test_graph_pathfinding() {
    let mut graph = Graph::new();

    // 0 -> 1 -> 2 -> 3
    let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    let n1 = graph.add_node(XffValue::from("1"), XffValue::Null);
    let n2 = graph.add_node(XffValue::from("2"), XffValue::Null);
    let n3 = graph.add_node(XffValue::from("3"), XffValue::Null);

    graph.add_connection(n0, n1, XffValue::Null).unwrap();
    graph.add_connection(n1, n2, XffValue::Null).unwrap();
    graph.add_connection(n2, n3, XffValue::Null).unwrap();

    let path = graph.find_path(n0, n3);
    assert_eq!(path, Some(vec![n0, n1, n2, n3]));

    let no_path = graph.find_path(n3, n0);
    assert_eq!(no_path, None);
}

#[test]
fn test_graph_helpers() {
    let mut graph = Graph::new();

    let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    let n1 = graph.add_node(XffValue::from("1"), XffValue::Null);

    assert!(graph.is_root_node(n0));
    assert!(graph.is_leaf(n0));

    graph.add_connection(n0, n1, XffValue::Null).unwrap();

    assert_eq!(graph.in_degree(n1), Some(1));
    assert_eq!(graph.out_degree(n0), Some(1));
    assert!(!graph.is_leaf(n0));
    assert!(!graph.is_root_node(n1));

    let conns = graph.find_connections(n0, n1);
    assert_eq!(conns.len(), 1);
    assert_eq!(conns[0], 0);
}

#[test]
fn test_graph_sccs() {
    let mut graph = Graph::new();

    // SCC 1: 0 <-> 1
    let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    let n1 = graph.add_node(XffValue::from("1"), XffValue::Null);
    graph.add_connection(n0, n1, XffValue::Null).unwrap();
    graph.add_connection(n1, n0, XffValue::Null).unwrap();

    // SCC 2: 2 (isolated)
    let _n2 = graph.add_node(XffValue::from("2"), XffValue::Null);

    let sccs = graph.find_sccs();
    assert_eq!(sccs.len(), 2);

    // One SCC should contain 0 and 1, the other just 2
    let mut found_pair = false;
    let mut found_single = false;

    for scc in sccs {
        if scc.len() == 2 {
            assert!(scc.contains(&0) && scc.contains(&1));
            found_pair = true;
        } else if scc.len() == 1 {
            assert!(scc.contains(&2));
            found_single = true;
        }
    }

    assert!(found_pair && found_single);
}

#[test]
fn test_graph_export_topo() {
    let mut graph = Graph::new();

    // a -> b -> c
    let na = graph.add_node(XffValue::from("a"), XffValue::Null);
    let nb = graph.add_node(XffValue::from("b"), XffValue::Null);
    let nc = graph.add_node(XffValue::from("c"), XffValue::Null);

    graph.add_connection(na, nb, XffValue::Null).unwrap();
    graph.add_connection(nb, nc, XffValue::Null).unwrap();

    let export = graph.export_for_topological_sort();

    // Expect: [("0", []), ("1", ["0"]), ("2", ["1"])]
    assert_eq!(export.len(), 3);

    for (name, parents) in export {
        match name.as_str() {
            "0" => assert!(parents.is_empty()),
            "1" => assert_eq!(parents, vec!["0"]),
            "2" => assert_eq!(parents, vec!["1"]),
            _ => panic!("Unexpected node name: {}", name),
        }
    }
}
