// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

#[macro_export]
macro_rules! impl_to_from_float {
    ($float:ty, $core:ty, $number:ty) => {
        /// Convert a floating point number into UFDRNum (rounding as required).
        ///
        /// Example:
        /// ```
        /// use fixed_denominator_rational_numbers::UFDRNum;
        ///
        /// assert_eq!(UFDRNum::from(1.0), UFDRNum::ONE);
        /// assert_eq!(UFDRNum::from(0.0), UFDRNum::ZERO);
        /// assert_eq!(UFDRNum::from(5.0), UFDRNum::ONE * 5_u32);
        /// ```
        impl From<$float> for $number {
            fn from(arg: $float) -> Self {
                Self((arg * u64::MAX as $float) as $core)
            }
        }

        /// Convert an UFDRNum into a floating point number.
        ///
        /// Example:
        /// ```
        /// use fixed_denominator_rational_numbers::UFDRNum;
        ///
        /// assert_eq!(1.0, UFDRNum::from(1.0_f32).into());
        /// assert_eq!(0.0, UFDRNum::from(0.0_f64).into());
        /// assert_eq!(5.0, UFDRNum::from(5.0_f64).into());
        /// ```
        impl From<$number> for $float {
            fn from(arg: $number) -> Self {
                arg.0 as $float / u64::MAX as $float
            }
        }
    };
}
