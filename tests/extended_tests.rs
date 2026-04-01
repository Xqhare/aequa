use aequa::hp_float::f64;
use aequa::value::{Table, OrderedObject, XffValue};

#[test]
fn test_hp_float_scientific_notation() {
    // Current implementation falls back to f64 for scientific notation.
    let a: f64 = "1.23e-2".parse().unwrap();
    assert_eq!(a.to_string(), "0.0123");

    let b: f64 = "1.23e+2".parse().unwrap();
    assert_eq!(b.to_string(), "123");
}

#[test]
fn test_hp_float_large_numbers() {
    // Test that large numbers still work correctly (within i128 limits)
    let a: f64 = "1000000000000".parse().unwrap();
    let b: f64 = "1000000000000".parse().unwrap();
    let res = a * b;
    // 10^12 * 10^12 = 10^24
    assert_eq!(res.to_string(), "1000000000000000000000000");
}

#[test]
fn test_hp_float_trim_scale_edge_cases() {
    let a = f64::new(100, 3); // 0.100
    assert_eq!(a.to_string(), "0.1");
    assert_eq!(a.trim_scale().get_scale(), 1);

    let b = f64::new(0, 5); // 0.00000
    assert_eq!(b.to_string(), "0");
    assert_eq!(b.trim_scale().get_scale(), 0);
}

#[test]
fn test_table_row_column_mismatch() {
    let mut table = Table::with_columns(vec!["A".to_string(), "B".to_string()]);
    // Valid row
    assert!(table.add_row(vec![XffValue::from(1), XffValue::from(2)]).is_ok());
    // Invalid row (too few)
    assert!(table.add_row(vec![XffValue::from(3)]).is_err());
    // Invalid row (too many)
    assert!(table.add_row(vec![XffValue::from(4), XffValue::from(5), XffValue::from(6)]).is_err());
}

#[test]
fn test_ordered_object_index_access() {
    let mut obj = OrderedObject::new();
    obj.push("key1", 1);
    obj.push("key2", 2);

    assert_eq!(obj[0].0, "key1");
    assert_eq!(obj[0].1, XffValue::from(1));
    assert_eq!(obj[1].0, "key2");
    assert_eq!(obj[1].1, XffValue::from(2));

    assert_eq!(obj["key1"], XffValue::from(1));
    assert_eq!(obj["key2"], XffValue::from(2));
}

#[test]
fn test_xff_value_nan_infinity_display() {
    let nan = XffValue::NaN;
    let inf = XffValue::Infinity;
    let ninf = XffValue::NegInfinity;

    assert_eq!(format!("{}", nan), "NaN");
    assert_eq!(format!("{}", inf), "Infinity");
    assert_eq!(format!("{}", ninf), "NegInfinity");

    // Equality
    assert_eq!(nan, XffValue::NaN);
    assert_eq!(inf, XffValue::Infinity);
    assert_eq!(ninf, XffValue::NegInfinity);
    assert_ne!(nan, inf);
}

#[test]
fn test_graph_remove_connection_explicit() {
    use aequa::graph::Graph;
    let mut graph = Graph::new();
    let n0 = graph.add_node(XffValue::from("0"), XffValue::Null);
    let n1 = graph.add_node(XffValue::from("1"), XffValue::Null);

    let c0 = graph.add_connection(n0, n1, XffValue::Null).unwrap();
    assert!(graph.get_connection(c0).is_some());

    graph.remove_connection(c0);
    assert!(graph.get_connection(c0).is_none());
    assert!(graph.get_node(n0).unwrap().outbound_connections.is_empty());
    assert!(graph.get_node(n1).unwrap().inbound_connections.is_empty());
}

#[test]
fn test_xff_value_indexing() {
    // Array indexing
    let arr = XffValue::from(vec![1, 2, 3]);
    assert_eq!(arr[0], XffValue::from(1));
    assert_eq!(arr[1], XffValue::from(2));
    assert_eq!(arr[2], XffValue::from(3));

    // Object indexing
    let mut obj = aequa::value::Object::new();
    obj.insert("a", 1);
    let obj_val = XffValue::Object(obj);
    assert_eq!(obj_val["a"], XffValue::from(1));

    // OrderedObject indexing
    let mut ord_obj = OrderedObject::new();
    ord_obj.push("first", "apple");
    let ord_obj_val = XffValue::OrderedObject(ord_obj);
    assert_eq!(ord_obj_val[0], XffValue::from("apple"));
    assert_eq!(ord_obj_val["first"], XffValue::from("apple"));
}
