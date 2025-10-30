use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Immediate, RegisterOperand, VariableSymbol},
        instruction::getctarank::{DataType, Getctarank},
    },
};

#[test]
fn parses_generic_register_source() {
    assert_eq!(
        parse::<Getctarank>("getctarank.u32 %r1, %r2;"),
        Getctarank::Generic {
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r1".into()),
            source: RegisterOperand::Single("%r2".into()),
        }
    );
    assert_roundtrip::<Getctarank>("getctarank.u32 %r1, %r2;");
}

#[test]
fn parses_shared_register_source() {
    assert_eq!(
        parse::<Getctarank>("getctarank.shared::cluster.u64 %rd1, %rd2;"),
        Getctarank::SharedRegister {
            data_type: DataType::U64,
            destination: RegisterOperand::Single("%rd1".into()),
            source: RegisterOperand::Single("%rd2".into()),
        }
    );
    assert_roundtrip::<Getctarank>("getctarank.shared::cluster.u64 %rd1, %rd2;");
}

#[test]
fn parses_shared_variable_source() {
    assert_eq!(
        parse::<Getctarank>("getctarank.shared::cluster.u32 %r1, foo;"),
        Getctarank::SharedVariable {
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r1".into()),
            symbol: VariableSymbol("foo".into()),
        }
    );
    assert_roundtrip::<Getctarank>("getctarank.shared::cluster.u32 %r1, foo;");
}

#[test]
fn parses_shared_variable_with_immediate() {
    assert_eq!(
        parse::<Getctarank>("getctarank.shared::cluster.u64 %rd5, bar + 8;"),
        Getctarank::SharedVariableWithImmediate {
            data_type: DataType::U64,
            destination: RegisterOperand::Single("%rd5".into()),
            symbol: VariableSymbol("bar".into()),
            immediate: Immediate("8".into()),
        }
    );
    assert_roundtrip::<Getctarank>("getctarank.shared::cluster.u64 %rd5, bar + 8;");
}

#[test]
fn rejects_invalid_opcode() {
    let err = parse_result::<Getctarank>("getctaid.u32 %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_space_modifier() {
    let err = parse_result::<Getctarank>("getctarank.shared::cta.u32 %r1, %r2;")
        .expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_data_type() {
    let err =
        parse_result::<Getctarank>("getctarank.u16 %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
