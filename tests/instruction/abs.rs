use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::abs::{Abs, DataType as AbsDataType},
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Abs>(source);
}

#[test]
fn parses_abs_instruction() {
    assert_eq!(
        parse::<Abs>("abs.s32 %r1,%r2;"),
        Abs {
            data_type: AbsDataType::S32,
            destination: RegisterOperand::Single("%r1".into()),
            source: RegisterOperand::Single("%r2".into()),
        }
    );
    assert_roundtrip("abs.s32 %r1,%r2;");
}

#[test]
fn parses_abs_instruction_with_spaces() {
    assert_eq!(
        parse::<Abs>("abs.s64 %rd7, %rd8;"),
        Abs {
            data_type: AbsDataType::S64,
            destination: RegisterOperand::Single("%rd7".into()),
            source: RegisterOperand::Single("%rd8".into()),
        }
    );
    assert_roundtrip("abs.s64 %rd7, %rd8;");
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Abs>("abs.u32 %r1,%r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("abs.s16 %r1,%r2;");
}

#[test]
fn rejects_wrong_opcode() {
    let err = parse_result::<Abs>("mov.s32 %r1,%r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("abs.s32 %r1,%r2;");
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Abs>("abs.s16 %r1,%r2")
        .expect_err("parse should fail when semicolon is missing");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
    assert_roundtrip("abs.s16 %r1,%r2;");
}
