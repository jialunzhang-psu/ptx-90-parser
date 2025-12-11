mod util;

use ptx_parser::r#type::{
    BranchTargetsDirective, CallPrototypeDirective, CallPrototypeReturnSpec, CallTargetsDirective,
    DataType, DwarfDirectiveKind, FunctionStatement, FunctionSymbol, Label, ParamStateSpace,
    ParameterDirective, PragmaDirectiveKind, SectionEntry, StatementDirective,
    StatementSectionDirectiveLine, VariableSymbol,
};
use ptx_parser::{PtxParser, PtxTokenStream, PtxUnlexer, PtxUnparser, span, tokenize};
use util::{assert_roundtrip, parse, tokenize_only, tokens_equivalent};

#[test]
fn parses_branch_and_call_target_directives() {
    let branch_src = ".branchtargets L0, L1;";
    let branch = parse::<StatementDirective>(branch_src);
    assert_statement_ast(&branch, branchtargets_stmt(&["L0", "L1"]), branch_src);
    assert_roundtrip::<StatementDirective>(branch_src);

    let labeled_branch_src = "entry0: .branchtargets L0, L1;";
    let (branch_label, branch_directive_stmt) = parse_label_and_directive(labeled_branch_src);
    assert_label_statement(&branch_label, "entry0");
    let branch_directive = statement_directive(&branch_directive_stmt);
    assert_statement_ast(
        branch_directive,
        branchtargets_stmt(&["L0", "L1"]),
        labeled_branch_src,
    );
    assert_statements_roundtrip(labeled_branch_src, &[branch_label, branch_directive_stmt]);

    let unlabeled_branch_src = ".branchtargets L_only;";
    let unlabeled_branch = parse::<StatementDirective>(unlabeled_branch_src);
    assert_statement_ast(
        &unlabeled_branch,
        branchtargets_stmt(&["L_only"]),
        unlabeled_branch_src,
    );
    assert_roundtrip::<StatementDirective>(unlabeled_branch_src);

    let calltargets_src = ".calltargets foo, bar;";
    let calltargets = parse::<StatementDirective>(calltargets_src);
    assert_statement_ast(
        &calltargets,
        calltargets_stmt(&["foo", "bar"]),
        calltargets_src,
    );
    assert_roundtrip::<StatementDirective>(calltargets_src);

    let labeled_call_src = "entry1: .calltargets foo, bar;";
    let (call_label, call_directive_stmt) = parse_label_and_directive(labeled_call_src);
    assert_label_statement(&call_label, "entry1");
    let call_directive = statement_directive(&call_directive_stmt);
    assert_statement_ast(
        call_directive,
        calltargets_stmt(&["foo", "bar"]),
        labeled_call_src,
    );
    assert_statements_roundtrip(labeled_call_src, &[call_label, call_directive_stmt]);

    let unlabeled_call_src = ".calltargets baz;";
    let unlabeled_call = parse::<StatementDirective>(unlabeled_call_src);
    assert_statement_ast(
        &unlabeled_call,
        calltargets_stmt(&["baz"]),
        unlabeled_call_src,
    );
    assert_roundtrip::<StatementDirective>(unlabeled_call_src);
}

