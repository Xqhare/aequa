#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// A timestamp value (milliseconds since epoch).
pub struct DateTime(pub u64);

impl DateTime {
    /// Returns the timestamp in milliseconds
    #[must_use]
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Returns the timestamp in seconds (UNIX timestamp)
    #[must_use]
    pub fn as_unix_timestamp(&self) -> f64 {
        self.0 as f64 / 1000.0
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
