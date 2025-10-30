use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::sqrt::{Rounding, Sqrt},
    },
};

#[test]
fn parses_sqrt_approx_f32_without_ftz() {
    assert_eq!(
        parse::<Sqrt>("sqrt.approx.f32 %f0, %f1;"),
        Sqrt::ApproxF32 {
            flush_to_zero: false,
            destination: RegisterOperand::Single("%f0".into()),
            source: RegisterOperand::Single("%f1".into()),
        }
    );
    assert_roundtrip::<Sqrt>("sqrt.approx.f32 %f0, %f1;");
}

#[test]
fn parses_sqrt_approx_f32_with_ftz() {
    assert_eq!(
        parse::<Sqrt>("sqrt.approx.ftz.f32 %f2, %f3;"),
        Sqrt::ApproxF32 {
            flush_to_zero: true,
            destination: RegisterOperand::Single("%f2".into()),
            source: RegisterOperand::Single("%f3".into()),
        }
    );
    assert_roundtrip::<Sqrt>("sqrt.approx.ftz.f32 %f2, %f3;");
}

#[test]
fn parses_sqrt_rnd_f32_with_ftz() {
    assert_eq!(
        parse::<Sqrt>("sqrt.rnd.rn.ftz.f32 %f4, %f5;"),
        Sqrt::RndF32 {
            rounding: Rounding::Rn,
            flush_to_zero: true,
            destination: RegisterOperand::Single("%f4".into()),
            source: RegisterOperand::Single("%f5".into()),
        }
    );
    assert_roundtrip::<Sqrt>("sqrt.rnd.rn.ftz.f32 %f4, %f5;");
}

#[test]
fn parses_sqrt_rnd_f64_without_ftz() {
    assert_eq!(
        parse::<Sqrt>("sqrt.rnd.rz.f64 %fd6, %fd7;"),
        Sqrt::RndF64 {
            rounding: Rounding::Rz,
            destination: RegisterOperand::Single("%fd6".into()),
            source: RegisterOperand::Single("%fd7".into()),
        }
    );
    assert_roundtrip::<Sqrt>("sqrt.rnd.rz.f64 %fd6, %fd7;");
}

#[test]
fn rejects_sqrt_approx_with_f64() {
    let err = parse_result::<Sqrt>("sqrt.approx.f64 %f0, %f1;")
        .expect_err("approx sqrt with .f64 should be rejected");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_sqrt_rnd_f64_with_ftz() {
    let err = parse_result::<Sqrt>("sqrt.rnd.rp.ftz.f64 %fd0, %fd1;")
        .expect_err("sqrt .rnd .f64 must not accept .ftz");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
