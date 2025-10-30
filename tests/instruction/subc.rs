use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::subc::{ConditionCode, DataType, Subc},
    },
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Subc>(source);
}

#[test]
fn parses_basic_subc_instruction() {
    let source = "subc.u32 %r0, %r1, %r2;";
    assert_roundtrip(source);
    assert_eq!(
        parse::<Subc>(source),
        Subc {
            condition_code: ConditionCode::None,
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            minuend: RegisterOperand::Single("%r1".into()),
            subtrahend: RegisterOperand::Single("%r2".into()),
        }
    );
}

#[test]
fn parses_subc_with_condition_code() {
    let source = "subc.cc.s64 %rd3, %rd1, %rd2;";
    assert_roundtrip(source);
    assert_eq!(
        parse::<Subc>(source),
        Subc {
            condition_code: ConditionCode::Cc,
            data_type: DataType::S64,
            destination: RegisterOperand::Single("%rd3".into()),
            minuend: RegisterOperand::Single("%rd1".into()),
            subtrahend: RegisterOperand::Single("%rd2".into()),
        }
    );
}

#[test]
fn rejects_non_subc_opcode() {
    let err = parse_result::<Subc>("sub.cc.s32 %r0, %r1, %r2;")
        .expect_err("parser should reject non-subc opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<Subc>("subc.u16 %r0, %r1, %r2;")
        .expect_err("parser should reject unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
