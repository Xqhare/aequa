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

#[cfg(test)]
mod tests {
    use crate::hp_float::{E, HpFloat, PI, TAU};

    #[test]
    fn overflow_div() {
        // Time between now and a char birthday in sec (value taken directly from sscc)
        let hp = HpFloat::cast(610642799);
        let t1: HpFloat = hp / 60.0 / 60.0;
        // Initial line from `space-ship-construction-company`
        let err = hp / 60.0_f64 / 60.0_f64 / 24.0_f64 / 365.2425_f64;

        assert_eq!(t1.get_integer_part(), 169622);
        assert_eq!(err.get_integer_part(), 19);
        assert!(
            err.to_string()
                .starts_with("19.350499978578412769395472667956")
        );

        // Division with high-precision constants (scale 31)
        let half_pi = PI / 2.0_f64;
        assert!(half_pi.to_string().starts_with("1.57079632679489661923"));
    }

    #[test]
    fn overflow_mul() {
        // Multiplying two high-precision values (scale 31 each -> 62 digits without rescaling)
        let pi_sq = PI * PI;
        assert_eq!(pi_sq.get_integer_part(), 9);
        assert!(pi_sq.to_string().starts_with("9.86960440108935861883"));

        // Multiplying result of repeating division by another float
        let hp = HpFloat::cast(610642799);
        let div_then_mul = (hp / 60.0_f64) * 60.0_f64;
        // Should be very close to 610642799 (610642798.99999999999999999999...)
        let diff = (hp - div_then_mul).get_integer_part();
        assert_eq!(diff, 0);
    }

    #[test]
    fn overflow_add() {
        // Adding a large integer (10 digits) to PI (scale 31 -> 10 + 31 = 41 digits > 38)
        let large = HpFloat::cast(1_000_000_000_i64);
        let sum = large + PI;
        assert_eq!(sum.get_integer_part(), 1_000_000_003);
        assert!(
            sum.to_string()
                .starts_with("1000000003.14159265358979323846")
        );

        let sum_constants = PI + E + TAU;
        assert_eq!(sum_constants.get_integer_part(), 12);
    }

    #[test]
    fn overflow_sub() {
        // Subtracting PI (scale 31) from a large integer (10 digits)
        let large = HpFloat::cast(1_000_000_000_i64);
        let diff = large - PI;
        assert_eq!(diff.get_integer_part(), 999_999_996);
        assert!(
            diff.to_string()
                .starts_with("999999996.85840734641020676153")
        );
    }
}
