use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::selp::{Selp, Type},
    },
};

#[test]
fn parses_selp_instruction() {
    assert_eq!(
        parse::<Selp>("selp.u32 %r0, %r1, %r2, %p3;"),
        Selp {
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
            c: Operand::Register(RegisterOperand::Single("%p3".into())),
        }
    );
    assert_roundtrip::<Selp>("selp.u32 %r0, %r1, %r2, %p3;");
}
