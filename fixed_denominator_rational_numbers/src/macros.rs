// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

#[macro_export]
macro_rules! impl_to_from_float {
    ($float:ty, $core:ty, $number:ty) => {
        impl From<$float> for $number {
            fn from(arg: $float) -> Self {
                Self((arg * u64::MAX as $float) as $core)
            }
        }

        impl From<$number> for $float {
            fn from(arg: $number) -> Self {
                arg.0 as $float / u64::MAX as $float
            }
        }
    };
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val).approx_eq(&*right_val, None) {
                    panic!(
                        "assertion failed: `left.approx_eq(right)\n` \
                         left:  `{:?}`\n right: `{:?}`.",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr,) => {{ $crate::assert_approx_eq!($left, $right) }};
    ($left:expr, $right:expr, $max_diff:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val).approx_eq(&*right_val, Some($max_diff)) {
                    panic!(
                        "assertion failed: `left.approx_eq(right)\n` \
                         left:  `{:?}`\n right: `{:?}`.",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $max_diff:expr,) => {{ $crate::assert_approx_eq!($left, $right, $max_diff) }};
}

#[macro_export]
macro_rules! debug_assert_approx_eq {
    ($left:expr, $right:expr) => {{
        if cfg!(debug_assertions) {
            $crate::assert_approx_eq!($left, $right)
        }
    }};
    ($left:expr, $right:expr,) => {{
        if cfg!(debug_assertions) {
            $crate::assert_approx_eq!($left, $right)
        }
    }};
    ($left:expr, $right:expr, $max_diff:expr) => {{
        if cfg!(debug_assertions) {
            $crate::assert_approx_eq!($left, $right, $max_diff: expr)
        }
    }};
    ($left:expr, $right:expr, $max_diff:expr,) => {{
        if cfg!(debug_assertions) {
            $crate::assert_approx_eq!($left, $right, $max_diff)
        }
    }};
}

#[macro_export]
macro_rules! assert_approx_ne {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (*left_val).approx_eq(&*right_val, None) {
                    panic!(
                        "assertion failed: `left.approx_ne(right)\n` \
                         left:  `{:?}`\n right: `{:?}`.",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr,) => {{ $crate::assert_approx_ne!($left, $right) }};
    ($left:expr, $right:expr, $max_diff:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (*left_val).approx_eq(&*right_val, Some($max_diff)) {
                    panic!(
                        "assertion failed: `left.approx_ne(right)\n` \
                         left:  `{:?}`\n right: `{:?}`.",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $max_diff:expr,) => {{ $crate::assert_approx_ne!($left, $right, $max_diff) }};
}
