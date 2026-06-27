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

impl TryFrom<String> for LocalDateTime {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for LocalDateTime {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.contains('T') && !value.contains('t') && !value.contains(' ') || value.len() < 16
        {
            return Err(());
        }
        let split_pattern = {
            if value.contains('t') {
                't'
            } else if value.contains('T') {
                'T'
            } else if value.contains(' ') {
                ' '
            } else {
                return Err(());
            }
        };
        let mut it = value.split(split_pattern);
        let date = it.next().ok_or(())?.try_into().map_err(|_| ())?;
        let time = it.next().ok_or(())?.try_into().map_err(|_| ())?;
        Ok(Self::new(date, time))
    }
}

#[test]
fn parse_rfc3339() {
    let str0 = "2023-09-03T02:10:00.987";
    let l_dt0 = LocalDateTime::try_from(str0).unwrap();
    assert_eq!(l_dt0.to_string(), str0);

    let str1 = "1979-05-27T07:32:00";
    let l_dt1 = LocalDateTime::try_from(str1).unwrap();
    assert_eq!(l_dt1.to_string(), "1979-05-27T07:32:00.0");

    let str2 = "1979-05-27T07:32:00.5";
    let l_dt2 = LocalDateTime::try_from(str2).unwrap();
    assert_eq!(l_dt2.to_string(), str2);

    let str3 = "1979-05-27T07:32";
    let l_dt3 = LocalDateTime::try_from(str3).unwrap();
    assert_eq!(l_dt3.to_string(), "1979-05-27T07:32:00.0");

    let str4 = "1979-05-27t07:32:00.5";
    let l_dt4 = LocalDateTime::try_from(str4).unwrap();
    assert_eq!(l_dt4.to_string(), "1979-05-27T07:32:00.5");

    let str5 = "1979-05-27 07:32:00.5";
    let l_dt5 = LocalDateTime::try_from(str5).unwrap();
    assert_eq!(l_dt5.to_string(), "1979-05-27T07:32:00.5");
}
