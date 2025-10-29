use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{common::Label, instruction::bra::Bra},
};

#[test]
fn parses_simple_bra_instruction() {
    assert_eq!(
        parse::<Bra>("bra target;"),
        Bra {
            uniform: false,
            target: Label("target".into()),
        }
    );
}

#[test]
fn parses_uniform_branch_instruction() {
    assert_eq!(
        parse::<Bra>("bra.uni L0;"),
        Bra {
            uniform: true,
            target: Label("L0".into()),
        }
    );
}

#[test]
fn rejects_unknown_modifier() {
    let err = parse_result::<Bra>("bra.foo label;")
        .expect_err("parse should fail when modifier is not .uni");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err =
        parse_result::<Bra>("bra label").expect_err("parse should fail when semicolon is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
