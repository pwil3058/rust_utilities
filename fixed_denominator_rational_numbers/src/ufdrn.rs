// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use gcd::Gcd;
use serde::{Deserialize, Serialize};

use crate::debug::{AbsDiff, ApproxEq, PropDiff};
use crate::{AlmostZero, FDRNum, Float, Prop, Unsigned, UnsignedInteger};

/// Fixed denominator rational number as an approximate representation of real numbers
#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UFDRNum(pub(crate) u128);

impl AlmostZero for UFDRNum {
    fn almost_zero() -> Self {
        UFDRNum(1)
    }

    fn times_almost_zero(times: u128) -> Self {
        UFDRNum(times)
    }
}

impl Unsigned for UFDRNum {
    const MAX: Self = UFDRNum(u128::MAX);
    const ZERO: Self = UFDRNum(0);
}

impl UFDRNum {
    pub const DENOMINATOR: u128 = u64::MAX as u128;
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(Self::DENOMINATOR);

    pub fn is_proportion(self) -> bool {
        self <= Self::ONE
    }

    /// Generate an UFDRNum from a rational number
    ///
    /// Example:
    /// ```
    /// use fixed_denominator_rational_numbers::{UFDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(UFDRNum::from_rational_num(4, 2), UFDRNum::ONE * 2_u8);
    /// assert_eq!(UFDRNum::from_rational_num(5, 2), UFDRNum::from(2.5));
    /// assert_eq!(UFDRNum::from_rational_num(17, 17), UFDRNum::ONE);
    /// assert_eq!(UFDRNum::from_rational_num(5, 10), UFDRNum::from(0.5));
    /// assert_eq!(UFDRNum::from_rational_num(u64::MAX as u128 * 2, u64::MAX as u128), UFDRNum::ONE * 2_u8);
    /// ```
    pub fn from_rational_num(enumerator: u128, denominator: u128) -> Self {
        assert!(denominator > 0);
        if enumerator == 0 {
            Self::ZERO
        } else {
            let gcd = enumerator.gcd(denominator);
            let enumerator = enumerator / gcd;
            let denominator = denominator / gcd;
            if enumerator == denominator {
                Self::ONE
            } else if denominator == 1 {
                Self(enumerator * Self::DENOMINATOR)
            } else {
                let shift = enumerator.leading_zeros();
                let temp = (enumerator << shift) / denominator;
                use Ordering::*;
                match shift.cmp(&64) {
                    Less => UFDRNum(temp << (64 - shift)),
                    Equal => UFDRNum(temp),
                    Greater => UFDRNum(temp >> (shift - 64)),
                }
            }
        }
    }

    /// Return the square root of our value.
    ///
    /// Example:
    /// ```
    /// use fixed_denominator_rational_numbers::{UFDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// let four: UFDRNum = UFDRNum::ONE * 4_u8;
    /// let two: UFDRNum = UFDRNum::ONE * 2_u8;
    /// assert_approx_eq!(four.sqrt(), two);
    /// let twenty_five: UFDRNum = UFDRNum::ONE * 25_u8;
    /// let five: UFDRNum = UFDRNum::ONE * 5_u8;
    /// assert_approx_eq!(twenty_five.sqrt(), five);
    /// assert_approx_eq!(UFDRNum::from_rational_num(1, 4).sqrt(), UFDRNum::from_rational_num(1, 2));
    /// ```
    pub fn sqrt(self) -> Self {
        let half_shift = self.0.leading_zeros() / 2;
        let temp = (self.0 << (half_shift as usize * 2)).isqrt();
        match half_shift.cmp(&32) {
            Ordering::Equal => Self(temp),
            Ordering::Greater => Self(temp >> (half_shift as usize - 32)),
            Ordering::Less => Self(temp << (32 - half_shift as u128)),
        }
    }
}

impl Debug for UFDRNum {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let int = self.0 / Self::DENOMINATOR;
        let frac = self.0 % Self::DENOMINATOR;
        formatter.write_fmt(format_args!("UFDRNumber({int:X}.{frac:016X})"))
    }
}

impl AbsDiff for UFDRNum {}

impl PropDiff for UFDRNum {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        if self.is_proportion() && other.is_proportion() {
            Some(self.abs_diff(other).into())
        } else {
            let denom = self.max(other);
            if *denom == UFDRNum::ZERO {
                Some(Prop::ZERO)
            } else {
                Some(Prop::from(self.abs_diff(other) / *denom))
            }
        }
    }
}

impl ApproxEq for UFDRNum {}

// Conversions
impl From<FDRNum> for UFDRNum {
    fn from(num: FDRNum) -> Self {
        assert!(!num.0.is_negative());
        Self(num.0.cast_unsigned())
    }
}

impl<U: UnsignedInteger> From<U> for UFDRNum {
    /// Convert an unsigned integer into UFDRNum with a value equal to the unsigned integer.
    ///
    /// Example:
    /// ```
    /// use fixed_denominator_rational_numbers::UFDRNum;
    ///
    /// assert_eq!(UFDRNum::from(1_u32), UFDRNum::ONE);
    /// assert_eq!(UFDRNum::from(0_u64), UFDRNum::ZERO);
    /// assert_eq!(UFDRNum::from(5_u32), UFDRNum::ONE * 5_u8);
    /// ```
    fn from(unsigned: U) -> Self {
        Self(unsigned.into() * Self::DENOMINATOR)
    }
}

