use aequa::{Data, Metadata, Object, XffValue};

fn main() {
    // 1. Create a complex nested structure
    // {
    //   "project": "Aequa",
    //   "tags": ["rust", "precision", "data"],
    //   "stats": { "stars": 100, "active": true }
    // }

    let mut stats = Object::new();
    stats.insert("stars", 100);
    stats.insert("active", true);

    let mut root_obj = Object::new();
    root_obj.insert("project", "Aequa");
    root_obj.insert("tags", vec!["rust", "precision", "data"]);
    root_obj.insert("stats", stats);

    let val = XffValue::from(root_obj);
    println!("Complex Object: {}", val);

    // 2. Metadata (for file-level information)
    let mut meta = Metadata::new();
    meta.set_creator("Xqhare".to_string());
    meta.set_description("Example metadata".to_string());

    let meta_val = XffValue::from(meta);
    if let Some(m) = meta_val.as_metadata() {
        println!("Creator: {}", m.get_creator().unwrap_or_default());
    }

    // 3. Binary Data
    let binary = Data::from(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    let data_val = XffValue::from(binary);
    println!(
        "Binary Data length: {} bytes",
        data_val.as_data().unwrap().len()
    );

    // 4. Accessing nested data
    if let Some(obj) = val.as_object() {
        if let Some(tags) = obj.get("tags").and_then(|v| v.as_array()) {
            println!("First tag: {}", tags[0]);
        }
    }

    // 5. Ordered Object (preserves insertion order)
    let mut ordered = aequa::OrderedObject::new();
    ordered.insert("first", 1);
    ordered.insert("second", 2);
    ordered.insert("third", 3);
    let ordered_val = XffValue::from(ordered);
    println!("Ordered Object: {}", ordered_val);
}

#[test]
fn test_main() {
    main();
}
