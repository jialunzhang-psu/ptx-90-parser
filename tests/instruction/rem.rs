use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Operand, RegisterOperand},
    r#type::instruction::rem::{Type, Rem},
};

#[test]
fn parses_rem_unsigned() {
    assert_eq!(
        parse::<Rem>("rem.u32 %r1, %r2, %r3;"),
        Rem {
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r1".into())),
            a: Operand::Register(RegisterOperand::Single("%r2".into())),
            b: Operand::Register(RegisterOperand::Single("%r3".into())),
        }
    );
    assert_roundtrip::<Rem>("rem.u32 %r1, %r2, %r3;");
}

#[test]
fn parses_rem_signed() {
    assert_eq!(
        parse::<Rem>("rem.s64 %rd4, %rd5, %rd6;"),
        Rem {
            type_: Type::S64,
            d: Operand::Register(RegisterOperand::Single("%rd4".into())),
            a: Operand::Register(RegisterOperand::Single("%rd5".into())),
            b: Operand::Register(RegisterOperand::Single("%rd6".into())),
        }
    );
    assert_roundtrip::<Rem>("rem.s64 %rd4, %rd5, %rd6;");
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
