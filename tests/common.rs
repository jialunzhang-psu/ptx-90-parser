mod util;
use util::{parse, parse_result};
use ptx_parser::{parser::ParseErrorKind, r#type::common::*};

#[test]
fn parses_linkage_directives() {
    assert_eq!(parse::<CodeLinkage>(".visible"), CodeLinkage::Visible);
    assert_eq!(parse::<CodeLinkage>(".extern"), CodeLinkage::Extern);
    assert_eq!(parse::<CodeLinkage>(".weak"), CodeLinkage::Weak);

    assert_eq!(parse::<DataLinkage>(".common"), DataLinkage::Common);
    assert_eq!(
        parse::<CodeOrDataLinkage>(".visible"),
        CodeOrDataLinkage::Visible
    );
}

#[test]
fn parses_attribute_directives() {
    assert_eq!(
        parse::<AttributeDirective>(".managed"),
        AttributeDirective::Managed
    );

    assert_eq!(
        parse::<AttributeDirective>(".unified(1, 2)"),
        AttributeDirective::Unified(1, 2)
    );
}

#[test]
fn parses_tex_type_and_data_type() {
    assert_eq!(parse::<TexType>(".texref"), TexType::TexRef);
    assert_eq!(parse::<TexType>(".surfref"), TexType::SurfRef);

    assert_eq!(parse::<DataType>(".u64"), DataType::U64);
    assert_eq!(parse::<DataType>(".f32"), DataType::F32);
    assert_eq!(parse::<DataType>(".pred"), DataType::Pred);
}

#[test]
fn parses_sign_and_immediate() {
    assert_eq!(parse::<Sign>("+"), Sign::Positive);
    assert_eq!(parse::<Sign>("-"), Sign::Negative);

    assert_eq!(parse::<Immediate>("42"), Immediate("42".into()));
    assert_eq!(parse::<Immediate>("0xFF"), Immediate("0xFF".into()));
}

#[test]
fn parses_register_operands() {
    assert_eq!(
        parse::<RegisterOperand>("%r1"),
        RegisterOperand::Single("%r1".into())
    );
    assert_eq!(
        parse::<RegisterOperand>("{%r1,%r2}"),
        RegisterOperand::Vector2(["%r1".into(), "%r2".into()])
    );
    assert_eq!(
        parse::<RegisterOperand>("{%r1,%r2,%r3,%r4}"),
        RegisterOperand::Vector4(["%r1".into(), "%r2".into(), "%r3".into(), "%r4".into()])
    );

    let err = parse_result::<RegisterOperand>("{%r0,%r1,%r2}").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}

#[test]
fn parses_predicate_register() {
    assert_eq!(
        parse::<PredicateRegister>("%p1"),
        PredicateRegister("%p1".into())
    );

    let err = parse_result::<PredicateRegister>("%r1").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}

#[test]
fn parses_special_register_variants() {
    assert_eq!(
        parse::<SpecialRegister>("%tid.y"),
        SpecialRegister::Tid(Axis::Y)
    );
    assert_eq!(
        parse::<SpecialRegister>("%pm3_64"),
        SpecialRegister::Pm64(3)
    );
    assert_eq!(
        parse::<SpecialRegister>("%envreg31"),
        SpecialRegister::Envreg(31)
    );

    let err = parse_result::<SpecialRegister>("%envreg99").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}

#[test]
fn parses_address_operands() {
    assert_eq!(
        parse::<AddressOperand>("arr[10]"),
        AddressOperand::Array(VariableSymbol("arr".into()), Immediate("10".into()))
    );

    assert_eq!(
        parse::<AddressOperand>("[%r1+4]"),
        AddressOperand::Offset(
            AddressBase::Register(RegisterOperand::Single("%r1".into())),
            Some(AddressOffset::Immediate(
                Sign::Positive,
                Immediate("4".into())
            ))
        )
    );

    assert_eq!(
        parse::<AddressOperand>("[%r2+%r3]"),
        AddressOperand::Offset(
            AddressBase::Register(RegisterOperand::Single("%r2".into())),
            Some(AddressOffset::Register(RegisterOperand::Single(
                "%r3".into()
            )))
        )
    );

    assert_eq!(
        parse::<AddressOperand>("[foo+8]"),
        AddressOperand::Offset(
            AddressBase::Variable(VariableSymbol("foo".into())),
            Some(AddressOffset::Immediate(
                Sign::Positive,
                Immediate("8".into())
            ))
        )
    );

    assert_eq!(
        parse::<AddressOperand>("[64]"),
        AddressOperand::ImmediateAddress(Immediate("64".into()))
    );
    assert_eq!(
        parse::<AddressOperand>("[-8]"),
        AddressOperand::ImmediateAddress(Immediate("-8".into()))
    );
}

#[test]
fn parses_generic_operand() {
    assert_eq!(
        parse::<Operand>("%r1"),
        Operand::Register(RegisterOperand::Single("%r1".into()))
    );
    assert_eq!(
        parse::<Operand>("123"),
        Operand::Immediate(Immediate("123".into()))
    );
}

#[test]
fn parses_symbols_and_labels() {
    assert_eq!(parse::<FunctionSymbol>("foo"), FunctionSymbol("foo".into()));
    assert_eq!(parse::<VariableSymbol>("bar"), VariableSymbol("bar".into()));
    assert_eq!(parse::<Label>("L0"), Label("L0".into()));
}

#[test]
fn parses_address_space_variants() {
    assert_eq!(parse::<AddressSpace>(".global"), AddressSpace::Global);
    assert_eq!(parse::<AddressSpace>(".shared"), AddressSpace::Shared);
    assert_eq!(parse::<AddressSpace>(".reg"), AddressSpace::Reg);
}
