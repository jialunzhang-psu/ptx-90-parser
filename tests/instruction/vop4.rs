use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::vop4::{
        ASource, Accumulate as Vop4Accumulate, BSource, DataType as Vop4DataType, Destination,
        Lane, Mask as Vop4Mask, Merge as Vop4Merge, Operation as Vop4Operation, Selector, Vop4,
    },
};

#[test]
fn parses_merge_with_explicit_modifiers() {
    assert_eq!(
        parse::<Vop4>("vadd4.s32.u32.s32.sat %r1.b3210, %r2.b0123, %r3.b7654, %r4;"),
        Vop4::Merge(Vop4Merge {
            operation: Vop4Operation::Vadd4,
            data_type: Vop4DataType::S32,
            a_type: Vop4DataType::U32,
            b_type: Vop4DataType::S32,
            saturate: true,
            destination: Destination {
                register: RegisterOperand::Single("%r1".into()),
                mask: Some(Vop4Mask::B3210),
            },
            a: ASource {
                register: RegisterOperand::Single("%r2".into()),
                selector: Some(Selector {
                    lanes: [Lane::B0, Lane::B1, Lane::B2, Lane::B3],
                }),
            },
            b: BSource {
                register: RegisterOperand::Single("%r3".into()),
                selector: Some(Selector {
                    lanes: [Lane::B7, Lane::B6, Lane::B5, Lane::B4],
                }),
            },
            c: RegisterOperand::Single("%r4".into()),
        })
    );
    assert_roundtrip::<Vop4>("vadd4.s32.u32.s32.sat %r1.b3210, %r2.b0123, %r3.b7654, %r4;");
}

#[test]
fn parses_accumulate_without_optional_modifiers() {
    assert_eq!(
        parse::<Vop4>("vmax4.u32.s32.u32.add %r5, %r6, %r7, %r8;"),
        Vop4::Accumulate(Vop4Accumulate {
            operation: Vop4Operation::Vmax4,
            data_type: Vop4DataType::U32,
            a_type: Vop4DataType::S32,
            b_type: Vop4DataType::U32,
            destination: Destination {
                register: RegisterOperand::Single("%r5".into()),
                mask: None,
            },
            a: ASource {
                register: RegisterOperand::Single("%r6".into()),
                selector: None,
            },
            b: BSource {
                register: RegisterOperand::Single("%r7".into()),
                selector: None,
            },
            c: RegisterOperand::Single("%r8".into()),
        })
    );
    assert_roundtrip::<Vop4>("vmax4.u32.s32.u32.add %r5, %r6, %r7, %r8;");
}

#[test]
fn rejects_unknown_operation() {
    let err = parse_result::<Vop4>("vfoo4.s32.s32.s32 %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail for unknown opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_saturate_with_add() {
    let err = parse_result::<Vop4>("vsub4.s32.s32.s32.sat.add %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail when .sat and .add are combined");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_selector_digits() {
    let err = parse_result::<Vop4>("vadd4.s32.s32.s32 %r0, %r1.b8888, %r2, %r3;")
        .expect_err("parse should fail for invalid lane digits");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
