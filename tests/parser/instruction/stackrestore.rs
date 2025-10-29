use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::stackrestore::{DataType, Stackrestore},
    },
};

#[test]
fn parses_stackrestore_u32() {
    assert_eq!(
        parse::<Stackrestore>("stackrestore.u32 %r1;"),
        Stackrestore {
            data_type: DataType::U32,
            register: RegisterOperand::Single("%r1".into()),
        }
    );
}

#[test]
fn parses_stackrestore_u64() {
    assert_eq!(
        parse::<Stackrestore>("stackrestore.u64 %rd2;"),
        Stackrestore {
            data_type: DataType::U64,
            register: RegisterOperand::Single("%rd2".into()),
        }
    );
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Stackrestore>("stackrestore.s32 %r1;")
        .expect_err("parsing should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_non_register_operand() {
    let err = parse_result::<Stackrestore>("stackrestore.u32 0;")
        .expect_err("parsing should fail for non-register operand");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
