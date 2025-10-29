use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::rem::{DataType as RemDataType, Rem},
};

#[test]
fn parses_rem_unsigned() {
    assert_eq!(
        parse::<Rem>("rem.u32 %r1, %r2, %r3;"),
        Rem {
            data_type: RemDataType::U32,
            destination: RegisterOperand::Single("%r1".into()),
            lhs: RegisterOperand::Single("%r2".into()),
            rhs: RegisterOperand::Single("%r3".into()),
        }
    );
}

#[test]
fn parses_rem_signed() {
    assert_eq!(
        parse::<Rem>("rem.s64 %rd4, %rd5, %rd6;"),
        Rem {
            data_type: RemDataType::S64,
            destination: RegisterOperand::Single("%rd4".into()),
            lhs: RegisterOperand::Single("%rd5".into()),
            rhs: RegisterOperand::Single("%rd6".into()),
        }
    );
}

#[test]
fn rejects_rem_invalid_type() {
    let err = parse_result::<Rem>("rem.f32 %r1, %r2, %r3;").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_rem_missing_semicolon() {
    let err = parse_result::<Rem>("rem.u32 %r1, %r2, %r3").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
