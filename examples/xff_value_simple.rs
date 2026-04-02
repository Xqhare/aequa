use aequa::XffValue;

fn main() {
    // XffValue is a versatile enum that can hold many different data types.

    // 1. Strings
    let s = XffValue::from("Hello, XFF!");
    assert!(s.is_string());
    println!("String value: {:?}", s.into_string());

    // 2. Numbers
    let n_int = XffValue::from(-42);
    let n_float = XffValue::from(3.14159);
    assert!(n_int.is_number());
    assert!(n_float.is_number());
    println!("Integer: {}, Float: {}", n_int, n_float);

    // 3. Booleans
    let b = XffValue::from(true);
    assert!(b.is_boolean());
    println!("Boolean: {}", b);

    // 4. Null
    let null = XffValue::Null;
    assert!(null.is_null());
    println!("Null value: {:?}", null);

    // 5. Convenient checks and conversions
    let val = XffValue::from(100);
    if let Some(num) = val.as_number() {
        println!("Value is a number: {}", num);
    }
}

#[test]
fn test_main() {
    main();
}
