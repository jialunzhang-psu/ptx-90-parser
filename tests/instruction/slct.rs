use crate::util::{parse, parse_result};
use ptx_parser::r#type::common::RegisterOperand;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::instruction::slct::{DataType, Slct},
};

#[test]
fn parses_slct_s32_variant() {
    assert_eq!(
        parse::<Slct>("slct.u32.s32 %r0, %r1, %r2, %r3;"),
        Slct::S32 {
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            on_true: RegisterOperand::Single("%r1".into()),
            on_false: RegisterOperand::Single("%r2".into()),
            selector: RegisterOperand::Single("%r3".into()),
        }
    );
}

#[test]
fn parses_slct_f32_variant_with_ftz() {
    assert_eq!(
        parse::<Slct>("slct.ftz.u64.f32 %rd0, %rd1, %rd2, %f3;"),
        Slct::F32 {
            flush_to_zero: true,
            data_type: DataType::U64,
            destination: RegisterOperand::Single("%rd0".into()),
            on_true: RegisterOperand::Single("%rd1".into()),
            on_false: RegisterOperand::Single("%rd2".into()),
            selector: RegisterOperand::Single("%f3".into()),
        }
    );
}

#[test]
fn rejects_slct_ftz_with_s32_selector() {
    let err = parse_result::<Slct>("slct.ftz.u32.s32 %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail when .ftz is used with .s32 selector");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_slct_with_invalid_selector_type() {
    let err = parse_result::<Slct>("slct.u32.u32 %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail when selector type is not .s32 or .f32");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_slct_with_invalid_data_type() {
    let err = parse_result::<Slct>("slct.p32.s32 %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail when data type is unsupported");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
