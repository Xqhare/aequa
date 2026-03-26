use aequa::hp_float::{PI, f64};

#[test]
fn test_pi() {
    let pi = PI;
    // Should be 3.1415926535...
    assert!(pi.to_string().starts_with("3.1415926535"));
}

#[test]
fn test_negation() {
    let a: f64 = 0.5.into();
    let b = -a;
    assert_eq!(b.to_string(), "-0.5");
}

#[test]
fn test_subtraction() {
    let a: f64 = 0.5.into();
    let b: f64 = 0.2.into();
    let res = a - b;
    assert_eq!(res.to_string(), "0.3");
}
