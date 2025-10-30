use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{common::*, instruction::multimem::*},
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn addr_reg(name: &str) -> AddressOperand {
    AddressOperand::Offset(AddressBase::Register(reg(name)), None)
}

#[test]
fn parses_ld_reduce_int() {
    let source = "multimem.ld_reduce.relaxed.cta.global.min.s32 %r1, [%rd1];";
    assert_eq!(
        parse::<Instruction>(source),
        Instruction::LdReduce(LdReduce::Int(LdReduceInt {
            semantics: Some(LoadSemantics::Relaxed),
            scope: Some(Scope::Cta),
            state_space: Some(StateSpace::Global),
            operation: IntegerOp::Min,
            data_type: IntegerType::S32,
            destination: reg("%r1"),
            address: addr_reg("%rd1"),
        }))
    );
    assert_roundtrip::<Instruction>(source);
}

#[test]
fn parses_ld_reduce_float_vector() {
    let source = "multimem.ld_reduce.acquire.sys.global.max.acc_f32.v2.f32 {%f0,%f1}, [%rd3];";
    assert_eq!(
        parse::<Instruction>(source),
        Instruction::LdReduce(LdReduce::Float(LdReduceFloat {
            semantics: Some(LoadSemantics::Acquire),
            scope: Some(Scope::Sys),
            state_space: Some(StateSpace::Global),
            operation: FloatOp::Max,
            accumulator_precision: Some(AccumulatorPrecision::AccF32),
            vector: Some(VectorWidth::V2),
            data_type: FloatType::F32,
            destination: VectorDestination::Vector2([reg("%f0"), reg("%f1")]),
            address: addr_reg("%rd3"),
        }))
    );
    assert_roundtrip::<Instruction>(source);
}

#[test]
fn parses_ld_reduce_weak_float_vector() {
    let source = "multimem.ld_reduce.weak.global.add.acc_f16.v4.f16 \
             {%f0,%f1,%f2,%f3}, [%rd0];";
    assert_eq!(
        parse::<Instruction>(source),
        Instruction::LdReduce(LdReduce::WeakFloat(LdReduceWeakFloat {
            state_space: Some(StateSpace::Global),
            operation: FloatOp::Add,
            accumulator_precision: Some(AccumulatorPrecision::AccF16),
            vector: Some(VectorWidth::V4),
            data_type: FloatType::F16,
            destination: VectorDestination::Vector4([
                reg("%f0"),
                reg("%f1"),
                reg("%f2"),
                reg("%f3")
            ]),
            address: addr_reg("%rd0"),
        }))
    );
    assert_roundtrip::<Instruction>(source);
}

#[test]
fn parses_store_int() {
    let source = "multimem.st.release.cluster.global.s64 [%rd1], %rd2;";
    assert_eq!(
        parse::<Instruction>(source),
        Instruction::Store(Store::Int(StoreInt {
            semantics: Some(StoreSemantics::Release),
            scope: Some(Scope::Cluster),
            state_space: Some(StateSpace::Global),
            data_type: IntegerType::S64,
            address: addr_reg("%rd1"),
            value: reg("%rd2"),
        }))
    );
    assert_roundtrip::<Instruction>(source);
}

#[test]
fn parses_store_float_vector() {
    let source = "multimem.st.relaxed.sys.global.v4.f16 [%rd4], {%f0,%f1,%f2,%f3};";
    assert_eq!(
        parse::<Instruction>(source),
        Instruction::Store(Store::Float(StoreFloat {
            semantics: Some(StoreSemantics::Relaxed),
            scope: Some(Scope::Sys),
            state_space: Some(StateSpace::Global),
            vector: Some(VectorWidth::V4),
            data_type: FloatType::F16,
            address: addr_reg("%rd4"),
            value: VectorValue::Vector4([reg("%f0"), reg("%f1"), reg("%f2"), reg("%f3")]),
        }))
    );
    assert_roundtrip::<Instruction>(source);
}

#[test]
fn parses_store_weak_float_vector() {
    let source = "multimem.st.weak.global.v2.f32 [%rd2], {%f4,%f5};";
    assert_eq!(
        parse::<Instruction>(source),
        Instruction::Store(Store::WeakFloat(StoreWeakFloat {
            state_space: Some(StateSpace::Global),
            vector: Some(VectorWidth::V2),
            data_type: FloatType::F32,
            address: addr_reg("%rd2"),
            value: VectorValue::Vector2([reg("%f4"), reg("%f5")]),
        }))
    );
    assert_roundtrip::<Instruction>(source);
}

#[test]
fn parses_reduction_int() {
    let source = "multimem.red.release.cta.global.xor.u64 [%rd3], %rd4;";
    assert_eq!(
        parse::<Instruction>(source),
        Instruction::Red(Red::Int(RedInt {
            semantics: Some(ReductionSemantics::Release),
            scope: Some(Scope::Cta),
            state_space: Some(StateSpace::Global),
            operation: IntegerOp::Xor,
            data_type: IntegerType::U64,
            address: addr_reg("%rd3"),
            value: reg("%rd4"),
        }))
    );
    assert_roundtrip::<Instruction>(source);
}

#[test]
fn parses_reduction_float_vector() {
    let source = "multimem.red.relaxed.sys.global.add.v2.bf16 [%rd5], {%f0,%f1};";
    assert_eq!(
        parse::<Instruction>(source),
        Instruction::Red(Red::Float(RedFloat {
            semantics: Some(ReductionSemantics::Relaxed),
            scope: Some(Scope::Sys),
            state_space: Some(StateSpace::Global),
            operation: FloatRedOp::Add,
            vector: Some(VectorWidth::V2),
            data_type: FloatReductionType::Bf16,
            address: addr_reg("%rd5"),
            value: VectorValue::Vector2([reg("%f0"), reg("%f1")]),
        }))
    );
    assert_roundtrip::<Instruction>(source);
}

#[test]
fn rejects_invalid_opcode() {
    let err = parse_result::<Instruction>("multimemx.ld_reduce.min.s32 %r1, [%rd1];")
        .expect_err("parsing should fail for unknown opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_integer_vector_width() {
    let err = parse_result::<Instruction>("multimem.ld_reduce.min.v2.s32 {%r1,%r2}, [%rd3];")
        .expect_err("vector width is invalid for integer variants");
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}

#[test]
fn rejects_invalid_float_reduction_operation() {
    let err = parse_result::<Instruction>(
        "multimem.red.relaxed.sys.global.min.v2.f16 [%rd0], {%f0,%f1};",
    )
    .expect_err("float reductions only allow .add");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
