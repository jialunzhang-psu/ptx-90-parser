use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, RegisterOperand},
        instruction::ldu::{DataType, Ldu, Scalar, StateSpace, Vector, VectorDestination},
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn address_from_register(name: &str) -> AddressOperand {
    AddressOperand::Offset(AddressBase::Register(reg(name)), None)
}

#[test]
fn parses_scalar_ldu_with_state_space() {
    assert_eq!(
        parse::<Ldu>("ldu.global.f32 %f1, [%rd1];"),
        Ldu::Scalar(Scalar {
            state_space: Some(StateSpace::Global),
            data_type: DataType::F32,
            destination: reg("%f1"),
            address: address_from_register("%rd1"),
        })
    );
}

#[test]
fn parses_vector_ldu() {
    assert_eq!(
        parse::<Ldu>("ldu.v2.s32 {%r1, %r2}, [%rd0];"),
        Ldu::Vector(Vector {
            state_space: None,
            data_type: DataType::S32,
            destination: VectorDestination::V2([reg("%r1"), reg("%r2")]),
            address: address_from_register("%rd0"),
        })
    );
}

#[test]
fn rejects_vector_ldu_without_braces() {
    let error = parse_result::<Ldu>("ldu.v2.s32 %r1, [%rd0];")
        .expect_err("vector destinations require braces");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
