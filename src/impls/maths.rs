use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::Aequa;

impl Mul for Aequa {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value, self.scale + rhs.scale)
    }
}

impl Mul<std::primitive::f64> for Aequa {
    type Output = Self;

    fn mul(self, rhs: std::primitive::f64) -> Self::Output {
        self * Aequa::from(rhs)
    }
}

impl Mul<std::primitive::f32> for Aequa {
    type Output = Self;

    fn mul(self, rhs: std::primitive::f32) -> Self::Output {
        self * Aequa::from(rhs)
    }
}

impl Mul<Aequa> for std::primitive::f64 {
    type Output = Aequa;

    fn mul(self, rhs: Aequa) -> Self::Output {
        Aequa::from(self) * rhs
    }
}

impl Mul<Aequa> for std::primitive::f32 {
    type Output = Aequa;

    fn mul(self, rhs: Aequa) -> Self::Output {
        Aequa::from(self) * rhs
    }
}

impl Div for Aequa {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // To maintain precision in integer division, we shift the dividend.
        // We'll aim for about 20 digits of decimal precision for the result.
        let target_precision = 20;

        // New scale = self.scale - rhs.scale + target_precision
        // We use i32 for intermediate scale calculation to avoid u32 underflow panics.
        let intermediate_scale = self.scale as i32 - rhs.scale as i32 + target_precision as i32;

        let shifted_self = self.value * 10i128.pow(target_precision);
        let result_value = shifted_self / rhs.value;

        // If intermediate_scale is negative, it means the result is a very large integer.
        // We should shift the value further and set scale to 0.
        if intermediate_scale < 0 {
            let final_value = result_value * 10i128.pow((-intermediate_scale) as u32);
            Self::new(final_value, 0)
        } else {
            Self::new(result_value, intermediate_scale as u32)
        }
    }
}

impl Div<std::primitive::f64> for Aequa {
    type Output = Self;

    fn div(self, rhs: std::primitive::f64) -> Self::Output {
        self / Aequa::from(rhs)
    }
}

impl Div<std::primitive::f32> for Aequa {
    type Output = Self;

    fn div(self, rhs: std::primitive::f32) -> Self::Output {
        self / Aequa::from(rhs)
    }
}

impl Div<Aequa> for std::primitive::f64 {
    type Output = Aequa;

    fn div(self, rhs: Aequa) -> Self::Output {
        Aequa::from(self) / rhs
    }
}

impl Div<Aequa> for std::primitive::f32 {
    type Output = Aequa;

    fn div(self, rhs: Aequa) -> Self::Output {
        Aequa::from(self) / rhs
    }
}

impl Add for Aequa {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Match scales
        if self.scale == rhs.scale {
            Self::new(self.value + rhs.value, self.scale)
        } else if self.scale > rhs.scale {
            let diff = self.scale - rhs.scale;
            let adjusted_rhs = rhs.value * 10i128.pow(diff);
            Self::new(self.value + adjusted_rhs, self.scale)
        } else {
            let diff = rhs.scale - self.scale;
            let adjusted_lhs = self.value * 10i128.pow(diff);
            Self::new(adjusted_lhs + rhs.value, rhs.scale)
        }
    }
}

impl Add<std::primitive::f64> for Aequa {
    type Output = Self;

    fn add(self, rhs: std::primitive::f64) -> Self::Output {
        self + Aequa::from(rhs)
    }
}

impl Add<std::primitive::f32> for Aequa {
    type Output = Self;

    fn add(self, rhs: std::primitive::f32) -> Self::Output {
        self + Aequa::from(rhs)
    }
}

impl Add<Aequa> for std::primitive::f64 {
    type Output = Aequa;

    fn add(self, rhs: Aequa) -> Self::Output {
        Aequa::from(self) + rhs
    }
}

impl Add<Aequa> for std::primitive::f32 {
    type Output = Aequa;

    fn add(self, rhs: Aequa) -> Self::Output {
        Aequa::from(self) + rhs
    }
}

impl Sub for Aequa {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // match scales
        if self.scale == rhs.scale {
            Self::new(self.value - rhs.value, self.scale)
        } else if self.scale > rhs.scale {
            let diff = self.scale - rhs.scale;
            let adjusted_rhs = rhs.value * 10i128.pow(diff);
            Self::new(self.value - adjusted_rhs, self.scale)
        } else {
            let diff = rhs.scale - self.scale;
            let adjusted_lhs = self.value * 10i128.pow(diff);
            Self::new(adjusted_lhs - rhs.value, rhs.scale)
        }
    }
}

impl Sub<std::primitive::f64> for Aequa {
    type Output = Self;

    fn sub(self, rhs: std::primitive::f64) -> Self::Output {
        self - Aequa::from(rhs)
    }
}

impl Sub<std::primitive::f32> for Aequa {
    type Output = Self;

    fn sub(self, rhs: std::primitive::f32) -> Self::Output {
        self - Aequa::from(rhs)
    }
}

impl Sub<Aequa> for std::primitive::f64 {
    type Output = Aequa;

    fn sub(self, rhs: Aequa) -> Self::Output {
        Aequa::from(self) - rhs
    }
}

impl Sub<Aequa> for std::primitive::f32 {
    type Output = Aequa;

    fn sub(self, rhs: Aequa) -> Self::Output {
        Aequa::from(self) - rhs
    }
}

impl Neg for Aequa {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.value, self.scale)
    }
}
