use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{common::RegisterOperand, instruction::vset2},
};

#[test]
fn parses_simd_merge_with_selectors() {
    let source = "vset2.s32.u32.ne %r1.h0, %r2.h10, %r3.h32, %r4;";
    assert_eq!(
        parse::<vset2::Vset2>(source),
        vset2::Vset2::SimdMerge(vset2::SimdMerge {
            a_type: vset2::DataType::S32,
            b_type: vset2::DataType::U32,
            comparison: vset2::CompareOp::Ne,
            destination: vset2::Destination {
                register: RegisterOperand::Single("%r1".into()),
                mask: Some(vset2::Mask::H0),
            },
            a: vset2::ASource {
                register: RegisterOperand::Single("%r2".into()),
                selector: Some(vset2::Selector {
                    halves: [vset2::Half::H1, vset2::Half::H0],
                }),
            },
            b: vset2::BSource {
                register: RegisterOperand::Single("%r3".into()),
                selector: Some(vset2::Selector {
                    halves: [vset2::Half::H3, vset2::Half::H2],
                }),
            },
            c: RegisterOperand::Single("%r4".into()),
        })
    );
    assert_roundtrip::<vset2::Vset2>(source);
}

#[test]
fn parses_accumulate_without_optional_modifiers() {
    let source = "vset2.u32.s32.ge.add %r5, %r6, %r7, %r8;";
    assert_eq!(
        parse::<vset2::Vset2>(source),
        vset2::Vset2::Accumulate(vset2::Accumulate {
            a_type: vset2::DataType::U32,
            b_type: vset2::DataType::S32,
            comparison: vset2::CompareOp::Ge,
            destination: vset2::Destination {
                register: RegisterOperand::Single("%r5".into()),
                mask: None,
            },
            a: vset2::ASource {
                register: RegisterOperand::Single("%r6".into()),
                selector: None,
            },
            b: vset2::BSource {
                register: RegisterOperand::Single("%r7".into()),
                selector: None,
            },
            c: RegisterOperand::Single("%r8".into()),
        })
    );
    assert_roundtrip::<vset2::Vset2>(source);
}

#[test]
fn rejects_unknown_comparison() {
    let err = parse_result::<vset2::Vset2>("vset2.s32.s32.foo %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail for unknown comparison modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