impl From<f64> for UFDRNum {
    /// Convert from f64
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{Float,UFDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_approx_eq!(UFDRNum::from(1.0), UFDRNum::ONE);
    /// assert_approx_eq!(UFDRNum::from(0.25), UFDRNum::ONE / 4u8)
    /// ```
    fn from(arg: f64) -> Self {
        assert!(!arg.is_negative());
        Self((arg * Self::DENOMINATOR as f64) as u128)
    }
}

impl From<FDRNum> for f64 {
    fn from(arg: FDRNum) -> Self {
        arg.0 as f64 / u64::MAX as f64
    }
}

// Ops
// Add
impl Add for UFDRNum {
    type Output = Self;

    /// Add one UFDRNum from this number.
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{UFDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(UFDRNum::ONE * 2_u8 + UFDRNum::ONE, UFDRNum::ONE * 3_u8);
    /// assert_approx_eq!(UFDRNum::from_rational_num(15, 2) + UFDRNum::from_rational_num(1, 2), UFDRNum::from(8_u8), 1);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for UFDRNum {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// Sub
impl Sub for UFDRNum {
    type Output = Self;

    /// Subtract one UFDRNum from this number. (Will panic if rhs > self).
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{UFDRNum};
    ///
    /// assert_eq!(UFDRNum::ONE * 2_u8 - UFDRNum::ONE, UFDRNum::ONE);
    /// assert_eq!(UFDRNum::from(7.5) - UFDRNum::from(5.2), UFDRNum::from(2.3));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.0 >= rhs.0);
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for UFDRNum {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// Mul
impl Mul for UFDRNum {
    type Output = Self;

    /// Implement multiplication for UFDRNum type
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{UFDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_approx_eq!(UFDRNum::from(0.5) * UFDRNum::from(2_u8), UFDRNum::ONE);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        let l_int = self.0 / Self::DENOMINATOR;
        let l_rem = self.0 % Self::DENOMINATOR;
        let r_int = rhs.0 / Self::DENOMINATOR;
        let r_rem = rhs.0 % Self::DENOMINATOR;
        Self(
            l_int * r_int * Self::DENOMINATOR
                + l_int * r_rem
                + r_int * l_rem
                + r_rem * l_rem / Self::DENOMINATOR,
        )
    }
}

impl MulAssign for UFDRNum {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<U: UnsignedInteger> Mul<U> for UFDRNum {
    type Output = Self;

    /// Multiply an UFDRnum by an unsigned integer
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{UFDRNum};
    ///
    /// assert_eq!(UFDRNum::ONE * 5_u32, UFDRNum::from(5_u32));
    /// ```
    fn mul(self, rhs: U) -> Self::Output {
        // N.B. Do this separately to avoid ambiguity with U::into::<UFDRNum>().
        let rhs: u128 = rhs.into();
        Self(self.0 * rhs)
    }
}

// Div
impl Div for UFDRNum {
    type Output = Self;

    /// Implement division for UFDRNum type
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{UFDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(UFDRNum::from(5_u64) / UFDRNum::from(10_u64), UFDRNum::from_rational_num(1,2));
    /// assert_eq!(UFDRNum::from(5_u64) / UFDRNum::from(2_u8), UFDRNum::from_rational_num(5,2));
    /// assert_eq!(UFDRNum::from(6_u64) / UFDRNum::from(2_u8), UFDRNum::from(3_u64));
    /// assert_eq!(UFDRNum::from(6_u64) / UFDRNum::from(6_u8), UFDRNum::ONE);
    /// assert_eq!(UFDRNum::from(7_u64) / UFDRNum::from(4_u8), UFDRNum::from_rational_num(7, 4));
    /// assert_eq!(UFDRNum::from_rational_num(1,2) / UFDRNum::from_rational_num(1,4), UFDRNum::from(2u8));
    /// assert_eq!(UFDRNum::ONE / UFDRNum::from(5u8), UFDRNum::from_rational_num(1,5));
    /// ```
    fn div(self, rhs: Self) -> Self {
        Self::from_rational_num(self.0, rhs.0)
    }
}

impl DivAssign for UFDRNum {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<U: UnsignedInteger> Div<U> for UFDRNum {
    type Output = Self;

    /// Divide an UFDRnum by an unsigned integer
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{UFDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_approx_eq!(UFDRNum::ONE / 4_u32, UFDRNum::from(0.25), 2);
    /// ```
    fn div(self, rhs: U) -> Self::Output {
        // N.B. Do this separately to avoid ambiguity with U::into::<UFDRNum>().
        let rhs: u128 = rhs.into();
        Self(self.0 / rhs)
    }
}

// Rem
impl Rem for UFDRNum {
    type Output = Self;

    /// Return the remainder after dividing self by rhs
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::UFDRNum;
    ///
    /// assert_eq!(UFDRNum::ONE % (UFDRNum::ONE * 4_u8), UFDRNum::ONE);
    /// assert_eq!((UFDRNum::ONE * 5u8) % (UFDRNum::ONE * 4_u8), UFDRNum::ONE);
    /// ```
    fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }
}

impl RemAssign for UFDRNum {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl<U: UnsignedInteger> Rem<U> for UFDRNum {
    type Output = Self;

    /// Remainder of an UFDRnum after division by an unsigned integer
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{UFDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_approx_eq!(UFDRNum::ONE / 4_u32, UFDRNum::from(0.25), 2);
    /// ```
    fn rem(self, rhs: U) -> Self::Output {
        // N.B. Do this separately to avoid ambiguity with U::into::<UFDRNum>().
        let rhs: u128 = rhs.into();
        Self(self.0 % rhs)
    }
}
