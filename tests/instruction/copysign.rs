use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::copysign::{Copysign, DataType as CopysignDataType},
};

#[test]
fn parses_copysign_f32() {
    assert_eq!(
        parse::<Copysign>("copysign.f32 %f1,%f2,%f3;"),
        Copysign {
            data_type: CopysignDataType::F32,
            destination: RegisterOperand::Single("%f1".into()),
            a: RegisterOperand::Single("%f2".into()),
            b: RegisterOperand::Single("%f3".into()),
        }
    );
    assert_roundtrip::<Copysign>("copysign.f32 %f1,%f2,%f3;");
}

#[test]
fn parses_copysign_f64_with_spaces() {
    assert_eq!(
        parse::<Copysign>("copysign.f64 %fd0, %fd1, %fd2;"),
        Copysign {
            data_type: CopysignDataType::F64,
            destination: RegisterOperand::Single("%fd0".into()),
            a: RegisterOperand::Single("%fd1".into()),
            b: RegisterOperand::Single("%fd2".into()),
        }
    );
    assert_roundtrip::<Copysign>("copysign.f64 %fd0, %fd1, %fd2;");
}

#[test]
fn rejects_invalid_copysign_data_type() {
    let err = parse_result::<Copysign>("copysign.u32 %f1,%f2,%f3;")
        .expect_err("parsing should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_wrong_copysign_opcode() {
    let err = parse_result::<Copysign>("mov.f32 %f1,%f2,%f3;")
        .expect_err("parsing should fail for non-copysign opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_copysign_semicolon() {
    let err = parse_result::<Copysign>("copysign.f32 %f1,%f2,%f3")
        .expect_err("parsing should fail when semicolon is missing");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
