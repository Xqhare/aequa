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
            "{:02}:{:02}:{:02}.{}",
            self.hour, self.minute, self.second, self.subseconds
        )
    }
}

impl TryFrom<&str> for LocalTime {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut it: [&str; 4] = ["0", "0", "0", "0"];
        if !value.contains(':') || value.len() < 5 {
            return Err(());
        }
        for (i, v) in value.split(':').enumerate() {
            if v.contains('.') {
                let (left, right) = v.split_once('.').unwrap();
                it[3] = right;
                it[i] = left;
            } else {
                it[i] = v;
            }
        }
        let hour = it[0].parse().map_err(|_| ())?;
        let minute = it[1].parse().map_err(|_| ())?;
        let second = it[2].parse().map_err(|_| ())?;
        let subseconds = it[3].parse().map_err(|_| ())?;
        Ok(Self::new(hour, minute, second, subseconds))
    }
}

impl TryFrom<String> for LocalTime {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

#[test]
fn parse_rfc3339() {
    let str0 = "02:10:00.987";
    let l_time0 = LocalTime::try_from(str0).unwrap();
    assert_eq!(l_time0.to_string(), str0);

    let str1 = "07:32:00";
    let l_time1 = LocalTime::try_from(str1).unwrap();
    assert_eq!(l_time1.to_string(), "07:32:00.0");

    let str2 = "07:48";
    let l_time2 = LocalTime::try_from(str2).unwrap();
    assert_eq!(l_time2.to_string(), "07:48:00.0");
}
