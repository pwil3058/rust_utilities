// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use approximately_equal::assert_approx_eq;

use crate::{
    AsProportion, FDRNum, FromAsProp, IntoAsProp, IntoProp, NumberConstants, Prop, UFDRNum,
    debug::ApproxEq,
};

#[test]
fn sqrt_2() {
    assert_eq!(f64::from(FDRNum::SQRT_2), std::f64::consts::SQRT_2);
    assert_eq!(f64::from(UFDRNum::SQRT_2), std::f64::consts::SQRT_2);
    assert_eq!(UFDRNum::from(2u8).sqrt(), UFDRNum::SQRT_2);
    assert_eq!(UFDRNum::from(3u8).sqrt(), UFDRNum::SQRT_3);
}

#[test]
fn ufdrn_mul() {
    for [lhs, rhs] in &[
        [1.1_f64, 3.0],
        [0.0, 0.3],
        [1.024, 0.5],
        [0.18, 0.5],
        [3000.0, 0.02],
        [1.0, 1.001],
        [25.0, 5.0],
    ] {
        let expected = UFDRNum::from(lhs * rhs);
        let result = UFDRNum::from(*lhs) * UFDRNum::from(*rhs);
        assert_approx_eq!(result, expected, 1000);
    }
}

#[test]
fn ufdrn_div() {
    for [lhs, rhs] in &[
        [1.1_f64, 3.0],
        [0.0, 0.3],
        [1.024, 0.5],
        [0.18, 0.5],
        [3000.0, 0.02],
        [1.0, 1.001],
        [25.0, 5.0],
    ] {
        let expected = UFDRNum::from(lhs / rhs);
        let result = UFDRNum::from(*lhs) / UFDRNum::from(*rhs);
        assert_approx_eq!(result, expected, 1000);
        let go_back = result * UFDRNum::from(*rhs);
        assert_approx_eq!(go_back, UFDRNum::from(*lhs));
    }
}

#[test]
fn ufdrn_add() {
    for [a, b] in &[
        [0.0f64, 0.3],
        [0.024, 0.5],
        [0.18, 0.5],
        [0.5, 0.8],
        [1.5, 0.6],
    ] {
        let expected = UFDRNum::from(a + b);
        let result = UFDRNum::from(*a) + UFDRNum::from(*b);
        assert_approx_eq!(result, expected, 10000);
        assert_approx_eq!(f64::from(result), &(a + b));
    }
}

#[test]
fn ufdrn_sub() {
    for [a, b] in &[
        [0.5f64, 0.3],
        [0.524, 0.5],
        [0.18, 0.15],
        [0.5, 0.08],
        [1.2, 1.1],
    ] {
        let expected = UFDRNum::from(a - b);
        let result = UFDRNum::from(*a) - UFDRNum::from(*b);
        assert_approx_eq!(result, expected, 1000);
        assert_approx_eq!(f64::from(result), &(a - b));
    }
}

#[test]
fn ufdrn_div_u8() {
    for (a, b) in &[(0.9_f64, 3u8), (0.6, 2), (0.3, 2)] {
        let expected = UFDRNum::from(*a / *b as f64);
        let result = UFDRNum::from(*a) / *b;
        assert_approx_eq!(result, expected, 1000);
        assert_approx_eq!(f64::from(result), &(a / *b as f64),);
    }
}

#[test]
fn composition() {
    for a in &[0.001_f64] {
        let prop = Prop::from_as_prop(*a);
        for b in &[0.001_f64] {
            let second = Prop::from_as_prop(*b);
            let left = UFDRNum::ONE * 3u8 - (UFDRNum::ONE * 2u8 - second) * prop;
            let right = UFDRNum::ONE * 3u8 + second * prop - prop * 2u8;
            assert!(left > right);
            assert_approx_eq!(left, right);
        }
    }
}

#[test]
fn prop_mul() {
    for [a, b] in &[[0.0f64, 0.3], [0.024, 0.5], [0.8, 0.5]] {
        let expected = Prop::from_as_prop(a * b);
        let result = Prop::from_as_prop(*a) * Prop::from_as_prop(*b);
        assert_approx_eq!(result, expected);
        assert_approx_eq!(f64::from_as_prop(result), &(a * b));
    }
}

