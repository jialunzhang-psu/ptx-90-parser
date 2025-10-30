use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::not::{DataType as NotDataType, Not},
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
            data_type: NotDataType::Pred,
            destination: reg("%p1"),
            source: reg("%p0"),
        }
    );
}

#[test]
fn parses_not_bitwise() {
    assert_roundtrip::<Not>("not.b64 %rd3, %rd2;");
    assert_eq!(
        parse::<Not>("not.b64 %rd3, %rd2;"),
        Not {
            data_type: NotDataType::B64,
            destination: reg("%rd3"),
            source: reg("%rd2"),
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
