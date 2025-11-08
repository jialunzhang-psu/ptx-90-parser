mod util;
use ptx_parser::{ParseErrorKind, r#type::*};
use util::*;

#[test]
fn parses_linkage_directives() {
    assert_eq!(parse::<CodeLinkage>(".visible"), CodeLinkage::Visible);
    assert_eq!(parse::<CodeLinkage>(".extern"), CodeLinkage::Extern);
    assert_eq!(parse::<CodeLinkage>(".weak"), CodeLinkage::Weak);
    assert_roundtrip::<CodeLinkage>(".visible");
    assert_roundtrip::<CodeLinkage>(".extern");
    assert_roundtrip::<CodeLinkage>(".weak");

    assert_eq!(parse::<DataLinkage>(".common"), DataLinkage::Common);
    assert_eq!(
        parse::<CodeOrDataLinkage>(".visible"),
        CodeOrDataLinkage::Visible
    );
    assert_roundtrip::<DataLinkage>(".common");
    assert_roundtrip::<CodeOrDataLinkage>(".visible");
}

#[test]
fn parses_attribute_directives() {
    assert_eq!(
        parse::<AttributeDirective>(".managed"),
        AttributeDirective::Managed
    );
    assert_roundtrip::<AttributeDirective>(".managed");

    assert_eq!(
        parse::<AttributeDirective>(".unified(1, 2)"),
        AttributeDirective::Unified(1, 2)
    );
    assert_roundtrip::<AttributeDirective>(".unified(1,2)");
}

#[test]
fn parses_tex_type_and_data_type() {
    assert_eq!(parse::<TexType>(".texref"), TexType::TexRef);
    assert_eq!(parse::<TexType>(".surfref"), TexType::SurfRef);
    assert_roundtrip::<TexType>(".texref");
    assert_roundtrip::<TexType>(".surfref");

    assert_eq!(parse::<DataType>(".u64"), DataType::U64);
    assert_eq!(parse::<DataType>(".f32"), DataType::F32);
    assert_eq!(parse::<DataType>(".pred"), DataType::Pred);
    assert_roundtrip::<DataType>(".u64");
    assert_roundtrip::<DataType>(".f32");
    assert_roundtrip::<DataType>(".pred");
}

#[test]
fn parses_sign_and_immediate() {
    assert_eq!(parse::<Sign>("+"), Sign::Positive);
    assert_eq!(parse::<Sign>("-"), Sign::Negative);
    assert_roundtrip::<Sign>("+");
    assert_roundtrip::<Sign>("-");

    assert_eq!(parse::<Immediate>("42"), Immediate("42".into()));
    assert_eq!(parse::<Immediate>("0xFF"), Immediate("0xFF".into()));
    assert_roundtrip::<Immediate>("42");
    assert_roundtrip::<Immediate>("0xFF");
    assert_roundtrip::<Immediate>("1.5");
    assert_roundtrip::<Immediate>("1e3");
}

#[test]
fn parses_register_operands() {
    assert_eq!(
        parse::<RegisterOperand>("%r1"),
        RegisterOperand("%r1".into())
    );
    assert_roundtrip::<RegisterOperand>("%r1");
    let err = parse_result::<RegisterOperand>("{%r1,%r2}").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn parses_predicate_register() {
    assert_eq!(
        parse::<PredicateRegister>("%p1"),
        PredicateRegister("%p1".into())
    );
    assert_roundtrip::<PredicateRegister>("%p1");

    let err = parse_result::<PredicateRegister>("%r1").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}

#[test]
fn parses_special_register_variants() {
    assert_eq!(
        parse::<SpecialRegister>("%tid.y"),
        SpecialRegister::Tid(Axis::Y)
    );
    assert_roundtrip::<SpecialRegister>("%tid.y");
    assert_eq!(
        parse::<SpecialRegister>("%pm3_64"),
        SpecialRegister::Pm64(3)
    );
    assert_roundtrip::<SpecialRegister>("%pm3_64");
    assert_eq!(
        parse::<SpecialRegister>("%envreg31"),
        SpecialRegister::Envreg(31)
    );
    assert_roundtrip::<SpecialRegister>("%envreg31");

    let err = parse_result::<SpecialRegister>("%envreg99").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}

#[test]
fn parses_address_operands() {
    assert_eq!(
        parse::<AddressOperand>("arr[10]"),
        AddressOperand::Array(VariableSymbol("arr".into()), Immediate("10".into()))
    );
    assert_roundtrip::<AddressOperand>("arr[10]");

    assert_eq!(
        parse::<AddressOperand>("[%r1+4]"),
        AddressOperand::Offset(
            AddressBase::Register(RegisterOperand("%r1".into())),
            Some(AddressOffset::Immediate(
                Sign::Positive,
                Immediate("4".into())
            ))
        )
    );
    assert_roundtrip::<AddressOperand>("[%r1+4]");

    assert_eq!(
        parse::<AddressOperand>("[%r2+%r3]"),
        AddressOperand::Offset(
            AddressBase::Register(RegisterOperand("%r2".into())),
            Some(AddressOffset::Register(RegisterOperand("%r3".into())))
        )
    );
    assert_roundtrip::<AddressOperand>("[%r2+%r3]");

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
    assert_roundtrip::<AddressOperand>("[foo+8]");

    assert_eq!(
        parse::<AddressOperand>("[64]"),
        AddressOperand::ImmediateAddress(Immediate("64".into()))
    );
    assert_eq!(
        parse::<AddressOperand>("[-8]"),
        AddressOperand::ImmediateAddress(Immediate("-8".into()))
    );
    assert_roundtrip::<AddressOperand>("[-8]");
    assert_roundtrip::<AddressOperand>("[%r1-4]");
}

#[test]
fn parses_generic_operand() {
    assert_eq!(
        parse::<GeneralOperand>("%r1"),
        GeneralOperand::Single(Operand::Register(RegisterOperand("%r1".into())))
    );
    assert_roundtrip::<GeneralOperand>("%r1");
    assert_eq!(
        parse::<GeneralOperand>("123"),
        GeneralOperand::Single(Operand::Immediate(Immediate("123".into())))
    );
    assert_roundtrip::<GeneralOperand>("123");
}

#[test]
fn parses_operand_vectors() {
    assert_eq!(
        parse::<VectorOperand>("{%r1,%r2}"),
        VectorOperand::Vector2([
            Operand::Register(RegisterOperand("%r1".into())),
            Operand::Register(RegisterOperand("%r2".into())),
        ])
    );
    assert_roundtrip::<VectorOperand>("{%r1,%r2}");

    assert_eq!(
        parse::<VectorOperand>("{a,b,c}"),
        VectorOperand::Vector3([
            Operand::Symbol("a".into()),
            Operand::Symbol("b".into()),
            Operand::Symbol("c".into()),
        ])
    );
    assert_roundtrip::<VectorOperand>("{a,b,c}");

    assert_eq!(
        parse::<VectorOperand>("{1,2,3,4}"),
        VectorOperand::Vector4([
            Operand::Immediate(Immediate("1".into())),
            Operand::Immediate(Immediate("2".into())),
            Operand::Immediate(Immediate("3".into())),
            Operand::Immediate(Immediate("4".into())),
        ])
    );
    assert_roundtrip::<VectorOperand>("{1,2,3,4}");

    assert_eq!(
        parse::<VectorOperand>("{a,b,c,d,e,f,g,h}"),
        VectorOperand::Vector8([
            Operand::Symbol("a".into()),
            Operand::Symbol("b".into()),
            Operand::Symbol("c".into()),
            Operand::Symbol("d".into()),
            Operand::Symbol("e".into()),
            Operand::Symbol("f".into()),
            Operand::Symbol("g".into()),
            Operand::Symbol("h".into()),
        ])
    );
    assert_roundtrip::<VectorOperand>("{a,b,c,d,e,f,g,h}");
}

#[test]
fn parses_tex_handlers() {
    assert_eq!(
        parse::<TexHandler2>("[surf_B, {x}]"),
        TexHandler2([
            GeneralOperand::Single(Operand::Symbol("surf_B".into())),
            GeneralOperand::Vec(VectorOperand::Vector1(Operand::Symbol("x".into()))),
        ])
    );
    assert_roundtrip::<TexHandler2>("[surf_B, {x}]");

    assert_eq!(
        parse::<TexHandler3>("[tex_a, {f1,f2}, {f3}]"),
        TexHandler3 {
            handle: GeneralOperand::Single(Operand::Symbol("tex_a".into())),
            sampler: GeneralOperand::Vec(VectorOperand::Vector2([
                Operand::Symbol("f1".into()),
                Operand::Symbol("f2".into()),
            ])),
            coords: GeneralOperand::Vec(VectorOperand::Vector1(Operand::Symbol("f3".into()))),
        }
    );
    assert_roundtrip::<TexHandler3>("[tex_a, {f1,f2}, {f3}]");

    assert_eq!(
        parse::<TexHandler3Optional>("[tex_a, {f1,f2}]"),
        TexHandler3Optional {
            handle: GeneralOperand::Single(Operand::Symbol("tex_a".into())),
            sampler: None,
            coords: GeneralOperand::Vec(VectorOperand::Vector2([
                Operand::Symbol("f1".into()),
                Operand::Symbol("f2".into()),
            ])),
        }
    );
    assert_roundtrip::<TexHandler3Optional>("[tex_a, {f1,f2}]");
}

#[test]
fn parses_symbols_and_labels() {
    assert_eq!(parse::<FunctionSymbol>("foo"), FunctionSymbol("foo".into()));
    assert_eq!(parse::<VariableSymbol>("bar"), VariableSymbol("bar".into()));
    assert_eq!(parse::<Label>("L0"), Label("L0".into()));
    assert_roundtrip::<FunctionSymbol>("foo");
    assert_roundtrip::<VariableSymbol>("bar");
    assert_roundtrip::<Label>("L0");
}

#[test]
fn parses_address_space_variants() {
    assert_eq!(parse::<AddressSpace>(".global"), AddressSpace::Global);
    assert_eq!(parse::<AddressSpace>(".shared"), AddressSpace::Shared);
    assert_eq!(parse::<AddressSpace>(".reg"), AddressSpace::Reg);
    assert_roundtrip::<AddressSpace>(".global");
    assert_roundtrip::<AddressSpace>(".shared");
    assert_roundtrip::<AddressSpace>(".reg");
}

#[test]
fn parses_arithmetic_expressions() {
    // Test symbol + immediate
    assert_eq!(
        parse::<Operand>("sh + 4"),
        Operand::SymbolOffset("sh".into(), Immediate("4".into()))
    );
    assert_roundtrip::<Operand>("sh + 4");

    assert_eq!(
        parse::<Operand>("var + 0x10"),
        Operand::SymbolOffset("var".into(), Immediate("0x10".into()))
    );
    assert_roundtrip::<Operand>("var + 0x10");

    // Test that plain symbols still work
    assert_eq!(parse::<Operand>("symbol"), Operand::Symbol("symbol".into()));
    assert_roundtrip::<Operand>("symbol");
}
