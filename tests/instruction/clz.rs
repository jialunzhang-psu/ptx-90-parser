use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::clz::{Clz, DataType},
    },
};

#[test]
fn parses_clz_instruction() {
    let source = "clz.b32 %r1, %r2;";
    assert_eq!(
        parse::<Clz>(source),
        Clz {
            data_type: DataType::B32,
            destination: RegisterOperand::Single("%r1".into()),
            source: RegisterOperand::Single("%r2".into()),
        }
    );
    assert_roundtrip::<Clz>(source);
}

#[test]
fn rejects_clz_with_invalid_data_type() {
    let err = parse_result::<Clz>("clz.u32 %r1, %r2;").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip::<Clz>("clz.b32 %r1, %r2;");
}

#[test]
fn rejects_clz_missing_semicolon() {
    let err = parse_result::<Clz>("clz.b64 %rd1, %rd2").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
    assert_roundtrip::<Clz>("clz.b32 %r1, %r2;");
}
