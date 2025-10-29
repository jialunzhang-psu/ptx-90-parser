use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::bfi::{Bfi, DataType},
    },
};

#[test]
fn parses_bfi_b32() {
    assert_eq!(
        parse::<Bfi>("bfi.b32 %r1, %r2, %r3, %r4, %r5;"),
        Bfi {
            data_type: DataType::B32,
            destination: RegisterOperand::Single("%r1".into()),
            source: RegisterOperand::Single("%r2".into()),
            base: RegisterOperand::Single("%r3".into()),
            position: RegisterOperand::Single("%r4".into()),
            length: RegisterOperand::Single("%r5".into()),
        }
    );
}

#[test]
fn parses_bfi_b64() {
    assert_eq!(
        parse::<Bfi>("bfi.b64 %rd1, %rd2, %rd3, %rd4, %rd5;"),
        Bfi {
            data_type: DataType::B64,
            destination: RegisterOperand::Single("%rd1".into()),
            source: RegisterOperand::Single("%rd2".into()),
            base: RegisterOperand::Single("%rd3".into()),
            position: RegisterOperand::Single("%rd4".into()),
            length: RegisterOperand::Single("%rd5".into()),
        }
    );
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Bfi>("bfi.f32 %r1, %r2, %r3, %r4, %r5;")
        .expect_err("should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_non_register_operands() {
    let err = parse_result::<Bfi>("bfi.b32 %r1, %r2, %r3, %r4, 1;")
        .expect_err("should fail when operands are not registers");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
