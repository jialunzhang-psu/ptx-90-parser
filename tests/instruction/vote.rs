use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{PredicateRegister, RegisterOperand},
        instruction::vote::{Ballot, Mode, Predicate, PredicateOperand, Vote},
    },
};

#[test]
fn parses_vote_predicate_instruction() {
    assert_eq!(
        parse::<Vote>("vote.all.pred %p1, %p2;"),
        Vote::Predicate(Predicate {
            mode: Mode::All,
            destination: PredicateRegister("%p1".into()),
            source: PredicateOperand {
                register: PredicateRegister("%p2".into()),
                negated: false,
            },
        })
    );
}

#[test]
fn parses_vote_ballot_instruction() {
    assert_eq!(
        parse::<Vote>("vote.ballot.b32 %r2, !%p3;"),
        Vote::Ballot(Ballot {
            destination: RegisterOperand::Single("%r2".into()),
            source: PredicateOperand {
                register: PredicateRegister("%p3".into()),
                negated: true,
            },
        })
    );
}

#[test]
fn rejects_vote_with_invalid_mode() {
    let error =
        parse_result::<Vote>("vote.foo.pred %p1, %p2;").expect_err("parse should fail for mode");

    match error.kind {
        ParseErrorKind::UnexpectedToken { expected, found } => {
            assert_eq!(
                expected,
                vec![
                    ".all".to_string(),
                    ".any".to_string(),
                    ".uni".to_string(),
                    ".ballot".to_string()
                ]
            );
            assert_eq!(found, ".foo");
        }
        other => panic!("expected UnexpectedToken error, got {other:?}"),
    }
}

#[test]
fn rejects_vote_ballot_missing_type() {
    let error = parse_result::<Vote>("vote.ballot %r1, %p2;")
        .expect_err("parse should fail for missing data type");

    match error.kind {
        ParseErrorKind::UnexpectedToken { expected, found } => {
            assert_eq!(expected, vec!["Directive".to_string()]);
            assert_eq!(found, r#"Register("%r1")"#.to_string());
        }
        other => panic!("expected UnexpectedToken error, got {other:?}"),
    }
}
