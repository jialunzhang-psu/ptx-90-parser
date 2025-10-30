use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::Immediate,
        instruction::setmaxnreg::{Action, Setmaxnreg},
    },
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Setmaxnreg>(source);
}

#[test]
fn parses_setmaxnreg_instruction() {
    assert_eq!(
        parse::<Setmaxnreg>("setmaxnreg.dec.sync.aligned.u32 64;"),
        Setmaxnreg {
            action: Action::Dec,
            register_count: Immediate("64".into()),
        }
    );
    assert_roundtrip("setmaxnreg.dec.sync.aligned.u32 64;");
}

#[test]
fn rejects_setmaxnreg_missing_sync_modifier() {
    assert_roundtrip("setmaxnreg.inc.sync.aligned.u32 128;");
    let err = parse_result::<Setmaxnreg>("setmaxnreg.inc.aligned.u32 128;")
        .expect_err("parse should fail when .sync modifier is missing");

    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_setmaxnreg_with_invalid_action() {
    assert_roundtrip("setmaxnreg.dec.sync.aligned.u32 64;");
    let err = parse_result::<Setmaxnreg>("setmaxnreg.foo.sync.aligned.u32 64;")
        .expect_err("parse should fail for unknown action qualifier");

    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
