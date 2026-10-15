// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::ops::Sub;

pub trait Unsigned {}
pub trait UnsignedPrimitive: Unsigned + Into<u128> {}

impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}
impl UnsignedPrimitive for u8 {}
impl UnsignedPrimitive for u16 {}
impl UnsignedPrimitive for u32 {}
impl UnsignedPrimitive for u64 {}
impl UnsignedPrimitive for u128 {}

pub trait Signed: Unsigned {}

impl Signed for u8 {}
impl Signed for u16 {}
impl Signed for u32 {}
impl Signed for u64 {}
impl Signed for u128 {}

pub trait Float {}

impl Float for f32 {}
impl Float for f64 {}

pub trait MinNonZero {
    fn min_non_zero() -> Self;
    fn times_min_non_zero(times: u8) -> Self;
}

pub trait ApproxEq: Copy + PartialEq + PartialOrd + Ord + MinNonZero + Sub<Output = Self> {
    fn approx_eq(&self, other: &Self, max_diff: Option<u8>) -> bool {
        if self.eq(other) {
            true
        } else {
            let max_diff = if let Some(max_diff) = max_diff {
                Self::times_min_non_zero(max_diff)
            } else {
                Self::min_non_zero()
            };
            let diff = if self >= other {
                *self - *other
            } else {
                *other - *self
            };
            diff <= max_diff
        }
    }
}
