use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::bfe::{Bfe, Type},
    },
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Bfe>(source);
}

#[test]
fn parses_bfe_u32() {
    assert_eq!(
        parse::<Bfe>("bfe.u32 %r1, %r2, %r3, %r4;"),
        Bfe {
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r1".into())),
            a: Operand::Register(RegisterOperand::Single("%r2".into())),
            b: Operand::Register(RegisterOperand::Single("%r3".into())),
            c: Operand::Register(RegisterOperand::Single("%r4".into())),
        }
    );
    assert_roundtrip("bfe.u32 %r1, %r2, %r3, %r4;");
}

#[test]
fn parses_bfe_s64() {
    assert_eq!(
        parse::<Bfe>("bfe.s64 %rd1, %rd2, %rd3, %rd4;"),
        Bfe {
            type_: Type::S64,
            d: Operand::Register(RegisterOperand::Single("%rd1".into())),
            a: Operand::Register(RegisterOperand::Single("%rd2".into())),
            b: Operand::Register(RegisterOperand::Single("%rd3".into())),
            c: Operand::Register(RegisterOperand::Single("%rd4".into())),
        }
    );
    assert_roundtrip("bfe.s64 %rd1, %rd2, %rd3, %rd4;");
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Bfe>("bfe.f32 %r1, %r2, %r3, %r4;")
        .expect_err("should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

// FIXME: Parser allows immediate operands, this test needs adjustment
// #[test]
// fn rejects_non_register_operands() {
//     let err = parse_result::<Bfe>("bfe.u32 %r1, %r2, 1, %r4;")
//         .expect_err("should fail when operands are not registers");
//     assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
// }
