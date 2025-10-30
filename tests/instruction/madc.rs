use crate::util::*;
use ptx_parser::r#type::common::RegisterOperand;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::instruction::madc::{DataType, Madc, ResultPart},
};

#[test]
fn parses_madc_with_result_part_and_condition_code() {
    assert_eq!(
        parse::<Madc>("madc.hi.cc.u64 %rd0, %rd1, %rd2, %rd3;"),
        Madc {
            result_part: Some(ResultPart::Hi),
            condition_code: true,
            data_type: DataType::U64,
            destination: RegisterOperand::Single("%rd0".into()),
            multiplicand: RegisterOperand::Single("%rd1".into()),
            multiplier: RegisterOperand::Single("%rd2".into()),
            addend: RegisterOperand::Single("%rd3".into()),
        }
    );
    assert_roundtrip::<Madc>("madc.hi.cc.u64 %rd0, %rd1, %rd2, %rd3;");
}

#[test]
fn parses_madc_with_condition_code_only() {
    assert_eq!(
        parse::<Madc>("madc.cc.u32 %r0, %r1, %r2, %r3;"),
        Madc {
            result_part: None,
            condition_code: true,
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            multiplicand: RegisterOperand::Single("%r1".into()),
            multiplier: RegisterOperand::Single("%r2".into()),
            addend: RegisterOperand::Single("%r3".into()),
        }
    );
    assert_roundtrip::<Madc>("madc.cc.u32 %r0, %r1, %r2, %r3;");
}

#[test]
fn rejects_madc_with_invalid_type() {
    let err = parse_result::<Madc>("madc.lo.f32 %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_madc_with_missing_operand() {
    let err = parse_result::<Madc>("madc.u64 %rd0, %rd1, %rd2;")
        .expect_err("parse should fail when an operand is missing");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
