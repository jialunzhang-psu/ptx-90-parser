use crate::util::*;
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::griddepcontrol::Griddepcontrol};

#[test]
fn parses_launch_dependents_variant() {
    assert_roundtrip::<Griddepcontrol>("griddepcontrol.launch_dependents;");
    assert_eq!(
        parse::<Griddepcontrol>("griddepcontrol.launch_dependents;"),
        Griddepcontrol::LaunchDependents
    );
}

#[test]
fn parses_wait_variant() {
    assert_roundtrip::<Griddepcontrol>("griddepcontrol.wait;");
    assert_eq!(
        parse::<Griddepcontrol>("griddepcontrol.wait;"),
        Griddepcontrol::Wait
    );
}

#[test]
fn rejects_unknown_modifier() {
    let err = parse_result::<Griddepcontrol>("griddepcontrol.invalid;")
        .expect_err("parse should fail for unsupported modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_modifier() {
    let err = parse_result::<Griddepcontrol>("griddepcontrol;")
        .expect_err("parse should fail when modifier is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Griddepcontrol>("griddepcontrol.wait")
        .expect_err("parse should fail when semicolon is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