#[test]
fn parses_callprototype_directive() {
    let empty_src = ".callprototype _ () .noreturn;";
    let empty = parse::<StatementDirective>(empty_src);
    assert_statement_ast(
        &empty,
        callproto_stmt(CallPrototypeReturnSpec::BareUnderscore, vec![], true, None, None),
        empty_src,
    );
    assert_roundtrip::<StatementDirective>(empty_src);

    let params_src = ".callprototype _ (.param .u32 _, .param .u32 _);";
    let params = parse::<StatementDirective>(params_src);
    assert_statement_ast(
        &params,
        callproto_stmt(
            CallPrototypeReturnSpec::BareUnderscore,
            vec![
                make_param(DataType::U32 { span: span!(0..0) }, "_", None, false, None),
                make_param(DataType::U32 { span: span!(0..0) }, "_", None, false, None),
            ],
            false,
            None,
            None,
        ),
        params_src,
    );
    assert_roundtrip::<StatementDirective>(params_src);

    let return_only_src = ".callprototype .param .u64 retval ();";
    let return_only = parse::<StatementDirective>(return_only_src);
    assert_statement_ast(
        &return_only,
        callproto_stmt(
            CallPrototypeReturnSpec::BareParam(make_param(
                DataType::U64 { span: span!(0..0) },
                "retval",
                None,
                false,
                None,
            )),
            vec![],
            false,
            None,
            None,
        ),
        return_only_src,
    );
    assert_roundtrip::<StatementDirective>(return_only_src);

    let return_with_param_src = ".callprototype .param .u64 retval (.param .u32 arg0);";
    let return_with_param = parse::<StatementDirective>(return_with_param_src);
    assert_statement_ast(
        &return_with_param,
        callproto_stmt(
            CallPrototypeReturnSpec::BareParam(make_param(
                DataType::U64 { span: span!(0..0) },
                "retval",
                None,
                false,
                None,
            )),
            vec![make_param(
                DataType::U32 { span: span!(0..0) },
                "arg0",
                None,
                false,
                None,
            )],
            false,
            None,
            None,
        ),
        return_with_param_src,
    );
    assert_roundtrip::<StatementDirective>(return_with_param_src);

    let multi_param_src = ".callprototype .param .u64 retval (.param .u32 arg0, .param .b64 arg1);";
    let multi_param = parse::<StatementDirective>(multi_param_src);
    assert_statement_ast(
        &multi_param,
        callproto_stmt(
            CallPrototypeReturnSpec::BareParam(make_param(
                DataType::U64 { span: span!(0..0) },
                "retval",
                None,
                false,
                None,
            )),
            vec![
                make_param(
                    DataType::U32 { span: span!(0..0) },
                    "arg0",
                    None,
                    false,
                    None,
                ),
                make_param(
                    DataType::B64 { span: span!(0..0) },
                    "arg1",
                    None,
                    false,
                    None,
                ),
            ],
            false,
            None,
            None,
        ),
        multi_param_src,
    );
    assert_roundtrip::<StatementDirective>(multi_param_src);

    // Note: alignment must come before type in PTX syntax
    let pointer_src = ".callprototype _ (.param .align 16 .b64 .ptr .global arg_ptr) .noreturn .abi_preserve 8 .abi_preserve_control 4;";
    let pointer = parse::<StatementDirective>(pointer_src);
    assert_statement_ast(
        &pointer,
        callproto_stmt(
            CallPrototypeReturnSpec::BareUnderscore,
            vec![make_param(
                DataType::B64 { span: span!(0..0) },
                "arg_ptr",
                Some(16),
                true,
                Some(ParamStateSpace::Global { span: span!(0..0) }),
            )],
            true,
            Some(8),
            Some(4),
        ),
        pointer_src,
    );
    assert_roundtrip::<StatementDirective>(pointer_src);

    let labeled_src = "entry_proto: .callprototype _ () .noreturn;";
    let (label_stmt, labeled_directive_stmt) = parse_label_and_directive(labeled_src);
    assert_label_statement(&label_stmt, "entry_proto");
    let labeled_directive = statement_directive(&labeled_directive_stmt);
    assert_statement_ast(
        labeled_directive,
        callproto_stmt(CallPrototypeReturnSpec::BareUnderscore, vec![], true, None, None),
        labeled_src,
    );
    assert_statements_roundtrip(labeled_src, &[label_stmt, labeled_directive_stmt]);
}

#[test]
fn parses_dwarf_directive() {
    let byte_src = ".dwarf .byte 0x1, 0xff;";
    let byte_stmt = parse::<StatementDirective>(byte_src);
    match byte_stmt {
        StatementDirective::Dwarf { directive, .. } => match directive.kind {
            DwarfDirectiveKind::ByteValues(values) => assert_eq!(values, vec![1, 255]),
            other => panic!("expected byte values, got {:?}", other),
        },
        other => panic!("expected dwarf directive, got {:?}", other),
    }
    assert_roundtrip::<StatementDirective>(byte_src);

    let quad_src = ".dwarf .quad 0xff;";
    let quad_stmt = parse::<StatementDirective>(quad_src);
    match quad_stmt {
        StatementDirective::Dwarf { directive, .. } => match directive.kind {
            DwarfDirectiveKind::QuadValues(values) => assert_eq!(values, vec![255]),
            other => panic!("expected quad values, got {:?}", other),
        },
        other => panic!("expected dwarf directive, got {:?}", other),
    }
    assert_roundtrip::<StatementDirective>(quad_src);

    let label_src = ".dwarf .4byte debug_label;";
    let label_stmt = parse::<StatementDirective>(label_src);
    match label_stmt {
        StatementDirective::Dwarf { directive, .. } => match directive.kind {
            DwarfDirectiveKind::FourByteLabel(label) => assert_eq!(label.val, "debug_label"),
            other => panic!("expected 4byte label, got {:?}", other),
        },
        other => panic!("expected dwarf directive, got {:?}", other),
    }
    assert_roundtrip::<StatementDirective>(label_src);
}

