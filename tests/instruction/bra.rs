use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{common::{Operand, Immediate}, instruction::bra::Bra1},
};

#[test]
#[ignore = "bra needs label operand support in parser"]
fn parses_simple_bra_instruction() {
    assert_eq!(
        parse::<Bra1>("bra target;"),
        Bra1 {
            uni: false,
            tgt: Operand::Immediate(Immediate("target".to_string())),
        }
    );
    assert_roundtrip::<Bra1>("bra target;");
}

#[test]
#[ignore = "bra needs label operand support in parser"]
fn parses_uniform_branch_instruction() {
    assert_eq!(
        parse::<Bra1>("bra.uni L0;"),
        Bra1 {
            uni: true,
            tgt: Operand::Immediate(Immediate("L0".to_string())),
        }
    );
    assert_roundtrip::<Bra1>("bra.uni L0;");
}
