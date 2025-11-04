use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Operand, RegisterOperand},
    r#type::instruction::dp4a::{Atype, Btype, Dp4a},
};

#[test]
fn parses_dp4a_with_unsigned_operands() {
    assert_eq!(
        parse::<Dp4a>("dp4a.u32.u32 %r0, %r1, %r2, %r3;"),
        Dp4a {
            atype: Atype::U32,
            btype: Btype::U32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
            c: Operand::Register(RegisterOperand::Single("%r3".into())),
        }
    );
    assert_roundtrip::<Dp4a>("dp4a.u32.u32 %r0, %r1, %r2, %r3;");
}

#[test]
fn parses_dp4a_with_mixed_signedness() {
    assert_eq!(
        parse::<Dp4a>("dp4a.s32.u32 %rd4, %rd1, %rd2, %rd3;"),
        Dp4a {
            atype: Atype::S32,
            btype: Btype::U32,
            d: Operand::Register(RegisterOperand::Single("%rd4".into())),
            a: Operand::Register(RegisterOperand::Single("%rd1".into())),
            b: Operand::Register(RegisterOperand::Single("%rd2".into())),
            c: Operand::Register(RegisterOperand::Single("%rd3".into())),
        }
    );
    assert_roundtrip::<Dp4a>("dp4a.s32.u32 %rd4, %rd1, %rd2, %rd3;");
}

#[test]
fn rejects_dp4a_with_invalid_opcode() {
    let err = parse_result::<Dp4a>("dp4b.u32.u32 %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail for invalid opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_dp4a_with_invalid_type() {
    let err = parse_result::<Dp4a>("dp4a.u64.u32 %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail for invalid atype");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
