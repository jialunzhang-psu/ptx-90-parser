use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, Immediate, Operand, RegisterOperand},
        instruction::mbarrier::{DataType, Init, Mbarrier, SharedSpace},
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn address_from_register(name: &str) -> AddressOperand {
    AddressOperand::Offset(AddressBase::Register(reg(name)), None)
}

#[test]
fn parses_mbarrier_init_with_shared_cta() {
    assert_eq!(
        parse::<Mbarrier>("mbarrier.init.shared::cta.b64 [%rd1], 4;"),
        Mbarrier::Init(Init {
            shared_space: SharedSpace::SharedCta,
            data_type: DataType::B64,
            address: address_from_register("%rd1"),
            count: Operand::Immediate(Immediate("4".into())),
        })
    );
}

#[test]
fn rejects_mbarrier_init_with_invalid_data_type() {
    let err = parse_result::<Mbarrier>("mbarrier.init.b32 [%rd1], 4;")
        .expect_err("mbarrier.init should reject unsupported data types");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
