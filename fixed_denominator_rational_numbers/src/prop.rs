// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::debug::{AbsDiff, ApproxEq, PropDiff};
use crate::{FDRNum, UFDRNum};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Prop(pub(crate) u64);

impl Prop {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(u64::MAX);

    pub fn from_fraction(enumerator: u64, denominator: u64) -> Self {
        debug_assert!(denominator >= enumerator);
        Self(((enumerator as u128) * (u64::MAX as u128) / (denominator as u128)) as u64)
    }

    /// Return the square root of our value.
    ///
    /// Example:
    /// ```
    /// //use fixed_denominator_rational_numbers::{Prop, assert_approx_eq};
    ///
    /// //let one_quarter: Prop = Prop::ONE / 4;
    /// //assert_approx_eq!(one_quarter.sqrt(), Prop::ONE / 2);
    /// ```
    pub fn sqrt(self) -> Self {
        match self.0 {
            0 => Self(0),
            u64::MAX => Self(u64::MAX),
            _ => {
                let scaled = self.0 as u128 * u64::MAX as u128;
                Self(scaled.isqrt() as u64)
            }
        }
    }
}

impl AbsDiff for Prop {}

impl PropDiff for Prop {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        Some(self.abs_diff(other))
    }
}

impl ApproxEq for Prop {}

impl Debug for Prop {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Prop(0.{:016x})", self.0)
    }
}

impl fmt::UpperHex for Prop {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, formatter)
    }
}

impl fmt::LowerHex for Prop {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, formatter)
    }
}

impl fmt::Display for Prop {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "0.{:016x}", self.0)
    }
}

pub trait IntoProp: Sized + Copy + Into<Prop> {
    fn into_prop(self) -> Prop {
        self.into()
    }
}

impl From<[u64; 2]> for Prop {
    fn from(fraction: [u64; 2]) -> Self {
        debug_assert!(fraction[1] >= fraction[0]);
        Self(((fraction[0] as u128 * u64::MAX as u128) / fraction[1] as u128) as u64)
    }
}

#[macro_export]
macro_rules! impl_prop_to_from_float {
    ($float:ty, $number:ty) => {
        impl From<$float> for $number {
            fn from(arg: $float) -> Self {
                debug_assert!((0.0..=1.0).contains(&arg));
                // NB: watch out for floating point not being proper reals
                Self((arg * u64::MAX as $float) as u64)
            }
        }

        impl From<$number> for $float {
            fn from(arg: $number) -> Self {
                arg.0 as $float / u64::MAX as $float
            }
        }
    };
}

impl IntoProp for f32 {}

impl IntoProp for f64 {}

impl_prop_to_from_float!(f32, Prop);
impl_prop_to_from_float!(f64, Prop);

#[macro_export]
macro_rules! impl_to_from_number {
    ($number:ty, $core:ty, $proportion:ty) => {
        impl From<$number> for $proportion {
            #[allow(unused_comparisons)]
            fn from(arg: $number) -> Self {
                debug_assert!(arg.0 >= 0 && arg.0 <= u64::MAX as $core);
                Self(arg.0 as u64)
            }
        }

        impl From<$proportion> for $number {
            fn from(arg: $proportion) -> Self {
                Self(arg.0 as $core)
            }
        }
    };
}

impl_to_from_number!(UFDRNum, u128, Prop);
impl_to_from_number!(FDRNum, i128, Prop);

impl IntoProp for UFDRNum {}

impl IntoProp for FDRNum {}

#[macro_export]
macro_rules! impl_unsigned_to_from_prop {
    (u64) => {
        impl From<u64> for Prop {
            fn from(arg: u64) -> Self {
                Self(arg as u64)
            }
        }

        impl From<Prop> for u64 {
            fn from(arg: Prop) -> Self {
                arg.0 as u64
            }
        }

        impl IntoProp for u64 {}
    };
    ($unsigned:ty) => {
        impl From<$unsigned> for Prop {
            fn from(arg: $unsigned) -> Self {
                let val = arg as u128 * u64::MAX as u128 / <$unsigned>::MAX as u128;
                Self(val as u64)
            }
        }

        impl From<Prop> for $unsigned {
            fn from(arg: Prop) -> Self {
                let val = arg.0 as u128 * <$unsigned>::MAX as u128 / u64::MAX as u128;
                val as $unsigned
            }
        }

        impl IntoProp for $unsigned {}
    };
}

