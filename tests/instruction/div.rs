use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Operand, RegisterOperand},
    r#type::instruction::div::{Div1, Type},
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Div1>(source);
}

#[test]
fn parses_div_with_unsigned_type() {
    let source = "div.u32 %r0, %r1, %r2;";
    assert_eq!(
        parse::<Div1>(source),
        Div1 {
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
    assert_roundtrip(source);
}

#[test]
fn parses_div_with_signed_type() {
    let source = "div.s64 %rd4, %rd5, %rd6;";
    assert_eq!(
        parse::<Div1>(source),
        Div1 {
            type_: Type::S64,
            d: Operand::Register(RegisterOperand::Single("%rd4".into())),
            a: Operand::Register(RegisterOperand::Single("%rd5".into())),
            b: Operand::Register(RegisterOperand::Single("%rd6".into())),
        }
    );
    assert_roundtrip(source);
}

#[test]
fn rejects_div_with_invalid_type() {
    let err = parse_result::<Div1>("div.f32 %f0, %f1, %f2;")
        .expect_err("parsing should fail for f32 type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("div.u16 %r0, %r1, %r2;");
}

#[test]
fn rejects_div_missing_semicolon() {
    let err = parse_result::<Div1>("div.s32 %r0, %r1, %r2")
        .expect_err("parsing should fail without trailing semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
    assert_roundtrip("div.s32 %r0, %r1, %r2;");
}
