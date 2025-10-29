use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::neg::{DataType, Neg},
    },
};

#[test]
fn parses_neg_instruction() {
    assert_eq!(
        parse::<Neg>("neg.s32 %r4, %r5;"),
        Neg {
            data_type: DataType::S32,
            destination: RegisterOperand::Single("%r4".into()),
            source: RegisterOperand::Single("%r5".into()),
        }
    );
}

#[test]
fn parses_neg_instruction_with_spaces() {
    assert_eq!(
        parse::<Neg>("neg.s64 %rd10 , %rd11 ;"),
        Neg {
            data_type: DataType::S64,
            destination: RegisterOperand::Single("%rd10".into()),
            source: RegisterOperand::Single("%rd11".into()),
        }
    );
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Neg>("neg.f32 %f1, %f2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<Neg>("neg.s16 %r0, %r1")
        .expect_err("parse should fail when semicolon is missing");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