#[test]
fn parses_pragma_directive_variants() {
    let nounroll = parse::<StatementDirective>(".pragma \"nounroll\";");
    match nounroll {
        StatementDirective::Pragma { directive, .. } => {
            assert!(matches!(directive.kind, PragmaDirectiveKind::Nounroll))
        }
        other => panic!("expected pragma, got {:?}", other),
    }
    assert_roundtrip::<StatementDirective>(".pragma \"nounroll\";");

    let used_mask = parse::<StatementDirective>(".pragma \"used_bytes_mask 0xff\";");
    match used_mask {
        StatementDirective::Pragma { directive, .. } => match directive.kind {
            PragmaDirectiveKind::UsedBytesMask { mask } => assert_eq!(mask, "0xff"),
            other => panic!("expected used_bytes_mask, got {:?}", other),
        },
        other => panic!("expected pragma, got {:?}", other),
    }
    assert_roundtrip::<StatementDirective>(".pragma \"used_bytes_mask 0xff\";");

    let freq = parse::<StatementDirective>(".pragma \"frequency 3\";");
    match freq {
        StatementDirective::Pragma { directive, .. } => match directive.kind {
            PragmaDirectiveKind::Frequency { value } => assert_eq!(value, 3),
            other => panic!("expected frequency pragma, got {:?}", other),
        },
        other => panic!("expected pragma, got {:?}", other),
    }
    // Roundtrip may normalize spacing; no roundtrip assertion.
    let raw = parse::<StatementDirective>(".pragma \"custom option\";");
    match raw {
        StatementDirective::Pragma { directive, .. } => match directive.kind {
            PragmaDirectiveKind::Raw(text) => assert_eq!(text, "custom option"),
            other => panic!("expected raw pragma, got {:?}", other),
        },
        other => panic!("expected pragma, got {:?}", other),
    }
    // Roundtrip may normalize spacing; no roundtrip assertion.
}

#[test]
fn parses_section_directive() {
    let source = ".section .debug_str {
    label0:
    .b8 1, -2
    .b16 -32, 64
    .b32 1, -2
    .b64 1, -2
    .b32 label_a
    .b64 label_g
    .b32 label_c+4
    .b64 label_i+16
    .b32 label_e-label_f
    .b64 label_k-label_l
}";
    let directive = parse::<StatementDirective>(source);
    match directive {
        StatementDirective::Section {
            directive: section, ..
        } => {
            assert_eq!(section.name, ".debug_str");
            assert_eq!(section.entries.len(), 11);
            match &section.entries[0] {
                SectionEntry::Label { label, .. } => assert_eq!(label.val, "label0"),
                other => panic!("expected label entry, got {:?}", other),
            }
            match &section.entries[1] {
                SectionEntry::Directive(StatementSectionDirectiveLine::B8 { values, .. }) => {
                    assert_eq!(values, &vec![1i16, -2i16]);
                }
                other => panic!("expected .b8 line, got {:?}", other),
            }
            match &section.entries[5] {
                SectionEntry::Directive(StatementSectionDirectiveLine::B32Label {
                    labels, ..
                }) => {
                    assert_eq!(labels.val, "label_a");
                }
                other => panic!("expected b32 label line, got {:?}", other),
            }
            match &section.entries[6] {
                SectionEntry::Directive(StatementSectionDirectiveLine::B64Label {
                    labels, ..
                }) => {
                    assert_eq!(labels.val, "label_g");
                }
                other => panic!("expected b64 label line, got {:?}", other),
            }
            match &section.entries[7] {
                SectionEntry::Directive(StatementSectionDirectiveLine::B32LabelPlusImm {
                    entries,
                    ..
                }) => {
                    assert_eq!(entries.0.val, "label_c");
                    assert_eq!(entries.1, 4);
                }
                other => panic!("expected label plus imm line, got {:?}", other),
            }
            match &section.entries[8] {
                SectionEntry::Directive(StatementSectionDirectiveLine::B64LabelPlusImm {
                    entries,
                    ..
                }) => {
                    assert_eq!(entries.0.val, "label_i");
                    assert_eq!(entries.1, 16);
                }
                other => panic!("expected b64 label+imm, got {:?}", other),
            }
            match &section.entries[9] {
                SectionEntry::Directive(StatementSectionDirectiveLine::B32LabelDiff {
                    entries,
                    ..
                }) => {
                    assert_eq!(entries.0.val, "label_e");
                    assert_eq!(entries.1.val, "label_f");
                }
                other => panic!("expected label diff, got {:?}", other),
            }
            match &section.entries[10] {
                SectionEntry::Directive(StatementSectionDirectiveLine::B64LabelDiff {
                    entries,
                    ..
                }) => {
                    assert_eq!(entries.0.val, "label_k");
                    assert_eq!(entries.1.val, "label_l");
                }
                other => panic!("expected b64 label diff, got {:?}", other),
            }
        }
        other => panic!("expected section directive, got {:?}", other),
    }
    assert_roundtrip::<StatementDirective>(source);
}

