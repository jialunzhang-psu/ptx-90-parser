use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{common::Immediate, instruction::pmevent::Pmevent},
};

#[test]
fn parses_single_variant() {
    assert_eq!(
        parse::<Pmevent>("pmevent 15;"),
        Pmevent::Single {
            event: Immediate("15".into())
        }
    );
}

#[test]
fn parses_mask_variant() {
    assert_eq!(
        parse::<Pmevent>("pmevent.mask 0x1;"),
        Pmevent::Mask {
            mask: Immediate("0x1".into())
        }
    );
}

#[test]
fn rejects_unknown_modifier() {
    let err = parse_result::<Pmevent>("pmevent.other 1;")
        .expect_err("parsing should fail with unsupported modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_immediate() {
    let err = parse_result::<Pmevent>("pmevent.mask;")
        .expect_err("parsing should fail with missing immediate operand");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
