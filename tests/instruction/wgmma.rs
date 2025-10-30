use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{PredicateRegister, RegisterOperand},
        instruction::wgmma::*,
    },
};

#[test]
fn parses_f16_descriptor_variant() {
    let source = "wgmma.mma_async.sync.aligned.shape.m64n16k16.dtype.f16.f16 \
                  %rd0, %rd1, %rd2, %p1, -1, 1, 0, 1;";
    let instruction = parse_result::<Wgmma>(source).expect("expected descriptor variant to parse");
    assert_roundtrip::<Wgmma>(source);

    match instruction {
        Wgmma::F16Descriptor(F16Descriptor {
            shape,
            dtype,
            destination,
            a_descriptor,
            b_descriptor,
            scale_d,
            imm_scale_a,
            imm_scale_b,
            imm_trans_a,
            imm_trans_b,
        }) => {
            assert_eq!(shape, ShapeK16::M64N16K16);
            assert_eq!(dtype, HalfAccumulatorType::F16);
            assert_eq!(destination, RegisterOperand::Single("%rd0".into()));
            assert_eq!(a_descriptor, RegisterOperand::Single("%rd1".into()));
            assert_eq!(b_descriptor, RegisterOperand::Single("%rd2".into()));
            assert_eq!(scale_d, PredicateRegister("%p1".into()));
            assert_eq!(imm_scale_a, ScaleImmediate::MinusOne);
            assert_eq!(imm_scale_b, ScaleImmediate::PlusOne);
            assert_eq!(imm_trans_a, TransposeImmediate::Identity);
            assert_eq!(imm_trans_b, TransposeImmediate::Transpose);
        }
        other => panic!("unexpected instruction variant: {other:?}"),
    }
}

#[test]
fn parses_integer_register_variant() {
    let source = "wgmma.mma_async.sync.aligned.shape.m64n32k32.s32.atype.s8.btype.u8 \
                  %r0, {%r10, %r11}, %r12, %p2;";
    let instruction =
        parse_result::<Wgmma>(source).expect("expected integer register variant to parse");
    assert_roundtrip::<Wgmma>(source);

    match instruction {
        Wgmma::IntegerRegister(IntegerRegister {
            shape,
            satfinite,
            atype,
            btype,
            destination,
            a_register,
            b_descriptor,
            scale_d,
        }) => {
            assert_eq!(shape, IntegerShape::M64N32K32);
            assert!(!satfinite, "satfinite flag should default to false");
            assert_eq!(atype, IntegerInputType::S8);
            assert_eq!(btype, IntegerInputType::U8);
            assert_eq!(destination, RegisterOperand::Single("%r0".into()));
            assert_eq!(
                a_register,
                RegisterOperand::Vector2(["%r10".into(), "%r11".into()])
            );
            assert_eq!(b_descriptor, RegisterOperand::Single("%r12".into()));
            assert_eq!(scale_d, PredicateRegister("%p2".into()));
        }
        other => panic!("unexpected instruction variant: {other:?}"),
    }
}

#[test]
fn rejects_unknown_dtype_modifier() {
    let error = parse_result::<Wgmma>(
        "wgmma.mma_async.sync.aligned.shape.m64n16k16.dtype.f64.f16 \
         %rd0, %rd1, %rd2, %p0, -1, 1, 0, 1;",
    )
    .expect_err("invalid dtype modifier should fail");

    assert!(
        matches!(
            error.kind,
            ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::InvalidLiteral(_)
        ),
        "expected structured error, got {:?}",
        error.kind
    );
}
