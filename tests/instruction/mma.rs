use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::mma::{
        BlockScaleOperands, DataType, F8Shape, Layout, MmaInstruction, MxF4ScaleVecSize,
        SyncAlignedBlockScaleM16N8K64MxF4, SyncAlignedF8, SyncAlignedM8N8K4,
    },
};

#[test]
fn parses_m8n8k4_half_precision_variant() {
    let source = "mma.sync.aligned.m8n8k4.alayout.row.blayout.col.dtype.f32.f16.f16.ctype.f32 \
         %f0, %f1, %f2, %f3;";
    let instruction = parse_result(source).expect("mma.m8n8k4 should parse");

    match instruction {
        MmaInstruction::SyncAlignedM8N8K4(SyncAlignedM8N8K4 {
            a_layout,
            b_layout,
            d_type,
            c_type,
            destination,
            operand_a,
            operand_b,
            operand_c,
        }) => {
            assert_eq!(a_layout, Layout::Row);
            assert_eq!(b_layout, Layout::Col);
            assert_eq!(d_type, DataType::F32);
            assert_eq!(c_type, DataType::F32);
            assert_eq!(destination, RegisterOperand::Single("%f0".into()));
            assert_eq!(operand_a, RegisterOperand::Single("%f1".into()));
            assert_eq!(operand_b, RegisterOperand::Single("%f2".into()));
            assert_eq!(operand_c, RegisterOperand::Single("%f3".into()));
        }
        other => panic!("unexpected instruction variant: {other:?}"),
    }

    assert_roundtrip::<MmaInstruction>(source);
}

#[test]
fn parses_block_scale_mxf4_variant() {
    let source = "mma.sync.aligned.m16n8k64.row.col.kind.mxf4.block_scale.scale_vec::2X.f32.e2m1.e2m1.f32.stype.ue8m0 \
         %rd0, %rd1, %rd2, %rd3, %rd4, {1, 2}, %rd5, {3, 4};";
    let instruction = parse_result(source).expect("mma.m16n8k64 mxf4 should parse");

    match instruction {
        MmaInstruction::SyncAlignedBlockScaleM16N8K64MxF4(SyncAlignedBlockScaleM16N8K64MxF4 {
            scale_vec_size,
            stype: _,
            destination,
            operand_a,
            operand_b,
            operand_c,
            block_scale:
                BlockScaleOperands {
                    scale_a_data,
                    scale_a_byte_id,
                    scale_a_thread_id,
                    scale_b_data,
                    scale_b_byte_id,
                    scale_b_thread_id,
                },
        }) => {
            assert_eq!(
                scale_vec_size,
                Some(MxF4ScaleVecSize::TwoX),
                "scale_vec_size should default to TwoX"
            );
            assert_eq!(destination, RegisterOperand::Single("%rd0".into()));
            assert_eq!(operand_a, RegisterOperand::Single("%rd1".into()));
            assert_eq!(operand_b, RegisterOperand::Single("%rd2".into()));
            assert_eq!(operand_c, RegisterOperand::Single("%rd3".into()));
            assert_eq!(scale_a_data, RegisterOperand::Single("%rd4".into()));
            assert_eq!(scale_b_data, RegisterOperand::Single("%rd5".into()));
            assert_eq!(scale_a_byte_id, 1);
            assert_eq!(scale_a_thread_id, 2);
            assert_eq!(scale_b_byte_id, 3);
            assert_eq!(scale_b_thread_id, 4);
        }
        other => panic!("unexpected instruction variant: {other:?}"),
    }

    assert_roundtrip::<MmaInstruction>(source);
}

#[test]
fn parses_shape_f8_variant() {
    let source = "mma.sync.aligned.shape.m16n8k32.row.col.dtype.f16.f8type.e4m3.f8type.e5m2.ctype.f32 \
         %f4, %f5, %f6, %f7;";
    let instruction = parse_result(source).expect("mma.shape F8 should parse");

    match instruction {
        MmaInstruction::SyncAlignedF8(SyncAlignedF8 {
            shape,
            d_type,
            a_type: _,
            b_type: _,
            c_type,
            destination,
            operand_a,
            operand_b,
            operand_c,
        }) => {
            assert_eq!(shape, F8Shape::M16N8K32);
            assert_eq!(d_type, DataType::F16);
            assert_eq!(c_type, DataType::F32);
            assert_eq!(destination, RegisterOperand::Single("%f4".into()));
            assert_eq!(operand_a, RegisterOperand::Single("%f5".into()));
            assert_eq!(operand_b, RegisterOperand::Single("%f6".into()));
            assert_eq!(operand_c, RegisterOperand::Single("%f7".into()));
        }
        other => panic!("unexpected instruction variant: {other:?}"),
    }

    assert_roundtrip::<MmaInstruction>(source);
}

#[test]
fn rejects_unknown_layout() {
    let err = parse_result::<MmaInstruction>(
        "mma.sync.aligned.m8n8k4.alayout.diag.blayout.col.dtype.f16.f16.f16.ctype.f16 \
         %f0, %f1, %f2, %f3;",
    )
    .expect_err("invalid layout should fail");

    assert!(
        matches!(
            err.kind,
            ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::InvalidLiteral(_)
        ),
        "unexpected error kind: {:?}",
        err.kind
    );
}

#[test]
fn rejects_missing_scale_vec_size_for_mxf4nvf4() {
    let err = parse_result::<MmaInstruction>(
        "mma.sync.aligned.m16n8k64.row.col.kind.mxf4nvf4.block_scale.f32.e2m1.e2m1.f32.stype.ue8m0 \
         %rd0, %rd1, %rd2, %rd3, %rd4, {1, 2}, %rd5, {3, 4};",
    )
    .expect_err("missing scale_vec modifier should fail");

    assert!(
        matches!(
            err.kind,
            ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::InvalidLiteral(_)
        ),
        "expected structured parse error, got {:?}",
        err.kind
    );
}
