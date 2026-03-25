use aequa::f64;

#[test]
fn test_div_precision() {
    let a: f64 = 1.0.into();
    let b: f64 = 10.0.into();
    let res = (a / b).trim_scale();
    // 1.0 / 10.0 = 0.1
    assert_eq!(res.to_string(), "0.1");
}

#[test]
fn test_div_underflow_no_panic() {
    let a: f64 = 10.0.into(); // scale 0
    let b: f64 = 0.1.into(); // scale 1
    let res = (a / b).trim_scale();
    // 10 / 0.1 = 100
    assert_eq!(res.to_string(), "100");
}

#[test]
fn test_complex_div() {
    let a: f64 = 1.0.into();
    let b: f64 = 3.0.into();
    let res = a / b;
    // 1 / 3 = 0.33333333333333333333 (20 digits)
    let s = res.to_string();
    assert!(s.starts_with("0.3333333333"));
    assert_eq!(s.len(), 22); // "0." + 20 digits
}
