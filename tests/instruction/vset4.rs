use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::vset4::{
            CompareOp, Destination, Lane, Mask, OperandType, Selector, SourceA, SourceB, Vset4,
        },
    },
};

#[test]
fn parses_simd_merge_with_modifiers() {
    assert_eq!(
        parse::<Vset4>("vset4.u32.s32.eq %r1.b210, %r2.b0123, %r3.b7654, %r4;"),
        Vset4::SimdMerge {
            a_type: OperandType::U32,
            b_type: OperandType::S32,
            compare_op: CompareOp::Eq,
            destination: Destination {
                register: RegisterOperand::Single("%r1".into()),
                mask: Some(Mask::B210),
            },
            a: SourceA {
                register: RegisterOperand::Single("%r2".into()),
                selector: Some(Selector {
                    x: Lane::B0,
                    y: Lane::B1,
                    z: Lane::B2,
                    w: Lane::B3,
                }),
            },
            b: SourceB {
                register: RegisterOperand::Single("%r3".into()),
                selector: Some(Selector {
                    x: Lane::B7,
                    y: Lane::B6,
                    z: Lane::B5,
                    w: Lane::B4,
                }),
            },
            c: RegisterOperand::Single("%r4".into()),
        }
    );
}

#[test]
fn parses_accumulate_without_optional_modifiers() {
    assert_eq!(
        parse::<Vset4>("vset4.s32.s32.ge.add %r5, %r6, %r7, %r8;"),
        Vset4::Accumulate {
            a_type: OperandType::S32,
            b_type: OperandType::S32,
            compare_op: CompareOp::Ge,
            destination: Destination {
                register: RegisterOperand::Single("%r5".into()),
                mask: None,
            },
            a: SourceA {
                register: RegisterOperand::Single("%r6".into()),
                selector: None,
            },
            b: SourceB {
                register: RegisterOperand::Single("%r7".into()),
                selector: None,
            },
            c: RegisterOperand::Single("%r8".into()),
        }
    );
}

#[test]
fn rejects_unknown_compare_modifier() {
    let err = parse_result::<Vset4>("vset4.u32.u32.foo %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail for invalid compare modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_selector_digits() {
    let err = parse_result::<Vset4>("vset4.s32.s32.eq %r0, %r1.b8888, %r2, %r3;")
        .expect_err("parse should fail for invalid selector digits");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
