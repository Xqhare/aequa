/// A drop-in replacement alias for high-precision f64
#[allow(non_camel_case_types)]
pub type f64 = Aequa;

/// A drop-in replacement alias for high-precision f32
#[allow(non_camel_case_types)]
pub type f32 = Aequa;

mod impls;
pub use impls::*;
mod constants;
pub use constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AequaError {
    InvalidScale,
    InvalidValue,
}
