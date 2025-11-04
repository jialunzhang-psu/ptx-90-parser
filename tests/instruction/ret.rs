use crate::util::*;
use ptx_parser::{parser::ParseErrorKind, r#type::instruction::ret::Ret};

#[test]
fn parses_default_return_instruction() {
    let source = "ret;";
    assert_eq!(parse::<Ret>(source), Ret { uni: false });
    assert_roundtrip::<Ret>(source);
}

#[test]
fn parses_uniform_return_instruction() {
    let source = "ret.uni;";
    assert_eq!(parse::<Ret>(source), Ret { uni: true });
    assert_roundtrip::<Ret>(source);
}
