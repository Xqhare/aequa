use aequa::{Table, XffValue};

fn main() {
    println!("--- Table Usage ---");

    // 1. Create a table with defined columns
    let mut table = Table::with_columns(vec![
        "ID".to_string(),
        "Name".to_string(),
        "Active".to_string(),
    ]);

    // 2. Add some rows
    table
        .add_row(vec![
            XffValue::from(1),
            XffValue::from("Project A"),
            XffValue::from(true),
        ])
        .unwrap();

    table
        .add_row(vec![
            XffValue::from(2),
            XffValue::from("Project B"),
            XffValue::from(false),
        ])
        .unwrap();

    println!("Table Info: {}", table);
    println!(
        "Column count: {}, Row count: {}",
        table.column_count(),
        table.row_count()
    );

    // 3. Iterate over the raw rows
    println!("\nRaw data:");
    for (i, row) in table.rows.iter().enumerate() {
        println!("  Row {}: {:?}", i, row);
    }

    // 4. Extracting rows as OrderedObjects (handy for per-row key-value access)
    println!("\nStructured row access:");
    if let Some(row_0) = table.get_row(0) {
        println!("  Row 0 as OrderedObject: {}", row_0);

        // Accessing columns from the extracted row
        if let Some(obj) = row_0.as_ordered_object() {
            println!("    ID: {}", obj.get("ID").unwrap());
            println!("    Name: {}", obj.get("Name").unwrap());
        }
    }

    // 5. Schema validation check (adding a row with wrong length fails)
    let invalid_row = vec![XffValue::from(3), XffValue::from("Should fail")];
    let result = table.add_row(invalid_row);
    println!("\nAdding invalid row: {:?}", result);
    assert!(result.is_err());
}

#[test]
fn test_main() {
    main();
}
