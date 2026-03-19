use crate::Aequa;

/// The Archimedes' constant (π). 32 digits of precision.
pub const PI: Aequa = Aequa::new(31415926535897932384626433832795, 31);

/// Euler's number (e). 32 digits of precision.
pub const E: Aequa = Aequa::new(27182818284590452353602874713527, 31);

/// The full circle constant (τ) = 2π. 32 digits of precision.
pub const TAU: Aequa = Aequa::new(62831853071795864769252867665590, 31);

/// The maximum value of an i128 representing an Aequa with scale 0.
pub const MAX: Aequa = Aequa::new(i128::MAX, 0);

/// The minimum value of an i128 representing an Aequa with scale 0.
pub const MIN: Aequa = Aequa::new(i128::MIN, 0);

// For INFINITY and NAN, we currently don't have a special state.
// We should implement an enum or special scale in the future.
// For now, these are not truly represented.
