use crate::util::*;
use ptx_parser::r#type::common::{Operand, RegisterOperand};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::instruction::madc::{Hilo, Madc, Type},
};

#[test]
fn parses_madc_with_result_part_and_condition_code() {
    assert_eq!(
        parse::<Madc>("madc.hi.cc.u64 %rd0, %rd1, %rd2, %rd3;"),
        Madc {
            hilo: Hilo::Hi,
            cc: true,
            type_: Type::U64,
            d: Operand::Register(RegisterOperand::Single("%rd0".into())),
            a: Operand::Register(RegisterOperand::Single("%rd1".into())),
            b: Operand::Register(RegisterOperand::Single("%rd2".into())),
            c: Operand::Register(RegisterOperand::Single("%rd3".into())),
        }
    );
    assert_roundtrip::<Madc>("madc.hi.cc.u64 %rd0, %rd1, %rd2, %rd3;");
}

#[test]
fn parses_madc_with_lo() {
    assert_eq!(
        parse::<Madc>("madc.lo.u32 %r0, %r1, %r2, %r3;"),
        Madc {
            hilo: Hilo::Lo,
            cc: false,
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
            c: Operand::Register(RegisterOperand::Single("%r3".into())),
        }
    );
    assert_roundtrip::<Madc>("madc.lo.u32 %r0, %r1, %r2, %r3;");
}
