#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// A timestamp value (milliseconds since epoch).
///
/// Can be created with `DateTime::from()` or `DateTime::new()`.
///
/// Most functionality needed for interacting with the underlying timestamp is provided trough the struct itself.
///
/// Access to the underlying timestamp is provided through the `value()` method.
///
/// `DateTime` implements `From<u64>`.
///
/// # Examples
/// ```rust
/// use aequa::DateTime;
///
/// let dt = DateTime::from(1000);
/// assert_eq!(dt.value(), 1000);
/// assert_eq!(dt.as_unix_timestamp(), 1.0);
///
/// let dt2 = DateTime::new(0);
/// assert_eq!(dt2.value(), 0);
/// ```
pub struct DateTime(pub u64);

impl DateTime {
    /// Creates a new `DateTime`.
    ///
    /// # Example
    /// ```rust
    /// use aequa::DateTime;
    /// let dt = DateTime::new(123456789);
    /// assert_eq!(dt.value(), 123456789);
    /// ```
    #[must_use]
    pub fn new(value: u64) -> Self {
        DateTime(value)
    }

    /// Returns the timestamp in milliseconds
    ///
    /// # Example
    /// ```rust
    /// use aequa::DateTime;
    /// let dt = DateTime::new(123456789);
    /// assert_eq!(dt.value(), 123456789);
    /// ```
    #[must_use]
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Returns the timestamp in seconds (UNIX timestamp)
    ///
    /// # Example
    /// ```rust
    /// use aequa::DateTime;
    /// let dt = DateTime::new(1000);
    /// assert_eq!(dt.as_unix_timestamp(), 1.0);
    /// ```
    #[must_use]
    pub fn as_unix_timestamp(&self) -> f64 {
        self.0 as f64 / 1000.0
    }

    /// Returns the timestamp in milliseconds
    ///
    /// # Example
    /// ```rust
    /// use aequa::DateTime;
    /// let dt = DateTime::new(1000);
    /// assert_eq!(dt.as_millis(), 1000);
    /// ```
    #[must_use]
    pub fn as_millis(&self) -> u64 {
        self.0
    }

    /// Creates a new `DateTime` from a UNIX timestamp (seconds since epoch)
    ///
    /// # Example
    /// ```rust
    /// use aequa::DateTime;
    /// let dt = DateTime::from_unix_timestamp(1.0);
    /// assert_eq!(dt.as_millis(), 1000);
    /// ```
    pub fn from_unix_timestamp(seconds: f64) -> Self {
        // Remember, DateTime is in milliseconds
        DateTime((seconds * 1000.0) as u64)
    }

    /// Creates a new `DateTime` from milliseconds since epoch
    ///
    /// # Example
    /// ```rust
    /// use aequa::DateTime;
    /// let dt = DateTime::from_unix_timestamp_millis(1000);
    /// assert_eq!(dt.as_millis(), 1000);
    /// ```
    pub fn from_unix_timestamp_millis(ms: u64) -> Self {
        DateTime(ms)
    }
}

impl From<u64> for DateTime {
    fn from(value: u64) -> Self {
        DateTime(value)
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DT({})", self.0)
    }
}
