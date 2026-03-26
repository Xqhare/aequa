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
