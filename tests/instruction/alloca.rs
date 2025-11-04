use crate::util::*;
use ptx_parser::r#type::common::{Immediate, Operand, RegisterOperand};
use ptx_parser::r#type::instruction::alloca::Type;
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::alloca::Alloca};

#[test]
fn parses_default_alloca() {
    assert_eq!(
        parse::<Alloca>("alloca.u32 %r1, %r2;"),
        Alloca {
            type_: Type::U32,
            ptr: Operand::Register(RegisterOperand::Single("%r1".into())),
            size: Operand::Register(RegisterOperand::Single("%r2".into())),
            immalign: None,
        }
    );
    assert_roundtrip::<Alloca>("alloca.u32 %r1, %r2;");
}

#[test]
fn parses_aligned_alloca() {
    assert_eq!(
        parse::<Alloca>("alloca.u64 %rd3, %rd4, 16;"),
        Alloca {
            type_: Type::U64,
            ptr: Operand::Register(RegisterOperand::Single("%rd3".into())),
            size: Operand::Register(RegisterOperand::Single("%rd4".into())),
            immalign: Some(Operand::Immediate(Immediate("16".into()))),
        }
    );
    assert_roundtrip::<Alloca>("alloca.u64 %rd3, %rd4, 16;");
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Alloca>("alloca.s32 %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip::<Alloca>("alloca.u32 %r1, %r2;");
}

#[test]
fn parses_alignment_with_register() {
    // The parser currently accepts register operands for alignment
    assert_eq!(
        parse::<Alloca>("alloca.u32 %r1, %r2, %r3;"),
        Alloca {
            type_: Type::U32,
            ptr: Operand::Register(RegisterOperand::Single("%r1".into())),
            size: Operand::Register(RegisterOperand::Single("%r2".into())),
            immalign: Some(Operand::Register(RegisterOperand::Single("%r3".into()))),
        }
    );
    assert_roundtrip::<Alloca>("alloca.u32 %r1, %r2, %r3;");
}
