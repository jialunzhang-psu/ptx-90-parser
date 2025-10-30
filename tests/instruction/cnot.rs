use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::cnot::{Cnot, DataType as CnotDataType},
    },
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Cnot>(source);
}

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

#[test]
fn parses_cnot_b16() {
    assert_roundtrip("cnot.b16 %r1, %r2;");
    assert_eq!(
        parse::<Cnot>("cnot.b16 %r1, %r2;"),
        Cnot {
            data_type: CnotDataType::B16,
            destination: reg("%r1"),
            source: reg("%r2"),
        }
    );
}

#[test]
fn parses_cnot_b64() {
    assert_roundtrip("cnot.b64 %rd3, %rd4;");
    assert_eq!(
        parse::<Cnot>("cnot.b64 %rd3, %rd4;"),
        Cnot {
            data_type: CnotDataType::B64,
            destination: reg("%rd3"),
            source: reg("%rd4"),
        }
    );
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Cnot>("cnot.b32x %r1, %r2;").expect_err("invalid type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_wrong_opcode() {
    let err = parse_result::<Cnot>("xor.b32 %r1, %r2;").expect_err("wrong opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
