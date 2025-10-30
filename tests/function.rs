mod util;
use ptx_parser::r#type::common::CodeLinkage;
use ptx_parser::r#type::function::{
    FunctionEntryDirective, FunctionHeaderDirective, FunctionKernelDirective, FunctionStatement,
};
use ptx_parser::unlexer::PtxUnlexer;
use ptx_parser::unparser::PtxUnparser;
use util::{assert_roundtrip, parse, parse_result};

fn parse_and_roundtrip(source: &str) -> FunctionKernelDirective {
    let directive = parse::<FunctionKernelDirective>(source);
    let serialized = PtxUnlexer::to_string(&directive.to_tokens())
        .expect("serializing function directive to PTX");
    assert_roundtrip::<FunctionKernelDirective>(&serialized);
    directive
}

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
    let directive = parse_and_roundtrip(".visible .entry simple() {\n}\n");
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
    let directive = parse_and_roundtrip(".func device() {\n}\n");
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
    let directive = parse_and_roundtrip(".entry bad() {\n    ret;\n}\n");
    assert!(matches!(directive, FunctionKernelDirective::Entry(_)));
}

#[test]
fn parses_parameter_lists() {
    let directive = parse_and_roundtrip(".entry param(.param .b32 value) {\n}\n");
    assert!(matches!(directive, FunctionKernelDirective::Entry(_)));
}

#[test]
fn parses_return_parameter_in_func() {
    let directive = parse_and_roundtrip(".func (.param .b32 retval) foo() {\n}\n");
    assert!(matches!(directive, FunctionKernelDirective::Func(_)));
}

#[test]
fn parses_alias_directive() {
    let directive = parse_and_roundtrip(".alias _Zfoo, _Zbar;\n");
    let FunctionKernelDirective::Alias(alias) = directive else {
        panic!("expected alias directive");
    };
    assert_eq!(alias.alias, "_Zfoo");
    assert_eq!(alias.target, "_Zbar");
}

#[test]
fn parses_alias_with_header_directive() {
    let directive = parse::<FunctionKernelDirective>(".visible .alias foo, bar;\n");
    let serialized =
        PtxUnlexer::to_string(&directive.to_tokens()).expect("serializing alias directive");
    assert_roundtrip::<FunctionKernelDirective>(&serialized);
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
    let directive = parse_and_roundtrip(
        ".extern .func (.param .b32 func_retval0) vprintf(\n    .param .b64 vprintf_param_0,\n    .param .b64 vprintf_param_1\n);\n",
    );
    assert!(matches!(directive, FunctionKernelDirective::Func(_)));
}

#[test]
fn parses_weak_function_prototype() {
    let directive = parse_and_roundtrip(
        ".weak .func _ZN6Kokkos4Impl12device_abortEPKc\n(\n.param .b64 _ZN6Kokkos4Impl12device_abortEPKc_param_0\n)\n;\n",
    );
    assert!(matches!(directive, FunctionKernelDirective::Func(_)));
}

#[test]
fn parse_hello_world() {
    let directive = parse_and_roundtrip(
        ".visible .entry _Z12hello_kernelv()
{
    .local .align 8 .b8 __local_depot0[8];
    .reg .b64 %SP;
    .reg .b64 %SPL;
    .reg .b32 %r<7>;
    .reg .b64 %rd<4>;

    mov.u64 %SPL, __local_depot0;
    cvta.local.u64 %SP, %SPL;

    mov.u32 %r1, %ctaid.x;
    mov.u32 %r2, %ntid.x;
    mul.lo.s32 %r3, %r1, %r2;
    mov.u32 %r4, %tid.x;
    add.s32 %r5, %r3, %r4;

    st.u32 [%SP+0], %r5;
    mov.u64 %rd1, $str;
    cvta.global.u64 %rd2, %rd1;
    add.u64 %rd3, %SP, 0;
    {
        .reg .b32 temp_param_reg;
        .param .b64 param0;
        st.param.b64 [param0+0], %rd2;
        .param .b64 param1;
        st.param.b64 [param1+0], %rd3;
        .param .b32 retval0;
        call.uni (retval0), vprintf, (param0, param1);
        ld.param.b32 %r6, [retval0+0];
    }

    ret;
}
    ",
    );
    assert!(matches!(directive, FunctionKernelDirective::Entry(_)));
    // Verify that there are statements in the function body
    let FunctionKernelDirective::Entry(entry) = directive else {
        panic!("expected entry function");
    };
    assert!(!entry.body.statements.is_empty());
    // Verify that there are 4 .reg
    let reg_count = entry
        .body
        .entry_directives
        .iter()
        .filter(|x| matches!(x, FunctionEntryDirective::Reg { .. }))
        .count();
    assert_eq!(reg_count, 4);
    // Verify that there is 1 NestedBlock
    let nested_block_count = entry
        .body
        .statements
        .iter()
        .filter(|x| matches!(x, FunctionStatement::ExternCallBlock { .. }))
        .count();
    assert_eq!(nested_block_count, 1);
    // Verify the last statement is ret
    let last_statement = entry.body.statements.last().unwrap();
    let FunctionStatement::Instruction(instr) = last_statement else {
        panic!("expected last statement to be an instruction");
    };
    use ptx_parser::r#type::instruction::InstructionOpcode;
    assert!(matches!(instr.opcode, InstructionOpcode::Ret { .. }));
}
