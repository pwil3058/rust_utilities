// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::fmt::Debug;
use std::ops::{Mul, Sub};

pub trait Unsigned: Copy + Debug {
    const MAX: Self;
    const ZERO: Self;
    const MIN: Self = Self::ZERO;
}
pub trait UnsignedInteger: Unsigned + Into<u128> {}

impl Unsigned for u8 {
    const MAX: Self = u8::MAX;
    const ZERO: Self = 0;
}

impl Unsigned for u16 {
    const MAX: Self = u16::MAX;
    const ZERO: Self = 0;
}

impl Unsigned for u32 {
    const MAX: Self = u32::MAX;
    const ZERO: Self = 0;
}

impl Unsigned for u64 {
    const MAX: Self = u64::MAX;
    const ZERO: Self = 0;
}

impl Unsigned for u128 {
    const MAX: Self = u128::MAX;
    const ZERO: Self = 0;
}

impl UnsignedInteger for u8 {}
impl UnsignedInteger for u16 {}
impl UnsignedInteger for u32 {}
impl UnsignedInteger for u64 {}
impl UnsignedInteger for u128 {}

pub trait Signed: Copy + Debug {}
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

pub trait Float:
    Copy + Sized + Mul<Output = Self> + Sub<Output = Self> + From<f32> + Debug
{
    fn is_negative(&self) -> bool;
    fn into_u128(self) -> u128;
    fn into_i128(self) -> i128;
    fn from_u128(u: u128) -> Self;
    fn from_i128(i: i128) -> Self;
}

impl Float for f32 {
    fn is_negative(&self) -> bool {
        *self < 0.0
    }

    fn into_u128(self) -> u128 {
        self as u128
    }

    fn into_i128(self) -> i128 {
        self as i128
    }

    fn from_u128(u: u128) -> Self {
        u as Self
    }

    fn from_i128(i: i128) -> Self {
        i as Self
    }
}

impl Float for f64 {
    fn is_negative(&self) -> bool {
        *self < 0.0
    }

    fn into_u128(self) -> u128 {
        self as u128
    }

    fn into_i128(self) -> i128 {
        self as i128
    }

    fn from_u128(u: u128) -> Self {
        u as Self
    }

    fn from_i128(i: i128) -> Self {
        i as Self
    }
}

// pub trait FromNumber<T: Copy + Sized>: Sized + Copy + Debug {
//     fn from_number(number: T) -> Self;
// }
//
// pub trait IntoNumber<T: Copy + Sized + FromNumber<Self>>: Sized + Copy + Debug {
//     fn into_number(self) -> T {
//         T::from_number(self)
//     }
// }

pub trait AlmostZero {
    fn almost_zero() -> Self;
    fn times_almost_zero(times: u128) -> Self;
}

pub trait ApproxEq: Copy + PartialEq + PartialOrd + Ord + AlmostZero + Sub<Output = Self> {
    fn approx_eq(&self, other: &Self, max_diff: Option<u128>) -> bool {
        if self.eq(other) {
            true
        } else {
            let max_diff = if let Some(max_diff) = max_diff {
                Self::times_almost_zero(max_diff)
            } else {
                Self::almost_zero()
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