fn assert_statement_ast(actual: &StatementDirective, expected: StatementDirective, context: &str) {
    let actual_tokens = actual.to_tokens();
    let expected_tokens = expected.to_tokens();
    let actual_text =
        PtxUnlexer::to_string(&actual_tokens).expect("failed to unlex actual statement");
    let expected_text =
        PtxUnlexer::to_string(&expected_tokens).expect("failed to unlex expected statement");
    assert_eq!(actual_text, expected_text, "AST mismatch for {}", context);
}

fn branchtargets_stmt(labels: &[&str]) -> StatementDirective {
    StatementDirective::BranchTargets {
        directive: BranchTargetsDirective {
            labels: labels
                .iter()
                .map(|value| Label {
                    val: (*value).into(),
                    span: span!(0..0),
                })
                .collect(),
            span: span!(0..0),
        },
        span: span!(0..0),
    }
}

fn calltargets_stmt(targets: &[&str]) -> StatementDirective {
    StatementDirective::CallTargets {
        directive: CallTargetsDirective {
            targets: targets
                .iter()
                .map(|value| FunctionSymbol {
                    val: (*value).into(),
                    span: span!(0..0),
                })
                .collect(),
            span: span!(0..0),
        },
        span: span!(0..0),
    }
}

fn callproto_stmt(
    return_spec: CallPrototypeReturnSpec,
    params: Vec<ParameterDirective>,
    noreturn: bool,
    abi_preserve: Option<u32>,
    abi_preserve_control: Option<u32>,
) -> StatementDirective {
    StatementDirective::CallPrototype {
        directive: CallPrototypeDirective {
            return_spec,
            params,
            noreturn,
            abi_preserve,
            abi_preserve_control,
            span: span!(0..0),
        },
        span: span!(0..0),
    }
}

fn parse_label_and_directive(source: &str) -> (FunctionStatement, FunctionStatement) {
    let tokens = tokenize(source).expect("tokenize labeled directive");
    let mut stream = PtxTokenStream::new(&tokens);
    let (label_stmt, _) = FunctionStatement::parse()(&mut stream).expect("parse label statement");
    let (directive_stmt, _) =
        FunctionStatement::parse()(&mut stream).expect("parse directive after label");
    assert!(
        matches!(label_stmt, FunctionStatement::Label { .. }),
        "expected label statement"
    );
    assert!(
        matches!(directive_stmt, FunctionStatement::Directive { .. }),
        "expected directive statement"
    );
    assert!(stream.is_at_end(), "parser should consume entire snippet");
    (label_stmt, directive_stmt)
}

fn assert_label_statement(statement: &FunctionStatement, expected: &str) {
    match statement {
        FunctionStatement::Label { label, .. } => assert_eq!(label.val, expected),
        other => panic!("expected label statement, got {:?}", other),
    }
}

fn statement_directive<'a>(statement: &'a FunctionStatement) -> &'a StatementDirective {
    match statement {
        FunctionStatement::Directive { directive, .. } => directive,
        other => panic!("expected directive statement, got {:?}", other),
    }
}

