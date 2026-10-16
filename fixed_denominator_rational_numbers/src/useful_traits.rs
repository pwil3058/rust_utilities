// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::ops::Sub;

pub trait Unsigned {}
pub trait UnsignedInteger: Unsigned + Into<u128> {}

impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}
impl UnsignedInteger for u8 {}
impl UnsignedInteger for u16 {}
impl UnsignedInteger for u32 {}
impl UnsignedInteger for u64 {}
impl UnsignedInteger for u128 {}

pub trait Signed {}
pub trait SignedInteger: Signed + Into<i128> {}

impl Signed for i8 {}
impl Signed for i16 {}
impl Signed for i32 {}
impl Signed for i64 {}
impl Signed for i128 {}
impl SignedInteger for i8 {}
impl SignedInteger for i16 {}
impl SignedInteger for i32 {}
impl SignedInteger for i64 {}
impl SignedInteger for i128 {}

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
