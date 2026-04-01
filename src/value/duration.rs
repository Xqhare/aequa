#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// A temporal duration (in milliseconds).
pub struct Duration(pub u64);

impl Duration {
    /// Returns the duration in milliseconds
    #[must_use]
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Returns the duration in seconds
    #[must_use]
    pub fn as_seconds(&self) -> f64 {
        self.0 as f64 / 1000.0
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
