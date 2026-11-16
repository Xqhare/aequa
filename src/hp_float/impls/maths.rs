use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::hp_float::HpFloat;

/// Aligns two `(value, scale)` pairs so they share the same scale and can be added or subtracted
/// without overflowing `i128`, preserving maximum possible decimal precision.
fn align_scales(
    mut a_val: i128,
    mut a_scale: u32,
    mut b_val: i128,
    mut b_scale: u32,
) -> (i128, i128, u32) {
    // Step 1: Shift the lower-scale operand up by powers of 10 as far as fits in i128.
    while a_scale < b_scale {
        if let Some(shifted) = a_val.checked_mul(10) {
            a_val = shifted;
            a_scale += 1;
        } else {
            break;
        }
    }
    while b_scale < a_scale {
        if let Some(shifted) = b_val.checked_mul(10) {
            b_val = shifted;
            b_scale += 1;
        } else {
            break;
        }
    }

    // Step 2: If scales still differ (because the lower-scale operand hit the i128 ceiling),
    // shift the higher-scale operand down until scales match.
    while a_scale > b_scale {
        let rem = a_val % 10;
        a_val /= 10;
        a_scale -= 1;
        if a_scale == b_scale && rem.abs() >= 5 {
            a_val += rem.signum();
        }
    }
    while b_scale > a_scale {
        let rem = b_val % 10;
        b_val /= 10;
        b_scale -= 1;
        if b_scale == a_scale && rem.abs() >= 5 {
            b_val += rem.signum();
        }
    }

    (a_val, b_val, a_scale)
}

/// Multiplies two `(value, scale)` pairs using exact 256-bit intermediate arithmetic,
/// scaling down only the digits that cannot fit in `i128`.
fn mul_high_precision(a_val: i128, a_scale: u32, b_val: i128, b_scale: u32) -> HpFloat {
    let negative = (a_val < 0) ^ (b_val < 0);
    let ua = a_val.unsigned_abs();
    let ub = b_val.unsigned_abs();

    // 128x128 -> 256-bit unsigned multiplication
    let a_lo = ua & 0xFFFF_FFFF_FFFF_FFFF;
    let a_hi = ua >> 64;
    let b_lo = ub & 0xFFFF_FFFF_FFFF_FFFF;
    let b_hi = ub >> 64;

    let p0 = a_lo * b_lo;
    let p1 = a_lo * b_hi;
    let p2 = a_hi * b_lo;
    let p3 = a_hi * b_hi;

    let mid1 = (p0 >> 64) + (p1 & 0xFFFF_FFFF_FFFF_FFFF) + (p2 & 0xFFFF_FFFF_FFFF_FFFF);
    let lo = (p0 & 0xFFFF_FFFF_FFFF_FFFF) | (mid1 << 64);
    let hi = p3 + (p1 >> 64) + (p2 >> 64) + (mid1 >> 64);

    let mut cur_hi = hi;
    let mut cur_lo = lo;
    let mut scale = a_scale.saturating_add(b_scale);
    let mut last_rem = 0u128;

    let max_u128 = i128::MAX as u128;

    // Divide the 256-bit number (cur_hi, cur_lo) by 10 until it fits in i128::MAX.
    while cur_hi > 0 || cur_lo > max_u128 {
        if scale == 0 {
            panic!("attempt to multiply with overflow");
        }
        // Divide (cur_hi, cur_lo) by 10 using 64-bit limbs
        let limb3 = cur_hi >> 64;
        let limb2 = cur_hi & 0xFFFF_FFFF_FFFF_FFFF;
        let limb1 = cur_lo >> 64;
        let limb0 = cur_lo & 0xFFFF_FFFF_FFFF_FFFF;

        let q3 = limb3 / 10;
        let r3 = limb3 % 10;

        let cur2 = (r3 << 64) | limb2;
        let q2 = cur2 / 10;
        let r2 = cur2 % 10;

        let cur1 = (r2 << 64) | limb1;
        let q1 = cur1 / 10;
        let r1 = cur1 % 10;

        let cur0 = (r1 << 64) | limb0;
        let q0 = cur0 / 10;
        let r0 = cur0 % 10;

        cur_hi = (q3 << 64) | q2;
        cur_lo = (q1 << 64) | q0;
        last_rem = r0;
        scale -= 1;
    }

    if last_rem >= 5 && cur_lo < max_u128 {
        cur_lo += 1;
    }

    let signed_val = if negative {
        -(cur_lo as i128)
    } else {
        cur_lo as i128
    };

    HpFloat::new(signed_val, scale)
}

