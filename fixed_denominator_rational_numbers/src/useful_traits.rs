// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::fmt::Debug;
use std::ops::{Mul, Sub};

pub trait FromAsProp<T: Sized> {
    fn from_as_prop(value: T) -> Self;
}

pub trait IntoAsProp<T: Sized> {
    fn into_as_prop(self) -> T;
}

#[macro_export]
macro_rules! impl_into_as_prop {
    ($into_type:ident, $for_type:ty) => {
        impl IntoAsProp<$into_type> for $for_type {
            fn into_as_prop(self) -> $into_type {
                $into_type::from_as_prop(self)
            }
        }
    };
}

pub trait AsProportion: Sized + Copy + PartialEq + PartialOrd + Debug {
    const ZERO: Self;
    const ALMOST_ZERO: Self;
    const ONE: Self;
    const ALMOST_ONE: Self;
    const ONE_QUARTER: Self;
    const HALF: Self;
    const THREE_QUARTERS: Self;

    fn is_valid_proportion(&self) -> bool {
        *self >= Self::ZERO && *self <= Self::ONE
    }
}

impl AsProportion for f64 {
    const ONE: Self = 1.0;
    const ALMOST_ONE: Self = 1.0 - 1.0e-20;
    const ZERO: Self = 0.0;
    const ALMOST_ZERO: Self = 1.0e-20;
    const ONE_QUARTER: Self = 0.25;
    const HALF: Self = 0.5;
    const THREE_QUARTERS: Self = 0.75;
}

macro_rules! impl_as_proportion {
    ($unsigned:ty) => {
        impl AsProportion for $unsigned {
            const ZERO: Self = 0;
            const ALMOST_ZERO: Self = 1;
            const ONE: Self = Self::MAX;
            const ALMOST_ONE: Self = Self::MAX - 1;
            const ONE_QUARTER: Self = Self::MAX / 4;
            const HALF: Self = Self::MAX / 2;
            const THREE_QUARTERS: Self = Self::MAX / 4 * 3;
        }
    };
}

impl_as_proportion!(u8);
impl_as_proportion!(u16);
impl_as_proportion!(u32);
impl_as_proportion!(u64);

pub trait IntoFromRawU128 {
    /// Raw from an u128 value (will panic if value is greater than the maximum value of Self)
    fn from_u128(value: u128) -> Self;
    /// Into a raw u128 value
    fn into_u128(self) -> u128;
}

impl<U: UnsignedInteger> IntoFromRawU128 for U {
    fn from_u128(value: u128) -> Self {
        value
            .try_into()
            .map_err(|_| "whatever")
            .expect("Value is greater than the maximum value of Self")
    }

    fn into_u128(self) -> u128 {
        self.into()
    }
}

pub trait Unsigned: Copy + Debug {}
pub trait UnsignedInteger: Unsigned + Into<u128> + TryFrom<u128> {}

impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}

impl UnsignedInteger for u8 {}
impl UnsignedInteger for u16 {}
impl UnsignedInteger for u32 {}
impl UnsignedInteger for u64 {}

pub trait IntoFromRawI128 {
    /// Raw from an i128 value (will panic if value is greater than the maximum value of Self)
    fn from_i128(value: i128) -> Self;
    /// Into a raw u128 value
    fn into_i128(self) -> i128;
}

impl<S: SignedInteger> IntoFromRawI128 for S {
    fn from_i128(value: i128) -> Self {
        value
            .try_into()
            .map_err(|_| "whatever")
            .expect("Value is greater than the maximum value of u8")
    }

    fn into_i128(self) -> i128 {
        self.into()
    }
}

pub trait Signed: Copy + Debug {}
pub trait SignedInteger: Signed + Into<i128> + TryFrom<i128> {}

impl Signed for i8 {}
impl Signed for i16 {}
impl Signed for i32 {}
impl Signed for i64 {}
impl Signed for i128 {}
impl SignedInteger for i8 {}
impl SignedInteger for i16 {}
impl SignedInteger for i32 {}
impl SignedInteger for i64 {}

pub trait Float:
    Signed + Sized + Mul<Output = Self> + Sub<Output = Self> + From<f32> + Debug
{
    fn is_negative(&self) -> bool;
    fn into_u128(self) -> u128;
    fn into_i128(self) -> i128;
    fn from_u128(u: u128) -> Self;
    fn from_i128(i: i128) -> Self;
}

impl Signed for f64 {}

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
