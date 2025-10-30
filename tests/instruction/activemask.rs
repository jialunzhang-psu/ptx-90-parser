use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::parser::ParseErrorKind;
use ptx_parser::r#type::{common::RegisterOperand, instruction::activemask::Activemask};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Activemask>(source);
}

#[test]
fn parses_activemask_instruction() {
    assert_eq!(
        parse::<Activemask>("activemask.b32 %r1;"),
        Activemask {
            destination: RegisterOperand::Single("%r1".into()),
        }
    );
    assert_roundtrip("activemask.b32 %r1;");
}

#[test]
fn rejects_invalid_modifier() {
    let err = parse_result::<Activemask>("activemask.b16 %r1;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("activemask.b32 %r1;");
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Activemask>("activemask.b32 %r1")
        .expect_err("parse should fail without terminating semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
    assert_roundtrip("activemask.b32 %r1;");
}
