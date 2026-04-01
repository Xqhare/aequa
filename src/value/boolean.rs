#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// A boolean value.
pub struct Boolean(pub bool);

impl Boolean {
    /// Returns the boolean value
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
