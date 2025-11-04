use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::fns::Fns,
    },
};

#[test]
fn parses_fns_instruction() {
    assert_eq!(
        parse::<Fns>("fns.b32 %r0, %r1, %r2, %r3;"),
        Fns {
            b32: (),
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            mask: Operand::Register(RegisterOperand::Single("%r1".into())),
            base: Operand::Register(RegisterOperand::Single("%r2".into())),
            offset: Operand::Register(RegisterOperand::Single("%r3".into())),
        }
    );
    assert_roundtrip::<Fns>("fns.b32 %r0, %r1, %r2, %r3;");
}

#[test]
fn parses_fns_with_u32_operands() {
    assert_eq!(
        parse::<Fns>("fns.b32 %r4, %r5, %r6, %r7;"),
        Fns {
            b32: (),
            d: Operand::Register(RegisterOperand::Single("%r4".into())),
            mask: Operand::Register(RegisterOperand::Single("%r5".into())),
            base: Operand::Register(RegisterOperand::Single("%r6".into())),
            offset: Operand::Register(RegisterOperand::Single("%r7".into())),
        }
    );
    assert_roundtrip::<Fns>("fns.b32 %r4, %r5, %r6, %r7;");
}

#[test]
fn rejects_non_fns_opcode() {
    let err = parse_result::<Fns>("foo.b32 %r0, %r1, %r2, %r3;")
        .expect_err("parsing should fail for unsupported opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip::<Fns>("fns.b32 %r0, %r1, %r2, %r3;");
}

// FIXME: Parser doesn't have type modifiers on operands
// #[test]
// fn rejects_invalid_mask_type() {
//     let err = parse_result::<Fns>("fns.b32 %r0, .f32 %r1, .b32 %r2, %r3;")
//         .expect_err("parsing should fail for unsupported mask type");
//     assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
//     assert_roundtrip::<Fns>("fns.b32 %r0, %r1, %r2, %r3;");
// }
