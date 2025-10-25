use ptx_parser::{
    GlobalInitializer, InitializerValue, ModuleDirective, ModuleVariableDirective, NumericLiteral,
    PtxParseError, ScalarType, VariableDirective, parse_module_directive,
};

#[test]
fn parses_const_scalar_initializer() {
    let variable = expect_variable(".const .u32 foo = 42;", VariableKind::Const);

    let value = match variable.initializer.as_ref().expect("expected initializer") {
        GlobalInitializer::Scalar(inner) => inner,
        other => panic!("expected scalar initializer, got {:?}", other),
    };
    assert_eq!(initializer_numeric_value(value), 42);
}

#[test]
fn parses_global_array_numeric_initializer() {
    let variable = expect_variable(".global .u32 bar[] = { 2, 3, 5 };", VariableKind::Global);

    let initializer = variable.initializer.as_ref().expect("expected initializer");
    assert_eq!(collect_numeric_array(initializer), vec![2, 3, 5]);
}

#[test]
fn parses_global_scalar_without_initializer() {
    let variable = expect_variable(".global .u32 loc;", VariableKind::Global);
    assert!(variable.initializer.is_none());
}

#[test]
fn parses_global_symbol_reference() {
    let variable = expect_variable(".global .u32 p1 = foo;", VariableKind::Global);
    let initializer = variable.initializer.as_ref().expect("expected initializer");
    assert_eq!(collect_symbol(initializer), "foo");
}

#[test]
fn parses_global_generic_symbol_reference() {
    let variable = expect_variable(".global .u32 p2 = generic(foo);", VariableKind::Global);
    let initializer = variable.initializer.as_ref().expect("expected initializer");
    assert_eq!(collect_symbol(initializer), "generic(foo)");
}

#[test]
fn parses_global_generic_pointer_array() {
    let variable = expect_variable(
        ".global .u32 parr[] = { generic(bar), generic(bar)+4, generic(bar)+8 };",
        VariableKind::Global,
    );
    let initializer = variable.initializer.as_ref().expect("expected initializer");
    assert_eq!(
        collect_symbol_array(initializer),
        vec!["generic(bar)", "generic(bar)+4", "generic(bar)+8"]
    );
}

#[test]
fn parses_global_mask_expressions() {
    let variable = expect_variable(
        ".global .u8 addr[] = {0xff(foo), 0xff00(foo), 0xff0000(foo)};",
        VariableKind::Global,
    );
    let initializer = variable.initializer.as_ref().expect("expected initializer");
    assert_eq!(
        collect_symbol_array(initializer),
        vec!["0xff(foo)", "0xff00(foo)", "0xff0000(foo)"]
    );
}

#[test]
fn parses_global_mask_with_offset() {
    let variable = expect_variable(
        ".global .u8 addr2[] = {0xff(foo+4), 0xff00(foo+4), 0xff0000(foo+4)};",
        VariableKind::Global,
    );
    let initializer = variable.initializer.as_ref().expect("expected initializer");
    assert_eq!(
        collect_symbol_array(initializer),
        vec!["0xff(foo+4)", "0xff00(foo+4)", "0xff0000(foo+4)"]
    );
}

#[test]
fn parses_global_mask_with_generic_expressions() {
    let variable = expect_variable(
        ".global .u8 addr3[] = {0xff(generic(foo)), 0xff00(generic(foo))};",
        VariableKind::Global,
    );
    let initializer = variable.initializer.as_ref().expect("expected initializer");
    assert_eq!(
        collect_symbol_array(initializer),
        vec!["0xff(generic(foo))", "0xff00(generic(foo))"]
    );
}

#[test]
fn parses_global_mask_with_generic_offset_expressions() {
    let variable = expect_variable(
        ".global .u8 addr4[] = {0xff(generic(foo)+4), 0xff00(generic(foo)+4)};",
        VariableKind::Global,
    );
    let initializer = variable.initializer.as_ref().expect("expected initializer");
    assert_eq!(
        collect_symbol_array(initializer),
        vec!["0xff(generic(foo)+4)", "0xff00(generic(foo)+4)"]
    );
}

#[test]
fn parses_global_mask_with_const_expression() {
    let variable = expect_variable(
        ".global .u8 addr5[] = { 0xFF(1000 + 546), 0xFF00(131187) };",
        VariableKind::Global,
    );
    let initializer = variable.initializer.as_ref().expect("expected initializer");
    assert_eq!(
        collect_symbol_array(initializer),
        vec!["0xFF(1000 + 546)", "0xFF00(131187)"]
    );
}

#[test]
fn parses_multi_dimensional_global_array() {
    let variable = expect_variable(
        ".global .s32 offset[][2] = { {-1, 0}, {0, -1}, {1, 0}, {0, 1} };",
        VariableKind::Global,
    );

    let array = variable
        .array
        .as_ref()
        .expect("expected array dimensions on global");
    assert_eq!(array.dimensions, vec![None, Some(2)]);

    let initializer = variable
        .initializer
        .as_ref()
        .expect("expected initializer on global");
    let rows = match initializer {
        GlobalInitializer::Aggregate(rows) => rows,
        other => panic!("expected aggregate initializer, got {:?}", other),
    };
    assert_eq!(rows.len(), 4);

    let expected = [[-1, 0], [0, -1], [1, 0], [0, 1]];

    for (row, expected_values) in rows.iter().zip(expected.iter()) {
        let cols = match row {
            GlobalInitializer::Aggregate(cols) => cols,
            other => panic!("expected aggregate row, got {:?}", other),
        };
        assert_eq!(cols.len(), 2);
        for (value, expected_value) in cols.iter().zip(expected_values.iter()) {
            let numeric = match value {
                GlobalInitializer::Scalar(inner) => inner,
                other => panic!("expected scalar initializer, got {:?}", other),
            };
            let actual = initializer_numeric_value(numeric);
            assert_eq!(actual, *expected_value);
        }
    }
}

