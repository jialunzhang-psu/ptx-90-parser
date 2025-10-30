use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{PredicateRegister, RegisterOperand},
        instruction::selp::{DataType, Selp},
    },
};

#[test]
fn parses_selp_instruction() {
    assert_eq!(
        parse::<Selp>("selp.u32 %r0, %r1, %r2, %p3;"),
        Selp {
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            true_value: RegisterOperand::Single("%r1".into()),
            false_value: RegisterOperand::Single("%r2".into()),
            predicate: PredicateRegister("%p3".into()),
        }
    );
    assert_roundtrip::<Selp>("selp.u32 %r0, %r1, %r2, %p3;");
}

#[test]
fn rejects_selp_with_invalid_data_type() {
    let err = parse_result::<Selp>("selp.p32 %r0, %r1, %r2, %p3;")
        .expect_err("parse should fail when data type is unsupported");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_selp_with_non_predicate_operand() {
    let err = parse_result::<Selp>("selp.u32 %r0, %r1, %r2, %r3;")
        .expect_err("parse should fail when predicate operand is not a predicate register");
    assert!(matches!(err.kind, ParseErrorKind::InvalidLiteral(_)));
}