#[test]
fn prop_mul_u8() {
    for (a, b) in &[(0.9_f64, 3_u8), (0.6, 2), (0.3, 2)] {
        let expected = UFDRNum::from(*a * *b as f64);
        let result = Prop::from_as_prop(*a) * *b;
        assert_approx_eq!(result, expected, 10000);
        assert_approx_eq!(f64::from(result), &(a * *b as f64),);
    }
}

#[test]
fn prop_div() {
    for [a, b] in &[[0.0f64, 0.3], [0.024, 0.5], [0.18, 0.5], [0.004, 0.6]] {
        let expected = UFDRNum::from(a / b);
        let result = Prop::from_as_prop(*a) / Prop::from_as_prop(*b);
        assert_approx_eq!(result, expected);
        assert_approx_eq!(f64::from(result), (a / b));
        let go_back: Prop = (result * UFDRNum::from(*b)).into_as_prop();
        assert_approx_eq!(go_back, Prop::from_as_prop(*a));
    }
}

#[test]
fn prop_add() {
    for [a, b] in &[
        [0.0f64, 0.3],
        [0.024, 0.5],
        [0.18, 0.5],
        [0.5, 0.8],
        [0.9, 0.04],
    ] {
        let expected = UFDRNum::from(a + b);
        let result = Prop::from_as_prop(*a) + Prop::from_as_prop(*b);
        assert_approx_eq!(result, expected, 10000);
        assert_approx_eq!(f64::from(result), &(a + b));
    }
}

#[test]
fn prop_sub() {
    for [a, b] in &[[0.5f64, 0.3], [0.524, 0.5], [0.18, 0.15], [0.5, 0.08]] {
        let expected = Prop::from_as_prop(a - b);
        let result = Prop::from_as_prop(*a) - Prop::from_as_prop(*b);
        assert_approx_eq!(result, expected, 2000);
        assert_approx_eq!(f64::from_as_prop(result), &(a - b));
    }
}

#[test]
fn from_fraction() {
    assert_eq!(Prop::from([1, 2]), Prop::from_as_prop(0.5));
}

#[test]
fn from_unsigned() {
    assert_eq!(Prop::from_as_prop(u32::MAX), Prop::ONE);
    assert_eq!(Prop::from_as_prop(u8::MAX / 2), Prop::from([127, 255]));
}

#[test]
fn from_ufdrn_to_prop() {
    assert_eq!(
        Prop::from_fraction(1, 2),
        UFDRNum::from_rational_num(1, 2).into_as_prop()
    );
    assert_eq!(
        Prop::from_fraction(1, 5),
        UFDRNum::from_rational_num(1, 5).into_as_prop()
    );
}

#[test]
fn from_prop_to_ufdrn() {
    assert_eq!(
        UFDRNum::from_rational_num(1, 2),
        Prop::from_fraction(1, 2).into_as_prop()
    );
    assert_eq!(
        UFDRNum::from_rational_num(2, 5),
        Prop::from_fraction(2, 5).into_as_prop()
    );
    assert_eq!(UFDRNum(1), UFDRNum::from_as_prop(1u64));
}

#[test]
fn from_fdrn_to_prop() {
    assert_eq!(
        Prop::from_fraction(1, 2),
        FDRNum::from_rational_num(1, 2).into_as_prop()
    );
    assert_eq!(
        Prop::from_fraction(1, 5),
        FDRNum::from_rational_num(1, 5).into_as_prop()
    );
}

#[test]
fn from_prop_to_fdrn() {
    assert_eq!(
        FDRNum::from_rational_num(1, 2),
        Prop::from_fraction(1, 2).into_as_prop()
    );
    assert_eq!(
        FDRNum::from_rational_num(2, 5),
        Prop::from_fraction(2, 5).into_as_prop()
    );
}

#[test]
fn into_prop() {
    assert_eq!(
        UFDRNum::from_rational_num(1, 4).into_prop(),
        Prop::from_fraction(1, 4)
    );
    assert_eq!(1u64.into_prop(), Prop(1));
}
