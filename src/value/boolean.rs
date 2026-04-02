#[derive(Debug, Clone, PartialEq, PartialOrd, Default, Copy)]
/// A boolean value.
///
/// Can be created with `Boolean::from()` or `Boolean::new()`.
///
/// Most functionality needed for interacting with the underlying boolean is provided trough the struct itself.
///
/// Access to the underlying boolean is provided through the `value()` method.
///
/// `Boolean` implements `From<bool>`.
///
/// # Examples
/// ```rust
/// use aequa::Boolean;
///
/// let b = Boolean::from(true);
/// assert!(b.value());
///
/// let b2 = Boolean::new(false);
/// assert!(!b2.value());
/// ```
pub struct Boolean(pub bool);

impl Boolean {
    /// Creates a new `Boolean`.
    ///
    /// # Example
    /// ```rust
    /// use aequa::Boolean;
    /// let b = Boolean::new(true);
    /// assert!(b.value());
    /// ```
    #[must_use]
    pub fn new(value: bool) -> Self {
        Boolean(value)
    }

    /// Returns the boolean value
    ///
    /// # Example
    /// ```rust
    /// use aequa::Boolean;
    /// let b = Boolean::new(false);
    /// assert!(!b.value());
    /// ```
    #[must_use]
    pub fn value(&self) -> bool {
        self.0
    }
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Boolean(value)
    }
}

impl std::fmt::Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
