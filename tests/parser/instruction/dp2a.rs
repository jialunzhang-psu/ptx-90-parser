use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::dp2a::{DataType as Dp2aDataType, Dp2a, Mode as Dp2aMode},
};

#[test]
fn parses_dp2a_lo_with_mixed_types() {
    assert_eq!(
        parse::<Dp2a>("dp2a.lo.u32.s32 %r0, %r1, %r2, %r3;"),
        Dp2a {
            mode: Dp2aMode::Lo,
            atype: Dp2aDataType::U32,
            btype: Dp2aDataType::S32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
            c: RegisterOperand::Single("%r3".into()),
        }
    );
}

#[test]
fn parses_dp2a_hi_with_signed_types() {
    assert_eq!(
        parse::<Dp2a>("dp2a.hi.s32.s32 %rd4, %rd1, %rd2, %rd3;"),
        Dp2a {
            mode: Dp2aMode::Hi,
            atype: Dp2aDataType::S32,
            btype: Dp2aDataType::S32,
            destination: RegisterOperand::Single("%rd4".into()),
            a: RegisterOperand::Single("%rd1".into()),
            b: RegisterOperand::Single("%rd2".into()),
            c: RegisterOperand::Single("%rd3".into()),
        }
    );
}

#[test]
fn rejects_dp2a_with_invalid_mode() {
    let err = parse_result::<Dp2a>("dp2a.mid.u32.u32 %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail for invalid mode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_dp2a_with_invalid_data_type() {
    let err = parse_result::<Dp2a>("dp2a.lo.u64.u32 %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail for invalid atype");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
