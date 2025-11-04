use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Operand, RegisterOperand},
    r#type::instruction::abs::{Abs1, Type},
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Abs1>(source);
}

#[test]
fn parses_abs_instruction() {
    assert_eq!(
        parse::<Abs1>("abs.s32 %r1,%r2;"),
        Abs1 {
            type_: Type::S32,
            d: Operand::Register(RegisterOperand::Single("%r1".into())),
            a: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
    assert_roundtrip("abs.s32 %r1,%r2;");
}

#[test]
fn parses_abs_instruction_with_spaces() {
    assert_eq!(
        parse::<Abs1>("abs.s64 %rd7, %rd8;"),
        Abs1 {
            type_: Type::S64,
            d: Operand::Register(RegisterOperand::Single("%rd7".into())),
            a: Operand::Register(RegisterOperand::Single("%rd8".into())),
        }
    );
    assert_roundtrip("abs.s64 %rd7, %rd8;");
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Abs1>("abs.u32 %r1,%r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("abs.s16 %r1,%r2;");
}

#[test]
fn rejects_wrong_opcode() {
    let err = parse_result::<Abs1>("mov.s32 %r1,%r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("abs.s32 %r1,%r2;");
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Abs1>("abs.s16 %r1,%r2")
        .expect_err("parse should fail when semicolon is missing");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
    assert_roundtrip("abs.s16 %r1,%r2;");
}
