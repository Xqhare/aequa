#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// A temporal duration (in milliseconds).
///
/// Can be created with `Duration::from()` or `Duration::new()`.
///
/// Most functionality needed for interacting with the underlying duration is provided trough the struct itself.
///
/// Access to the underlying duration is provided through the `value()` method.
///
/// `Duration` implements `From<u64>`.
///
/// # Examples
/// ```rust
/// use aequa::Duration;
///
/// let dur = Duration::from(1000);
/// assert_eq!(dur.value(), 1000);
/// assert_eq!(dur.as_seconds(), 1.0);
///
/// let dur2 = Duration::new(500);
/// assert_eq!(dur2.value(), 500);
/// ```
pub struct Duration(pub u64);

impl Duration {
    /// Creates a new `Duration`.
    ///
    /// # Example
    /// ```rust
    /// use aequa::Duration;
    /// let dur = Duration::new(1000);
    /// assert_eq!(dur.value(), 1000);
    /// ```
    #[must_use]
    pub fn new(value: u64) -> Self {
        Duration(value)
    }

    /// Returns the duration in milliseconds
    ///
    /// # Example
    /// ```rust
    /// use aequa::Duration;
    /// let dur = Duration::new(1000);
    /// assert_eq!(dur.value(), 1000);
    /// ```
    #[must_use]
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Returns the duration in seconds
    ///
    /// # Example
    /// ```rust
    /// use aequa::Duration;
    /// let dur = Duration::new(1500);
    /// assert_eq!(dur.as_seconds(), 1.5);
    /// ```
    #[must_use]
    pub fn as_seconds(&self) -> f64 {
        self.0 as f64 / 1000.0
    }

    /// Returns the duration in milliseconds
    ///
    /// # Example
    /// ```rust
    /// use aequa::Duration;
    /// let dur = Duration::new(1500);
    /// assert_eq!(dur.as_millis(), 1500);
    /// ```
    #[must_use]
    pub fn as_millis(&self) -> u64 {
        self.0
    }

    /// Creates a new `Duration` from milliseconds
    ///
    /// # Example
    /// ```rust
    /// use aequa::Duration;
    /// let dur = Duration::from_millis(1500);
    /// assert_eq!(dur.as_millis(), 1500);
    /// ```
    #[must_use]
    pub fn from_millis(millis: u64) -> Self {
        Duration(millis)
    }

    /// Creates a new `Duration` from seconds
    ///
    /// # Example
    /// ```rust
    /// use aequa::Duration;
    /// let dur = Duration::from_seconds(1.5);
    /// assert_eq!(dur.as_millis(), 1500);
    /// ```
    #[must_use]
    pub fn from_seconds(seconds: f64) -> Self {
        Duration((seconds * 1000.0) as u64)
    }
}

impl From<u64> for Duration {
    fn from(value: u64) -> Self {
        Duration(value)
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DUR({})", self.0)
    }
}
