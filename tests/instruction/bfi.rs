use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::bfi::{Bfi, Type},
    },
};

#[test]
fn parses_bfi_b32() {
    assert_eq!(
        parse::<Bfi>("bfi.b32 %r1, %r2, %r3, %r4, %r5;"),
        Bfi {
            type_: Type::B32,
            f: Operand::Register(RegisterOperand::Single("%r1".into())),
            a: Operand::Register(RegisterOperand::Single("%r2".into())),
            b: Operand::Register(RegisterOperand::Single("%r3".into())),
            c: Operand::Register(RegisterOperand::Single("%r4".into())),
            d: Operand::Register(RegisterOperand::Single("%r5".into())),
        }
    );
    assert_roundtrip::<Bfi>("bfi.b32 %r1, %r2, %r3, %r4, %r5;");
}

#[test]
fn parses_bfi_b64() {
    assert_eq!(
        parse::<Bfi>("bfi.b64 %rd1, %rd2, %rd3, %rd4, %rd5;"),
        Bfi {
            type_: Type::B64,
            f: Operand::Register(RegisterOperand::Single("%rd1".into())),
            a: Operand::Register(RegisterOperand::Single("%rd2".into())),
            b: Operand::Register(RegisterOperand::Single("%rd3".into())),
            c: Operand::Register(RegisterOperand::Single("%rd4".into())),
            d: Operand::Register(RegisterOperand::Single("%rd5".into())),
        }
    );
    assert_roundtrip::<Bfi>("bfi.b64 %rd1, %rd2, %rd3, %rd4, %rd5;");
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Bfi>("bfi.f32 %r1, %r2, %r3, %r4, %r5;")
        .expect_err("should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

// FIXME: Parser allows immediate operands, this test needs adjustment
// #[test]
// fn rejects_non_register_operands() {
//     let err = parse_result::<Bfi>("bfi.b32 %r1, %r2, %r3, %r4, 1;")
//         .expect_err("should fail when operands are not registers");
//     assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
// }
