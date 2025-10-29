use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, RegisterOperand},
        instruction::ldmatrix::{
            DataType, DestinationFormat, Ldmatrix, M8N16, M16N16, Num, Shape, SourceFormat,
            Standard, StateSpace,
        },
    },
};

#[test]
fn parses_standard_m8n8_variant() {
    assert_eq!(
        parse::<Ldmatrix>("ldmatrix.sync.aligned.m8n8.x1.shared.b16 %r0, [%rd1];"),
        Ldmatrix::Standard(Standard {
            shape: Shape::M8N8,
            num: Num::X1,
            trans: false,
            state_space: Some(StateSpace::Shared),
            data_type: DataType::B16,
            destination: RegisterOperand::Single("%r0".into()),
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd1".into())),
                None
            ),
        })
    );
}

#[test]
fn parses_standard_m16n16_with_trans() {
    assert_eq!(
        parse::<Ldmatrix>("ldmatrix.sync.aligned.m16n16.x4.trans.b8 {%r0, %r1, %r2, %r3}, [%rd5];"),
        Ldmatrix::Standard(Standard {
            shape: Shape::M16N16,
            num: Num::X4,
            trans: true,
            state_space: None,
            data_type: DataType::B8,
            destination: RegisterOperand::Vector4([
                "%r0".into(),
                "%r1".into(),
                "%r2".into(),
                "%r3".into()
            ]),
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd5".into())),
                None
            ),
        })
    );
}

#[test]
fn parses_m8n16_variant_with_state_space() {
    assert_eq!(
        parse::<Ldmatrix>(
            "ldmatrix.sync.aligned.m8n16.x2.shared::cta.b8x16.b6x16_p32 {%r0, %r1}, [%rd0];"
        ),
        Ldmatrix::M8N16(M8N16 {
            num: Num::X2,
            state_space: Some(StateSpace::SharedCta),
            destination_format: DestinationFormat::B8x16,
            source_format: SourceFormat::B6x16P32,
            destination: RegisterOperand::Vector2(["%r0".into(), "%r1".into()]),
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd0".into())),
                None
            ),
        })
    );
}

#[test]
fn parses_m16n16_special_variant() {
    assert_eq!(
        parse::<Ldmatrix>(
            "ldmatrix.sync.aligned.m16n16.x1.trans.shared.b8x16.b4x16_p64 %r2, [%rd3];"
        ),
        Ldmatrix::M16N16(M16N16 {
            num: Num::X1,
            state_space: Some(StateSpace::Shared),
            destination_format: DestinationFormat::B8x16,
            source_format: SourceFormat::B4x16P64,
            destination: RegisterOperand::Single("%r2".into()),
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd3".into())),
                None
            ),
        })
    );
}

#[test]
fn rejects_invalid_opcode() {
    let err = parse_result::<Ldmatrix>("stmatrix.sync.aligned.m8n8.x1.shared.b16 %r0, [%rd1];")
        .expect_err("parser should reject non-ldmatrix opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_trans_in_m16n16_special_variant() {
    let err = parse_result::<Ldmatrix>(
        "ldmatrix.sync.aligned.m16n16.x2.shared.b8x16.b6x16_p32 %r0, [%rd0];",
    )
    .expect_err("m16n16 special variant requires .trans modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_unexpected_modifier_after_shape() {
    let err = parse_result::<Ldmatrix>("ldmatrix.sync.aligned.m8n8.x1.invalid.b16 %r0, [%rd1];")
        .expect_err("invalid modifier should produce parse error");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
