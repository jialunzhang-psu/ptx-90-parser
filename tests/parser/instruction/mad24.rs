use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::mad24::{DataType, Mad24, Mode},
    },
};

#[test]
fn parses_mad24_lo_with_unsigned_type() {
    assert_eq!(
        parse::<Mad24>("mad24.lo.u32 %r0, %r1, %r2, %r3;"),
        Mad24::Mode {
            mode: Mode::Lo,
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
            c: RegisterOperand::Single("%r3".into()),
        }
    );
}

#[test]
fn parses_mad24_hi_sat_s32() {
    assert_eq!(
        parse::<Mad24>("mad24.hi.sat.s32 %r4, %r5, %r6, %r7;"),
        Mad24::HiSatS32 {
            destination: RegisterOperand::Single("%r4".into()),
            a: RegisterOperand::Single("%r5".into()),
            b: RegisterOperand::Single("%r6".into()),
            c: RegisterOperand::Single("%r7".into()),
        }
    );
}

#[test]
fn rejects_mad24_with_invalid_mode() {
    let err = parse_result::<Mad24>("mad24.wide.u32 %r0, %r1, %r2, %r3;")
        .expect_err("parser should reject unknown mode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_mad24_hi_sat_with_non_s32_type() {
    let err = parse_result::<Mad24>("mad24.hi.sat.u32 %r0, %r1, %r2, %r3;")
        .expect_err("parser should reject non-.s32 type for mad24.hi.sat");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
