use crate::util::*;
use ptx_parser::parser::ParseErrorKind;
use ptx_parser::r#type::{
    common::{Operand, RegisterOperand},
    instruction::sub::{Sub1, Sub2, Type},
};

#[test]
fn parses_sub_with_unsigned_type() {
    assert_eq!(
        parse::<Sub1>("sub.u32 %r0, %r1, %r2;"),
        Sub1 {
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
    assert_roundtrip::<Sub1>("sub.u32 %r0, %r1, %r2;");
}

#[test]
fn parses_sub_with_saturate() {
    assert_eq!(
        parse::<Sub2>("sub.sat.s32 %r3, %r4, %r5;"),
        Sub2 {
            sat: true,
            s32: (),
            d: Operand::Register(RegisterOperand::Single("%r3".into())),
            a: Operand::Register(RegisterOperand::Single("%r4".into())),
            b: Operand::Register(RegisterOperand::Single("%r5".into())),
        }
    );
    assert_roundtrip::<Sub2>("sub.sat.s32 %r3, %r4, %r5;");
}