impl Mul for HpFloat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.value == 0 || rhs.value == 0 {
            return Self::new(0, 0);
        }

        if let Some(prod) = self.value.checked_mul(rhs.value) {
            return Self::new(prod, self.scale.saturating_add(rhs.scale));
        }

        mul_high_precision(self.value, self.scale, rhs.value, rhs.scale)
    }
}

impl Mul<std::primitive::f64> for HpFloat {
    type Output = Self;

    fn mul(self, rhs: std::primitive::f64) -> Self::Output {
        self * HpFloat::from(rhs)
    }
}

impl Mul<std::primitive::f32> for HpFloat {
    type Output = Self;

    fn mul(self, rhs: std::primitive::f32) -> Self::Output {
        self * HpFloat::from(rhs)
    }
}

impl Mul<HpFloat> for std::primitive::f64 {
    type Output = HpFloat;

    fn mul(self, rhs: HpFloat) -> Self::Output {
        HpFloat::from(self) * rhs
    }
}

impl Mul<HpFloat> for std::primitive::f32 {
    type Output = HpFloat;

    fn mul(self, rhs: HpFloat) -> Self::Output {
        HpFloat::from(self) * rhs
    }
}

impl Div for HpFloat {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(rhs.value != 0, "attempt to divide by zero");
        if self.value == 0 {
            return Self::new(0, 0);
        }

        let negative = (self.value < 0) ^ (rhs.value < 0);
        let ua = self.value.unsigned_abs();
        let ub = rhs.value.unsigned_abs();

        let mut q = ua / ub;
        let mut r = ua % ub;
        let mut scale = self.scale as i64 - rhs.scale as i64;

        let max_u128 = i128::MAX as u128;

        // Extract exact decimal digits from the remainder as long as `q` fits in `i128`
        // (or while `scale < 0`, which means we must still shift to reach scale 0).
        while r != 0 || scale < 0 {
            // Compute `(r * 10) / ub` and `(r * 10) % ub` without overflowing u128:
            // Note that `r < ub`. If `r * 10` overflows `u128`, we use 256-bit or identity:
            // `ub_q = ub / 10`, `ub_r = ub % 10`, or 128-bit step.
            let (next_digit, next_r) = if let Some(r10) = r.checked_mul(10) {
                (r10 / ub, r10 % ub)
            } else {
                // Since r < ub, r * 10 / ub is in 0..=9.
                // We can find `d` in 0..=9 using binary/linear search with 256-bit comparison,
                // or simply split r and ub.
                div_rem_u128_times_10(r, ub)
            };

            if let Some(q10) = q.checked_mul(10) {
                let candidate = q10.saturating_add(next_digit);
                if candidate <= max_u128 {
                    q = candidate;
                    r = next_r;
                    scale += 1;
                    continue;
                }
            }

            // Cannot fit another digit in `i128`!
            if scale < 0 {
                panic!("attempt to divide with overflow");
            }

            // Round the last digit if the next fractional digit is >= 5
            if next_digit >= 5 && q < max_u128 {
                q += 1;
            }
            break;
        }

        let signed_q = if negative { -(q as i128) } else { q as i128 };
        Self::new(signed_q, scale as u32)
    }
}

/// Computes `((r * 10) / ub, (r * 10) % ub)` exactly when `r < ub` and `r * 10` overflows `u128`.
fn div_rem_u128_times_10(r: u128, ub: u128) -> (u128, u128) {
    // Represent `r * 10` as 256-bit `(hi, lo)`
    let r_lo = r & 0xFFFF_FFFF_FFFF_FFFF;
    let r_hi = r >> 64;
    let p0 = r_lo * 10;
    let p1 = r_hi * 10 + (p0 >> 64);
    let hi = p1 >> 64;
    let lo = (p1 << 64) | (p0 & 0xFFFF_FFFF_FFFF_FFFF);

    // Since `r < ub`, `d = (r * 10) / ub` is at most 9.
    let mut d = 0u128;
    let mut cur_hi = hi;
    let mut cur_lo = lo;
    while cur_hi > 0 || cur_lo >= ub {
        let borrow = if cur_lo < ub { 1u128 } else { 0u128 };
        cur_lo = cur_lo.wrapping_sub(ub);
        cur_hi = cur_hi.wrapping_sub(borrow);
        d += 1;
    }
    (d, cur_lo)
}

