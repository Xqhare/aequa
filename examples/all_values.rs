use aequa::{Data, DateTime, Duration, Uuid, XffValue};

fn main() {
    println!("--- All Non-Collection XffValue Types ---");

    // 1. String
    let s = XffValue::from("Standard String");
    println!("String:   {}", s);

    // 2. Numbers (Unsigned, Integer, Float)
    let u = XffValue::from(100u32);
    let i = XffValue::from(-100);
    let f = XffValue::from(123.456);
    println!("Unsigned: {}", u);
    println!("Integer:  {}", i);
    println!("Float:    {}", f);

    // 3. Boolean
    let b_true = XffValue::from(true);
    let b_false = XffValue::from(false);
    println!("Boolean:  {}, {}", b_true, b_false);

    // 4. DateTime (milliseconds since epoch)
    let dt = XffValue::from(DateTime::from(1647081600000));
    println!("DateTime: {}", dt);

    // 5. Duration (milliseconds)
    let dur = XffValue::from(Duration::from_millis(5000));
    println!("Duration: {}", dur);

    // 6. Uuid (16 bytes)
    let uuid = XffValue::from(Uuid::new([0x12; 16]));
    println!("Uuid:     {}", uuid);

    // 7. Binary Data
    let data = XffValue::from(Data::from(vec![1, 2, 3, 4, 5]));
    println!("Data:     {}", data);

    // 8. IEEE 754 Specials
    let nan = XffValue::NaN;
    let inf = XffValue::Infinity;
    let neg_inf = XffValue::NegInfinity;
    println!("Specials: {}, {}, {}", nan, inf, neg_inf);

    // 9. Null
    let null = XffValue::Null;
    println!("Null:     {}", null);

    println!("\nVerification:");
    assert!(s.is_string());
    assert!(u.is_number());
    assert!(b_true.is_boolean());
    assert!(dt.is_datetime());
    assert!(dur.is_duration());
    assert!(uuid.is_uuid());
    assert!(data.is_data());
    assert!(null.is_null());
}

#[test]
fn test_main() {
    main();
}
