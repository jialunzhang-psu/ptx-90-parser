use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::*,
    r#type::instruction::shr::{DataType as ShrDataType, Shr},
};

#[test]
fn parses_shr_with_register_shift_amount() {
    assert_eq!(
        parse::<Shr>("shr.u32 %r1, %r2, %r3;"),
        Shr {
            data_type: ShrDataType::U32,
            destination: RegisterOperand::Single("%r1".into()),
            a: RegisterOperand::Single("%r2".into()),
            b: Operand::Register(RegisterOperand::Single("%r3".into())),
        }
    );
    assert_roundtrip::<Shr>("shr.u32 %r1, %r2, %r3;");
}

#[test]
fn parses_shr_with_immediate_shift_amount() {
    assert_eq!(
        parse::<Shr>("shr.s64 %rd1, %rd2, 4;"),
        Shr {
            data_type: ShrDataType::S64,
            destination: RegisterOperand::Single("%rd1".into()),
            a: RegisterOperand::Single("%rd2".into()),
            b: Operand::Immediate(Immediate("4".into())),
        }
    );
    assert_roundtrip::<Shr>("shr.s64 %rd1, %rd2, 4;");
}

#[test]
fn rejects_invalid_shr_data_type() {
    let err = parse_result::<Shr>("shr.f32 %r1, %r2, %r3;").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_shr_missing_semicolon() {
    let err = parse_result::<Shr>("shr.u32 %r1, %r2, %r3").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}
