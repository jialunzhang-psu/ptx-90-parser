use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::neg::{Neg1, Type},
    },
};

#[test]
fn parses_neg_instruction() {
    assert_roundtrip::<Neg1>("neg.s32 %r4, %r5;");
    assert_eq!(
        parse::<Neg1>("neg.s32 %r4, %r5;"),
        Neg1 {
            type_: Type::S32,
            d: Operand::Register(RegisterOperand::Single("%r4".into())),
            a: Operand::Register(RegisterOperand::Single("%r5".into())),
        }
    );
}

#[test]
fn parses_neg_instruction_with_spaces() {
    assert_roundtrip::<Neg1>("neg.s64 %rd10 , %rd11 ;");
    assert_eq!(
        parse::<Neg1>("neg.s64 %rd10 , %rd11 ;"),
        Neg1 {
            type_: Type::S64,
            d: Operand::Register(RegisterOperand::Single("%rd10".into())),
            a: Operand::Register(RegisterOperand::Single("%rd11".into())),
        }
    );
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Neg1>("neg.f32 %f1, %f2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Neg1>("neg.s16 %r0, %r1")
        .expect_err("parse should fail when semicolon is missing");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
