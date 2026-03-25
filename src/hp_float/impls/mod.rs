use std::fmt::Display;

pub mod maths;
/// A number with arbitrary precision.
///
/// It trades off speed for precision, and is intended to be used as a drop in replacement for both `f64` and `f32`.
///
/// It follows my "no dependencies but the standard library and libc" philosophy.
///
/// ## Features
///
/// - Arbitrary precision
/// - Fast
/// - No dependencies
/// - Drop-in replacement for `f64` and `f32`
///
/// ## Usage
///
/// ### Importing
///
/// Add the following to your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// aequa = { git = "https://github.com/xqhare/aequa" }
/// ```
///
/// ### In Your Code
///
/// Simply shadow the primitive type to use `Aequa` throughout your module with full precision.
///
/// ```rust
/// use aequa::f64; // Shadows primitive f64 locally
///
/// let a: f64 = 0.1.into();
/// let b: f64 = 0.2.into();
///
/// // Standard operators work seamlessly
/// let sum = a + b;
///
/// // Even mixing with literals is supported
/// let result = sum + 0.4;
///
/// assert_eq!(result, 0.7.into());
/// ```
///
/// ## How It Works
///
/// The idea is really quite simple.
///
/// Let's take the infamous example of 0.1 + 0.2:
///
/// If done using IEEE floats: 0.1 + 0.2 = 0.30000000000000004
///
/// If done using Aequa: 0.1 + 0.2 = 0.3
///
/// ### Examples
///
/// 0.1 => (1 * 10^-1) => 1 | 1
/// 0.2 => (2 * 10^-1) => 2 | 1
/// 0.1 + 0.2 => 3(3 * 10^-1) => 3 | 1
///
/// 1.2 + 0.004 = 1.204
/// 1.2 => (12 * 10^-1) => 12 | 1
/// 0.004 => (4 * 10^-3) => 4 | 3
///
/// To add, you align the scales by multiplying the value of the smaller scale by 10^(scale_diff):
///
/// scale_diff = 3 - 1 = 2
/// 1.2 => (12 * 10^2) | 1 = 1200 | 1
///
/// Then add the values together:
/// 1200 | 1 + 4 | 3 = 1204 | 3
///
/// To go backwards:
///
/// 1. Take the value as a string: "1204"
/// 2. Insert the decimal point 3 places from the right: "1.204"
///
/// If the value is smaller than the scale (e.g., 3 | 1):
///
/// 1. Pad the string with leading zeros: "03"
/// 2. Insert the decimal point: "0.3"
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HpFloat {
    /// The integer representation of the number (e.g., 1204 for 1.204)
    value: i128,
    /// The power of 10 applied to the value (e.g., 3 for 10^-3)
    scale: u32,
}

impl HpFloat {
    /// Creates a new `HpFloat` with the given value and scale
    #[must_use]
    pub const fn new(value: i128, scale: u32) -> Self {
        Self { value, scale }
    }

    /// Trims trailing zeros from the decimal part to simplify the scale.
    #[must_use]
    pub fn trim_scale(mut self) -> Self {
        while self.scale > 0 && self.value % 10 == 0 {
            self.value /= 10;
            self.scale -= 1;
        }
        self
    }

    /// Returns the value of the number
    ///
    /// Usefull for custom serialization
    /// Use `get_scale()` to get the scale
    pub fn get_value(&self) -> i128 {
        self.value
    }

    /// Returns the scale of the number
    ///
    /// Usefull for custom serialization
    /// Use `get_value()` to get the value
    pub fn get_scale(&self) -> u32 {
        self.scale
    }
}

impl From<std::primitive::f64> for HpFloat {
    fn from(f: std::primitive::f64) -> Self {
        // Rust's to_string() uses the Ryu algorithm to find the shortest
        // decimal representation that rounds back to the same float.
        // This is ideal for converting 0.1 back to "0.1" exactly.
        f.to_string().parse().unwrap_or(Self::new(0, 0))
    }
}

impl From<std::primitive::f32> for HpFloat {
    fn from(f: std::primitive::f32) -> Self {
        f.to_string().parse().unwrap_or(Self::new(0, 0))
    }
}

impl std::str::FromStr for HpFloat {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Handle scientific notation if present (e.g., "1e-10")
        if s.contains('e') || s.contains('E') {
            // For now, let's fall back to f64 for scientific notation parsing
            // or we could implement a custom parser later.
            return Ok(std::primitive::f64::from_str(s)
                .map(HpFloat::from)
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

impl Display for HpFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trimmed = self.trim_scale();
        let s = trimmed.value.abs().to_string();
        let sign = if trimmed.value < 0 { "-" } else { "" };

        if trimmed.scale == 0 {
            write!(f, "{sign}{s}")
        } else if trimmed.scale < s.len() as u32 {
            let dot_pos = s.len() - trimmed.scale as usize;
            write!(f, "{}{}.{}", sign, &s[..dot_pos], &s[dot_pos..])
        } else {
            let padding = trimmed.scale as usize - s.len();
            write!(f, "{}0.{}{}", sign, "0".repeat(padding), s)
        }
    }
}
