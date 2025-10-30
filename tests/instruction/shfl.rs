use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Immediate, Operand, PredicateRegister, RegisterOperand},
    r#type::instruction::shfl::{DataType, Destination, Mode, Shfl},
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Shfl>(source);
}

#[test]
fn parses_shfl_without_predicate() {
    assert_eq!(
        parse::<Shfl>("shfl.up.b32 %r1, %r2, %r3, 0x1f;"),
        Shfl {
            mode: Mode::Up,
            data_type: DataType::B32,
            destination: Destination {
                register: RegisterOperand::Single("%r1".into()),
                predicate: None,
            },
            source: RegisterOperand::Single("%r2".into()),
            lane: Operand::Register(RegisterOperand::Single("%r3".into())),
            clamp: Operand::Immediate(Immediate("0x1f".into())),
        }
    );
    assert_roundtrip("shfl.up.b32 %r1, %r2, %r3, 0x1f;");
}

#[test]
fn parses_shfl_with_predicate() {
    assert_eq!(
        parse::<Shfl>("shfl.down.b32 %r4|%p1, %r5, 8, %r6;"),
        Shfl {
            mode: Mode::Down,
            data_type: DataType::B32,
            destination: Destination {
                register: RegisterOperand::Single("%r4".into()),
                predicate: Some(PredicateRegister("%p1".into())),
            },
            source: RegisterOperand::Single("%r5".into()),
            lane: Operand::Immediate(Immediate("8".into())),
            clamp: Operand::Register(RegisterOperand::Single("%r6".into())),
        }
    );
    assert_roundtrip("shfl.down.b32 %r4|%p1, %r5, 8, %r6;");
}

#[test]
fn rejects_shfl_with_invalid_mode() {
    let err =
        parse_result::<Shfl>("shfl.sideways.b32 %r1, %r2, %r3, 0;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("shfl.bfly.b32 %r1, %r2, %r3, 0x1f;");
}

#[test]
fn rejects_shfl_with_invalid_data_type() {
    let err = parse_result::<Shfl>("shfl.up.u32 %r1, %r2, %r3, 0;").expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("shfl.idx.b32 %r1, %r2, %r3, 0;");
}
