use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Operand, RegisterOperand},
        instruction::mapa::{Mapa, Space, Type},
    },
};

#[test]
#[ignore = "mapa needs parser support for scope modifiers like shared::cluster"]
fn parses_with_shared_cluster() {
    assert_eq!(
        parse::<Mapa>("mapa.shared::cluster.u32 %r0, %r1, %r2;"),
        Mapa {
            space: Some(Space::SharedCluster),
            type_: Type::U32,
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
    assert_roundtrip::<Mapa>("mapa.shared::cluster.u32 %r0, %r1, %r2;");
}

#[test]
#[ignore = "mapa needs parser support for scope modifiers like shared::cluster"]
fn parses_with_u64_type() {
    assert_eq!(
        parse::<Mapa>("mapa.shared::cluster.u64 %rd0, %rd1, %rd2;"),
        Mapa {
            space: Some(Space::SharedCluster),
            type_: Type::U64,
            d: Operand::Register(RegisterOperand::Single("%rd0".into())),
            a: Operand::Register(RegisterOperand::Single("%rd1".into())),
            b: Operand::Register(RegisterOperand::Single("%rd2".into())),
        }
    );
    assert_roundtrip::<Mapa>("mapa.shared::cluster.u64 %rd0, %rd1, %rd2;");
}
