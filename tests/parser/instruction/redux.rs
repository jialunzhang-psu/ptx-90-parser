use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Immediate, Operand, RegisterOperand},
        instruction::redux::{
            Bitwise, BitwiseOperator, DataType, Float, FloatOperator, Integer, IntegerOperator,
            Redux,
        },
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn immediate(value: &str) -> Operand {
    Operand::Immediate(Immediate(value.into()))
}

#[test]
fn parses_integer_add() {
    assert_eq!(
        parse::<Redux>("redux.sync.add.s32 %r0, %r1, 0xff;"),
        Redux::Integer(Integer {
            operator: IntegerOperator::Add,
            data_type: DataType::S32,
            destination: reg("%r0"),
            source: reg("%r1"),
            member_mask: immediate("0xff"),
        })
    );
}

#[test]
fn parses_bitwise_and() {
    assert_eq!(
        parse::<Redux>("redux.sync.and.b32 %r2, %r3, %r4;"),
        Redux::Bitwise(Bitwise {
            operator: BitwiseOperator::And,
            destination: reg("%r2"),
            source: reg("%r3"),
            member_mask: Operand::Register(reg("%r4")),
        })
    );
}

#[test]
fn parses_float_min_with_modifiers() {
    assert_eq!(
        parse::<Redux>("redux.sync.min.abs.NaN.f32 %f0, %f1, %r5;"),
        Redux::Float(Float {
            operator: FloatOperator::Min,
            absolute: true,
            propagate_nan: true,
            destination: reg("%f0"),
            source: reg("%f1"),
            member_mask: Operand::Register(reg("%r5")),
        })
    );
}

#[test]
fn rejects_missing_sync_modifier() {
    let error = parse_result::<Redux>("redux.min.s32 %r0, %r1, %r2;")
        .expect_err("redux should require the .sync modifier");

    match error.kind {
        ParseErrorKind::UnexpectedToken { expected, found } => {
            assert_eq!(expected, vec![".sync".to_string()]);
            assert_eq!(found, ".min".to_string());
        }
        other => panic!("expected UnexpectedToken error, got {other:?}"),
    }
}

#[test]
fn rejects_unknown_operator() {
    let error = parse_result::<Redux>("redux.sync.foo.b32 %r0, %r1, %r2;")
        .expect_err("redux should reject unknown operators");

    match error.kind {
        ParseErrorKind::UnexpectedToken { expected, found } => {
            assert_eq!(
                expected,
                vec![
                    ".add".to_string(),
                    ".min".to_string(),
                    ".max".to_string(),
                    ".and".to_string(),
                    ".or".to_string(),
                    ".xor".to_string(),
                ]
            );
            assert_eq!(found, ".foo".to_string());
        }
        other => panic!("expected UnexpectedToken error, got {other:?}"),
    }
}

#[test]
fn rejects_duplicate_abs_modifier() {
    let error = parse_result::<Redux>("redux.sync.max.abs.abs.f32 %f0, %f1, %r2;")
        .expect_err("redux should reject duplicate .abs modifiers");

    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