impl Div<std::primitive::f64> for HpFloat {
    type Output = Self;

    fn div(self, rhs: std::primitive::f64) -> Self::Output {
        self / HpFloat::from(rhs)
    }
}

impl Div<std::primitive::f32> for HpFloat {
    type Output = Self;

    fn div(self, rhs: std::primitive::f32) -> Self::Output {
        self / HpFloat::from(rhs)
    }
}

impl Div<HpFloat> for std::primitive::f64 {
    type Output = HpFloat;

    fn div(self, rhs: HpFloat) -> Self::Output {
        HpFloat::from(self) / rhs
    }
}

impl Div<HpFloat> for std::primitive::f32 {
    type Output = HpFloat;

    fn div(self, rhs: HpFloat) -> Self::Output {
        HpFloat::from(self) / rhs
    }
}

impl Add for HpFloat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (mut a, mut b, mut scale) = align_scales(self.value, self.scale, rhs.value, rhs.scale);
        while a.checked_add(b).is_none() {
            if scale == 0 {
                panic!("attempt to add with overflow");
            }
            let rem_sum = (a % 10) + (b % 10);
            let carry = if rem_sum >= 5 {
                (rem_sum + 5) / 10
            } else if rem_sum <= -5 {
                (rem_sum - 5) / 10
            } else {
                0
            };
            a = (a / 10) + carry;
            b /= 10;
            scale -= 1;
        }
        Self::new(a + b, scale)
    }
}

impl Add<std::primitive::f64> for HpFloat {
    type Output = Self;

    fn add(self, rhs: std::primitive::f64) -> Self::Output {
        self + HpFloat::from(rhs)
    }
}

impl Add<std::primitive::f32> for HpFloat {
    type Output = Self;

    fn add(self, rhs: std::primitive::f32) -> Self::Output {
        self + HpFloat::from(rhs)
    }
}

impl Add<HpFloat> for std::primitive::f64 {
    type Output = HpFloat;

    fn add(self, rhs: HpFloat) -> Self::Output {
        HpFloat::from(self) + rhs
    }
}

impl Add<HpFloat> for std::primitive::f32 {
    type Output = HpFloat;

    fn add(self, rhs: HpFloat) -> Self::Output {
        HpFloat::from(self) + rhs
    }
}

impl Sub for HpFloat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (mut a, mut b, mut scale) = align_scales(self.value, self.scale, rhs.value, rhs.scale);
        while a.checked_sub(b).is_none() {
            if scale == 0 {
                panic!("attempt to subtract with overflow");
            }
            let rem_diff = (a % 10) - (b % 10);
            let carry = if rem_diff >= 5 {
                (rem_diff + 5) / 10
            } else if rem_diff <= -5 {
                (rem_diff - 5) / 10
            } else {
                0
            };
            a = (a / 10) + carry;
            b /= 10;
            scale -= 1;
        }
        Self::new(a - b, scale)
    }
}

impl Sub<std::primitive::f64> for HpFloat {
    type Output = Self;

    fn sub(self, rhs: std::primitive::f64) -> Self::Output {
        self - HpFloat::from(rhs)
    }
}

impl Sub<std::primitive::f32> for HpFloat {
    type Output = Self;

    fn sub(self, rhs: std::primitive::f32) -> Self::Output {
        self - HpFloat::from(rhs)
    }
}

impl Sub<HpFloat> for std::primitive::f64 {
    type Output = HpFloat;

    fn sub(self, rhs: HpFloat) -> Self::Output {
        HpFloat::from(self) - rhs
    }
}

impl Sub<HpFloat> for std::primitive::f32 {
    type Output = HpFloat;

    fn sub(self, rhs: HpFloat) -> Self::Output {
        HpFloat::from(self) - rhs
    }
}

impl Neg for HpFloat {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.value, self.scale)
    }
}
