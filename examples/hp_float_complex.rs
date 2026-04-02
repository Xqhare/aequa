use aequa::hp_float::HpFloat;

fn main() {
    // A high-precision geometry calculation within i128 limits.
    // Total digits in a multiplication (excluding leading zeros) should stay under ~38.

    // Calculate area of a circle with a fairly precise radius
    let radius = HpFloat::from(1234.5678); // 8 digits
    println!("Radius: {}", radius);

    // Let's just use a string-parsed PI with 15 digits.
    let pi_15: HpFloat = "3.14159265358979".parse().unwrap();
    println!("PI (15 digits): {}", pi_15);

    let area = (pi_15 * radius * radius).trim_scale();
    println!("Area: {}", area);

    // Demonstrating very long addition (safe from overflow since it only adds digits at the end or matches scale)
    let mut sum = HpFloat::from(0.0);
    let step = HpFloat::from(0.0000000000001); // 10^-13
    println!("Adding {} for 1000 steps...", step);
    for _ in 0..1000 {
        sum = sum + step;
    }
    println!("Total sum: {}", sum);
    assert_eq!(sum.to_string(), "0.0000000001");

    // Mixing different scales
    let a = HpFloat::new(1, 20); // 10^-20
    let b = HpFloat::from(100.0);
    let c = a + b;
    println!("100.0 + 10^-20 = {}", c);

    // Parsing high precision from string
    let high_p: HpFloat = "123.45678901234567890123456789".parse().unwrap();
    println!("High precision parsed: {}", high_p);
    println!(
        "Value: {}, Scale: {}",
        high_p.get_value(),
        high_p.get_scale()
    );
}

#[test]
fn test_main() {
    main();
}
