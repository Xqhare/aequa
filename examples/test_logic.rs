use aequa::f64;
use std::str::FromStr;

fn main() {
    // Test 1: Scientific notation in From
    let small: f64 = 0.0000000001.into();
    println!("0.0000000001 as Aequa: {:?}", small);

    // Test 2: Multiplication logic
    // 0.1 * 0.1 should be 0.01 (scale 2)
    let a: f64 = 0.1.into();
    let b: f64 = 0.1.into();
    let res = a * b;
    println!("0.1 * 0.1 = {:?}", res);

    // Test 3: High Precision String (Bypass f64 16-digit limit)
    // 30 decimal places
    let hp_str = "0.123456789012345678901234567890";
    let hp: f64 = f64::from_str(hp_str).unwrap();
    println!("High Precision String (30 digits): {:?}", hp);
    assert_eq!(hp.scale(), 30);
}
