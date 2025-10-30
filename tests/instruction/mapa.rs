use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{Immediate, Operand, RegisterOperand, VariableSymbol},
        instruction::mapa::{DataType, Mapa},
    },
};

#[test]
fn parses_generic_mapa() {
    assert_eq!(
        parse::<Mapa>("mapa.u32 %r0, %r1, %r2;"),
        Mapa::Generic {
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            address: RegisterOperand::Single("%r1".into()),
            cta: Operand::Register(RegisterOperand::Single("%r2".into())),
        }
    );
    assert_roundtrip::<Mapa>("mapa.u32 %r0, %r1, %r2;");
}

#[test]
fn parses_shared_cluster_register_form() {
    assert_eq!(
        parse::<Mapa>("mapa.shared::cluster.u64 %rd0, %rd1, 4;"),
        Mapa::SharedRegister {
            data_type: DataType::U64,
            destination: RegisterOperand::Single("%rd0".into()),
            address: RegisterOperand::Single("%rd1".into()),
            cta: Operand::Immediate(Immediate("4".into())),
        }
    );
    assert_roundtrip::<Mapa>("mapa.shared::cluster.u64 %rd0, %rd1, 4;");
}

#[test]
fn parses_shared_cluster_variable_forms() {
    assert_eq!(
        parse::<Mapa>("mapa.shared::cluster.u32 %r3, shared_var, %r4;"),
        Mapa::SharedVariable {
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r3".into()),
            variable: VariableSymbol("shared_var".into()),
            cta: Operand::Register(RegisterOperand::Single("%r4".into())),
        }
    );
    assert_roundtrip::<Mapa>("mapa.shared::cluster.u32 %r3, shared_var, %r4;");

    assert_eq!(
        parse::<Mapa>("mapa.shared::cluster.u32 %r5, shared_var + 16, %r6;"),
        Mapa::SharedVariableWithImmediate {
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r5".into()),
            variable: VariableSymbol("shared_var".into()),
            immediate: Immediate("16".into()),
            cta: Operand::Register(RegisterOperand::Single("%r6".into())),
        }
    );
    assert_roundtrip::<Mapa>("mapa.shared::cluster.u32 %r5, shared_var + 16, %r6;");
}

#[test]
fn rejects_mapa_with_invalid_space() {
    let error = parse_result::<Mapa>("mapa.shared::cta.u32 %r0, %r1, %r2;")
        .expect_err("parser should reject unsupported space");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_mapa_generic_with_variable_source() {
    let error = parse_result::<Mapa>("mapa.u32 %r0, shared_var, %r1;")
        .expect_err("generic mapa should require register source");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
