use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Label, RegisterOperand},
        instruction::brx::Brx,
    },
};

#[test]
fn parses_brx_instruction() {
    assert_eq!(
        parse::<Brx>("brx.idx %r3, target;"),
        Brx {
            uniform: false,
            index: RegisterOperand::Single("%r3".into()),
            targets: Label("target".into()),
        }
    );
}

#[test]
fn parses_uniform_brx_instruction() {
    assert_eq!(
        parse::<Brx>("brx.idx.uni %r5, L0;"),
        Brx {
            uniform: true,
            index: RegisterOperand::Single("%r5".into()),
            targets: Label("L0".into()),
        }
    );
}

#[test]
fn rejects_missing_idx_modifier() {
    let err = parse_result::<Brx>("brx %r0, target;")
        .expect_err("parse should fail when .idx modifier is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Brx>("brx.idx %r0, target")
        .expect_err("parse should fail when semicolon is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