impl_unsigned_to_from_prop!(u8);
impl_unsigned_to_from_prop!(u16);
impl_unsigned_to_from_prop!(u32);
impl_unsigned_to_from_prop!(u64);

impl Mul for Prop {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(((self.0 as u128 * rhs.0 as u128) / u64::MAX as u128) as u64)
    }
}

impl Mul<u8> for Prop {
    type Output = UFDRNum;

    fn mul(self, rhs: u8) -> UFDRNum {
        UFDRNum(self.0 as u128 * rhs as u128)
    }
}

impl Mul<u32> for Prop {
    type Output = UFDRNum;

    fn mul(self, rhs: u32) -> UFDRNum {
        UFDRNum(self.0 as u128 * rhs as u128)
    }
}

impl Mul<i32> for Prop {
    type Output = UFDRNum;

    fn mul(self, rhs: i32) -> UFDRNum {
        UFDRNum(self.0 as u128 * TryInto::<u32>::try_into(rhs).unwrap() as u128)
    }
}

impl Div for Prop {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if rhs == Self::ONE {
            self
        } else {
            Self((((u128::MAX / rhs.0 as u128) * self.0 as u128) / u64::MAX as u128) as u64)
        }
    }
}

impl Div<u8> for Prop {
    type Output = Self;

    fn div(self, rhs: u8) -> Self {
        Prop(self.0 / rhs as u64)
    }
}

impl Div<u32> for Prop {
    type Output = Self;

    fn div(self, rhs: u32) -> Self {
        Prop(self.0 / rhs as u64)
    }
}

impl Div<i32> for Prop {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Prop(self.0 / TryInto::<u32>::try_into(rhs).unwrap() as u64)
    }
}

impl Add for Prop {
    type Output = UFDRNum;

    fn add(self, rhs: Self) -> UFDRNum {
        UFDRNum(self.0 as u128 + rhs.0 as u128)
    }
}

impl Sub for Prop {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        debug_assert!(self.0 >= rhs.0);
        Self(self.0 - rhs.0)
    }
}

impl Add<Prop> for UFDRNum {
    type Output = Self;

    fn add(self, rhs: Prop) -> UFDRNum {
        UFDRNum(self.0 + rhs.0 as u128)
    }
}

impl Sub<Prop> for UFDRNum {
    type Output = Self;

    fn sub(self, rhs: Prop) -> Self {
        debug_assert!(self.0 >= rhs.0 as u128);
        Self(self.0 - rhs.0 as u128)
    }
}

impl Div<Prop> for UFDRNum {
    type Output = Self;

    fn div(self, rhs: Prop) -> Self {
        if rhs == Prop::ONE {
            self
        } else {
            match self.0.cmp(&(rhs.0 as u128)) {
                Ordering::Equal => Self::ONE,
                Ordering::Less | Ordering::Greater => self.mul(Self(u128::MAX / rhs.0 as u128)),
            }
        }
    }
}

impl Mul<Prop> for UFDRNum {
    type Output = Self;

    fn mul(self, rhs: Prop) -> Self {
        if rhs.0 == u64::MAX {
            self
        } else if self.0 >= u64::MAX as u128 {
            let one = u64::MAX as u128;
            let l_int = self.0 / one;
            let l_rem = self.0 % one;
            Self(l_int * rhs.0 as u128 + rhs.0 as u128 * l_rem / one)
        } else {
            Self((self.0 * rhs.0 as u128) / u64::MAX as u128)
        }
    }
}
