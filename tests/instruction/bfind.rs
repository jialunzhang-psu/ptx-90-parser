use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::bfind::{Bfind, DataType},
    },
};

#[test]
fn parses_plain_bfind() {
    assert_eq!(
        parse::<Bfind>("bfind.u32 %r1, %r2;"),
        Bfind::Plain {
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r1".into()),
            source: RegisterOperand::Single("%r2".into()),
        }
    );
}

#[test]
fn parses_shift_amount_bfind() {
    assert_eq!(
        parse::<Bfind>("bfind.shiftamt.s64 %rd3, %rd4;"),
        Bfind::ShiftAmount {
            data_type: DataType::S64,
            destination: RegisterOperand::Single("%rd3".into()),
            source: RegisterOperand::Single("%rd4".into()),
        }
    );
}

#[test]
fn rejects_invalid_bfind_data_type() {
    let err = parse_result::<Bfind>("bfind.f32 %r1, %r2;")
        .expect_err("should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_non_register_operand() {
    let err = parse_result::<Bfind>("bfind.shiftamt.u64 %r1, 1;")
        .expect_err("should fail when operand is not a register");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
