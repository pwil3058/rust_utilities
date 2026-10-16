// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use serde::{Deserialize, Serialize};

use crate::{ApproxEq, MinNonZero, Signed, SignedInteger, UFDRNum, impl_to_from_float};

/// Fixed denominator rational number as an approximate representation of real numbers
#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct FDRNum(pub(crate) i128);

impl Debug for FDRNum {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let int = self.0.abs() / Self::DENOMINATOR;
        let frac = self.0.abs() % Self::DENOMINATOR;
        if self.0 < 0 {
            formatter.write_fmt(format_args!("FDRNumber(-{int:X}.{frac:016X})"))
        } else {
            formatter.write_fmt(format_args!("FDRNumber({int:X}.{frac:016X})"))
        }
    }
}

impl MinNonZero for FDRNum {
    fn min_non_zero() -> Self {
        FDRNum(1)
    }

    fn times_min_non_zero(times: u8) -> Self {
        FDRNum(times as i128)
    }
}

impl Signed for UFDRNum {}
impl ApproxEq for FDRNum {}

impl FDRNum {
    pub const DENOMINATOR: i128 = u64::MAX as i128;
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(Self::DENOMINATOR);
    pub const MINUS_ONE: Self = Self(-Self::DENOMINATOR);

    pub fn is_negative(self) -> bool {
        self.0.is_negative()
    }

    /// Generate an UFDRNum from a rational number
    ///
    /// Example:
    /// ```
    /// use fixed_denominator_rational_numbers::{assert_approx_eq, FDRNum, ApproxEq, MinNonZero};
    ///
    /// assert_eq!(FDRNum::from_rational_num(4, 2), FDRNum::ONE * 2);
    /// assert_approx_eq!(FDRNum::from_rational_num(-5, 2), FDRNum::from(-2.5_f64), 2);
    /// assert_eq!(FDRNum::from_rational_num(17, 17), FDRNum::ONE);
    /// assert_approx_eq!(FDRNum::from_rational_num(5, 10), FDRNum::ONE / 2, 1);
    /// assert_eq!(FDRNum::from_rational_num(u64::MAX as i128 * 2, u64::MAX as u128), FDRNum::ONE * 2);
    /// ```
    pub fn from_rational_num(enumerator: i128, denominator: u128) -> Self {
        if enumerator == 0 {
            return Self::ZERO;
        }
        let result = UFDRNum::from_rational_num(enumerator.abs().cast_unsigned(), denominator);
        if enumerator.is_negative() {
            Self(-(result.0.cast_signed()))
        } else {
            result.into()
        }
    }

    /// Return the square root of our value.
    ///
    /// Example:
    /// ```
    /// use fixed_denominator_rational_numbers::{assert_approx_eq, FDRNum, ApproxEq, MinNonZero};
    ///
    /// let four: FDRNum = FDRNum::ONE * 4;
    /// let two: FDRNum = FDRNum::ONE * 2;
    /// assert_approx_eq!(four.sqrt(), two);
    /// let twenty_five: FDRNum = FDRNum::ONE * 25;
    /// let five: FDRNum = FDRNum::ONE * 5;
    /// assert_approx_eq!(twenty_five.sqrt(), five, 5);
    /// assert_eq!(FDRNum::from_rational_num(1, 4).sqrt(), FDRNum::from_rational_num(1, 2));
    /// ```
    pub fn sqrt(self) -> Self {
        assert!(!self.0.is_negative());
        let unsigned: UFDRNum = self.into();
        unsigned.sqrt().into()
    }
}

impl From<UFDRNum> for FDRNum {
    fn from(num: UFDRNum) -> Self {
        Self(num.0.cast_signed())
    }
}

// Conversions
impl<S: SignedInteger> From<S> for FDRNum {
    /// Convert a signed integer into FDRNum with a value equal to the signed integer.
    ///
    /// Example:
    /// ```
    /// use fixed_denominator_rational_numbers::FDRNum;
    ///
    /// assert_eq!(FDRNum::from(1_i32), FDRNum::ONE);
    /// assert_eq!(FDRNum::from(0_i64), FDRNum::ZERO);
    /// assert_eq!(FDRNum::from(5_i32), FDRNum::ONE * 5);
    /// ```
    fn from(signed: S) -> Self {
        Self(signed.into() * Self::DENOMINATOR)
    }
}

