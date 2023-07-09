use {
    crate::*,
    std::cmp::Ordering,
};

// float trait
pub trait Float: Real {
    const DIGITS: u32;
    const MANTISSA_DIGITS: u32;
    const EPSILON: Self;
    const MIN_POSITIVE: Self;
    const MIN_EXP: i32;
    const MAX_EXP: i32;
    const MIN_10_EXP: i32;
    const MAX_10_EXP: i32;
    const NAN: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const RADIX: u32;
    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;
    fn is_subnormal(self) -> bool;
    fn is_normal(self) -> bool;
    fn total_cmp(&self,other: &Self) -> Ordering;
}

macro_rules! float_impl {
    ($($t:ty)+) => {
        $(
            impl Float for $t {
                const DIGITS: u32 = Self::DIGITS;
                const MANTISSA_DIGITS: u32 = Self::MANTISSA_DIGITS;
                const EPSILON: Self = Self::EPSILON;
                const MIN_POSITIVE: Self = Self::MIN_POSITIVE;
                const MIN_EXP: i32 = Self::MIN_EXP;
                const MAX_EXP: i32 = Self::MAX_EXP;
                const MIN_10_EXP: i32 = Self::MIN_10_EXP;
                const MAX_10_EXP: i32 = Self::MAX_10_EXP;
                const NAN: Self = Self::NAN;
                const INFINITY: Self = Self::INFINITY;
                const NEG_INFINITY: Self = Self::NEG_INFINITY;
                const RADIX: u32 = Self::RADIX;
                fn is_nan(self) -> bool { self.is_nan() }
                fn is_infinite(self) -> bool { self.is_infinite() }
                fn is_finite(self) -> bool { self.is_finite() }
                fn is_subnormal(self) -> bool { self.is_subnormal() }
                fn is_normal(self) -> bool { self.is_normal() }
                fn total_cmp(&self,other: &Self) -> Ordering { self.total_cmp(other) }
            }
        )+
    }
}

float_impl! { f32 f64 }
