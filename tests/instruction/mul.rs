use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Operand, RegisterOperand},
    r#type::instruction::mul::{Mode, Mul1, Type},
};

#[test]
fn parses_mul_lo_with_unsigned_type() {
    assert_eq!(
        parse::<Mul1>("mul.lo.u32 %r0, %r1, %r2;"),
        Mul1 {
            mode: Mode::Lo,
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
    assert_roundtrip::<Mul1>("mul.lo.u32 %r0, %r1, %r2;");
}

#[test]
fn parses_mul_wide_with_signed_type() {
    assert_eq!(
        parse::<Mul1>("mul.wide.s64 %rd3, %rd1, %rd2;"),
        Mul1 {
            mode: Mode::Wide,
            type_: Type::S64,
            d: Operand::Register(RegisterOperand::Single("%rd3".into())),
            a: Operand::Register(RegisterOperand::Single("%rd1".into())),
            b: Operand::Register(RegisterOperand::Single("%rd2".into())),
        }
    );
    assert_roundtrip::<Mul1>("mul.wide.s64 %rd3, %rd1, %rd2;");
}
