use ptx_parser::{
    parse, FunctionEntryDirective, FunctionHeaderDirective, FunctionKernelDirective,
    FunctionStatement, ModuleDirective, ParameterStorage,
};

#[test]
fn parses_func_with_return_param_and_body() {
    let source = r#".func (.reg .b32 rval) foo (.reg .b32 N, .reg .f64 dbl)
{
.reg .b32 localVar;

mov.b32 rval,result;
ret;
}
"#;

    let module = parse(source).expect("function should parse");

    let function = module
        .directives
        .iter()
        .find_map(|directive| match directive {
            ModuleDirective::FunctionKernel(FunctionKernelDirective::Func(function)) => {
                Some(function)
            }
            _ => None,
        })
        .expect("module should contain function");

    assert_eq!(function.name, "foo");
    assert_eq!(function.params.len(), 2);
    assert_eq!(function.params[0].name, "N");
    assert_eq!(function.params[1].name, "dbl");
    let return_param = function.return_param.as_ref().expect("return param");
    assert_eq!(return_param.name, "rval");
    assert!(return_param.storage.is_none());

    let entry_directives = &function.body.entry_directives;
    assert_eq!(entry_directives.len(), 1);
    assert!(matches!(
        entry_directives[0],
        FunctionEntryDirective::Reg(_)
    ));

    let statements = &function.body.statements;
    assert_eq!(statements.len(), 2);
    assert!(matches!(statements[0], FunctionStatement::Instruction(_)));
    assert!(matches!(statements[1], FunctionStatement::Instruction(_)));
}

#[test]
fn parses_func_with_param_storage_and_array() {
    let source = r#".func (.param .u32 rval) bar(.param .u32 N, .param .align 4 .b8 numbers[])
{
    .reg .b32 input0, input1;
    ld.param.b32   input0, [numbers + 0];
    ld.param.b32   input1, [numbers + 4];
    ret;
}
"#;

    let module = parse(source).expect("function should parse");

    let function = module
        .directives
        .iter()
        .find_map(|directive| match directive {
            ModuleDirective::FunctionKernel(FunctionKernelDirective::Func(function)) => {
                Some(function)
            }
            _ => None,
        })
        .expect("module should contain function");

    assert_eq!(function.name, "bar");
    assert_eq!(function.params.len(), 2);
    assert_eq!(function.params[0].name, "N");
    assert_eq!(function.params[1].name, "numbers");
    assert!(function.params[1].array.is_some());
    let return_param = function.return_param.as_ref().expect("return param");
    assert_eq!(return_param.name, "rval");
    assert_eq!(return_param.storage, Some(ParameterStorage::Param));

    let entry_directives = &function.body.entry_directives;
    assert_eq!(entry_directives.len(), 1);
    assert!(matches!(
        entry_directives[0],
        FunctionEntryDirective::Reg(_)
    ));

    let statements = &function.body.statements;
    assert_eq!(statements.len(), 3);
    assert!(matches!(statements[0], FunctionStatement::Instruction(_)));
    assert!(matches!(statements[1], FunctionStatement::Instruction(_)));
    assert!(matches!(statements[2], FunctionStatement::Instruction(_)));
}

#[test]
fn parses_func_without_return_value_and_trailing_qualifiers() {
    let source = r#".func foo (.reg .b32 N, .reg .f64 dbl) .noreturn
{
.reg .b32 localVar;
// ... use N, dbl;
// other code;
mov.b32 rval, result;
ret;
}
"#;

    let module = parse(source).expect("function should parse");
    let func = module
        .directives
        .iter()
        .find_map(|directive| match directive {
            ModuleDirective::FunctionKernel(FunctionKernelDirective::Func(function)) => {
                Some(function)
            }
            _ => None,
        })
        .expect("module should contain function");

    assert_eq!(func.name, "foo");
    assert!(func.return_param.is_none());
    assert_eq!(func.params.len(), 2);
    assert_eq!(func.params[0].name, "N");
    assert_eq!(func.params[1].name, "dbl");
    assert!(func
        .directives
        .iter()
        .any(|directive| matches!(directive, FunctionHeaderDirective::NoReturn)));
    let entry_directives = &func.body.entry_directives;
    assert_eq!(entry_directives.len(), 1);
    assert!(matches!(
        entry_directives[0],
        FunctionEntryDirective::Reg(_)
    ));

    assert_eq!(func.body.statements.len(), 2);
}

#[test]
fn parses_entry_with_large_array_param() {
    let source = r#".entry prefix_sum ( .param .align 4 .s32 pitch[8000] )
{
    .reg .s32 %t;
    ld.param::entry.s32  %t, [pitch];
}
"#;

    let module = parse(source).expect("function should parse");
    let entry = module
        .directives
        .iter()
        .find_map(|directive| match directive {
            ModuleDirective::FunctionKernel(FunctionKernelDirective::Entry(function)) => {
                Some(function)
            }
            _ => None,
        })
        .expect("module should contain entry");

    assert_eq!(entry.name, "prefix_sum");
    assert_eq!(entry.params.len(), 1);
    let param = &entry.params[0];
    assert_eq!(param.name, "pitch");
    assert!(param.array.is_some());
    assert_eq!(entry.body.entry_directives.len(), 1);
    assert!(matches!(
        entry.body.entry_directives[0],
        FunctionEntryDirective::Reg(_)
    ));
    assert_eq!(entry.body.statements.len(), 1);
}

#[test]
fn parses_entry_with_multiple_params() {
    let source = r#".entry filter ( .param .b32 x, .param .b32 y, .param .b32 z )
{
    .reg .b32 %r<99>;
    ld.param.b32  %r1, [x];
    ld.param.b32  %r2, [y];
    ld.param.b32  %r3, [z];
}
"#;

    let module = parse(source).expect("function should parse");
    let entry = module
        .directives
        .iter()
        .find_map(|directive| match directive {
            ModuleDirective::FunctionKernel(FunctionKernelDirective::Entry(function)) => {
                Some(function)
            }
            _ => None,
        })
        .expect("module should contain entry");

    assert_eq!(entry.name, "filter");
    assert_eq!(entry.params.len(), 3);
    assert_eq!(entry.params[0].name, "x");
    assert_eq!(entry.params[1].name, "y");
    assert_eq!(entry.params[2].name, "z");
    assert_eq!(entry.body.entry_directives.len(), 1);
    assert!(matches!(
        entry.body.entry_directives[0],
        FunctionEntryDirective::Reg(_)
    ));
    assert_eq!(entry.body.statements.len(), 3);
}
