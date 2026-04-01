use aequa::hp_float::f64;

#[test]
fn test_basic_addition() {
    let a: f64 = 0.1.into();
    let b: f64 = 0.2.into();
    let sum = a + b;

    // In Aequa, 0.1 + 0.2 should be exactly 0.3
    assert_eq!(sum, 0.3.into());
}

#[test]
fn test_literal_addition() {
    let a: f64 = 0.1.into();
    let sum = a + 0.2; // Uses Add<f64> for Aequa

    assert_eq!(sum, 0.3.into());
}

#[test]
fn test_multiplication() {
    let a: f64 = 1.2.into();
    let b: f64 = 3.0.into();
    let res = a * b;
    assert_eq!(res.to_string(), "3.6");

    let c: f64 = 0.01.into();
    let d: f64 = 0.02.into();
    let res2 = (c * d).trim_scale();
    assert_eq!(res2.to_string(), "0.0002");
}

#[test]
fn test_comparisons() {
    let a: f64 = 0.1.into();
    let b: f64 = 0.2.into();
    let c: f64 = 0.1.into();

    assert!(a < b);
    assert!(b > a);
    assert_eq!(a, c);
    assert!(a <= c);
    assert!(a >= c);
    assert_ne!(a, b);
}

#[test]
fn test_negative_numbers() {
    let a: f64 = (-0.1).into();
    let b: f64 = 0.2.into();
    assert_eq!((a + b).to_string(), "0.1");
    assert_eq!((a - b).to_string(), "-0.3");
    assert_eq!((a * b).to_string(), "-0.02");
}
