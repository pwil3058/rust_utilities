// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Prop(pub(crate) u64);

impl Prop {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(u64::MAX);

    pub(crate) const ALMOST_ONE: Self = Self(u64::MAX - 1);
    pub(crate) const HALF: Self = Self(u64::MAX / 2);

    pub fn from_fraction(enumerator: u64, denominator: u64) -> Self {
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
