use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::sad::{Type, Sad},
    },
};

#[test]
fn parses_sad_unsigned_type() {
    assert_eq!(
        parse::<Sad>("sad.u32 %r0, %r1, %r2, %r3;"),
        Sad {
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
            c: Operand::Register(RegisterOperand::Single("%r3".into())),
        }
    );
    assert_roundtrip::<Sad>("sad.u32 %r0, %r1, %r2, %r3;");
}

#[test]
fn parses_sad_signed_type() {
    assert_eq!(
        parse::<Sad>("sad.s16 %rs0, %rs1, %rs2, %rs3;"),
        Sad {
            type_: Type::S16,
            d: Operand::Register(RegisterOperand::Single("%rs0".into())),
            a: Operand::Register(RegisterOperand::Single("%rs1".into())),
            b: Operand::Register(RegisterOperand::Single("%rs2".into())),
            c: Operand::Register(RegisterOperand::Single("%rs3".into())),
        }
    );
    assert_roundtrip::<Sad>("sad.s16 %rs0, %rs1, %rs2, %rs3;");
}

#[test]
fn rejects_sad_invalid_type() {
    let err =
        parse_result::<Sad>("sad.f32 %r0, %r1, %r2, %r3;").expect_err("sad should reject .f32");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_sad_missing_semicolon() {
    let err =
        parse_result::<Sad>("sad.u32 %r0, %r1, %r2, %r3").expect_err("sad requires semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
