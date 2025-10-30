use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{
            AddressBase, AddressOffset, AddressOperand, Immediate, Operand, RegisterOperand, Sign,
        },
        instruction::wmma::*,
    },
};

#[test]
fn parses_load_a_with_shape_first() {
    let instruction = parse::<Instruction>(
        "wmma.load.a.sync.aligned.m16n16k16.row.f16 {%r0, %r1, %r2, %r3}, [%rd4];",
    );

    assert_eq!(
        instruction,
        Instruction::LoadA(LoadA {
            layout: Layout::Row,
            shape: Shape::M16N16K16,
            state_space: None,
            data_type: AType::F16,
            destination: RegisterOperand::Vector4([
                "%r0".into(),
                "%r1".into(),
                "%r2".into(),
                "%r3".into()
            ]),
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd4".into())),
                None
            ),
            stride: None,
        })
    );

    assert_roundtrip::<Instruction>(
        "wmma.load.a.sync.aligned.row.m16n16k16.f16 {%r0, %r1, %r2, %r3}, [%rd4];",
    );
}

#[test]
fn parses_load_b_with_layout_first_and_stride() {
    let instruction = parse::<Instruction>(
        "wmma.load.b.sync.aligned.col.m8n32k16.shared.s8 {%r5, %r6}, [%rd7+16], 32;",
    );

    assert_eq!(
        instruction,
        Instruction::LoadB(LoadB {
            layout: Layout::Col,
            shape: Shape::M8N32K16,
            state_space: Some(StateSpace::Shared),
            data_type: BType::S8,
            destination: RegisterOperand::Vector2(["%r5".into(), "%r6".into()]),
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd7".into())),
                Some(AddressOffset::Immediate(
                    Sign::Positive,
                    Immediate("16".into())
                ))
            ),
            stride: Some(Operand::Immediate(Immediate("32".into()))),
        })
    );

    assert_roundtrip::<Instruction>(
        "wmma.load.b.sync.aligned.col.m8n32k16.shared.s8 {%r5, %r6}, [%rd7+16], 32;",
    );
}

#[test]
fn parses_load_c_with_shared_cta_space() {
    let instruction =
        parse::<Instruction>("wmma.load.c.sync.aligned.row.m8n8k4.shared::cta.f64 %r1, [%rd2];");

    assert_eq!(
        instruction,
        Instruction::LoadC(LoadC {
            layout: Layout::Row,
            shape: Shape::M8N8K4,
            state_space: Some(StateSpace::SharedCta),
            data_type: CType::F64,
            destination: RegisterOperand::Single("%r1".into()),
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd2".into())),
                None
            ),
            stride: None,
        })
    );

    assert_roundtrip::<Instruction>(
        "wmma.load.c.sync.aligned.row.m8n8k4.shared::cta.f64 %r1, [%rd2];",
    );
}

#[test]
fn rejects_unknown_variant() {
    let error = parse_result::<Instruction>(
        "wmma.load.d.sync.aligned.m16n16k16.row.f16 {%r0, %r1}, [%rd0];",
    )
    .expect_err("unexpected variant should produce error");

    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_aligned_modifier() {
    let error =
        parse_result::<Instruction>("wmma.load.a.sync.m16n16k16.row.f16 {%r0, %r1}, [%rd0];")
            .expect_err("missing .aligned should produce error");

    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