#[test]
fn parses_const_float_array_initializer() {
    let variable = expect_variable(".const .f32 bias[] = {-1.0, 1.0};", VariableKind::Const);
    let initializer = variable.initializer.as_ref().expect("expected initializer");
    match initializer {
        GlobalInitializer::Aggregate(values) => {
            assert_eq!(values.len(), 2);
            for (value, expected) in values.iter().zip([-1.0_f64, 1.0_f64]) {
                let numeric = match value {
                    GlobalInitializer::Scalar(InitializerValue::Numeric(
                        NumericLiteral::Float64(bits),
                    )) => f64::from_bits(*bits),
                    other => panic!("expected float literal, got {:?}", other),
                };
                assert!((numeric - expected).abs() < f64::EPSILON);
            }
        }
        other => panic!("expected aggregate float initializer, got {:?}", other),
    }
}

#[test]
fn parses_global_byte_array_initializer() {
    let variable = expect_variable(".global .u8 bg[4] = {0, 0, 0, 0};", VariableKind::Global);
    let initializer = variable.initializer.as_ref().expect("expected initializer");
    assert_eq!(collect_numeric_array(initializer), vec![0, 0, 0, 0]);
}

#[test]
fn parses_shared_variable() {
    let variable = expect_variable(".shared .align 16 .u8 s0[4];", VariableKind::Shared);
    assert_eq!(variable.name, "s0");
    assert_eq!(variable.alignment, Some(16));
}

#[test]
fn parses_tex_variable() {
    let variable = expect_variable(".tex .sampler foo;", VariableKind::Tex);
    assert_eq!(variable.name, "foo");
}

#[test]
fn parses_global_texref_type() {
    let variable = expect_variable(".global .texref tex_handle;", VariableKind::Global);
    assert_eq!(variable.ty, Some(ScalarType::TexRef));
}

#[test]
fn parses_global_samplerref_type() {
    let variable = expect_variable(".global .samplerref sampler_handle;", VariableKind::Global);
    assert_eq!(variable.ty, Some(ScalarType::SamplerRef));
}

#[test]
fn parses_global_surfref_type() {
    let variable = expect_variable(".global .surfref surface_handle;", VariableKind::Global);
    assert_eq!(variable.ty, Some(ScalarType::SurfRef));
}

#[test]
fn rejects_reg_scalar_directive() {
    assert!(parse_module_directive(".reg .s32 i;", 1).is_err());
}

#[test]
fn rejects_reg_vector_directive() {
    assert!(parse_module_directive(".reg .v4 .f32 accel;", 1).is_err());
}

#[test]
fn rejects_reg_predicate_directive() {
    assert!(parse_module_directive(".reg .pred p, q, r;", 1).is_err());
}

#[test]
fn rejects_volatile_global_declaration() {
    let err = parse_module_directive(".global .volatile .b32 bad;", 1)
        .expect_err(".volatile global should be rejected");
    match err {
        PtxParseError::InvalidGlobal { message, .. } => {
            assert!(message.contains(".volatile"), "unexpected error: {message}");
        }
        other => panic!("expected InvalidGlobal error, got {other:?}"),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VariableKind {
    Const,
    Global,
    Shared,
    Tex,
}

fn expect_variable(line: &str, kind: VariableKind) -> VariableDirective {
    match parse_module_directive(line, 1).expect("directive should parse") {
        ModuleDirective::ModuleVariable(module_var) => match module_var {
            ModuleVariableDirective::Const(var) => match kind {
                VariableKind::Const => var,
                other => panic!("expected {:?} directive, got const", other),
            },
            ModuleVariableDirective::Global(var) => match kind {
                VariableKind::Global => var,
                other => panic!("expected {:?} directive, got global", other),
            },
            ModuleVariableDirective::Shared(var) => match kind {
                VariableKind::Shared => var,
                other => panic!("expected {:?} directive, got shared", other),
            },
            ModuleVariableDirective::Tex(var) => match kind {
                VariableKind::Tex => var,
                other => panic!("expected {:?} directive, got tex", other),
            },
        },
        other => panic!("expected module variable directive, got {:?}", other),
    }
}

fn initializer_numeric_value(value: &InitializerValue) -> i64 {
    match value {
        InitializerValue::Numeric(NumericLiteral::Signed(v)) => *v,
        InitializerValue::Numeric(NumericLiteral::Unsigned(v)) => *v as i64,
        other => panic!("expected integer numeric initializer, got {:?}", other),
    }
}

fn collect_numeric_array(initializer: &GlobalInitializer) -> Vec<i64> {
    match initializer {
        GlobalInitializer::Aggregate(items) => items
            .iter()
            .map(|item| match item {
                GlobalInitializer::Scalar(inner) => initializer_numeric_value(inner),
                other => panic!("expected scalar numeric element, got {:?}", other),
            })
            .collect(),
        GlobalInitializer::Scalar(inner) => vec![initializer_numeric_value(inner)],
    }
}

fn collect_symbol_array(initializer: &GlobalInitializer) -> Vec<&str> {
    match initializer {
        GlobalInitializer::Aggregate(items) => items.iter().map(collect_symbol).collect(),
        GlobalInitializer::Scalar(_) => vec![collect_symbol(initializer)],
    }
}

fn collect_symbol(initializer: &GlobalInitializer) -> &str {
    match initializer {
        GlobalInitializer::Scalar(InitializerValue::Symbol(symbol)) => symbol.as_str(),
        other => panic!("expected scalar symbol initializer, got {:?}", other),
    }
}
