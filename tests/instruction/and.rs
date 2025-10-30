use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::r#type::common::RegisterOperand;
use ptx_parser::r#type::instruction::and::DataType as AndDataType;
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::and::And};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<And>(source);
}

#[test]
fn parses_and_predicate() {
    assert_eq!(
        parse::<And>("and.pred %p0, %p1, %p2;"),
        And {
            data_type: AndDataType::Pred,
            destination: RegisterOperand::Single("%p0".into()),
            a: RegisterOperand::Single("%p1".into()),
            b: RegisterOperand::Single("%p2".into()),
        }
    );
    assert_roundtrip("and.pred %p0, %p1, %p2;");
}

#[test]
fn parses_and_bitwise() {
    assert_eq!(
        parse::<And>("and.b32 %r3, %r1, %r2;"),
        And {
            data_type: AndDataType::B32,
            destination: RegisterOperand::Single("%r3".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
        }
    );
    assert_roundtrip("and.b32 %r3, %r1, %r2;");
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<And>("and.u32 %r0, %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_wrong_opcode() {
    let err = parse_result::<And>("or.b32 %r0, %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<And>("and.b16 %r0, %r1, %r2")
        .expect_err("parse should fail without terminating semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
