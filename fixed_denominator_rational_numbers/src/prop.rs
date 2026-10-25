// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::debug::{AbsDiff, ApproxEq, PropDiff};
use crate::{
    AsProportion, FDRNum, FromAsProp, IntoAsProp, IntoFromRawU128, NumberConstants, UFDRNum,
    UnsignedInteger, impl_into_as_prop,
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Prop(pub(crate) u64);

impl AsProportion for Prop {
    const ZERO: Self = Self(0);
    const ALMOST_ZERO: Self = Self(1);
    const ONE: Self = Self(u64::MAX);
    const ALMOST_ONE: Self = Self(u64::MAX - 1);
    const ONE_QUARTER: Self = Self(u64::ONE_QUARTER);
    const HALF: Self = Self(u64::HALF);
    const THREE_QUARTERS: Self = Self(u64::THREE_QUARTERS);
}

impl Prop {
    /// Create from a fraction
    ///
    /// Examples:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{Prop, debug::ApproxEq, FromAsProp};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(Prop::from_fraction(1, 2), Prop::from_as_prop(0.5));
    /// assert_approx_eq!(Prop::from_fraction(3, 5), Prop::from_as_prop(0.6), 1000);
    /// ```
    pub fn from_fraction(enumerator: u64, denominator: u64) -> Self {
        debug_assert!(denominator >= enumerator);
        let ufdrn = UFDRNum::from_rational_num(enumerator as u128, denominator as u128);
        Self(ufdrn.0 as u64)
    }

    /// Return the square root of our value.
    ///
    /// Example:
    /// ```
    /// use fixed_denominator_rational_numbers::{Prop, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_approx_eq!(Prop::from_fraction(1, 4).sqrt(), Prop::from_fraction(1, 2));
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

    /// Return the complement of this proportion
    pub fn complement(self) -> Self {
        Self::ONE - self
    }
}

// Debug stuff
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

// Conversions
impl From<[u64; 2]> for Prop {
    fn from(fraction: [u64; 2]) -> Self {
        Self::from_fraction(fraction[0], fraction[1])
    }
}

impl FromAsProp<Prop> for Prop {
    fn from_as_prop(prop: Prop) -> Self {
        prop
    }
}
impl_into_as_prop!(Prop, Prop);

impl FromAsProp<f64> for Prop {
    fn from_as_prop(arg: f64) -> Self {
        debug_assert!((0.0..=1.0).contains(&arg));
        Self((arg * u64::MAX as f64) as u64)
    }
}
impl_into_as_prop!(Prop, f64);

impl FromAsProp<Prop> for f64 {
    fn from_as_prop(arg: Prop) -> Self {
        arg.0 as f64 / u64::MAX as f64
    }
}
impl_into_as_prop!(f64, Prop);

impl<U: AsProportion + IntoFromRawU128 + Debug> FromAsProp<Prop> for U {
    /// Converts a Prop to a U as a proportion of U::MAX
    ///
    /// Examples:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{Prop, IntoAsProp, FromAsProp};
    ///
    /// assert_eq!(u8::from_as_prop(Prop::from_as_prop(0.5)), 128);
    /// assert_eq!(u8::from_as_prop(Prop::from_as_prop(0.999)), u8::MAX);
    /// assert_eq!(u8::from_as_prop(Prop::from_as_prop(0.998)), u8::MAX - 1);
    /// assert_eq!(u8::MAX - 1, Prop::from_as_prop(0.998).into_as_prop());
    /// assert_eq!(23_u64, Prop::from_as_prop(23_u64).into_as_prop());
    /// ```
    fn from_as_prop(prop: Prop) -> Self {
        let dividend = prop.0 as u128 * U::ONE.into_u128();
        let divisor = u64::MAX as u128;
        let val = (dividend + divisor / 2) / divisor;
        U::from_u128(val)
    }
}

impl<U: AsProportion + IntoFromRawU128> IntoAsProp<U> for Prop {
    fn into_as_prop(self) -> U {
        U::from_as_prop(self)
    }
}

impl<U: AsProportion + IntoFromRawU128> FromAsProp<U> for Prop {
    /// Convert from an unsigned integer treated as a proportion.
    ///
    /// Examples:
    ///
    ///```
    /// use fixed_denominator_rational_numbers::{Prop, IntoAsProp, FromAsProp, AsProportion};
    ///
    /// assert_eq!(Prop::from_as_prop(u8::MAX), Prop::ONE);
    /// assert_eq!(Prop::from_as_prop(u8::MAX - 1), Prop::from_fraction(254, 255));
    ///```
    fn from_as_prop(unsigned: U) -> Self {
        let dividend: u128 = unsigned.into_u128() * Prop::ONE.0 as u128;
        let divisor: u128 = U::ONE.into_u128();
        let val = (dividend + divisor / 2) / divisor;
        Prop(val as u64)
    }
}

impl<U: AsProportion + IntoFromRawU128> IntoAsProp<Prop> for U {
    fn into_as_prop(self) -> Prop {
        Prop::from_as_prop(self)
    }
}

impl<U: AsProportion + IntoFromRawU128 + IntoAsProp<Prop>> FromAsProp<UFDRNum> for U {
    fn from_as_prop(num: UFDRNum) -> Self {
        debug_assert!(num >= UFDRNum::ZERO && num <= UFDRNum::ONE);
        Prop(num.0 as u64).into_as_prop()
    }
}

impl<U: AsProportion + IntoFromRawU128 + IntoAsProp<Prop>> IntoAsProp<U> for UFDRNum {
    fn into_as_prop(self) -> U {
        U::from_as_prop(self)
    }
}

impl<U: AsProportion + IntoFromRawU128 + IntoAsProp<Prop>> FromAsProp<U> for UFDRNum {
    fn from_as_prop(a: U) -> Self {
        Self(a.into_as_prop().0 as u128)
    }
}

impl<U: AsProportion + IntoFromRawU128 + IntoAsProp<Prop>> IntoAsProp<UFDRNum> for U {
    fn into_as_prop(self) -> UFDRNum {
        UFDRNum::from_as_prop(self)
    }
}

impl<U: AsProportion + IntoFromRawU128 + IntoAsProp<Prop>> FromAsProp<FDRNum> for U {
    fn from_as_prop(num: FDRNum) -> Self {
        debug_assert!(num >= FDRNum::ZERO && num <= FDRNum::ONE);
        Prop(num.0 as u64).into_as_prop()
    }
}

impl<U: AsProportion + IntoFromRawU128 + IntoAsProp<Prop>> IntoAsProp<U> for FDRNum {
    fn into_as_prop(self) -> U {
        U::from_as_prop(self)
    }
}

impl<U: AsProportion + IntoFromRawU128 + IntoAsProp<Prop>> FromAsProp<U> for FDRNum {
    fn from_as_prop(a: U) -> Self {
        Self(a.into_as_prop().0 as i128)
    }
}

impl<U: AsProportion + IntoFromRawU128 + IntoAsProp<Prop>> IntoAsProp<FDRNum> for U {
    fn into_as_prop(self) -> FDRNum {
        FDRNum::from_as_prop(self)
    }
}

#[macro_export]
macro_rules! impl_into_from_as_prop_number {
    ($number:ident, $core:ty, $proportion:ident) => {
        impl FromAsProp<$number> for $proportion {
            #[allow(unused_comparisons)]
            fn from_as_prop(arg: $number) -> Self {
                debug_assert!(arg.is_proportion());
                Self::from_as_prop(u64::from_as_prop(arg))
            }
        }
        impl_into_as_prop!($proportion, $number);

        impl FromAsProp<$proportion> for $number {
            fn from_as_prop(arg: $proportion) -> Self {
                Self::from_rational_num(u64::from_as_prop(arg) as $core, Self::DENOMINATOR)
            }
        }
        impl_into_as_prop!($number, $proportion);
    };
}

impl_into_from_as_prop_number!(UFDRNum, u128, Prop);
impl_into_from_as_prop_number!(FDRNum, i128, Prop);

pub trait IntoProp: Sized + Copy {
    fn into_prop(self) -> Prop;
}

impl IntoProp for UFDRNum {
    fn into_prop(self) -> Prop {
        Prop::from_as_prop(self)
    }
}

impl IntoProp for FDRNum {
    fn into_prop(self) -> Prop {
        Prop::from_as_prop(self)
    }
}

impl IntoProp for Prop {
    fn into_prop(self) -> Prop {
        Prop::from_as_prop(self)
    }
}

impl IntoProp for f64 {
    fn into_prop(self) -> Prop {
        Prop::from_as_prop(self)
    }
}

impl<U: AsProportion + UnsignedInteger> IntoProp for U {
    fn into_prop(self) -> Prop {
        Prop::from_as_prop(self)
    }
}

pub trait FromProp: Sized + Copy + FromAsProp<Prop> {
    fn from_prop(prop: Prop) -> Self {
        Self::from_as_prop(prop)
    }
}

impl FromProp for UFDRNum {}
impl FromProp for FDRNum {}
impl FromProp for Prop {}
impl FromProp for f64 {}
impl<U: UnsignedInteger + FromAsProp<Prop>> FromProp for U {}

// Ops
impl Mul for Prop {
    type Output = Self;

    /// Multiply by another Prop.
    ///
    /// Examples:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{Prop, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(Prop::from_fraction(1, 2) * Prop::from_fraction(1, 2), Prop::from_fraction(1, 4));
    /// assert_eq!(Prop::from_fraction(1, 2) * Prop::from_fraction(1, 3), Prop::from_fraction(1, 6));
    /// ```
    fn mul(self, rhs: Self) -> Self {
        Self(((self.0 as u128 * rhs.0 as u128) / u64::MAX as u128) as u64)
    }
}

impl MulAssign for Prop {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<U: UnsignedInteger> Mul<U> for Prop {
    type Output = UFDRNum;

    /// Multiply by an unsigned integer yielding an unsigned fixed denominator rational number.
    ///
    /// Examples:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{self::*, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_approx_eq!(Prop::from_fraction(1, 2) * 2u32, UFDRNum::ONE);
    /// assert_approx_eq!(Prop::from_fraction(3, 5) * 2u8, UFDRNum::from_rational_num(6, 5));
    /// ```
    fn mul(self, rhs: U) -> UFDRNum {
        UFDRNum(self.0 as u128 * rhs.into())
    }
}

// Div
impl Div for Prop {
    type Output = UFDRNum;

    /// Divide by an Prop yielding an unsigned fixed denominator rational number.
    ///
    /// Examples:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{self::*, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_approx_eq!(Prop::from_fraction(1, 2) / Prop::from_fraction(1,2) , UFDRNum::ONE);
    /// assert_approx_eq!(Prop::from_fraction(3, 5) / Prop::from_fraction(2,5), UFDRNum::from_rational_num(3, 2));
    /// ```
    fn div(self, rhs: Self) -> UFDRNum {
        assert_ne!(rhs, Self::ZERO);
        if rhs == Self::ONE {
            UFDRNum::ONE
        } else {
            UFDRNum(self.0 as u128) / UFDRNum(rhs.0 as u128)
        }
    }
}

impl<U: UnsignedInteger> Div<U> for Prop {
    type Output = Self;

    /// Divide by an unsigned integer yielding a Prop.
    ///
    /// Examples:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::*;
    ///
    /// assert_eq!(Prop::from_fraction(1, 2) / 2u32, Prop::from_fraction(1, 4));
    /// assert_eq!(Prop::from_fraction(3, 5) / 2u8, Prop::from_fraction(3, 10));
    /// ```
    fn div(self, rhs: U) -> Self {
        Self((self.0 as u128 / rhs.into_u128()) as u64)
    }
}

impl<U: UnsignedInteger> DivAssign<U> for Prop {
    fn div_assign(&mut self, rhs: U) {
        *self = *self / rhs;
    }
}

// Add
impl Add for Prop {
    type Output = UFDRNum;

    /// Add a Prop yielding an UFDRNum.
    ///
    /// Examples:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::{self::*, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(Prop::from_fraction(1, 2) + Prop::from_fraction(1, 4), UFDRNum::from(0.75));
    /// assert_approx_eq!(Prop::from_fraction(3, 5) + Prop::from_fraction(6, 10), UFDRNum::from(1.2), 1000);
    /// ```
    fn add(self, rhs: Self) -> UFDRNum {
        UFDRNum(self.0 as u128 + rhs.0 as u128)
    }
}

// Sub
impl Sub for Prop {
    type Output = Self;

    /// Subtract a Prop yielding a Prop.
    ///
    /// Examples:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::*;
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(Prop::from_fraction(1, 2) - Prop::from_fraction(1, 4), Prop::from_fraction(1, 4));
    /// assert_eq!(Prop::from_fraction(3, 5) - Prop::from_fraction(6, 10), Prop::ZERO);
    /// ```
    fn sub(self, rhs: Self) -> Self {
        assert!(self.0 >= rhs.0);
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Prop {
    fn sub_assign(&mut self, rhs: Self) {
        debug_assert!(self.0 >= rhs.0);
        self.0 -= rhs.0;
    }
}

// Ops with UFDRNum
impl Add<Prop> for UFDRNum {
    type Output = Self;

    fn add(self, rhs: Prop) -> UFDRNum {
        UFDRNum(self.0 + rhs.0 as u128)
    }
}

impl AddAssign<Prop> for UFDRNum {
    fn add_assign(&mut self, rhs: Prop) {
        self.0 += rhs.0 as u128;
    }
}

impl Sub<Prop> for UFDRNum {
    type Output = Self;

    fn sub(self, rhs: Prop) -> Self {
        debug_assert!(self.0 >= rhs.0 as u128);
        Self(self.0 - rhs.0 as u128)
    }
}

impl SubAssign<Prop> for UFDRNum {
    fn sub_assign(&mut self, rhs: Prop) {
        debug_assert!(self.0 >= rhs.0 as u128);
        self.0 -= rhs.0 as u128;
    }
}

impl Div<Prop> for UFDRNum {
    type Output = Self;

    fn div(self, rhs: Prop) -> Self {
        assert_ne!(rhs.0, 0);
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

impl DivAssign<Prop> for UFDRNum {
    fn div_assign(&mut self, rhs: Prop) {
        *self = self.div(rhs);
    }
}

impl Mul<Prop> for UFDRNum {
    type Output = Self;

    /// Multiply by a Prop
    ///
    /// Examples:
    ///
    /// ```
    /// use fixed_denominator_rational_numbers::*;
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(UFDRNum::from(2.5) * Prop::ONE, UFDRNum::from(2.5));
    /// assert_eq!(UFDRNum::ONE * Prop::ZERO, UFDRNum::ZERO);
    /// assert_eq!(UFDRNum::ONE * Prop::from_as_prop(0.5), UFDRNum::from(0.5));
    /// ```
    fn mul(self, rhs: Prop) -> Self {
        match rhs {
            Prop::ZERO => Self::ZERO,
            Prop::ONE => self,
            _ => self.mul(Self(rhs.0 as u128)),
        }
    }
}

impl MulAssign<Prop> for UFDRNum {
    fn mul_assign(&mut self, rhs: Prop) {
        *self = self.mul(rhs);
    }
}
