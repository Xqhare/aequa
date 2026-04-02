use aequa::{XffValue, graph::Graph};

fn main() {
    // A more complex example: a simple "Skill Tree"
    // demonstrating BFS/DFS traversals and node removal.

    let mut skill_tree = Graph::new();

    // 1. Define nodes
    let root = skill_tree.add_node(XffValue::from("Basics"), XffValue::Null); // idx 0
    let smithing =
        skill_tree.add_node(XffValue::from("Smithing"), XffValue::from("Prereq: Basics")); // idx 1
    let mining = skill_tree.add_node(XffValue::from("Mining"), XffValue::from("Prereq: Basics")); // idx 2
    let smelting = skill_tree.add_node(
        XffValue::from("Smelting"),
        XffValue::from("Prereq: Smithing, Mining"),
    ); // idx 3
    let crafting =
        skill_tree.add_node(XffValue::from("Crafting"), XffValue::from("Prereq: Basics")); // idx 4

    // 2. Add dependencies
    skill_tree
        .add_connection(root, smithing, XffValue::Null)
        .unwrap();
    skill_tree
        .add_connection(root, mining, XffValue::Null)
        .unwrap();
    skill_tree
        .add_connection(root, crafting, XffValue::Null)
        .unwrap();

    // Smelting requires both Smithing and Mining
    skill_tree
        .add_connection(smithing, smelting, XffValue::Null)
        .unwrap();
    skill_tree
        .add_connection(mining, smelting, XffValue::Null)
        .unwrap();

    // 3. Traversal: Breadth-First Search (Level-by-level processing)
    println!("--- BFS: Skills to process in levels ---");
    let bfs_order = skill_tree.traverse_bfs(root);
    for idx in bfs_order {
        if let Some(node) = skill_tree.get_node(idx) {
            println!("Level processing idx {}: {}", idx, node.payload);
        }
    }

    // 4. Traversal: Depth-First Search (Deep dive into dependencies)
    println!("\n--- DFS: Deep dependency exploration ---");
    let dfs_order = skill_tree.traverse_dfs(root);
    for idx in dfs_order {
        if let Some(node) = skill_tree.get_node(idx) {
            println!("Exploring idx {}: {}", idx, node.payload);
        }
    }

    // 5. Node Removal and Index Stability
    println!("\n--- Node Removal ---");
    println!("Removing skill 'Mining' (idx {})...", mining);
    skill_tree.remove_node(mining);

    // The connections involving 'Mining' should be gone.
    // 'Smelting' now has fewer parents.
    let smelting_parents = skill_tree.get_parents(smelting);
    println!(
        "'Smelting' now has {} parent(s): {:?}",
        smelting_parents.len(),
        smelting_parents
    );

    // Indices are stable: 'Smelting' is still at index 3.
    assert!(skill_tree.has_node(smelting));
    assert_eq!(smelting, 3);

    // Add a new node - it should reuse index 2 ('Mining's old index).
    let cooking = skill_tree.add_node(XffValue::from("Cooking"), XffValue::Null);
    println!("Added 'Cooking'. It received index {}.", cooking);
    assert_eq!(cooking, 2);
}

#[test]
fn test_main() {
    main();
}
