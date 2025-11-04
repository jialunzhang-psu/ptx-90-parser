use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Operand, RegisterOperand},
    r#type::instruction::dp2a::{Atype, Btype, Dp2a, Mode as Dp2aMode},
};

#[test]
fn parses_dp2a_lo_with_mixed_types() {
    assert_eq!(
        parse::<Dp2a>("dp2a.lo.u32.s32 %r0, %r1, %r2, %r3;"),
        Dp2a {
            mode: Dp2aMode::Lo,
            atype: Atype::U32,
            btype: Btype::S32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
            c: Operand::Register(RegisterOperand::Single("%r3".into())),
        }
    );
    assert_roundtrip::<Dp2a>("dp2a.lo.u32.s32 %r0, %r1, %r2, %r3;");
}

#[test]
fn parses_dp2a_hi_with_signed_types() {
    assert_eq!(
        parse::<Dp2a>("dp2a.hi.s32.s32 %rd4, %rd1, %rd2, %rd3;"),
        Dp2a {
            mode: Dp2aMode::Hi,
            atype: Atype::S32,
            btype: Btype::S32,
            d: Operand::Register(RegisterOperand::Single("%rd4".into())),
            a: Operand::Register(RegisterOperand::Single("%rd1".into())),
            b: Operand::Register(RegisterOperand::Single("%rd2".into())),
            c: Operand::Register(RegisterOperand::Single("%rd3".into())),
        }
    );
    assert_roundtrip::<Dp2a>("dp2a.hi.s32.s32 %rd4, %rd1, %rd2, %rd3;");
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
