use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::dp4a::{DataType as Dp4aDataType, Dp4a},
};

#[test]
fn parses_dp4a_with_unsigned_operands() {
    assert_eq!(
        parse::<Dp4a>("dp4a.u32.u32 %r0, %r1, %r2, %r3;"),
        Dp4a {
            atype: Dp4aDataType::U32,
            btype: Dp4aDataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
            c: RegisterOperand::Single("%r3".into()),
        }
    );
    assert_roundtrip::<Dp4a>("dp4a.u32.u32 %r0, %r1, %r2, %r3;");
}

#[test]
fn parses_dp4a_with_mixed_signedness() {
    assert_eq!(
        parse::<Dp4a>("dp4a.s32.u32 %rd4, %rd1, %rd2, %rd3;"),
        Dp4a {
            atype: Dp4aDataType::S32,
            btype: Dp4aDataType::U32,
            destination: RegisterOperand::Single("%rd4".into()),
            a: RegisterOperand::Single("%rd1".into()),
            b: RegisterOperand::Single("%rd2".into()),
            c: RegisterOperand::Single("%rd3".into()),
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
