use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::prmt::{Mode, Prmt},
    },
};

#[test]
fn parses_prmt_with_mode() {
    let source = "prmt.b32.f4e %r1, %r2, %r3, %r4;";
    assert_eq!(
        parse::<Prmt>(source),
        Prmt {
            mode: Some(Mode::F4e),
            destination: RegisterOperand::Single("%r1".into()),
            a: RegisterOperand::Single("%r2".into()),
            b: RegisterOperand::Single("%r3".into()),
            c: RegisterOperand::Single("%r4".into()),
        }
    );
    assert_roundtrip::<Prmt>(source);
}

#[test]
fn parses_prmt_without_mode() {
    let source = "prmt.b32 %r5, %r6, %r7, %r8;";
    assert_eq!(
        parse::<Prmt>(source),
        Prmt {
            mode: None,
            destination: RegisterOperand::Single("%r5".into()),
            a: RegisterOperand::Single("%r6".into()),
            b: RegisterOperand::Single("%r7".into()),
            c: RegisterOperand::Single("%r8".into()),
        }
    );
    assert_roundtrip::<Prmt>(source);
}

#[test]
fn rejects_prmt_with_invalid_data_type() {
    let err = parse_result::<Prmt>("prmt.b64 %r1, %r2, %r3, %r4;").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_prmt_with_invalid_mode() {
    let err = parse_result::<Prmt>("prmt.b32.xyz %r1, %r2, %r3, %r4;").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_prmt_missing_semicolon() {
    let err = parse_result::<Prmt>("prmt.b32 %r1, %r2, %r3, %r4").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
