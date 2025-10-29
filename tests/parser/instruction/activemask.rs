use crate::util::{parse, parse_result};
use ptx_parser::parser::ParseErrorKind;
use ptx_parser::r#type::{common::RegisterOperand, instruction::activemask::Activemask};

#[test]
fn parses_activemask_instruction() {
    assert_eq!(
        parse::<Activemask>("activemask.b32 %r1;"),
        Activemask {
            destination: RegisterOperand::Single("%r1".into()),
        }
    );
}

#[test]
fn rejects_invalid_modifier() {
    let err = parse_result::<Activemask>("activemask.b16 %r1;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Activemask>("activemask.b32 %r1")
        .expect_err("parse should fail without terminating semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
