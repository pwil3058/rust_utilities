// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cmp::Ordering;
// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Sub};

use serde::{Deserialize, Serialize};

use crate::{ApproxEq, MinNonZero, Unsigned, UnsignedPrimitive, impl_to_from_float};

/// Fixed denominator rational number as an approximate representation of real numbers
#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UFDRNum(pub(crate) u128);

impl Debug for UFDRNum {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let int = self.0 / Self::DENOMINATOR;
        let frac = self.0 % Self::DENOMINATOR;
        formatter.write_fmt(format_args!("UFDRNumber({int:X}.{frac:016X})"))
    }
}

impl MinNonZero for UFDRNum {
    fn min_non_zero() -> Self {
        UFDRNum(1)
    }

    fn times_min_non_zero(times: u8) -> Self {
        UFDRNum(times as u128)
    }
}

impl Unsigned for UFDRNum {}
impl ApproxEq for UFDRNum {}

impl UFDRNum {
    pub const DENOMINATOR: u128 = u64::MAX as u128;
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(Self::DENOMINATOR);

    /// Generate an UFDRNum from a rational number
    ///
    /// Example:
    /// ```
    /// use fixed_denominator_rational_numbers::{assert_approx_eq, UFDRNum, ApproxEq, MinNonZero};
    ///
    /// assert_eq!(UFDRNum::from_rational_num(4, 2), UFDRNum::ONE * 2_u8);
    /// assert_approx_eq!(UFDRNum::from_rational_num(5, 2), UFDRNum::from(2.5_f64), 2);
    /// assert_eq!(UFDRNum::from_rational_num(17, 17), UFDRNum::ONE);
    /// assert_approx_eq!(UFDRNum::from_rational_num(5, 10), UFDRNum::ONE / 2_u8, 1);
    /// assert_eq!(UFDRNum::from_rational_num(u64::MAX as u128 * 2, u64::MAX as u128), UFDRNum::ONE * 2_u8);
    /// //assert_approx_eq!(UFDRNum::from_rational_num(u128::MAX, u64::MAX as u128), UFDRNum::ONE * u64::MAX);
    /// ```
    pub fn from_rational_num(enumerator: u128, denominator: u128) -> Self {
        if enumerator == 0 {
            return Self::ZERO;
        }
        use Ordering::*;
        match denominator.cmp(&enumerator) {
            Equal => Self::ONE,
            Greater => {
                let shift = enumerator.leading_zeros();
                let result = (enumerator << shift) / denominator as u128;
                Self(result >> (shift - 64))
            }
            Less => {
                let int = Self(enumerator / denominator * Self::DENOMINATOR);
                int + Self::from_rational_num(enumerator % denominator, denominator)
            }
        }
    }
}

// Conversions
impl<U: UnsignedPrimitive> From<U> for UFDRNum {
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

impl_to_from_float!(f64, u128, UFDRNum);
impl_to_from_float!(f32, u128, UFDRNum);

// Ops
// Mul
impl Mul for UFDRNum {
    type Output = Self;

    /// Implement multiplication for UFDRNum type
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{assert_approx_eq, UFDRNum, ApproxEq};
    ///
    /// assert_approx_eq!(UFDRNum::from(0.5_f64) * UFDRNum::from(2_u8), UFDRNum::ONE);
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

impl<U: UnsignedPrimitive> Mul<U> for UFDRNum {
    type Output = Self;

    /// Multiply an UFDRnum by an unsigned integer
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::UFDRNum;
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
    /// use fixed_denominator_rational_numbers::{assert_approx_eq, UFDRNum, ApproxEq, MinNonZero};
    ///
    /// assert_approx_eq!(UFDRNum::from(5_u64) / UFDRNum::from(2_u8), UFDRNum::from(2.5_f64), 3_u8);
    /// ```
    fn div(self, rhs: Self) -> Self {
        match rhs.0 % Self::DENOMINATOR {
            // NB: faster AND more accurate
            0 => Self(self.0 / (rhs.0 / Self::DENOMINATOR)),
            _ => match self.0.cmp(&rhs.0) {
                Ordering::Equal => Self::ONE,
                Ordering::Less | Ordering::Greater => self.mul(Self(u128::MAX / rhs.0)),
            },
        }
    }
}

impl<U: UnsignedPrimitive> Div<U> for UFDRNum {
    type Output = Self;

    /// Multiply an UFDRnum by an unsigned integer
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{assert_approx_eq, UFDRNum, ApproxEq, MinNonZero};
    ///
    /// assert_approx_eq!(UFDRNum::ONE / 4_u32, UFDRNum::from(0.25_f64), 2_u8);
    /// ```
    fn div(self, rhs: U) -> Self::Output {
        // N.B. Do this separately to avoid ambiguity with U::into::<UFDRNum>().
        let rhs: u128 = rhs.into();
        Self(self.0 / rhs)
    }
}

// Add
impl Add for UFDRNum {
    type Output = Self;

    /// Add one UFDRNum from this number.
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{UFDRNum, ApproxEq, MinNonZero, assert_approx_eq};
    ///
    /// assert_eq!(UFDRNum::ONE * 2_u8 + UFDRNum::ONE, UFDRNum::ONE * 3_u8);
    /// assert_approx_eq!(UFDRNum::from_rational_num(15, 2) + UFDRNum::from_rational_num(1, 2), UFDRNum::from(8_u8), 1_u8);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.0 >= rhs.0);
        Self(self.0 + rhs.0)
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
    /// use fixed_denominator_rational_numbers::{UFDRNum, ApproxEq, MinNonZero};
    ///
    /// assert_eq!(UFDRNum::ONE * 2_u8 - UFDRNum::ONE, UFDRNum::ONE);
    /// assert_eq!(UFDRNum::from(7.5_f64) - UFDRNum::from(5.2_f64), UFDRNum::from(2.3_f64));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.0 >= rhs.0);
        Self(self.0 - rhs.0)
    }
}
