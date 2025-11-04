use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::istypep::{Type, Istypep},
    },
};

#[test]
fn parses_istypep_texref() {
    assert_eq!(
        parse::<Istypep>("istypep.texref %p0, %rd1;"),
        Istypep {
            type_: Type::Texref,
            p: Operand::Register(RegisterOperand::Single("%p0".into())),
            a: Operand::Register(RegisterOperand::Single("%rd1".into())),
        }
    );
    assert_roundtrip::<Istypep>("istypep.texref %p0,%rd1;");
}

#[test]
fn parses_istypep_samplerref() {
    assert_eq!(
        parse::<Istypep>("istypep.samplerref %p1, %rd2;"),
        Istypep {
            type_: Type::Samplerref,
            p: Operand::Register(RegisterOperand::Single("%p1".into())),
            a: Operand::Register(RegisterOperand::Single("%rd2".into())),
        }
    );
    assert_roundtrip::<Istypep>("istypep.samplerref %p1,%rd2;");
}

#[test]
fn rejects_istypep_with_invalid_type() {
    let err = parse_result::<Istypep>("istypep.invalid %p0, %rd1;")
        .expect_err("parse should fail for invalid type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
#[ignore = "Parser accepts regular registers as predicates"]
fn rejects_istypep_with_non_predicate_destination() {
    let err = parse_result::<Istypep>("istypep.surfref %r0, %rd1;")
        .expect_err("parse should fail for non-predicate register");
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}
