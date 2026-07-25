use aequa::hp_float::f64;
use std::collections::{HashMap, HashSet};

#[test]
fn test_ordering_different_scales() {
    let a: f64 = 0.2.into();  // 2|1
    let b: f64 = 0.15.into(); // 15|2

    // 0.2 is greater than 0.15
    assert!(a > b);
    assert!(b < a);

    let c = f64::new(1, 0);  // 1.0
    let d = f64::new(10, 1); // 1.00 -> canonicalized to 1|0

    assert_eq!(c, d);
    assert!(c <= d);
    assert!(c >= d);
}

#[test]
fn test_equality_and_hashing() {
    let a = f64::new(10, 1);
    let b = f64::new(1, 0);

    assert_eq!(a, b);

    // Verify Hash invariant: equal items must produce identical hashes
    let mut set = HashSet::new();
    set.insert(a);

    assert!(set.contains(&b));

    let mut map = HashMap::new();
    map.insert(a, "one");
    assert_eq!(map.get(&b), Some(&"one"));
}

#[test]
fn test_zero_canonicalization() {
    let z1 = f64::new(0, 5);
    let z2 = f64::new(0, 0);

    assert_eq!(z1, z2);
    assert_eq!(z1.get_scale(), 0);
    assert_eq!(z2.get_scale(), 0);
}

#[test]
fn test_sorting_mixed_scales() {
    let mut numbers = vec![
        f64::new(2, 1),   // 0.2
        f64::new(15, 2),  // 0.15
        f64::new(10, 1),  // 1.0
        f64::new(5, 2),   // 0.05
        f64::new(-1, 1),  // -0.1
    ];

    numbers.sort();

    let expected = vec![
        f64::new(-1, 1),  // -0.1
        f64::new(5, 2),   // 0.05
        f64::new(15, 2),  // 0.15
        f64::new(2, 1),   // 0.2
        f64::new(1, 0),   // 1.0
    ];

    assert_eq!(numbers, expected);
}
