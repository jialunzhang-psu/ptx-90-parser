use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{common::Immediate, instruction::pmevent::Pmevent},
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Pmevent>(source);
}

#[test]
fn parses_single_variant() {
    let source = "pmevent 15;";
    assert_eq!(
        parse::<Pmevent>(source),
        Pmevent::Single {
            event: Immediate("15".into())
        }
    );
    assert_roundtrip(source);
}

#[test]
fn parses_mask_variant() {
    let source = "pmevent.mask 0x1;";
    assert_eq!(
        parse::<Pmevent>(source),
        Pmevent::Mask {
            mask: Immediate("0x1".into())
        }
    );
    assert_roundtrip(source);
}

#[test]
fn rejects_unknown_modifier() {
    let err = parse_result::<Pmevent>("pmevent.other 1;")
        .expect_err("parsing should fail with unsupported modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("pmevent.mask 0x1;");
}

#[test]
fn rejects_missing_immediate() {
    let err = parse_result::<Pmevent>("pmevent.mask;")
        .expect_err("parsing should fail with missing immediate operand");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("pmevent 15;");
}
