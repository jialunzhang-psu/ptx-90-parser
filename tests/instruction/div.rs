use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::div::{DataType as DivDataType, Div},
};

#[test]
fn parses_div_with_unsigned_type() {
    assert_eq!(
        parse::<Div>("div.u32 %r0, %r1, %r2;"),
        Div {
            data_type: DivDataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
        }
    );
}

#[test]
fn parses_div_with_signed_type() {
    assert_eq!(
        parse::<Div>("div.s64 %rd4, %rd5, %rd6;"),
        Div {
            data_type: DivDataType::S64,
            destination: RegisterOperand::Single("%rd4".into()),
            a: RegisterOperand::Single("%rd5".into()),
            b: RegisterOperand::Single("%rd6".into()),
        }
    );
}

#[test]
fn rejects_div_with_invalid_type() {
    let err = parse_result::<Div>("div.f32 %f0, %f1, %f2;")
        .expect_err("parsing should fail for f32 type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_div_missing_semicolon() {
    let err = parse_result::<Div>("div.s32 %r0, %r1, %r2")
        .expect_err("parsing should fail without trailing semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
