// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{
    fmt::{self, Debug, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

use serde::{Deserialize, Serialize};

use crate::{
    FromAsProp, Prop, Signed, SignedInteger, UFDRNum,
    debug::{AbsDiff, ApproxEq, PropDiff},
};

/// Fixed denominator rational number as an approximate representation of real numbers
#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct FDRNum(pub(crate) i128);

impl Signed for FDRNum {}

impl FDRNum {
    pub const DENOMINATOR: i128 = u64::MAX as i128;
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(Self::DENOMINATOR);
    pub const MINUS_ONE: Self = Self(-Self::DENOMINATOR);
    pub const SQRT_2: Self = FDRNum(0x16A09E667F3BCC906);

    pub fn is_negative(self) -> bool {
        self.0.is_negative()
    }

    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    pub fn is_proportion(self) -> bool {
        self >= Self::ZERO && self <= Self::ONE
    }

    /// Generate an UFDRNum from a rational number
    ///
    /// Example:
    /// ```
    /// use fixed_denominator_rational_numbers::{FDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(FDRNum::from_rational_num(4, 2), FDRNum::ONE * 2);
    /// assert_approx_eq!(FDRNum::from_rational_num(-5, 2), FDRNum::from(-2.5_f64), 2);
    /// assert_eq!(FDRNum::from_rational_num(17, 17), FDRNum::ONE);
    /// assert_approx_eq!(FDRNum::from_rational_num(5, 10), FDRNum::ONE / 2, 2);
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
    /// use fixed_denominator_rational_numbers::{FDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
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

impl AbsDiff for FDRNum {}

impl PropDiff for FDRNum {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        let denom = self.abs().max(other.abs());
        if denom == FDRNum::ZERO {
            Some(Prop::ZERO)
        } else {
            Some(Prop::from_as_prop(self.abs_diff(other) / denom))
        }
    }
}

impl ApproxEq for FDRNum {}

// Conversions
impl From<UFDRNum> for FDRNum {
    /// Convert from UFDRNum
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{FDRNum, UFDRNum};
    ///
    /// assert_eq!(FDRNum::ONE, UFDRNum::ONE.into());
    /// assert_eq!(FDRNum::ONE / 2, (UFDRNum::ONE / 2u8).into());
    /// assert_eq!(FDRNum::ONE * 5, (UFDRNum::ONE * 5u8).into());
    /// ```
    fn from(num: UFDRNum) -> Self {
        Self(num.0.cast_signed())
    }
}

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

impl From<f64> for FDRNum {
    /// Convert from f64
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{Float,FDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_approx_eq!(FDRNum::from(1.0), FDRNum::ONE);
    /// assert_approx_eq!(FDRNum::from(0.25), FDRNum::from_rational_num(1, 4));
    /// assert_approx_eq!(FDRNum::from(-0.25), FDRNum::MINUS_ONE / 4, 4);
    /// ```
    fn from(arg: f64) -> Self {
        Self((arg * Self::DENOMINATOR as f64) as i128)
    }
}

impl From<UFDRNum> for f64 {
    fn from(arg: UFDRNum) -> Self {
        arg.0 as f64 / u64::MAX as f64
    }
}

// Ops
// Add
impl Add for FDRNum {
    type Output = Self;

    /// Add one FDRNum from this number.
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{FDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(FDRNum::ONE * 2 + FDRNum::ONE, FDRNum::ONE * 3);
    /// assert_approx_eq!(FDRNum::from_rational_num(15, 2) + FDRNum::from_rational_num(1, 2), FDRNum::from(8), 1);
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
    /// use fixed_denominator_rational_numbers::{FDRNum};
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

// Mul
impl Mul for FDRNum {
    type Output = Self;

    /// Implement multiplication for UFDRNum type
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{FDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
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

impl<S: SignedInteger> MulAssign<S> for FDRNum {
    fn mul_assign(&mut self, rhs: S) {
        *self = *self * rhs;
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
    /// use fixed_denominator_rational_numbers::{FDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_approx_eq!(FDRNum::from(5) / FDRNum::from(2), FDRNum::from(2.5_f64), 3);
    /// assert_approx_eq!(FDRNum::from(-5) / FDRNum::from(2), FDRNum::from(-2.5_f64), 3);
    /// assert_approx_eq!(FDRNum::from(-5) / FDRNum::from(-2), FDRNum::from(2.5_f64), 3);
    /// assert_approx_eq!(FDRNum::from(5) / FDRNum::from(-2), FDRNum::from(-2.5_f64), 3);
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

    /// Divide an UFDRnum by an unsigned integer
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{FDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_approx_eq!(FDRNum::ONE / 4, FDRNum::from(0.25), 4);
    /// ```
    fn div(self, rhs: S) -> Self::Output {
        // N.B. Do this separately to avoid ambiguity with U::into::<UFDRNum>().
        let rhs: i128 = rhs.into();
        Self(self.0 / rhs)
    }
}

impl<S: SignedInteger> DivAssign<S> for FDRNum {
    fn div_assign(&mut self, rhs: S) {
        *self = *self / rhs;
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

impl<S: SignedInteger> Rem<S> for FDRNum {
    type Output = Self;

    /// Remainder of an FDRnum after division by an signed integer
    ///
    /// Example:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{FDRNum, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_approx_eq!(FDRNum::ONE / 4, FDRNum::from(0.25), 4);
    /// ```
    fn rem(self, rhs: S) -> Self::Output {
        // N.B. Do this separately to avoid ambiguity with U::into::<UFDRNum>().
        let rhs: i128 = rhs.into();
        Self(self.0 % rhs)
    }
}

impl<S: SignedInteger> RemAssign<S> for FDRNum {
    fn rem_assign(&mut self, rhs: S) {
        *self = *self % rhs;
    }
}
