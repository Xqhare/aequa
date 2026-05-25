/// Represents a naive time (Hour, Minute, Second, Subseconds) without timezone information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalTime {
    /// Hour (0-23)
    pub hour: u8,
    /// Minute (0-59)
    pub minute: u8,
    /// Second (0-59)
    pub second: u8,
    /// Subseconds (milliseconds)
    pub subseconds: u64,
}

impl LocalTime {
    /// Creates a new `LocalTime`.
    pub fn new(hour: u8, minute: u8, second: u8, subseconds: u64) -> Self {
        Self {
            hour,
            minute,
            second,
            subseconds,
        }
    }
}

impl std::fmt::Display for LocalTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}.{:03}",
            self.hour, self.minute, self.second, self.subseconds
        )
    }
}
