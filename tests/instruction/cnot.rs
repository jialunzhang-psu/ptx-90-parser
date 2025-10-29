use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::cnot::{Cnot, DataType as CnotDataType},
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

#[test]
fn parses_cnot_b16() {
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
