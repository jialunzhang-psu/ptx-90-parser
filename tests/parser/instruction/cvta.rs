use crate::util::{parse, parse_result};
use ptx_parser::r#type::common::{Immediate, RegisterOperand, VariableSymbol};
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::cvta::*};

#[test]
fn parses_cvta_to_generic_with_register_source() {
    assert_eq!(
        parse::<Cvta>("cvta.shared.u64 %rd1, %rd2;"),
        Cvta::ToGeneric(ToGeneric {
            space: Space::Shared,
            size: Size::U64,
            destination: RegisterOperand::Single("%rd1".into()),
            source: GenericSource::Register(RegisterOperand::Single("%rd2".into())),
        })
    );
}

#[test]
fn parses_cvta_to_address_space_with_shared_cta_space() {
    assert_eq!(
        parse::<Cvta>("cvta.to.shared::cta.u32 %r1, %r2;"),
        Cvta::ToAddressSpace(ToAddressSpace {
            space: Space::SharedCta,
            size: Size::U32,
            destination: RegisterOperand::Single("%r1".into()),
            source: RegisterOperand::Single("%r2".into()),
        })
    );
}

#[test]
fn parses_cvta_to_generic_with_variable_plus_immediate() {
    assert_eq!(
        parse::<Cvta>("cvta.param.u32 %r3, my_var+16;"),
        Cvta::ToGeneric(ToGeneric {
            space: Space::Param,
            size: Size::U32,
            destination: RegisterOperand::Single("%r3".into()),
            source: GenericSource::VariableWithImmediate {
                variable: VariableSymbol("my_var".into()),
                immediate: Immediate("16".into()),
            },
        })
    );
}

#[test]
fn rejects_cvta_with_invalid_space() {
    let err = parse_result::<Cvta>("cvta.invalid.u32 %r0, %r1;")
        .expect_err("parse should fail for bad space");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_cvta_missing_trailing_semicolon() {
    let err = parse_result::<Cvta>("cvta.shared.u32 %r0, %r1")
        .expect_err("parse should fail without trailing semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
