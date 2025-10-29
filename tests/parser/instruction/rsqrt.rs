use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::rsqrt::{ApproxF32, ApproxF64, Rsqrt},
    },
};

#[test]
fn parses_rsqrt_approx_f32_without_ftz() {
    assert_eq!(
        parse::<Rsqrt>("rsqrt.approx.f32 %f0, %f1;"),
        Rsqrt::ApproxF32(ApproxF32 {
            flush_to_zero: false,
            destination: RegisterOperand::Single("%f0".into()),
            source: RegisterOperand::Single("%f1".into()),
        })
    );
}

#[test]
fn parses_rsqrt_approx_f32_with_ftz() {
    assert_eq!(
        parse::<Rsqrt>("rsqrt.approx.ftz.f32 %f2, %f3;"),
        Rsqrt::ApproxF32(ApproxF32 {
            flush_to_zero: true,
            destination: RegisterOperand::Single("%f2".into()),
            source: RegisterOperand::Single("%f3".into()),
        })
    );
}

#[test]
fn parses_rsqrt_approx_f64() {
    assert_eq!(
        parse::<Rsqrt>("rsqrt.approx.f64 %fd4, %fd5;"),
        Rsqrt::ApproxF64(ApproxF64 {
            destination: RegisterOperand::Single("%fd4".into()),
            source: RegisterOperand::Single("%fd5".into()),
        })
    );
}

#[test]
fn rejects_rsqrt_with_invalid_modifier() {
    let err = parse_result::<Rsqrt>("rsqrt.rnd.f32 %f0, %f1;")
        .expect_err("rsqrt should only accept .approx modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_rsqrt_f64_with_ftz() {
    let err = parse_result::<Rsqrt>("rsqrt.approx.ftz.f64 %fd0, %fd1;")
        .expect_err("rsqrt .f64 must not accept .ftz");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
