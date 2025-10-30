use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::*,
    r#type::instruction::shl::{DataType as ShlDataType, Shl},
};

#[test]
fn parses_shl_with_register_shift_amount() {
    assert_eq!(
        parse::<Shl>("shl.b32 %r1, %r2, %r3;"),
        Shl {
            data_type: ShlDataType::B32,
            destination: RegisterOperand::Single("%r1".into()),
            a: RegisterOperand::Single("%r2".into()),
            b: Operand::Register(RegisterOperand::Single("%r3".into())),
        }
    );
    assert_roundtrip::<Shl>("shl.b32 %r1, %r2, %r3;");
}

#[test]
fn parses_shl_with_immediate_shift_amount() {
    assert_eq!(
        parse::<Shl>("shl.b64 %rd1, %rd2, 8;"),
        Shl {
            data_type: ShlDataType::B64,
            destination: RegisterOperand::Single("%rd1".into()),
            a: RegisterOperand::Single("%rd2".into()),
            b: Operand::Immediate(Immediate("8".into())),
        }
    );
    assert_roundtrip::<Shl>("shl.b64 %rd1, %rd2, 8;");
}

#[test]
fn rejects_invalid_shl_data_type() {
    let err = parse_result::<Shl>("shl.u32 %r1, %r2, %r3;").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_shl_missing_semicolon() {
    let err = parse_result::<Shl>("shl.b32 %r1, %r2, %r3").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
