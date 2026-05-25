use crate::value::local_date::LocalDate;
use crate::value::local_time::LocalTime;

/// Represents a naive date and time without timezone information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalDateTime {
    /// Date component
    pub date: LocalDate,
    /// Time component
    pub time: LocalTime,
}

impl LocalDateTime {
    /// Creates a new `LocalDateTime`.
    pub fn new(date: LocalDate, time: LocalTime) -> Self {
        Self { date, time }
    }
}

impl std::fmt::Display for LocalDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}T{}", self.date, self.time)
    }
}
