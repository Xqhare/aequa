use aequa::{XffValue, graph::Graph};

fn main() {
    // 1. Create a new graph
    let mut graph = Graph::new();

    // 2. Add nodes (payload, metadata)
    // The first node added (index 0) is the "root" by convention.
    let root = graph.add_node(XffValue::from("root"), XffValue::from("The entry point"));
    let node1 = graph.add_node(XffValue::from("node1"), XffValue::Null);
    let node2 = graph.add_node(XffValue::from("node2"), XffValue::Null);

    println!(
        "Created graph with {} nodes",
        graph.get_all_nodes_indices().count()
    );

    // 3. Connect nodes
    // root -> node1
    // root -> node2
    graph
        .add_connection(root, node1, XffValue::from("Link A"))
        .unwrap();
    graph
        .add_connection(root, node2, XffValue::from("Link B"))
        .unwrap();

    // 4. Query relationships
    let children = graph.get_children(root);
    println!(
        "Root (idx {}) has {} children: {:?}",
        root,
        children.len(),
        children
    );

    for &child in &children {
        if let Some(node) = graph.get_node(child) {
            println!("  Child idx {}: payload={:?}", child, node.payload);
        }
    }

    // 5. Query parents
    let parents = graph.get_parents(node1);
    println!("Node 1 (idx {}) has parents: {:?}", node1, parents);
}

#[test]
fn test_main() {
    main();
}
