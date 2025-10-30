use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::min::{AType as MinAType, BType as MinBType, Min},
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Min>(source);
}

#[test]
fn parses_min_atype_without_relu() {
    let source = "min.u32 %r0, %r1, %r2;";
    assert_eq!(
        parse::<Min>(source),
        Min::AType {
            data_type: MinAType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
        }
    );
    assert_roundtrip(source);
}

#[test]
fn parses_min_btype_with_relu() {
    let source = "min.relu.s32 %r3, %r4, %r5;";
    assert_eq!(
        parse::<Min>(source),
        Min::BType {
            relu: true,
            data_type: MinBType::S32,
            destination: RegisterOperand::Single("%r3".into()),
            a: RegisterOperand::Single("%r4".into()),
            b: RegisterOperand::Single("%r5".into()),
        }
    );
    assert_roundtrip(source);
}

#[test]
fn parses_min_btype_without_relu() {
    let source = "min.s16x2 %r6, %r7, %r8;";
    assert_eq!(
        parse::<Min>(source),
        Min::BType {
            relu: false,
            data_type: MinBType::S16x2,
            destination: RegisterOperand::Single("%r6".into()),
            a: RegisterOperand::Single("%r7".into()),
            b: RegisterOperand::Single("%r8".into()),
        }
    );
    assert_roundtrip(source);
}

#[test]
fn rejects_min_atype_with_relu_modifier() {
    let err = parse_result::<Min>("min.relu.u16 %r0, %r1, %r2;")
        .expect_err("parse should fail when relu is used with an AType");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("min.s32 %r0, %r1, %r2;");
}

#[test]
fn rejects_min_with_invalid_type() {
    let err = parse_result::<Min>("min.f32 %f0, %f1, %f2;")
        .expect_err("parse should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("min.s16x2 %r0, %r1, %r2;");
}
