use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::not::{Not, Type},
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

#[test]
fn parses_not_predicate() {
    assert_roundtrip::<Not>("not.pred %p1, %p0;");
    assert_eq!(
        parse::<Not>("not.pred %p1, %p0;"),
        Not {
            type_: Type::Pred,
            d: Operand::Register(reg("%p1")),
            a: Operand::Register(reg("%p0")),
        }
    );
}

#[test]
fn parses_not_bitwise() {
    assert_roundtrip::<Not>("not.b64 %rd3, %rd2;");
    assert_eq!(
        parse::<Not>("not.b64 %rd3, %rd2;"),
        Not {
            type_: Type::B64,
            d: Operand::Register(reg("%rd3")),
            a: Operand::Register(reg("%rd2")),
        }
    );
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Not>("not.u32 %r1, %r2;").expect_err("parsing should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_wrong_opcode() {
    let err = parse_result::<Not>("xor.b32 %r1, %r2;").expect_err("opcodes should match");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
