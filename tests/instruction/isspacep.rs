use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::isspacep::{Isspacep, Space},
    },
};

#[test]
fn parses_isspacep_global() {
    assert_eq!(
        parse::<Isspacep>("isspacep.global %p1, %rd2;"),
        Isspacep {
            space: Space::Global,
            p: Operand::Register(RegisterOperand::Single("%p1".into())),
            a: Operand::Register(RegisterOperand::Single("%rd2".into())),
        }
    );
    assert_roundtrip::<Isspacep>("isspacep.global %p1, %rd2;");
}

#[test]
#[ignore = "Parser doesn't support :: syntax yet"]
fn parses_isspacep_shared_cta() {
    assert_eq!(
        parse::<Isspacep>("isspacep.shared::cta %p0, %rd3;"),
        Isspacep {
            space: Space::SharedCta,
            p: Operand::Register(RegisterOperand::Single("%p0".into())),
            a: Operand::Register(RegisterOperand::Single("%rd3".into())),
        }
    );
    assert_roundtrip::<Isspacep>("isspacep.shared::cta %p0, %rd3;");
}

#[test]
fn rejects_isspacep_with_invalid_space() {
    let err = parse_result::<Isspacep>("isspacep.invalid %p0, %rd1;")
        .expect_err("parse should fail for invalid space");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
#[ignore = "Parser accepts regular registers as predicates"]
fn rejects_isspacep_with_non_predicate_destination() {
    let err = parse_result::<Isspacep>("isspacep.const %r0, %rd1;")
        .expect_err("parse should fail for non-predicate destination");
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}
