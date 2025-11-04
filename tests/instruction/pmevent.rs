use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{common::{Immediate, Operand}, instruction::pmevent::{Pmevent1, Pmevent2}},
};

#[test]
fn parses_single_variant() {
    let source = "pmevent 15;";
    assert_eq!(
        parse::<Pmevent1>(source),
        Pmevent1 {
            a: Operand::Immediate(Immediate("15".into()))
        }
    );
    assert_roundtrip::<Pmevent1>(source);
}

#[test]
fn parses_mask_variant() {
    let source = "pmevent.mask 0x1;";
    assert_eq!(
        parse::<Pmevent2>(source),
        Pmevent2 {
            mask: (),
            a: Operand::Immediate(Immediate("0x1".into()))
        }
    );
    assert_roundtrip::<Pmevent2>(source);
}

#[test]
fn parses_single_variant_different_value() {
    let source = "pmevent 42;";
    assert_eq!(
        parse::<Pmevent1>(source),
        Pmevent1 {
            a: Operand::Immediate(Immediate("42".into()))
        }
    );
    assert_roundtrip::<Pmevent1>(source);
}
