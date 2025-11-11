mod util;
use ptx_parser::{ParseErrorKind, r#type::*};
use util::*;

#[test]
fn parses_linkage_directives() {
    assert_eq!(parse::<CodeLinkage>(".visible"), CodeLinkage::Visible { span: 0..8 });
    assert_eq!(parse::<CodeLinkage>(".extern"), CodeLinkage::Extern { span: 0..7 });
    assert_eq!(parse::<CodeLinkage>(".weak"), CodeLinkage::Weak { span: 0..5 });
    assert_roundtrip::<CodeLinkage>(".visible");
    assert_roundtrip::<CodeLinkage>(".extern");
    assert_roundtrip::<CodeLinkage>(".weak");

    assert_eq!(parse::<DataLinkage>(".common"), DataLinkage::Common { span: 0..7 });
    assert_eq!(
        parse::<CodeOrDataLinkage>(".visible"),
        CodeOrDataLinkage::Visible { span: 0..8 }
    );
    assert_roundtrip::<DataLinkage>(".common");
    assert_roundtrip::<CodeOrDataLinkage>(".visible");
}

#[test]
fn parses_attribute_directives() {
    assert_eq!(
        parse::<AttributeDirective>(".managed"),
        AttributeDirective::Managed { span: 0..8 }
    );
    assert_roundtrip::<AttributeDirective>(".managed");

    assert_eq!(
        parse::<AttributeDirective>(".unified(1, 2)"),
        AttributeDirective::Unified { uuid1: 1, uuid2: 2, span: 0..8 }
    );
    assert_roundtrip::<AttributeDirective>(".unified(1,2)");
}

#[test]
fn parses_tex_type_and_data_type() {
    assert_eq!(parse::<TexType>(".texref"), TexType::TexRef { span: 0..7 });
    assert_eq!(parse::<TexType>(".surfref"), TexType::SurfRef { span: 0..8 });
    assert_roundtrip::<TexType>(".texref");
    assert_roundtrip::<TexType>(".surfref");

    assert_eq!(parse::<DataType>(".u64"), DataType::U64 { span: 0..4 });
    assert_eq!(parse::<DataType>(".f32"), DataType::F32 { span: 0..4 });
    assert_eq!(parse::<DataType>(".pred"), DataType::Pred { span: 0..5 });
    assert_roundtrip::<DataType>(".u64");
    assert_roundtrip::<DataType>(".f32");
    assert_roundtrip::<DataType>(".pred");
}

#[test]
fn parses_sign_and_immediate() {
    assert_eq!(parse::<Sign>("+"), Sign::Positive { span: 0..1 });
    assert_eq!(parse::<Sign>("-"), Sign::Negative { span: 0..1 });
    assert_roundtrip::<Sign>("+");
    assert_roundtrip::<Sign>("-");

    assert_eq!(parse::<Immediate>("42"), Immediate { value: "42".into(), span: 0..2 });
    assert_eq!(parse::<Immediate>("0xFF"), Immediate { value: "0xFF".into(), span: 0..4 });
    assert_roundtrip::<Immediate>("42");
    assert_roundtrip::<Immediate>("0xFF");
    assert_roundtrip::<Immediate>("1.5");
    assert_roundtrip::<Immediate>("1e3");
}

#[test]
fn parses_register_operands() {
    assert_eq!(
        parse::<RegisterOperand>("%r1"),
        RegisterOperand { name: "%r1".into(), span: 0..3 }
    );
    assert_roundtrip::<RegisterOperand>("%r1");
    let err = parse_result::<RegisterOperand>("{%r1,%r2}").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn parses_predicate_register() {
    assert_eq!(
        parse::<PredicateRegister>("%p1"),
        PredicateRegister { name: "%p1".into(), span: 0..3 }
    );
    assert_roundtrip::<PredicateRegister>("%p1");

    let err = parse_result::<PredicateRegister>("%r1").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}

#[test]
fn parses_special_register_variants() {
    assert_eq!(
        parse::<SpecialRegister>("%tid.y"),
        SpecialRegister::Tid { axis: Axis::Y { span: 0..6 }, span: 0..6 }
    );
    assert_roundtrip::<SpecialRegister>("%tid.y");
    assert_eq!(
        parse::<SpecialRegister>("%pm3_64"),
        SpecialRegister::Pm64 { index: 3, span: 0..7 }
    );
    assert_roundtrip::<SpecialRegister>("%pm3_64");
    assert_eq!(
        parse::<SpecialRegister>("%envreg31"),
        SpecialRegister::Envreg { index: 31, span: 0..9 }
    );
    assert_roundtrip::<SpecialRegister>("%envreg31");

    let err = parse_result::<SpecialRegister>("%envreg99").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}

#[test]
fn parses_address_operands() {
    assert_eq!(
        parse::<AddressOperand>("arr[10]"),
        AddressOperand::Array {
            base: VariableSymbol { name: "arr".into(), span: 0..3 },
            index: Immediate { value: "10".into(), span: 4..6 },
            span: 0..7
        }
    );
    assert_roundtrip::<AddressOperand>("arr[10]");

    assert_eq!(
        parse::<AddressOperand>("[%r1+4]"),
        AddressOperand::Offset {
            base: AddressBase::Register {
                operand: RegisterOperand { name: "%r1".into(), span: 1..4 },
                span: 1..4
            },
            offset: Some(AddressOffset::Immediate {
                sign: Sign::Positive { span: 4..5 },
                value: Immediate { value: "4".into(), span: 5..6 },
                span: 4..6
            }),
            span: 0..7
        }
    );
    assert_roundtrip::<AddressOperand>("[%r1+4]");

    assert_eq!(
        parse::<AddressOperand>("[%r2+%r3]"),
        AddressOperand::Offset {
            base: AddressBase::Register {
                operand: RegisterOperand { name: "%r2".into(), span: 1..4 },
                span: 1..4
            },
            offset: Some(AddressOffset::Register {
                operand: RegisterOperand { name: "%r3".into(), span: 5..8 },
                span: 4..8
            }),
            span: 0..9
        }
    );
    assert_roundtrip::<AddressOperand>("[%r2+%r3]");

    assert_eq!(
        parse::<AddressOperand>("[foo+8]"),
        AddressOperand::Offset {
            base: AddressBase::Variable {
                symbol: VariableSymbol { name: "foo".into(), span: 1..4 },
                span: 1..4
            },
            offset: Some(AddressOffset::Immediate {
                sign: Sign::Positive { span: 4..5 },
                value: Immediate { value: "8".into(), span: 5..6 },
                span: 4..6
            }),
            span: 0..7
        }
    );
    assert_roundtrip::<AddressOperand>("[foo+8]");

    assert_eq!(
        parse::<AddressOperand>("[64]"),
        AddressOperand::ImmediateAddress {
            addr: Immediate { value: "64".into(), span: 1..3 },
            span: 0..4
        }
    );
    assert_eq!(
        parse::<AddressOperand>("[-8]"),
        AddressOperand::ImmediateAddress {
            addr: Immediate { value: "-8".into(), span: 2..3 },
            span: 0..4
        }
    );
    assert_roundtrip::<AddressOperand>("[-8]");
    assert_roundtrip::<AddressOperand>("[%r1-4]");
}

#[test]
fn parses_generic_operand() {
    assert_eq!(
        parse::<GeneralOperand>("%r1"),
        GeneralOperand::Single {
            operand: Operand::Register {
                operand: RegisterOperand { name: "%r1".into(), span: 0..3 },
                span: 0..3
            },
            span: 0..3
        }
    );
    assert_roundtrip::<GeneralOperand>("%r1");
    assert_eq!(
        parse::<GeneralOperand>("123"),
        GeneralOperand::Single {
            operand: Operand::Immediate {
                operand: Immediate { value: "123".into(), span: 0..3 },
                span: 0..3
            },
            span: 0..3
        }
    );
    assert_roundtrip::<GeneralOperand>("123");
}

#[test]
fn parses_operand_vectors() {
    assert_eq!(
        parse::<VectorOperand>("{%r1,%r2}"),
        VectorOperand::Vector2 {
            operands: [
                Operand::Register {
                    operand: RegisterOperand { name: "%r1".into(), span: 1..4 },
                    span: 1..4
                },
                Operand::Register {
                    operand: RegisterOperand { name: "%r2".into(), span: 5..8 },
                    span: 5..8
                },
            ],
            span: 0..9
        }
    );
    assert_roundtrip::<VectorOperand>("{%r1,%r2}");

    assert_eq!(
        parse::<VectorOperand>("{a,b,c}"),
        VectorOperand::Vector3 {
            operands: [
                Operand::Symbol { name: "a".into(), span: 1..2 },
                Operand::Symbol { name: "b".into(), span: 3..4 },
                Operand::Symbol { name: "c".into(), span: 5..6 },
            ],
            span: 0..7
        }
    );
    assert_roundtrip::<VectorOperand>("{a,b,c}");

    assert_eq!(
        parse::<VectorOperand>("{1,2,3,4}"),
        VectorOperand::Vector4 {
            operands: [
                Operand::Immediate {
                    operand: Immediate { value: "1".into(), span: 1..2 },
                    span: 1..2
                },
                Operand::Immediate {
                    operand: Immediate { value: "2".into(), span: 3..4 },
                    span: 3..4
                },
                Operand::Immediate {
                    operand: Immediate { value: "3".into(), span: 5..6 },
                    span: 5..6
                },
                Operand::Immediate {
                    operand: Immediate { value: "4".into(), span: 7..8 },
                    span: 7..8
                },
            ],
            span: 0..9
        }
    );
    assert_roundtrip::<VectorOperand>("{1,2,3,4}");

    assert_eq!(
        parse::<VectorOperand>("{a,b,c,d,e,f,g,h}"),
        VectorOperand::Vector8 {
            operands: [
                Operand::Symbol { name: "a".into(), span: 1..2 },
                Operand::Symbol { name: "b".into(), span: 3..4 },
                Operand::Symbol { name: "c".into(), span: 5..6 },
                Operand::Symbol { name: "d".into(), span: 7..8 },
                Operand::Symbol { name: "e".into(), span: 9..10 },
                Operand::Symbol { name: "f".into(), span: 11..12 },
                Operand::Symbol { name: "g".into(), span: 13..14 },
                Operand::Symbol { name: "h".into(), span: 15..16 },
            ],
            span: 0..17
        }
    );
    assert_roundtrip::<VectorOperand>("{a,b,c,d,e,f,g,h}");
}

#[test]
fn parses_tex_handlers() {
    assert_eq!(
        parse::<TexHandler2>("[surf_B, {x}]"),
        TexHandler2 {
            operands: [
                GeneralOperand::Single {
                    operand: Operand::Symbol { name: "surf_B".into(), span: 1..7 },
                    span: 1..7
                },
                GeneralOperand::Vec {
                    operand: VectorOperand::Vector1 {
                        operand: Operand::Symbol { name: "x".into(), span: 10..11 },
                        span: 9..12
                    },
                    span: 9..12
                },
            ],
            span: 0..13
        }
    );
    assert_roundtrip::<TexHandler2>("[surf_B, {x}]");

    assert_eq!(
        parse::<TexHandler3>("[tex_a, {f1,f2}, {f3}]"),
        TexHandler3 {
            handle: GeneralOperand::Single {
                operand: Operand::Symbol { name: "tex_a".into(), span: 1..6 },
                span: 1..6
            },
            sampler: GeneralOperand::Vec {
                operand: VectorOperand::Vector2 {
                    operands: [
                        Operand::Symbol { name: "f1".into(), span: 9..11 },
                        Operand::Symbol { name: "f2".into(), span: 12..14 },
                    ],
                    span: 8..15
                },
                span: 8..15
            },
            coords: GeneralOperand::Vec {
                operand: VectorOperand::Vector1 {
                    operand: Operand::Symbol { name: "f3".into(), span: 18..20 },
                    span: 17..21
                },
                span: 17..21
            },
            span: 0..22
        }
    );
    assert_roundtrip::<TexHandler3>("[tex_a, {f1,f2}, {f3}]");

    assert_eq!(
        parse::<TexHandler3Optional>("[tex_a, {f1,f2}]"),
        TexHandler3Optional {
            handle: GeneralOperand::Single {
                operand: Operand::Symbol { name: "tex_a".into(), span: 1..6 },
                span: 1..6
            },
            sampler: None,
            coords: GeneralOperand::Vec {
                operand: VectorOperand::Vector2 {
                    operands: [
                        Operand::Symbol { name: "f1".into(), span: 9..11 },
                        Operand::Symbol { name: "f2".into(), span: 12..14 },
                    ],
                    span: 8..15
                },
                span: 8..15
            },
            span: 0..16
        }
    );
    assert_roundtrip::<TexHandler3Optional>("[tex_a, {f1,f2}]");
}

#[test]
fn parses_symbols_and_labels() {
    assert_eq!(parse::<FunctionSymbol>("foo"), FunctionSymbol { name: "foo".into(), span: 0..3 });
    assert_eq!(parse::<VariableSymbol>("bar"), VariableSymbol { name: "bar".into(), span: 0..3 });
    assert_eq!(parse::<Label>("L0"), Label { name: "L0".into(), span: 0..2 });
    assert_roundtrip::<FunctionSymbol>("foo");
    assert_roundtrip::<VariableSymbol>("bar");
    assert_roundtrip::<Label>("L0");
}

#[test]
fn parses_address_space_variants() {
    assert_eq!(parse::<AddressSpace>(".global"), AddressSpace::Global { span: 0..7 });
    assert_eq!(parse::<AddressSpace>(".shared"), AddressSpace::Shared { span: 0..7 });
    assert_eq!(parse::<AddressSpace>(".reg"), AddressSpace::Reg { span: 0..4 });
    assert_roundtrip::<AddressSpace>(".global");
    assert_roundtrip::<AddressSpace>(".shared");
    assert_roundtrip::<AddressSpace>(".reg");
}

#[test]
fn parses_arithmetic_expressions() {
    // Test symbol + immediate
    assert_eq!(
        parse::<Operand>("sh + 4"),
        Operand::SymbolOffset {
            symbol: "sh".into(),
            offset: Immediate { value: "4".into(), span: 5..6 },
            span: 0..6
        }
    );
    assert_roundtrip::<Operand>("sh + 4");

    assert_eq!(
        parse::<Operand>("var + 0x10"),
        Operand::SymbolOffset {
            symbol: "var".into(),
            offset: Immediate { value: "0x10".into(), span: 6..10 },
            span: 0..10
        }
    );
    assert_roundtrip::<Operand>("var + 0x10");

    // Test that plain symbols still work
    assert_eq!(parse::<Operand>("symbol"), Operand::Symbol { name: "symbol".into(), span: 0..6 });
    assert_roundtrip::<Operand>("symbol");
}
