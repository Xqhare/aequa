use aequa::f64;

fn main() {
    // Standard floats have precision issues with 0.1 and 0.2
    let f1: std::primitive::f64 = 0.1;
    let f2: std::primitive::f64 = 0.2;
    println!("Standard f64: {} + {} = {}", f1, f2, f1 + f2);

    // Aequa's f64 (HpFloat) provides exact decimal precision
    let a1: f64 = 0.1.into();
    let a2: f64 = 0.2.into();
    let sum = a1 + a2;
    println!("Aequa f64:   {} + {} = {}", a1, a2, sum);

    assert_eq!(sum.to_string(), "0.3");

    // Mixing with primitives is also supported
    let result: f64 = sum * 2.5;
    println!("Aequa f64:   {} * 2.5 = {}", sum, result);

    assert_eq!(result.to_string(), "0.75");
}

#[test]
fn test_main() {
    main();
}
