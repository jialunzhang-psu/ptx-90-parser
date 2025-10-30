use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::cvt::{
            Basic, Cvt, F8x2Type, F8x4Type, Frnd2, Frnd2Kind, Frnd2Rounding, Frnd3, Frnd3Kind,
            Frnd3Rounding, IntegerRounding, Rn, RnKind, Rna, Rounding, Rs, RsKind, ScalarType,
        },
    },
};

#[test]
fn parses_basic_integer_rounding() {
    let source = "cvt.rni.s32.f32 %r0, %f1;";
    assert_eq!(
        parse::<Cvt>(source),
        Cvt::Basic(Basic {
            rounding: Some(Rounding::Integer(IntegerRounding::Rni)),
            flush_to_zero: false,
            saturate: false,
            destination_type: ScalarType::S32,
            source_type: ScalarType::F32,
            destination: RegisterOperand::Single("%r0".into()),
            source: RegisterOperand::Single("%f1".into()),
        })
    );
    assert_roundtrip::<Cvt>(source);
}

#[test]
fn parses_frnd2_with_optional_flags() {
    let source = "cvt.frnd2.rz.relu.satfinite.f16x2.f32 %f0, %f1, %f2;";
    assert_eq!(
        parse::<Cvt>(source),
        Cvt::Frnd2(Frnd2 {
            rounding: Frnd2Rounding::Rz,
            relu: true,
            satfinite: true,
            kind: Frnd2Kind::F16x2FromF32,
            destination: RegisterOperand::Single("%f0".into()),
            a: RegisterOperand::Single("%f1".into()),
            b: Some(RegisterOperand::Single("%f2".into())),
        })
    );
    assert_roundtrip::<Cvt>(source);
}

#[test]
fn parses_rs_with_vector_sources() {
    let source = "cvt.rs.relu.satfinite.f8x4type.e4m3x4.f32 %r0, {%f1, %f2, %f3, %f4}, %r1;";
    assert_eq!(
        parse::<Cvt>(source),
        Cvt::Rs(Rs {
            relu: true,
            satfinite: true,
            kind: RsKind::F8x4FromF32 {
                data_type: F8x4Type::E4m3x4,
                a: RegisterOperand::Single("%f1".into()),
                b: RegisterOperand::Single("%f2".into()),
                e: RegisterOperand::Single("%f3".into()),
                f: RegisterOperand::Single("%f4".into()),
            },
            destination: RegisterOperand::Single("%r0".into()),
            rbits: RegisterOperand::Single("%r1".into()),
        })
    );
    assert_roundtrip::<Cvt>(source);
}

#[test]
fn parses_rna_variant() {
    let source = "cvt.rna.satfinite.tf32.f32 %r0, %f1;";
    assert_eq!(
        parse::<Cvt>(source),
        Cvt::Rna(Rna {
            satfinite: true,
            destination: RegisterOperand::Single("%r0".into()),
            source: RegisterOperand::Single("%f1".into()),
        })
    );
    assert_roundtrip::<Cvt>(source);
}

#[test]
fn parses_rn_f8x2_from_f32() {
    let source = "cvt.rn.satfinite.e4m3x2.f32 %r0, %f1, %f2;";
    assert_eq!(
        parse::<Cvt>(source),
        Cvt::Rn(Rn {
            satfinite: true,
            relu: false,
            kind: RnKind::F8x2FromF32 {
                data_type: F8x2Type::E4m3x2,
                destination: RegisterOperand::Single("%r0".into()),
                a: RegisterOperand::Single("%f1".into()),
                b: RegisterOperand::Single("%f2".into()),
            },
        })
    );
    assert_roundtrip::<Cvt>(source);
}

#[test]
fn parses_frnd3_with_two_sources() {
    let source = "cvt.frnd3.rz.satfinite.ue8m0x2.f32 %r0, %r1, %r2;";
    assert_eq!(
        parse::<Cvt>(source),
        Cvt::Frnd3(Frnd3 {
            rounding: Frnd3Rounding::Rz,
            satfinite: true,
            kind: Frnd3Kind::Ue8m0x2FromF32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: Some(RegisterOperand::Single("%r2".into())),
        })
    );
    assert_roundtrip::<Cvt>(source);
}

#[test]
fn rejects_unknown_basic_modifier() {
    let err = parse_result::<Cvt>("cvt.fast.s32.f32 %r0, %f0;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_rn_with_invalid_source_type() {
    let err =
        parse_result::<Cvt>("cvt.rn.e4m3x2.f64 %r0, %f1, %f2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
