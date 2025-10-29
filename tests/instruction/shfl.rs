use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::*,
    r#type::instruction::shfl::{
        DataType as ShflDataType, Destination as ShflDestination, Mode as ShflMode, Shfl,
    },
};

#[test]
fn parses_shfl_without_predicate() {
    assert_eq!(
        parse::<Shfl>("shfl.up.b32 %r1, %r2, %r3, 0x1f;"),
        Shfl {
            mode: ShflMode::Up,
            data_type: ShflDataType::B32,
            destination: ShflDestination {
                register: RegisterOperand::Single("%r1".into()),
                predicate: None,
            },
            source: RegisterOperand::Single("%r2".into()),
            lane: Operand::Register(RegisterOperand::Single("%r3".into())),
            clamp: Operand::Immediate(Immediate("0x1f".into())),
        }
    );
}

#[test]
fn parses_shfl_with_predicate() {
    assert_eq!(
        parse::<Shfl>("shfl.down.b32 %r4|%p1, %r5, 8, %r6;"),
        Shfl {
            mode: ShflMode::Down,
            data_type: ShflDataType::B32,
            destination: ShflDestination {
                register: RegisterOperand::Single("%r4".into()),
                predicate: Some(PredicateRegister("%p1".into())),
            },
            source: RegisterOperand::Single("%r5".into()),
            lane: Operand::Immediate(Immediate("8".into())),
            clamp: Operand::Register(RegisterOperand::Single("%r6".into())),
        }
    );
}

#[test]
fn rejects_shfl_with_invalid_mode() {
    let err =
        parse_result::<Shfl>("shfl.sideways.b32 %r1, %r2, %r3, 0;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_shfl_with_invalid_data_type() {
    let err = parse_result::<Shfl>("shfl.up.u32 %r1, %r2, %r3, 0;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
