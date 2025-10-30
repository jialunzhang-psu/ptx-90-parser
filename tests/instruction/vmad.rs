use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::vmad::{ComponentSelect, DataType, PlusOne, Scale, Standard, Vmad},
    },
};

#[test]
fn parses_standard_vmad_with_modifiers_and_negation() {
    assert_eq!(
        parse::<Vmad>("vmad.s32.s32.u32.sat.shr7 %r0, -%r1.b0, -%r2.h1, -%r3;"),
        Vmad::Standard(Standard {
            dtype: DataType::S32,
            atype: DataType::S32,
            btype: DataType::U32,
            saturate: true,
            scale: Some(Scale::Shr7),
            destination: RegisterOperand::Single("%r0".into()),
            a_negated: true,
            a: RegisterOperand::Single("%r1".into()),
            asel: Some(ComponentSelect::B0),
            b_negated: true,
            b: RegisterOperand::Single("%r2".into()),
            bsel: Some(ComponentSelect::H1),
            c_negated: true,
            c: RegisterOperand::Single("%r3".into()),
        })
    );
    assert_roundtrip::<Vmad>("vmad.s32.s32.u32.sat.shr7 %r0, -%r1.b0, -%r2.h1, -%r3;");
}

#[test]
fn parses_plus_one_vmad_with_scale() {
    assert_eq!(
        parse::<Vmad>("vmad.u32.u32.s32.po.sat.shr15 %r4, %r5.h0, %r6.b3, %r7;"),
        Vmad::PlusOne(PlusOne {
            dtype: DataType::U32,
            atype: DataType::U32,
            btype: DataType::S32,
            saturate: true,
            scale: Some(Scale::Shr15),
            destination: RegisterOperand::Single("%r4".into()),
            a: RegisterOperand::Single("%r5".into()),
            asel: Some(ComponentSelect::H0),
            b: RegisterOperand::Single("%r6".into()),
            bsel: Some(ComponentSelect::B3),
            c: RegisterOperand::Single("%r7".into()),
        })
    );
    assert_roundtrip::<Vmad>("vmad.u32.u32.s32.po.sat.shr15 %r4, %r5.h0, %r6.b3, %r7;");
}

#[test]
fn rejects_vmad_with_invalid_data_type() {
    let err = parse_result::<Vmad>("vmad.f32.s32.u32 %r0, %r1, %r2, %r3;")
        .expect_err("parser should reject invalid data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_vmad_with_invalid_component_select() {
    let err = parse_result::<Vmad>("vmad.u32.u32.u32.po %r0, %r1.b4, %r2, %r3;")
        .expect_err("parser should reject invalid component selection");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
