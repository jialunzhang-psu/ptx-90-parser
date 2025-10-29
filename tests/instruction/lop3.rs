use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Immediate, PredicateRegister, RegisterOperand},
        instruction::lop3,
    },
};

#[test]
fn parses_plain_lop3() {
    assert_eq!(
        parse::<lop3::Lop3>("lop3.b32 %r0, %r1, %r2, %r3, 0xf0;"),
        lop3::Lop3::Plain(lop3::Plain {
            data_type: lop3::DataType::B32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
            c: RegisterOperand::Single("%r3".into()),
            lut: Immediate("0xf0".into()),
        })
    );
}

#[test]
fn parses_boolean_lop3() {
    assert_eq!(
        parse::<lop3::Lop3>("lop3.or.b32 %r4|%p0, %r1, %r2, %r3, 0x1e, %p1;"),
        lop3::Lop3::Boolean(lop3::Boolean {
            operator: lop3::BoolOp::Or,
            data_type: lop3::DataType::B32,
            destination: lop3::Destination::Register(RegisterOperand::Single("%r4".into())),
            predicate: PredicateRegister("%p0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
            c: RegisterOperand::Single("%r3".into()),
            lut: Immediate("0x1e".into()),
            predicate_input: PredicateRegister("%p1".into()),
        })
    );
}

#[test]
fn rejects_invalid_bool_op() {
    let err = parse_result::<lop3::Lop3>("lop3.xor.b32 %r0, %r1, %r2, %r3, 0xf0;")
        .expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_data_type() {
    let err = parse_result::<lop3::Lop3>("lop3.and.b16 %r0|%p0, %r1, %r2, %r3, 0xf0, %p1;")
        .expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
