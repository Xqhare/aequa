#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
/// A 128-bit Universally Unique Identifier (UUID).
pub struct Uuid {
    /// The 16 raw bytes of the UUID
    pub bytes: [u8; 16],
}

impl Uuid {
    /// Creates a new Uuid from 16 bytes
    #[must_use] 
    pub fn new(bytes: [u8; 16]) -> Self {
        Self { bytes }
    }

    /// Returns the raw bytes of the UUID
    #[must_use] 
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }
}

impl From<[u8; 16]> for Uuid {
    fn from(bytes: [u8; 16]) -> Self {
        Self { bytes }
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, byte) in self.bytes.iter().enumerate() {
            if i == 4 || i == 6 || i == 8 || i == 10 {
                write!(f, "-")?;
            }
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}
