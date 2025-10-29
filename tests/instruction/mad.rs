use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::mad::{DataType, Mad, Mode},
    },
};

#[test]
fn parses_mad_lo_with_signed_type() {
    assert_eq!(
        parse::<Mad>("mad.lo.s32 %r0, %r1, %r2, %r3;"),
        Mad::Mode {
            mode: Mode::Lo,
            data_type: DataType::S32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
            c: RegisterOperand::Single("%r3".into()),
        }
    );
}

#[test]
fn parses_mad_hi_sat_s32() {
    assert_eq!(
        parse::<Mad>("mad.hi.sat.s32 %r4, %r5, %r6, %r7;"),
        Mad::HiSatS32 {
            destination: RegisterOperand::Single("%r4".into()),
            a: RegisterOperand::Single("%r5".into()),
            b: RegisterOperand::Single("%r6".into()),
            c: RegisterOperand::Single("%r7".into()),
        }
    );
}

#[test]
fn rejects_mad_with_invalid_mode() {
    let err = parse_result::<Mad>("mad.fast.s32 %r0, %r1, %r2, %r3;")
        .expect_err("parser should reject unknown mode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_mad_hi_sat_with_non_s32_type() {
    let err = parse_result::<Mad>("mad.hi.sat.u32 %r0, %r1, %r2, %r3;")
        .expect_err("parser should reject non-.s32 type for mad.hi.sat");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
