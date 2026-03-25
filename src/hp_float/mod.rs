/// A drop-in replacement alias for high-precision f64
#[allow(non_camel_case_types)]
pub type f64 = HpFloat;

/// A drop-in replacement alias for high-precision f32
#[allow(non_camel_case_types)]
pub type f32 = HpFloat;

mod impls;
pub use impls::*;
mod constants;
pub use constants::*;
