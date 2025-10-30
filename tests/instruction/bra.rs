use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{common::Label, instruction::bra::Bra},
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Bra>(source);
}

#[test]
fn parses_simple_bra_instruction() {
    assert_eq!(
        parse::<Bra>("bra target;"),
        Bra {
            uniform: false,
            target: Label("target".into()),
        }
    );
    assert_roundtrip("bra target;");
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
    assert_roundtrip("bra.uni L0;");
}

#[test]
fn rejects_unknown_modifier() {
    let err = parse_result::<Bra>("bra.foo label;")
        .expect_err("parse should fail when modifier is not .uni");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("bra target;");
}

#[test]
fn rejects_missing_semicolon() {
    let err =
        parse_result::<Bra>("bra label").expect_err("parse should fail when semicolon is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
    assert_roundtrip("bra target;");
}
