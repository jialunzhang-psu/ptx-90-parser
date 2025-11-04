use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::getctarank::{Getctarank3, Type},
    },
};

#[test]
fn parses_generic_register_source() {
    assert_eq!(
        parse::<Getctarank3>("getctarank.u32 %r1, %r2;"),
        Getctarank3 {
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r1".into())),
            a: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
    assert_roundtrip::<Getctarank3>("getctarank.u32 %r1, %r2;");
}

#[test]
fn parses_with_u64_type() {
    assert_eq!(
        parse::<Getctarank3>("getctarank.u64 %rd1, %rd2;"),
        Getctarank3 {
            type_: Type::U64,
            d: Operand::Register(RegisterOperand::Single("%rd1".into())),
            a: Operand::Register(RegisterOperand::Single("%rd2".into())),
        }
    );
    assert_roundtrip::<Getctarank3>("getctarank.u64 %rd1, %rd2;");
}
