mod util;
use ptx_parser::r#type::{
    common::{AddressSpace, DataLinkage, DataType},
    variable::{
        GlobalInitializer, InitializerValue, ModuleVariableDirective, NumericLiteral,
        VariableDirective, VariableModifier,
    },
};
use util::*;

fn unwrap_numeric(value: &InitializerValue) -> &NumericLiteral {
    match value {
        InitializerValue::Numeric(literal) => literal,
        other => panic!("expected numeric literal, got {:?}", other),
    }
}

#[test]
fn parses_module_global_with_initializer() {
    let directive =
        parse::<ModuleVariableDirective>(".visible .global .align 1 .b8 data[2] = {1, 2};");
    assert_roundtrip::<ModuleVariableDirective>(".visible .global .align 1 .b8 data[2] = {1, 2};");

    let ModuleVariableDirective::Global(inner) = directive else {
        panic!("expected global module directive");
    };

    assert_eq!(inner.address_space, Some(AddressSpace::Global));
    assert_eq!(
        inner.modifiers,
        vec![
            VariableModifier::Linkage(DataLinkage::Visible),
            VariableModifier::Alignment(1),
        ]
    );
    assert_eq!(inner.ty, Some(DataType::B8));
    assert_eq!(inner.name, "data");
    assert_eq!(inner.array, vec![Some(2)]);

    let Some(GlobalInitializer::Aggregate(values)) = &inner.initializer else {
        panic!("expected aggregate initializer");
    };
    assert_eq!(values.len(), 2);
    let first = unwrap_numeric(match &values[0] {
        GlobalInitializer::Scalar(value) => value,
        GlobalInitializer::Aggregate(_) => panic!("unexpected aggregate"),
    });
    assert_eq!(first, &NumericLiteral::Unsigned(1));
}

#[test]
fn parses_local_variable_directive() {
    let directive = parse::<VariableDirective>(".local .align 8 .b8 __local_depot0[8];");
    assert_roundtrip::<VariableDirective>(".local .align 8 .b8 __local_depot0[8];");
    assert_eq!(directive.address_space, Some(AddressSpace::Local));
    assert_eq!(directive.modifiers, vec![VariableModifier::Alignment(8)]);
    assert_eq!(directive.ty, Some(DataType::B8));
    assert_eq!(directive.name, "__local_depot0");
    assert_eq!(directive.array, vec![Some(8)]);
    assert!(directive.initializer.is_none());
}

#[test]
fn parses_param_without_initializer() {
    let directive = parse::<VariableDirective>(".param .b64 param0;");
    assert_roundtrip::<VariableDirective>(".param .b64 param0;");
    assert_eq!(directive.address_space, Some(AddressSpace::Param));
    assert!(directive.modifiers.is_empty());
    assert_eq!(directive.ty, Some(DataType::B64));
    assert_eq!(directive.name, "param0");
    assert!(directive.array.is_empty());
    assert!(directive.initializer.is_none());
}
