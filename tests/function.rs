mod util;
use util::{parse, parse_result};
use ptx_parser::r#type::common::CodeLinkage;
use ptx_parser::r#type::function::{FunctionHeaderDirective, FunctionKernelDirective};

fn linkage_is_visible(directives: &[FunctionHeaderDirective]) -> bool {
    directives.iter().any(|directive| {
        matches!(
            directive,
            FunctionHeaderDirective::Linkage(CodeLinkage::Visible)
        )
    })
}

#[test]
fn parses_simple_entry_function() {
    let directive = parse::<FunctionKernelDirective>(".visible .entry simple() {\n}\n");
    let FunctionKernelDirective::Entry(entry) = directive else {
        panic!("expected entry function");
    };
    assert_eq!(entry.name, "simple");
    assert!(linkage_is_visible(&entry.directives));
    assert!(entry.params.is_empty());
    assert!(entry.body.entry_directives.is_empty());
    assert!(entry.body.statements.is_empty());
}

#[test]
fn parses_simple_device_function() {
    let directive = parse::<FunctionKernelDirective>(".func device() {\n}\n");
    let FunctionKernelDirective::Func(func) = directive else {
        panic!("expected device function");
    };
    assert_eq!(func.name, "device");
    assert!(func.directives.is_empty());
    assert!(func.return_param.is_none());
    assert!(func.params.is_empty());
    assert!(func.body.entry_directives.is_empty());
    assert!(func.body.statements.is_empty());
}

#[test]
fn parses_non_empty_function_body() {
    let directive = parse::<FunctionKernelDirective>(".entry bad() {\n    ret;\n}\n");
    assert!(matches!(directive, FunctionKernelDirective::Entry(_)));
}

#[test]
fn parses_parameter_lists() {
    let directive = parse::<FunctionKernelDirective>(".entry param(.param .b32 value) {\n}\n");
    assert!(matches!(directive, FunctionKernelDirective::Entry(_)));
}

#[test]
fn parses_return_parameter_in_func() {
    let directive = parse::<FunctionKernelDirective>(".func (.param .b32 retval) foo() {\n}\n");
    assert!(matches!(directive, FunctionKernelDirective::Func(_)));
}

#[test]
fn parses_alias_directive() {
    let directive = parse::<FunctionKernelDirective>(".alias _Zfoo, _Zbar;\n");
    let FunctionKernelDirective::Alias(alias) = directive else {
        panic!("expected alias directive");
    };
    assert_eq!(alias.alias, "_Zfoo");
    assert_eq!(alias.target, "_Zbar");
}

#[test]
fn parses_alias_with_header_directive() {
    let directive = parse::<FunctionKernelDirective>(".visible .alias foo, bar;\n");
    assert!(matches!(directive, FunctionKernelDirective::Alias(_)));
}

#[test]
fn rejects_entry_without_parentheses() {
    let err = parse_result::<FunctionKernelDirective>(".entry missing_paren {\n}\n")
        .expect_err("missing parentheses should be rejected");
    assert!(matches!(
        err.kind,
        ptx_parser::parser::ParseErrorKind::UnexpectedToken { .. }
    ));
}

#[test]
fn parses_extern_function_prototype() {
    let directive = parse::<FunctionKernelDirective>(
        ".extern .func (.param .b32 func_retval0) vprintf(\n    .param .b64 vprintf_param_0,\n    .param .b64 vprintf_param_1\n);\n",
    );
    assert!(matches!(directive, FunctionKernelDirective::Func(_)));
}

#[test]
fn parses_weak_function_prototype() {
    let directive = parse::<FunctionKernelDirective>(
        ".weak .func _ZN6Kokkos4Impl12device_abortEPKc\n(\n.param .b64 _ZN6Kokkos4Impl12device_abortEPKc_param_0\n)\n;\n",
    );
    assert!(matches!(directive, FunctionKernelDirective::Func(_)));
}
