use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::popc::{DataType, Popc},
    },
};

#[test]
fn parses_popc_instruction() {
    assert_eq!(
        parse::<Popc>("popc.b32 %r1, %r2;"),
        Popc {
            data_type: DataType::B32,
            destination: RegisterOperand::Single("%r1".into()),
            source: RegisterOperand::Single("%r2".into()),
        }
    );
}

#[test]
fn rejects_popc_with_invalid_data_type() {
    let err = parse_result::<Popc>("popc.u32 %r1, %r2;").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_popc_missing_semicolon() {
    let err = parse_result::<Popc>("popc.b64 %rd1, %rd2").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
