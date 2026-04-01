use super::XffValue;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// A schema-based table.
///
/// Tables consist of a list of column names and a list of rows.
/// Each row must have the same number of elements as there are columns.
pub struct Table {
    /// Column names
    pub columns: Vec<String>,
    /// Row data
    pub rows: Vec<Vec<XffValue>>,
}

impl Table {
    /// Creates a new, empty Table
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a Table with specified columns
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
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Gets the number of rows
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns a specific row as an `XffValue::OrderedObject`.
    ///
    /// The object will contain key-value pairs where keys are column names.
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
