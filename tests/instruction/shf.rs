use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::shf::{
        DataType as ShfDataType, Direction as ShfDirection, Mode as ShfMode, Shf,
    },
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Shf>(source);
}

#[test]
fn parses_shf_left_clamp() {
    assert_eq!(
        parse::<Shf>("shf.l.clamp.b32 %r1, %r2, %r3, %r4;"),
        Shf {
            direction: ShfDirection::Left,
            mode: ShfMode::Clamp,
            data_type: ShfDataType::B32,
            destination: RegisterOperand::Single("%r1".into()),
            a: RegisterOperand::Single("%r2".into()),
            b: RegisterOperand::Single("%r3".into()),
            c: RegisterOperand::Single("%r4".into()),
        }
    );
    assert_roundtrip("shf.l.clamp.b32 %r1, %r2, %r3, %r4;");
}

#[test]
fn parses_shf_right_wrap() {
    assert_eq!(
        parse::<Shf>("shf.r.wrap.b32 %r5, %r6, %r7, %r8;"),
        Shf {
            direction: ShfDirection::Right,
            mode: ShfMode::Wrap,
            data_type: ShfDataType::B32,
            destination: RegisterOperand::Single("%r5".into()),
            a: RegisterOperand::Single("%r6".into()),
            b: RegisterOperand::Single("%r7".into()),
            c: RegisterOperand::Single("%r8".into()),
        }
    );
    assert_roundtrip("shf.r.wrap.b32 %r5, %r6, %r7, %r8;");
}

#[test]
fn rejects_shf_with_invalid_direction() {
    let err =
        parse_result::<Shf>("shf.t.clamp.b32 %r1, %r2, %r3, %r4;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("shf.l.clamp.b32 %r1, %r2, %r3, %r4;");
}

#[test]
fn rejects_shf_missing_semicolon() {
    let err =
        parse_result::<Shf>("shf.l.wrap.b32 %r1, %r2, %r3, %r4").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
    assert_roundtrip("shf.r.wrap.b32 %r5, %r6, %r7, %r8;");
}
