use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::rcp::{ApproxF32, Rcp, RndF32, RndF64, Rounding},
    },
};

#[test]
fn parses_rcp_approx_f32_without_ftz() {
    assert_eq!(
        parse::<Rcp>("rcp.approx.f32 %f0, %f1;"),
        Rcp::ApproxF32(ApproxF32 {
            flush_to_zero: false,
            destination: RegisterOperand::Single("%f0".into()),
            source: RegisterOperand::Single("%f1".into()),
        })
    );
}

#[test]
fn parses_rcp_approx_f32_with_ftz() {
    assert_eq!(
        parse::<Rcp>("rcp.approx.ftz.f32 %f2, %f3;"),
        Rcp::ApproxF32(ApproxF32 {
            flush_to_zero: true,
            destination: RegisterOperand::Single("%f2".into()),
            source: RegisterOperand::Single("%f3".into()),
        })
    );
}

#[test]
fn parses_rcp_rnd_f32_without_ftz() {
    assert_eq!(
        parse::<Rcp>("rcp.rnd.rn.f32 %f4, %f5;"),
        Rcp::RndF32(RndF32 {
            rounding: Rounding::Rn,
            flush_to_zero: false,
            destination: RegisterOperand::Single("%f4".into()),
            source: RegisterOperand::Single("%f5".into()),
        })
    );
}

#[test]
fn parses_rcp_rnd_f32_with_ftz() {
    assert_eq!(
        parse::<Rcp>("rcp.rnd.rm.ftz.f32 %f6, %f7;"),
        Rcp::RndF32(RndF32 {
            rounding: Rounding::Rm,
            flush_to_zero: true,
            destination: RegisterOperand::Single("%f6".into()),
            source: RegisterOperand::Single("%f7".into()),
        })
    );
}

#[test]
fn parses_rcp_rnd_f64() {
    assert_eq!(
        parse::<Rcp>("rcp.rnd.rp.f64 %fd8, %fd9;"),
        Rcp::RndF64(RndF64 {
            rounding: Rounding::Rp,
            destination: RegisterOperand::Single("%fd8".into()),
            source: RegisterOperand::Single("%fd9".into()),
        })
    );
}

#[test]
fn rejects_rcp_approx_with_f64() {
    let err = parse_result::<Rcp>("rcp.approx.f64 %f0, %f1;")
        .expect_err("rcp.approx should only accept .f32");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_rcp_rnd_f64_with_ftz() {
    let err = parse_result::<Rcp>("rcp.rnd.rz.ftz.f64 %fd0, %fd1;")
        .expect_err("rcp.rnd with .f64 must reject .ftz modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_rcp_rnd_with_invalid_rounding_mode() {
    let err = parse_result::<Rcp>("rcp.rnd.rq.f32 %f0, %f1;")
        .expect_err("invalid rounding mode should be rejected");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
