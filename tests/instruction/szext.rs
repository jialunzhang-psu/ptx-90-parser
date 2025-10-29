use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::szext::{DataType, Mode, Szext},
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

#[test]
fn parses_clamp_u32() {
    assert_eq!(
        parse::<Szext>("szext.clamp.u32 %r1, %r2, %r3;"),
        Szext {
            mode: Mode::Clamp,
            data_type: DataType::U32,
            destination: reg("%r1"),
            a: reg("%r2"),
            b: reg("%r3"),
        }
    );
}

#[test]
fn parses_wrap_s32() {
    assert_eq!(
        parse::<Szext>("szext.wrap.s32 %rd4, %rd5, %rd6;"),
        Szext {
            mode: Mode::Wrap,
            data_type: DataType::S32,
            destination: reg("%rd4"),
            a: reg("%rd5"),
            b: reg("%rd6"),
        }
    );
}

#[test]
fn rejects_unknown_mode() {
    let error = parse_result::<Szext>("szext.bounce.u32 %r1, %r2, %r3;")
        .expect_err("mode must be clamp or wrap");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_unknown_data_type() {
    let error = parse_result::<Szext>("szext.wrap.s64 %r1, %r2, %r3;")
        .expect_err("data type must be s32 or u32");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