impl_to_from_float!(f64, i128, FDRNum);
impl_to_from_float!(f32, i128, FDRNum);

// Ops
// Mul
impl Mul for FDRNum {
    type Output = Self;

    /// Implement multiplication for UFDRNum type
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{assert_approx_eq, FDRNum, ApproxEq};
    ///
    /// assert_approx_eq!(FDRNum::from(0.5_f64) * FDRNum::from(2), FDRNum::ONE);
    /// assert_approx_eq!(FDRNum::from(0.5_f64) * FDRNum::from(-2), FDRNum::MINUS_ONE);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        let result = UFDRNum(self.0.abs().cast_unsigned()) * UFDRNum(rhs.0.abs().cast_unsigned());
        if self.is_negative() == rhs.is_negative() {
            Self(result.0.cast_signed())
        } else {
            Self(-result.0.cast_signed())
        }
    }
}

impl MulAssign for FDRNum {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<S: SignedInteger> Mul<S> for FDRNum {
    type Output = Self;

    /// Multiply an FDRnum by an unsigned integer
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::FDRNum;
    ///
    /// assert_eq!(FDRNum::ONE * 5, FDRNum::from(5_i32));
    /// assert_eq!(FDRNum::ONE * -5, FDRNum::from(-5_i32));
    /// ```
    fn mul(self, rhs: S) -> Self::Output {
        // N.B. Do this separately to avoid ambiguity with U::into::<UFDRNum>().
        let rhs: i128 = rhs.into();
        Self(self.0 * rhs)
    }
}

// Div
impl Div for FDRNum {
    type Output = Self;

    /// Implement division for FDRNum type
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{assert_approx_eq, FDRNum, ApproxEq, MinNonZero};
    ///
    /// assert_approx_eq!(FDRNum::from(5) / FDRNum::from(2), FDRNum::from(2.5_f64), 3_u8);
    /// ```
    fn div(self, rhs: Self) -> Self {
        let result = UFDRNum(self.0.abs().cast_unsigned()) / UFDRNum(rhs.0.abs().cast_unsigned());
        if self.is_negative() == rhs.is_negative() {
            Self(result.0.cast_signed())
        } else {
            Self(-result.0.cast_signed())
        }
    }
}

impl DivAssign for FDRNum {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<S: SignedInteger> Div<S> for FDRNum {
    type Output = Self;

    /// Multiply an UFDRnum by an unsigned integer
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{assert_approx_eq, FDRNum, ApproxEq, MinNonZero};
    ///
    /// assert_approx_eq!(FDRNum::ONE / 4, FDRNum::from(0.25_f64), 2_u8);
    /// ```
    fn div(self, rhs: S) -> Self::Output {
        // N.B. Do this separately to avoid ambiguity with U::into::<UFDRNum>().
        let rhs: i128 = rhs.into();
        Self(self.0 / rhs)
    }
}

// Add
impl Add for FDRNum {
    type Output = Self;

    /// Add one FDRNum from this number.
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{FDRNum, ApproxEq, MinNonZero, assert_approx_eq};
    ///
    /// assert_eq!(FDRNum::ONE * 2 + FDRNum::ONE, FDRNum::ONE * 3);
    /// assert_approx_eq!(FDRNum::from_rational_num(15, 2) + FDRNum::from_rational_num(1, 2), FDRNum::from(8), 1_u8);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for FDRNum {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// Sub
impl Sub for FDRNum {
    type Output = Self;

    /// Subtract one FDRNum from this number. (Will panic if rhs > self).
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{FDRNum, ApproxEq, MinNonZero};
    ///
    /// assert_eq!(FDRNum::ONE * 2 - FDRNum::ONE, FDRNum::ONE);
    /// assert_eq!(FDRNum::from(7.5_f64) - FDRNum::from(5.2_f64), FDRNum::from(2.3_f64));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for FDRNum {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// Rem
impl Rem for FDRNum {
    type Output = Self;

    /// Return the remainder after dividing self by rhs
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::FDRNum;
    ///
    /// assert_eq!(FDRNum::ONE % (FDRNum::ONE * 4), FDRNum::ONE);
    /// assert_eq!((FDRNum::ONE * 5) % (FDRNum::ONE * 4), FDRNum::ONE);
    /// ```
    fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }
}

impl RemAssign for FDRNum {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}
