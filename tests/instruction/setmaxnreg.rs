use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Immediate, Operand},
        instruction::setmaxnreg::{Action, Setmaxnreg},
    },
};

#[test]
fn parses_setmaxnreg_instruction() {
    assert_eq!(
        parse::<Setmaxnreg>("setmaxnreg.dec.sync.aligned.u32 64;"),
        Setmaxnreg {
            action: Action::Dec,
            sync: (),
            aligned: (),
            u32: (),
            imm_reg_count: Operand::Immediate(Immediate("64".into())),
        }
    );
    assert_roundtrip::<Setmaxnreg>("setmaxnreg.dec.sync.aligned.u32 64;");
}

#[test]
fn parses_setmaxnreg_inc() {
    assert_eq!(
        parse::<Setmaxnreg>("setmaxnreg.inc.sync.aligned.u32 128;"),
        Setmaxnreg {
            action: Action::Inc,
            sync: (),
            aligned: (),
            u32: (),
            imm_reg_count: Operand::Immediate(Immediate("128".into())),
        }
    );
    assert_roundtrip::<Setmaxnreg>("setmaxnreg.inc.sync.aligned.u32 128;");
}
