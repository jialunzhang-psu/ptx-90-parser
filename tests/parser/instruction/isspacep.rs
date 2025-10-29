use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{PredicateRegister, RegisterOperand},
        instruction::isspacep::{Isspacep, Space},
    },
};

#[test]
fn parses_isspacep_global() {
    assert_eq!(
        parse::<Isspacep>("isspacep.global %p1, %rd2;"),
        Isspacep {
            space: Space::Global,
            predicate: PredicateRegister("%p1".into()),
            address: RegisterOperand::Single("%rd2".into()),
        }
    );
}

#[test]
fn parses_isspacep_shared_cta() {
    assert_eq!(
        parse::<Isspacep>("isspacep.shared::cta %p0, %rd3;"),
        Isspacep {
            space: Space::SharedCta,
            predicate: PredicateRegister("%p0".into()),
            address: RegisterOperand::Single("%rd3".into()),
        }
    );
}

#[test]
fn rejects_isspacep_with_invalid_space() {
    let err = parse_result::<Isspacep>("isspacep.invalid %p0, %rd1;")
        .expect_err("parse should fail for invalid space");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_isspacep_with_non_predicate_destination() {
    let err = parse_result::<Isspacep>("isspacep.const %r0, %rd1;")
        .expect_err("parse should fail for non-predicate destination");
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}
