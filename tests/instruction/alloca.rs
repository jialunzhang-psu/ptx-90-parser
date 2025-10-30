use crate::util::*;
use ptx_parser::r#type::common::{Immediate, RegisterOperand};
use ptx_parser::r#type::instruction::alloca::DataType as AllocaDataType;
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::alloca::Alloca};

#[test]
fn parses_default_alloca() {
    assert_eq!(
        parse::<Alloca>("alloca.u32 %r1, %r2;"),
        Alloca::Default {
            data_type: AllocaDataType::U32,
            pointer: RegisterOperand::Single("%r1".into()),
            size: RegisterOperand::Single("%r2".into()),
        }
    );
    assert_roundtrip::<Alloca>("alloca.u32 %r1, %r2;");
}

#[test]
fn parses_aligned_alloca() {
    assert_eq!(
        parse::<Alloca>("alloca.u64 %rd3, %rd4, 16;"),
        Alloca::Aligned {
            data_type: AllocaDataType::U64,
            pointer: RegisterOperand::Single("%rd3".into()),
            size: RegisterOperand::Single("%rd4".into()),
            alignment: Immediate("16".into()),
        }
    );
    assert_roundtrip::<Alloca>("alloca.u64 %rd3, %rd4, 16;");
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Alloca>("alloca.s32 %r1, %r2;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip::<Alloca>("alloca.u32 %r1, %r2;");
}

#[test]
fn rejects_non_immediate_alignment() {
    let err = parse_result::<Alloca>("alloca.u32 %r1, %r2, %r3;")
        .expect_err("parse should fail for bad alignment");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip::<Alloca>("alloca.u32 %r1, %r2;");
}
