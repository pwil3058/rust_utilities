// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::fmt::Debug;
use std::ops::{Mul, Sub};

pub trait AsProportion: Sized + Copy + PartialEq + PartialOrd + Debug {
    const ONE: Self;
    const ZERO: Self;

    fn is_valid_proportion(&self) -> bool {
        *self >= Self::ZERO && *self <= Self::ONE
    }
}

impl AsProportion for f64 {
    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
}

impl AsProportion for u8 {
    const ONE: Self = u8::MAX;
    const ZERO: Self = 0;
}

impl AsProportion for u16 {
    const ONE: Self = u16::MAX;
    const ZERO: Self = 0;
}

impl AsProportion for u32 {
    const ONE: Self = u32::MAX;
    const ZERO: Self = 0;
}

impl AsProportion for u64 {
    const ONE: Self = u64::MAX;
    const ZERO: Self = 0;
}

pub trait FromU128: Copy + Into<u128> {
    fn from_u128(value: u128) -> Self;
}

pub trait Unsigned: Copy + Debug {
    const MAX: Self;
}

pub trait UnsignedInteger: Unsigned + Into<u128> + TryFrom<u128> + Debug {}

impl Unsigned for u8 {
    const MAX: Self = u8::MAX;
}

impl Unsigned for u16 {
    const MAX: Self = u16::MAX;
}

impl Unsigned for u32 {
    const MAX: Self = u32::MAX;
}

impl Unsigned for u64 {
    const MAX: Self = u64::MAX;
}

impl Unsigned for u128 {
    const MAX: Self = u128::MAX;
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
