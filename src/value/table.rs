use super::XffValue;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// A schema-based table.
///
/// Can be created with `Table::new()` or `Table::with_columns()`.
///
/// Tables consist of a list of column names and a list of rows.
/// Each row must have the same number of elements as there are columns.
///
/// Most functionality needed for interacting with the underlying table is provided trough the struct itself.
///
/// # Examples
/// ```rust
/// use aequa::{Table, XffValue};
///
/// let mut table = Table::with_columns(vec!["name".to_string(), "age".to_string()]);
/// table.add_row(vec![XffValue::from("Alice"), XffValue::from(30)]).unwrap();
///
/// assert_eq!(table.column_count(), 2);
/// assert_eq!(table.row_count(), 1);
/// ```
pub struct Table {
    /// Column names
    pub columns: Vec<String>,
    /// Row data
    pub rows: Vec<Vec<XffValue>>,
}

impl Table {
    /// Creates a new, empty Table
    ///
    /// # Example
    /// ```rust
    /// use aequa::Table;
    /// let table = Table::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a Table with specified columns
    ///
    /// # Example
    /// ```rust
    /// use aequa::Table;
    /// let table = Table::with_columns(vec!["A".to_string()]);
    /// ```
    #[must_use]
    pub fn with_columns(columns: Vec<String>) -> Self {
        Self {
            columns,
            rows: Vec::new(),
        }
    }

    /// Adds a row to the table.
    ///
    /// Returns an error if the row length does not match the column count.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{Table, XffValue};
    /// let mut table = Table::with_columns(vec!["A".to_string()]);
    /// table.add_row(vec![XffValue::from(1)]).unwrap();
    /// ```
    pub fn add_row(&mut self, row: Vec<XffValue>) -> Result<(), String> {
        if row.len() != self.columns.len() {
            return Err(format!(
                "Row length {} does not match column count {}",
                row.len(),
                self.columns.len()
            ));
        }
        self.rows.push(row);
        Ok(())
    }

    /// Gets the number of columns
    ///
    /// # Example
    /// ```rust
    /// use aequa::Table;
    /// let table = Table::with_columns(vec!["A".to_string(), "B".to_string()]);
    /// assert_eq!(table.column_count(), 2);
    /// ```
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Gets the number of rows
    ///
    /// # Example
    /// ```rust
    /// use aequa::{Table, XffValue};
    /// let mut table = Table::with_columns(vec!["A".to_string()]);
    /// table.add_row(vec![XffValue::from(1)]).unwrap();
    /// assert_eq!(table.row_count(), 1);
    /// ```
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns a specific row as an `XffValue::OrderedObject`.
    ///
    /// The object will contain key-value pairs where keys are column names.
    ///
    /// # Example
    /// ```rust
    /// use aequa::{Table, XffValue};
    /// let mut table = Table::with_columns(vec!["A".to_string()]);
    /// table.add_row(vec![XffValue::from(1)]).unwrap();
    /// let row = table.get_row(0).unwrap();
    /// assert!(row.is_ordered_object());
    /// ```
    #[must_use]
    pub fn get_row(&self, index: usize) -> Option<XffValue> {
        let row_data = self.rows.get(index)?;
        let mut ordered_obj = super::OrderedObject::new();
        for (i, col_name) in self.columns.iter().enumerate() {
            ordered_obj.push(
                col_name.clone(),
                row_data.get(i).cloned().unwrap_or_else(|| XffValue::default()),
            );
        }
        Some(XffValue::OrderedObject(ordered_obj))
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Table(cols: {:?}, rows: {})",
            self.columns,
            self.rows.len()
        )
    }
}
