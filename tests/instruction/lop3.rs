use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Immediate, Operand, RegisterOperand},
        instruction::lop3::{Lop31},
    },
};

#[test]
fn parses_plain_lop3() {
    assert_eq!(
        parse::<Lop31>("lop3.b32 %r0, %r1, %r2, %r3, 0xf0;"),
        Lop31 {
            b32: (),
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
            c: Operand::Register(RegisterOperand::Single("%r3".into())),
            immlut: Operand::Immediate(Immediate("0xf0".into())),
        }
    );
    assert_roundtrip::<Lop31>("lop3.b32 %r0, %r1, %r2, %r3, 0xf0;");
}
