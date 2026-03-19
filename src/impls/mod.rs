use std::fmt::Display;

use athena::encoding_and_decoding::{
    deserialize_leb128_signed, deserialize_leb128_unsigned, serialize_leb128_signed,
    serialize_leb128_unsigned,
};

use crate::AequaError;

pub mod maths;
/// Aequa represents a number with arbitrary precision.
///
/// It follows the principle that "exact exchange and accurate weights"
/// should be the default for financial or scientific calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Aequa {
    /// The integer representation of the number (e.g., 1204 for 1.204)
    value: i128,
    /// The power of 10 applied to the value (e.g., 3 for 10^-3)
    scale: u32,
}

impl Aequa {
    pub const fn new(value: i128, scale: u32) -> Self {
        Self { value, scale }
    }

    pub fn scale(&self) -> u32 {
        self.scale
    }

    /// Trims trailing zeros from the decimal part to simplify the scale.
    pub fn trim_scale(mut self) -> Self {
        while self.scale > 0 && self.value % 10 == 0 {
            self.value /= 10;
            self.scale -= 1;
        }
        self
    }

    /// Converts Aequa to a byte array.
    ///
    /// The array containes first the value, then the scale. Both are LEB-128 encoded.
    ///
    /// To reconstruct the Aequa from the byte array, use `Aequa::from_bytes`.
    pub fn to_bytes(self) -> Vec<u8> {
        let value_leb = serialize_leb128_signed(self.value);
        let scale_leb = serialize_leb128_unsigned(self.scale as u128);

        [value_leb, scale_leb].concat()
    }

    /// Reconstructs an Aequa from a byte array.
    ///
    /// The byte array contains first the value, then the scale. Both are LEB-128 encoded.
    /// Returns the Aequa and the number of bytes read.
    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, u16), AequaError> {
        let (value, bytes_read) = match deserialize_leb128_signed(&bytes[0..]) {
            Ok(v) => v,
            Err(_) => return Err(AequaError::InvalidValue),
        };
        let (scale, s_bytes_read) = match deserialize_leb128_unsigned(&bytes[bytes_read as usize..])
        {
            Ok(v) => v,
            Err(_) => return Err(AequaError::InvalidScale),
        };
        let total_bytes: u16 = (bytes_read + s_bytes_read) as u16;
        Ok((Aequa::new(value, scale as u32), total_bytes))
    }
}

impl From<std::primitive::f64> for Aequa {
    fn from(f: std::primitive::f64) -> Self {
        // Rust's to_string() uses the Ryu algorithm to find the shortest
        // decimal representation that rounds back to the same float.
        // This is ideal for converting 0.1 back to "0.1" exactly.
        f.to_string().parse().unwrap_or(Self::new(0, 0))
    }
}

impl From<std::primitive::f32> for Aequa {
    fn from(f: std::primitive::f32) -> Self {
        f.to_string().parse().unwrap_or(Self::new(0, 0))
    }
}

impl std::str::FromStr for Aequa {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Handle scientific notation if present (e.g., "1e-10")
        if s.contains('e') || s.contains('E') {
            // For now, let's fall back to f64 for scientific notation parsing
            // or we could implement a custom parser later.
            return Ok(std::primitive::f64::from_str(s)
                .map(Aequa::from)
                .unwrap_or(Self::new(0, 0)));
        }

        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() == 1 {
            return Ok(Self::new(parts[0].parse()?, 0));
        }

        let decimal = parts[1];
        let scale = decimal.len() as u32;
        let combined = format!("{}{}", parts[0], decimal);
        let value = combined.parse()?;

        Ok(Self::new(value, scale))
    }
}

impl Display for Aequa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trimmed = self.trim_scale();
        let s = trimmed.value.abs().to_string();
        let sign = if trimmed.value < 0 { "-" } else { "" };

        if trimmed.scale == 0 {
            write!(f, "{}{}", sign, s)
        } else if trimmed.scale < s.len() as u32 {
            let dot_pos = s.len() - trimmed.scale as usize;
            write!(f, "{}{}.{}", sign, &s[..dot_pos], &s[dot_pos..])
        } else {
            let padding = trimmed.scale as usize - s.len();
            write!(f, "{}0.{}{}", sign, "0".repeat(padding), s)
        }
    }
}
