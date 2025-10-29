use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::brev::{Brev, DataType},
    },
};

#[test]
fn parses_brev_instruction() {
    assert_eq!(
        parse::<Brev>("brev.b32 %r1, %r2;"),
        Brev {
            data_type: DataType::B32,
            destination: RegisterOperand::Single("%r1".into()),
            source: RegisterOperand::Single("%r2".into()),
        }
    );
}

#[test]
fn rejects_brev_with_invalid_data_type() {
    let err = parse_result::<Brev>("brev.u32 %r1, %r2;").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_brev_missing_semicolon() {
    let err = parse_result::<Brev>("brev.b64 %rd1, %rd2").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
