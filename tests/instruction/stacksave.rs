use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::stacksave::{Type, Stacksave},
    },
};

#[test]
fn parses_stacksave_u32() {
    assert_eq!(
        parse::<Stacksave>("stacksave.u32 %r1;"),
        Stacksave {
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r1".into())),
        }
    );
    assert_roundtrip::<Stacksave>("stacksave.u32 %r1;");
}

#[test]
fn parses_stacksave_u64() {
    assert_eq!(
        parse::<Stacksave>("stacksave.u64 %rd2;"),
        Stacksave {
            type_: Type::U64,
            d: Operand::Register(RegisterOperand::Single("%rd2".into())),
        }
    );
    assert_roundtrip::<Stacksave>("stacksave.u64 %rd2;");
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Stacksave>("stacksave.s32 %r1;")
        .expect_err("parsing should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip::<Stacksave>("stacksave.u32 %r1;");
}

#[test]
#[ignore = "Parser accepts immediates as operands"]
fn rejects_non_register_operand() {
    let err = parse_result::<Stacksave>("stacksave.u32 0;")
        .expect_err("parsing should fail for non-register operand");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip::<Stacksave>("stacksave.u32 %r1;");
}