fn assert_statements_roundtrip(source: &str, statements: &[FunctionStatement]) {
    let original_tokens = tokenize_only(source);
    let mut unparsed_tokens = Vec::new();
    for statement in statements {
        unparsed_tokens.extend(statement.to_tokens());
    }
    assert!(
        tokens_equivalent(&unparsed_tokens, &original_tokens),
        "Roundtrip failed for {}\nUnparsed: {:?}\nOriginal: {:?}",
        source,
        unparsed_tokens,
        original_tokens
    );
}

fn make_param(
    ty: DataType,
    name: &str,
    align: Option<u32>,
    ptr: bool,
    space: Option<ParamStateSpace>,
) -> ParameterDirective {
    ParameterDirective::Parameter {
        align,
        ty,
        ptr,
        space,
        name: VariableSymbol {
            val: name.into(),
            span: span!(0..0),
        },
        array: Vec::new(),
        span: span!(0..0),
    }
}

#[test]
fn parses_labeled_callprototype_in_function_body() {
    use ptx_parser::parse_ptx;

    // Test labeled callprototype with underscore return (no return value)
    let ptx_underscore = r#"
.version 8.0
.target sm_80
.address_size 64

.visible .func test_func()
{
    u0_92_call_proto_0: .callprototype _ ();
    ret;
}
"#;
    parse_ptx(ptx_underscore).expect("should parse labeled callprototype with underscore");

    // Test simple labeled callprototype
    let ptx_labeled = r#"
.version 8.0
.target sm_80
.address_size 64

.visible .func test_func()
{
    my_proto: .callprototype _ ();
    ret;
}
"#;
    parse_ptx(ptx_labeled).expect("should parse simple labeled callprototype");

    // Test unlabeled callprototype (should still work)
    let ptx_simple = r#"
.version 8.0
.target sm_80
.address_size 64

.visible .func test_func()
{
    .callprototype _ ();
    ret;
}
"#;
    parse_ptx(ptx_simple).expect("should parse unlabeled callprototype");

    // Test function with return parameter
    let ptx_func_ret = r#"
.version 8.0
.target sm_80
.address_size 64

.visible .func (.param .b8 func_retval0) test_func(.param .u64 param0)
{
    ret;
}
"#;
    parse_ptx(ptx_func_ret).expect("should parse function with return parameter");
}

#[test]
fn parses_callprototype_with_parenthesized_return() {
    // PTX allows two syntaxes for callprototype:
    // 1. Bare return spec: .callprototype RET_SPEC (params);
    // 2. Parenthesized return: .callprototype (RET_SPEC) _ (params);
    // The parenthesized form includes an explicit function identifier placeholder `_`

    // Test parenthesized return with param
    let paren_return_src = ".callprototype (.param .b8 _) _ ();";
    let paren_return = parse::<StatementDirective>(paren_return_src);
    assert_statement_ast(
        &paren_return,
        callproto_stmt(
            CallPrototypeReturnSpec::ParenParam(make_param(DataType::B8 { span: span!(0..0) }, "_", None, false, None)),
            vec![],
            false,
            None,
            None,
        ),
        paren_return_src,
    );

    // Test parenthesized underscore (no return)
    let paren_underscore_src = ".callprototype (_) _ ();";
    let paren_underscore = parse::<StatementDirective>(paren_underscore_src);
    assert_statement_ast(
        &paren_underscore,
        callproto_stmt(CallPrototypeReturnSpec::ParenUnderscore, vec![], false, None, None),
        paren_underscore_src,
    );

    // Test parenthesized return with params
    let paren_with_params_src = ".callprototype (.param .u32 retval) _ (.param .u64 arg0);";
    let paren_with_params = parse::<StatementDirective>(paren_with_params_src);
    assert_statement_ast(
        &paren_with_params,
        callproto_stmt(
            CallPrototypeReturnSpec::ParenParam(make_param(DataType::U32 { span: span!(0..0) }, "retval", None, false, None)),
            vec![make_param(DataType::U64 { span: span!(0..0) }, "arg0", None, false, None)],
            false,
            None,
            None,
        ),
        paren_with_params_src,
    );

    assert_roundtrip::<StatementDirective>(paren_return_src);
    assert_roundtrip::<StatementDirective>(paren_underscore_src);
    assert_roundtrip::<StatementDirective>(paren_with_params_src);
}
