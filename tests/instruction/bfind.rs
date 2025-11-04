use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::bfind::{Bfind1, Bfind2, Type},
    },
};

#[test]
fn parses_plain_bfind() {
    assert_eq!(
        parse::<Bfind1>("bfind.u32 %r1, %r2;"),
        Bfind1 {
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r1".into())),
            a: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
    assert_roundtrip::<Bfind1>("bfind.u32 %r1, %r2;");
}

#[test]
fn parses_shift_amount_bfind() {
    assert_eq!(
        parse::<Bfind2>("bfind.shiftamt.s64 %rd3, %rd4;"),
        Bfind2 {
            shiftamt: (),
            type_: Type::S64,
            d: Operand::Register(RegisterOperand::Single("%rd3".into())),
            a: Operand::Register(RegisterOperand::Single("%rd4".into())),
        }
    );
    assert_roundtrip::<Bfind2>("bfind.shiftamt.s64 %rd3, %rd4;");
}

#[test]
fn rejects_invalid_bfind_data_type() {
    let err = parse_result::<Bfind1>("bfind.f32 %r1, %r2;")
        .expect_err("should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn accepts_non_register_operand() {
    // The parser accepts immediate operands
    assert_eq!(
        parse::<Bfind2>("bfind.shiftamt.u64 %r1, 1;"),
        Bfind2 {
            shiftamt: (),
            type_: Type::U64,
            d: Operand::Register(RegisterOperand::Single("%r1".into())),
            a: Operand::Immediate(ptx_parser::r#type::common::Immediate("1".into())),
        }
    );
}
