use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::*,
    r#type::instruction::vop2::{
        ASource, Accumulate as Vop2Accumulate, BSource, DataType as Vop2DataType, Destination,
        Half, Mask as Vop2Mask, Merge as Vop2Merge, Operation as Vop2Operation, Selector, Vop2,
    },
};

#[test]
fn parses_merge_with_explicit_selectors() {
    assert_eq!(
        parse::<Vop2>("vadd2.s32.s32.u32.sat %r1.h0, %r2.h10, %r3.h32, %r4;"),
        Vop2::Merge(Vop2Merge {
            operation: Vop2Operation::Vadd2,
            dtype: Vop2DataType::S32,
            atype: Vop2DataType::S32,
            btype: Vop2DataType::U32,
            saturate: true,
            destination: Destination {
                register: RegisterOperand::Single("%r1".into()),
                mask: Some(Vop2Mask::H0),
            },
            a: ASource {
                register: RegisterOperand::Single("%r2".into()),
                selector: Some(Selector {
                    halves: [Half::H1, Half::H0],
                }),
            },
            b: BSource {
                register: RegisterOperand::Single("%r3".into()),
                selector: Some(Selector {
                    halves: [Half::H3, Half::H2],
                }),
            },
            c: RegisterOperand::Single("%r4".into()),
        })
    );
}

#[test]
fn parses_accumulate_without_modifiers() {
    assert_eq!(
        parse::<Vop2>("vmax2.u32.s32.u32.add %r5, %r6, %r7.h11, %r8;"),
        Vop2::Accumulate(Vop2Accumulate {
            operation: Vop2Operation::Vmax2,
            dtype: Vop2DataType::U32,
            atype: Vop2DataType::S32,
            btype: Vop2DataType::U32,
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
                selector: Some(Selector {
                    halves: [Half::H1, Half::H1],
                }),
            },
            c: RegisterOperand::Single("%r8".into()),
        })
    );
}

#[test]
fn rejects_unknown_operation() {
    let err = parse_result::<Vop2>("vfoo2.s32.s32.s32 %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail for unknown opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_saturate_with_add() {
    let err = parse_result::<Vop2>("vsub2.s32.s32.s32.sat.add %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail when .sat and .add are combined");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
