/// Represents a naive date (Year, Month, Day) without timezone information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalDate {
    /// Year
    pub year: u16,
    /// Month (1-12)
    pub month: u8,
    /// Day (1-31)
    pub day: u8,
}

impl LocalDate {
    /// Creates a new `LocalDate`.
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

impl std::fmt::Display for LocalDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl TryInto<LocalDate> for String {
    type Error = ();

    fn try_into(self) -> Result<LocalDate, Self::Error> {
        let mut iter = self.split('-');
        let year = iter.next().ok_or(())?.parse().map_err(|_| ())?;
        let month = iter.next().ok_or(())?.parse().map_err(|_| ())?;
        let day = iter.next().ok_or(())?.parse().map_err(|_| ())?;
        Ok(LocalDate::new(year, month, day))
    }
}
