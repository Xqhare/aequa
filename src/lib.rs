#![doc = include_str!("../README.md")]
pub mod hp_float;

pub mod graph;

pub mod value;

use nemesis::NemesisError;
pub use value::*;

/// A trait for serializing and deserializing XFF values.
///
/// When implemented for a type `T`, the following must hold:
/// ```ignore
/// let value: T = ...;
/// let xff = T::to_xff(&value);
/// let new_value = T::from_xff(&xff).unwrap();
/// assert_eq!(value, new_value);
/// ```
///
/// This ensures that structs that hold structs that implement `XffSerde` can be serialized and deserialized correctly.
pub trait XffSerde {
    /// Convert the value to an XFF value.
    ///
    /// Use the `from_xff` method to convert an XFF value back into the value.
    fn to_xff(&self) -> XffValue;
    /// Convert an XFF value to the value.
    ///
    /// Use the `to_xff` method to obtain the needed XFF value.
    fn from_xff(xff: &XffValue) -> Result<Self, NemesisError>
    where
        Self: Sized;
}
