use crate::util::{parse, parse_result};
use ptx_parser::r#type::common::RegisterOperand;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::instruction::fma::{Fma, Rounding},
};

#[test]
fn parses_fma_f32_with_all_modifiers() {
    assert_eq!(
        parse::<Fma>("fma.rn.ftz.sat.f32 %f0, %f1, %f2, %f3;"),
        Fma::F32 {
            rounding: Rounding::Rn,
            flush_to_zero: true,
            saturate: true,
            destination: RegisterOperand::Single("%f0".into()),
            multiplicand_a: RegisterOperand::Single("%f1".into()),
            multiplicand_b: RegisterOperand::Single("%f2".into()),
            addend: RegisterOperand::Single("%f3".into()),
        }
    );
}

#[test]
fn parses_fma_f64_without_optional_modifiers() {
    assert_eq!(
        parse::<Fma>("fma.rp.f64 %fd1, %fd2, %fd3, %fd4;"),
        Fma::F64 {
            rounding: Rounding::Rp,
            destination: RegisterOperand::Single("%fd1".into()),
            multiplicand_a: RegisterOperand::Single("%fd2".into()),
            multiplicand_b: RegisterOperand::Single("%fd3".into()),
            addend: RegisterOperand::Single("%fd4".into()),
        }
    );
}

#[test]
fn rejects_fma_with_saturate_for_f32x2() {
    let err = parse_result::<Fma>("fma.rz.sat.f32x2 %f0, %f1, %f2, %f3;")
        .expect_err("parser should reject .sat modifier for .f32x2");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_fma_with_ftz_for_f64() {
    let err = parse_result::<Fma>("fma.rm.ftz.f64 %fd0, %fd1, %fd2, %fd3;")
        .expect_err("parser should reject .ftz modifier for .f64");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
