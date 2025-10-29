use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::stacksave::{DataType, Stacksave},
    },
};

#[test]
fn parses_stacksave_u32() {
    assert_eq!(
        parse::<Stacksave>("stacksave.u32 %r1;"),
        Stacksave {
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r1".into()),
        }
    );
}

#[test]
fn parses_stacksave_u64() {
    assert_eq!(
        parse::<Stacksave>("stacksave.u64 %rd2;"),
        Stacksave {
            data_type: DataType::U64,
            destination: RegisterOperand::Single("%rd2".into()),
        }
    );
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Stacksave>("stacksave.s32 %r1;")
        .expect_err("parsing should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_non_register_operand() {
    let err = parse_result::<Stacksave>("stacksave.u32 0;")
        .expect_err("parsing should fail for non-register operand");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
