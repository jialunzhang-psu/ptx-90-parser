use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::subc::{Subc, Type},
    },
};

#[test]
fn parses_basic_subc_instruction() {
    let source = "subc.u32 %r0, %r1, %r2;";
    assert_roundtrip::<Subc>(source);
    assert_eq!(
        parse::<Subc>(source),
        Subc {
            cc: false,
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
}

#[test]
fn parses_subc_with_condition_code() {
    let source = "subc.cc.s64 %rd3, %rd1, %rd2;";
    assert_roundtrip::<Subc>(source);
    assert_eq!(
        parse::<Subc>(source),
        Subc {
            cc: true,
            type_: Type::S64,
            d: Operand::Register(RegisterOperand::Single("%rd3".into())),
            a: Operand::Register(RegisterOperand::Single("%rd1".into())),
            b: Operand::Register(RegisterOperand::Single("%rd2".into())),
        }
    );
}
