use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{PredicateRegister, RegisterOperand},
        instruction::testp::{DataType, PredicateTest, Testp},
    },
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Testp>(source);
}

#[test]
fn parses_testp_finite_f32() {
    assert_eq!(
        parse::<Testp>("testp.finite.f32 %p1, %f2;"),
        Testp {
            test: PredicateTest::Finite,
            data_type: DataType::F32,
            destination: PredicateRegister("%p1".into()),
            source: RegisterOperand::Single("%f2".into()),
        }
    );
    assert_roundtrip("testp.finite.f32 %p1, %f2;");
}

#[test]
fn parses_testp_notanumber_f64() {
    assert_eq!(
        parse::<Testp>("testp.notanumber.f64 %p0, {%f4, %f5};"),
        Testp {
            test: PredicateTest::NotANumber,
            data_type: DataType::F64,
            destination: PredicateRegister("%p0".into()),
            source: RegisterOperand::Vector2(["%f4".into(), "%f5".into()]),
        }
    );
    assert_roundtrip("testp.notanumber.f64 %p0, {%f4, %f5};");
}

#[test]
fn rejects_testp_with_invalid_test_modifier() {
    let err = parse_result::<Testp>("testp.invalid.f32 %p0, %f1;")
        .expect_err("parse should fail for invalid predicate test modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_testp_with_non_predicate_destination() {
    let err = parse_result::<Testp>("testp.finite.f32 %r1, %f2;")
        .expect_err("parse should fail for non-predicate destination");
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}
