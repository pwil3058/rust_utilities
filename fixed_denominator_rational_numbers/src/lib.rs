// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

pub mod debug;
pub mod fdrn;
pub mod prop;
pub mod ufdrn;
pub mod useful_traits;

pub use fdrn::*;
pub use prop::*;
pub use ufdrn::*;
pub use useful_traits::*;

pub trait NumberConstants {
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const THREE: Self;

    const SQRT_2: Self;
    const SQRT_3: Self;
}

pub trait TrigConstants {
    const COS_45: Self;
}

#[cfg(test)]
mod tests;
