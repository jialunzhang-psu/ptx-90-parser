use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::div::{DataType as DivDataType, Div},
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Div>(source);
}

#[test]
fn parses_div_with_unsigned_type() {
    let source = "div.u32 %r0, %r1, %r2;";
    assert_eq!(
        parse::<Div>(source),
        Div {
            data_type: DivDataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
        }
    );
    assert_roundtrip(source);
}

#[test]
fn parses_div_with_signed_type() {
    let source = "div.s64 %rd4, %rd5, %rd6;";
    assert_eq!(
        parse::<Div>(source),
        Div {
            data_type: DivDataType::S64,
            destination: RegisterOperand::Single("%rd4".into()),
            a: RegisterOperand::Single("%rd5".into()),
            b: RegisterOperand::Single("%rd6".into()),
        }
    );
    assert_roundtrip(source);
}

#[test]
fn rejects_div_with_invalid_type() {
    let err = parse_result::<Div>("div.f32 %f0, %f1, %f2;")
        .expect_err("parsing should fail for f32 type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("div.u16 %r0, %r1, %r2;");
}

#[test]
fn rejects_div_missing_semicolon() {
    let err = parse_result::<Div>("div.s32 %r0, %r1, %r2")
        .expect_err("parsing should fail without trailing semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
    assert_roundtrip("div.s32 %r0, %r1, %r2;");
}
