use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::r#type::common::{Operand, RegisterOperand};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::instruction::addc::{Addc, Type},
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Addc>(source);
}

#[test]
fn parses_addc_with_condition_code() {
    assert_eq!(
        parse::<Addc>("addc.cc.u32 %r0, %r1, %r2;"),
        Addc {
            cc: true,
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
    assert_roundtrip("addc.cc.u32 %r0, %r1, %r2;");
}

#[test]
fn parses_addc_without_condition_code() {
    assert_eq!(
        parse::<Addc>("addc.s64 %rd0, %rd1, %rd2;"),
        Addc {
            cc: false,
            type_: Type::S64,
            d: Operand::Register(RegisterOperand::Single("%rd0".into())),
            a: Operand::Register(RegisterOperand::Single("%rd1".into())),
            b: Operand::Register(RegisterOperand::Single("%rd2".into())),
        }
    );
    assert_roundtrip("addc.s64 %rd0, %rd1, %rd2;");
}

#[test]
fn rejects_addc_with_invalid_type() {
    let err = parse_result::<Addc>("addc.cc.f32 %r0, %r1, %r2;")
        .expect_err("parse should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_addc_missing_semicolon() {
    let err = parse_result::<Addc>("addc.u32 %r0, %r1, %r2")
        .expect_err("parse should fail without trailing semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
