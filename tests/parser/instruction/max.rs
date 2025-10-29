use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::max::{AType as MaxAType, BType as MaxBType, Max, Relu},
};

#[test]
fn parses_max_atype_without_relu() {
    assert_eq!(
        parse::<Max>("max.u32 %r0, %r1, %r2;"),
        Max::AType {
            data_type: MaxAType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
        }
    );
}

#[test]
fn parses_max_btype_with_relu() {
    assert_eq!(
        parse::<Max>("max.relu.s32 %r3, %r4, %r5;"),
        Max::BType {
            relu: Relu::Relu,
            data_type: MaxBType::S32,
            destination: RegisterOperand::Single("%r3".into()),
            a: RegisterOperand::Single("%r4".into()),
            b: RegisterOperand::Single("%r5".into()),
        }
    );
}

#[test]
fn parses_max_btype_without_relu() {
    assert_eq!(
        parse::<Max>("max.s16x2 %r6, %r7, %r8;"),
        Max::BType {
            relu: Relu::Default,
            data_type: MaxBType::S16x2,
            destination: RegisterOperand::Single("%r6".into()),
            a: RegisterOperand::Single("%r7".into()),
            b: RegisterOperand::Single("%r8".into()),
        }
    );
}

#[test]
fn rejects_max_atype_with_relu_modifier() {
    let err = parse_result::<Max>("max.relu.u16 %r0, %r1, %r2;")
        .expect_err("parse should fail when relu is used with an AType");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_max_with_invalid_type() {
    let err = parse_result::<Max>("max.f32 %f0, %f1, %f2;")
        .expect_err("parse should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
