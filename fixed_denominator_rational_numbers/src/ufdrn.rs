// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Mul;

use serde::{Deserialize, Serialize};

use crate::{Float, Unsigned, UnsignedPrimitive, impl_to_from_float};

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

impl Unsigned for UFDRNum {}

impl UFDRNum {
    pub const DENOMINATOR: u128 = u64::MAX as u128;
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(Self::DENOMINATOR);
}

// Conversions
/// Convert an unsigned integer into UFDRNum with a value equal to the unsigned integer.
///
/// Example:
/// ```
/// use fixed_denominator_rational_numbers::UFDRNum;
///
/// assert_eq!(UFDRNum::from(1_u32), UFDRNum::ONE);
/// assert_eq!(UFDRNum::from(0_u32), UFDRNum::ZERO);
/// assert_eq!(UFDRNum::from(5_u32), UFDRNum::from(5_u128));
/// ```
impl<U: UnsignedPrimitive> From<U> for UFDRNum {
    fn from(unsigned: U) -> Self {
        Self(unsigned.into() * Self::DENOMINATOR)
    }
}

impl_to_from_float!(f64, u128, UFDRNum);
impl_to_from_float!(f32, u128, UFDRNum);

// Ops
/// Multiply an UFDRnum by an unsigned integer
///
/// Example:
///
/// ```
/// use fixed_denominator_rational_numbers::UFDRNum;
///
/// assert_eq!(UFDRNum::ONE * 5_u32, UFDRNum::from(5_u32));
/// ```
impl<U: UnsignedPrimitive> Mul<U> for UFDRNum {
    type Output = Self;

    fn mul(self, rhs: U) -> Self::Output {
        // N.B. Do this separately to avoid ambiguity with U::into::<UFDRNum>().
        let rhs: u128 = rhs.into();
        Self(self.0 * rhs)
    }
}
