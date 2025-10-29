use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{RegisterOperand, VariableSymbol},
        instruction::suq::{Operand, Query, Suq},
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn var(name: &str) -> VariableSymbol {
    VariableSymbol(name.into())
}

#[test]
fn parses_surface_operand() {
    assert_eq!(
        parse::<Suq>("suq.width.b32 %r0, [surf_ref];"),
        Suq {
            query: Query::Width,
            destination: reg("%r0"),
            address: Operand::Surface(var("surf_ref")),
        }
    );
}

#[test]
fn parses_register_operand() {
    assert_eq!(
        parse::<Suq>("suq.array_size.b32 %rd2, [%rd3];"),
        Suq {
            query: Query::ArraySize,
            destination: reg("%rd2"),
            address: Operand::Register(reg("%rd3")),
        }
    );
}

#[test]
fn rejects_invalid_data_type() {
    let error =
        parse_result::<Suq>("suq.height.b64 %r0, [surf_ref];").expect_err("only .b32 is supported");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_unknown_query() {
    let error = parse_result::<Suq>("suq.size.b32 %r0, [surf_ref];")
        .expect_err("query modifier must be recognized");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
