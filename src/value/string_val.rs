#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// A string value.
///
/// Can be created from a `String` or `&str` using the `From` trait.
///
/// # Example
/// ```rust
/// use aequa::XffString;
///
/// let s = XffString::from("hello mom!");
/// assert_eq!(s.len(), 10);
/// assert_eq!(s.as_str(), "hello mom!");
/// ```
pub struct XffString {
    /// The actual string
    pub value: String,
}

// -----------------------------------------------------------
//                     General implementations
// -----------------------------------------------------------

impl XffString {
    /// Creates a new and empty `XffString`
    #[must_use]
    pub fn new() -> Self {
        XffString { value: String::new() }
    }

    /// Returns the length of the string
    #[must_use]
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Returns `true` if the string is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Returns the string as a `&str`
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the string as a `String`
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }
}

// -----------------------------------------------------------
//                     From implementations
// -----------------------------------------------------------

impl From<String> for XffString {
    fn from(value: String) -> Self {
        XffString { value }
    }
}

impl From<&str> for XffString {
    fn from(value: &str) -> Self {
        XffString { value: value.to_string() }
    }
}

// -----------------------------------------------------------
//                     Display implementation
// -----------------------------------------------------------

impl std::fmt::Display for XffString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
