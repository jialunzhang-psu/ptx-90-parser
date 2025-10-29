use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, RegisterOperand},
        instruction::stmatrix::{DataType, Num, Shape, Source, StateSpace, Stmatrix},
    },
};

#[test]
fn parses_basic_stmatrix_instruction() {
    assert_eq!(
        parse::<Stmatrix>("stmatrix.sync.aligned.m8n8.x1.shared.b16 [%rd1], {%r0};"),
        Stmatrix {
            shape: Shape::M8N8,
            num: Num::X1,
            trans: false,
            state_space: Some(StateSpace::Shared),
            data_type: DataType::B16,
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd1".into())),
                None
            ),
            source: Source::X1(RegisterOperand::Single("%r0".into())),
        }
    );
}

#[test]
fn parses_transposed_shared_cta_variant() {
    assert_eq!(
        parse::<Stmatrix>(
            "stmatrix.sync.aligned.m16n8.x2.trans.shared::cta.b8 [%rd2], {%r0, %r1};"
        ),
        Stmatrix {
            shape: Shape::M16N8,
            num: Num::X2,
            trans: true,
            state_space: Some(StateSpace::SharedCta),
            data_type: DataType::B8,
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd2".into())),
                None
            ),
            source: Source::X2([
                RegisterOperand::Single("%r0".into()),
                RegisterOperand::Single("%r1".into()),
            ]),
        }
    );
}

#[test]
fn rejects_non_stmatrix_opcode() {
    let err = parse_result::<Stmatrix>("ldmatrix.sync.aligned.m8n8.x1.shared.b16 [%rd1], {%r0};")
        .expect_err("parser should reject non-stmatrix opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_source_register_count_mismatch() {
    let err = parse_result::<Stmatrix>("stmatrix.sync.aligned.m16n8.x4.b8 [%rd1], {%r0, %r1};")
        .expect_err("parser should reject mismatched register count");
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}
