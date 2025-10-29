use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::sad::{DataType, Sad},
    },
};

#[test]
fn parses_sad_unsigned_type() {
    assert_eq!(
        parse::<Sad>("sad.u32 %r0, %r1, %r2, %r3;"),
        Sad {
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
            c: RegisterOperand::Single("%r3".into()),
        }
    );
}

#[test]
fn parses_sad_signed_type() {
    assert_eq!(
        parse::<Sad>("sad.s16 %rs0, %rs1, %rs2, %rs3;"),
        Sad {
            data_type: DataType::S16,
            destination: RegisterOperand::Single("%rs0".into()),
            a: RegisterOperand::Single("%rs1".into()),
            b: RegisterOperand::Single("%rs2".into()),
            c: RegisterOperand::Single("%rs3".into()),
        }
    );
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
