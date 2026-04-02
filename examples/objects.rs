use aequa::{Object, OrderedObject, XffValue};
use std::collections::HashMap;

fn main() {
    println!("--- Standard Object (BTreeMap based) ---");
    // Standard Objects use BTreeMap, so keys are always sorted alphabetically.
    let mut obj = Object::new();
    obj.insert("z_key", "last");
    obj.insert("a_key", "first");
    obj.insert("m_key", "middle");

    println!("Object (sorted keys): {}", obj);
    assert_eq!(obj.iter().next().unwrap().0, "a_key");

    // From HashMap
    let mut map = HashMap::new();
    map.insert("user", "xqhare");
    map.insert("role", "admin");
    map.insert("password", "secret");
    let obj_from_map = Object::from(map);
    println!("Object from HashMap: {}", obj_from_map);

    println!("\n--- Ordered Object (Vec based) ---");
    // Ordered Objects preserve the insertion order.
    let mut ordered = OrderedObject::new();
    ordered.push("z_key", "first inserted");
    ordered.push("a_key", "second inserted");
    ordered.push("m_key", "third inserted");

    println!("Ordered Object (preserves order): {}", ordered);
    assert_eq!(ordered.get_index(0).unwrap().0, "z_key");

    // Access by index
    let (first_key, first_val) = &ordered[0];
    println!("First item: {} -> {}", first_key, first_val);

    // Access by key (O(n))
    if let Some(val) = ordered.get("a_key") {
        println!("Value for 'a_key': {}", val);
    }

    // Convert to XffValue
    let val = XffValue::from(ordered);
    assert!(val.is_ordered_object());
    println!("XffValue variant: {}", val);
}

#[test]
fn test_main() {
    main();
}
