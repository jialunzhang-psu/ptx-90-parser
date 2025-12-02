mod util;

use std::fs;

use ptx_parser::r#type::{
    AddressSize, AddressSizeDirective, BranchTargetsDirective, CallPrototypeDirective,
    CallTargetsDirective, CodeLinkage, DataLinkage, DataType, EntryFunctionDirective,
    FileDirective, FuncFunctionDirective, FunctionBody, FunctionStatement, FunctionSymbol,
    GlobalInitializer, Immediate, InitializerValue, Instruction, Label, LocationDirective, Module,
    ModuleDebugDirective, ModuleDirective, ModuleInfoDirectiveKind, ModuleVariableDirective,
    ParameterDirective, PragmaDirective, PragmaDirectiveKind, RegisterDirective, RegisterTarget,
    StatementDirective, TargetDirective, TargetString, VariableDirective, VariableModifier,
    VariableSymbol, VersionDirective,
};
use ptx_parser::{PtxParser, PtxTokenStream, PtxUnlexer, PtxUnparser, span, tokenize};

fn load_module() -> Module {
    let source = fs::read_to_string("tests/sample/mini_step64.ptx").expect("sample PTX missing");
    let tokens = tokenize(&source).expect("tokenization failed");
    let mut stream = PtxTokenStream::new(&tokens);
    let (module, _) = Module::parse()(&mut stream).expect("module parsing failed");
    if !stream.is_at_end() {
        let pos = stream.position();
        eprintln!(
            "remaining token at {}: {:?}",
            pos.0,
            tokens.get(pos.0).map(|(tok, span)| (tok, span))
        );
        let mut dbg_stream = PtxTokenStream::new(&tokens);
        dbg_stream.set_position(pos);
        let res = ModuleDirective::parse()(&mut dbg_stream);
        eprintln!("next directive parse result: {res:?}");
        let mut link_stream = PtxTokenStream::new(&tokens);
        link_stream.set_position(pos);
        let linkage = CodeLinkage::parse()(&mut link_stream);
        eprintln!("code linkage parse result: {linkage:?}");
        if linkage.is_ok() {
            let func_res = FuncFunctionDirective::parse()(&mut link_stream);
            eprintln!("func directive parse result: {func_res:?}");
        }
    }
    assert!(stream.is_at_end(), "parser should consume all tokens");
    module
}

#[test]
fn parse_mini_step64_ast() {
    ptx_parser::run_with_large_stack(|| {
        // Compare by unparsing both ASTs to text and comparing the results
        // This ignores span differences while still verifying structural equality
        let parsed = load_module();
        let expected = expected_module();

        let parsed_text = PtxUnlexer::to_string(&parsed.to_tokens()).expect("unparse parsed");
        let expected_text = PtxUnlexer::to_string(&expected.to_tokens()).expect("unparse expected");

        assert_eq!(parsed_text, expected_text, "Unparsed ASTs should match");
    });
}

#[allow(dead_code)]
fn expected_module() -> Module {
    Module {
        directives: vec![
            ModuleDirective::ModuleInfo {
                directive: ModuleInfoDirectiveKind::Version {
                    directive: VersionDirective {
                        major: 8,
                        minor: 5,
                        span: span!(0..0),
                    },
                    span: span!(0..0),
                },
                span: span!(0..0),
            },
            ModuleDirective::ModuleInfo {
                directive: ModuleInfoDirectiveKind::Target {
                    directive: TargetDirective {
                        entries: vec![TargetString::Sm80 { span: span!(0..0) }],
                        span: span!(0..0),
                    },
                    span: span!(0..0),
                },
                span: span!(0..0),
            },
            ModuleDirective::ModuleInfo {
                directive: ModuleInfoDirectiveKind::AddressSize {
                    directive: AddressSizeDirective {
                        size: AddressSize::Size64 { span: span!(0..0) },
                        span: span!(0..0),
                    },
                    span: span!(0..0),
                },
                span: span!(0..0),
            },
            ModuleDirective::Debug {
                directive: ModuleDebugDirective::File {
                    directive: FileDirective {
                        index: 1,
                        path: "mini_step64.cu".into(),
                        timestamp: None,
                        file_size: None,
                        span: span!(0..0),
                    },
                    span: span!(0..0),
                },
                span: span!(0..0),
            },
            ModuleDirective::ModuleVariable {
                linkage: Some(DataLinkage::Visible { span: span!(0..0) }),
                directive: ModuleVariableDirective::Global {
                    directive: VariableDirective {
                        attributes: vec![],
                        ty: DataType::U64 { span: span!(0..0) },
                        modifiers: vec![VariableModifier::Alignment {
                            value: 8,
                            span: span!(0..0),
                        }],
                        name: variable_symbol("g_data"),
                        array_dims: vec![Some(2)],
                        initializer: Some(GlobalInitializer::Aggregate {
                            values: vec![
                                GlobalInitializer::Scalar {
                                    value: InitializerValue::NumericLiteral {
                                        value: Immediate {
                                            value: "1".into(),
                                            span: span!(0..0),
                                        },
                                        span: span!(0..0),
                                    },
                                    span: span!(0..0),
                                },
                                GlobalInitializer::Scalar {
                                    value: InitializerValue::NumericLiteral {
                                        value: Immediate {
                                            value: "2".into(),
                                            span: span!(0..0),
                                        },
                                        span: span!(0..0),
                                    },
                                    span: span!(0..0),
                                },
                            ],
                            span: span!(0..0),
                        }),
                        span: span!(0..0),
                    },
                    span: span!(0..0),
                },
                span: span!(0..0),
            },
            ModuleDirective::ModuleVariable {
                linkage: Some(DataLinkage::Visible { span: span!(0..0) }),
                directive: ModuleVariableDirective::Const {
                    directive: VariableDirective {
                        attributes: vec![],
                        ty: DataType::B32 { span: span!(0..0) },
                        modifiers: vec![VariableModifier::Alignment {
                            value: 4,
                            span: span!(0..0),
                        }],
                        name: variable_symbol("const_values"),
                        array_dims: vec![Some(2)],
                        initializer: Some(GlobalInitializer::Aggregate {
                            values: vec![
                                GlobalInitializer::Scalar {
                                    value: InitializerValue::NumericLiteral {
                                        value: Immediate {
                                            value: "3".into(),
                                            span: span!(0..0),
                                        },
                                        span: span!(0..0),
                                    },
                                    span: span!(0..0),
                                },
                                GlobalInitializer::Scalar {
                                    value: InitializerValue::NumericLiteral {
                                        value: Immediate {
                                            value: "4".into(),
                                            span: span!(0..0),
                                        },
                                        span: span!(0..0),
                                    },
                                    span: span!(0..0),
                                },
                            ],
                            span: span!(0..0),
                        }),
                        span: span!(0..0),
                    },
                    span: span!(0..0),
                },
                span: span!(0..0),
            },
            ModuleDirective::FuncFunction {
                linkage: Some(CodeLinkage::Visible { span: span!(0..0) }),
                directive: FuncFunctionDirective {
                    attributes: vec![],
                    return_param: None,
                    name: function_symbol("helper"),
                    params: vec![ParameterDirective::Parameter {
                        align: None,
                        ty: DataType::B32 { span: span!(0..0) },
                        ptr: false,
                        space: None,
                        name: variable_symbol("helper_param"),
                        array: vec![],
                        span: span!(0..0),
                    }],
                    directives: vec![],
                    pre_body_declarations: vec![],
                    body: Some(FunctionBody {
                        statements: vec![
                            reg("%r0", "b32"),
                            instruction("mov.u32 %r0, %r0;"),
                            instruction("ret;"),
                        ],
                        span: span!(0..0),
                    }),
                    span: span!(0..0),
                },
                span: span!(0..0),
            },
            ModuleDirective::EntryFunction {
                linkage: Some(CodeLinkage::Visible { span: span!(0..0) }),
                directive: EntryFunctionDirective {
                    name: function_symbol("step64_kernel"),
                    params: vec![
                        ParameterDirective::Parameter {
                            align: None,
                            ty: DataType::U64 { span: span!(0..0) },
                            ptr: false,
                            space: None,
                            name: variable_symbol("param0"),
                            array: vec![],
                            span: span!(0..0),
                        },
                        ParameterDirective::Parameter {
                            align: None,
                            ty: DataType::U32 { span: span!(0..0) },
                            ptr: false,
                            space: None,
                            name: variable_symbol("param1"),
                            array: vec![],
                            span: span!(0..0),
                        },
                    ],
                    directives: vec![],
                    body: Some(FunctionBody {
                        statements: vec![
                            reg("%p1", "pred"),
                            reg("%r1", "b32"),
                            reg("%r2", "b32"),
                            reg("%rd1", "b64"),
                            FunctionStatement::Directive {
                                directive: StatementDirective::Loc {
                                    directive: LocationDirective {
                                        file_index: 1,
                                        line: 42,
                                        column: 3,
                                        inlined_at: None,
                                        span: span!(0..0),
                                    },
                                    span: span!(0..0),
                                },
                                span: span!(0..0),
                            },
                            instruction("mov.u32 %r1, %tid.x;"),
                            instruction("mov.u32 %r2, %ntid.x;"),
                            instruction("add.s32 %r1, %r1, %r2;"),
                            FunctionStatement::Directive {
                                directive: StatementDirective::Pragma {
                                    directive: PragmaDirective {
                                        kind: PragmaDirectiveKind::Nounroll,
                                        span: span!(0..0),
                                    },
                                    span: span!(0..0),
                                },
                                span: span!(0..0),
                            },
                            label("$L_loop"),
                            FunctionStatement::Directive {
                                directive: StatementDirective::Loc {
                                    directive: LocationDirective {
                                        file_index: 1,
                                        line: 42,
                                        column: 3,
                                        inlined_at: None,
                                        span: span!(0..0),
                                    },
                                    span: span!(0..0),
                                },
                                span: span!(0..0),
                            },
                            instruction("setp.eq.s32 %p1, %r1, 0;"),
                            instruction("@%p1 bra $L_exit;"),
                            FunctionStatement::Block {
                                statements: vec![
                                    reg("%r3", "b32"),
                                    instruction("mov.u32 %r3, %r1;"),
                                ],
                                span: span!(0..0),
                            },
                            instruction("add.s32 %r1, %r1, -1;"),
                            instruction("bra $L_loop;"),
                            label("$L_exit"),
                            label("entry_br"),
                            FunctionStatement::Directive {
                                directive: StatementDirective::BranchTargets {
                                    directive: BranchTargetsDirective {
                                        labels: vec![label_symbol("$L_exit")],
                                        span: span!(0..0),
                                    },
                                    span: span!(0..0),
                                },
                                span: span!(0..0),
                            },
                            label("entry_call"),
                            FunctionStatement::Directive {
                                directive: StatementDirective::CallTargets {
                                    directive: CallTargetsDirective {
                                        targets: vec![function_symbol("helper")],
                                        span: span!(0..0),
                                    },
                                    span: span!(0..0),
                                },
                                span: span!(0..0),
                            },
                            label("entry_proto"),
                            FunctionStatement::Directive {
                                directive: StatementDirective::CallPrototype {
                                    directive: CallPrototypeDirective {
                                        return_param: None,
                                        params: vec![ParameterDirective::Parameter {
                                            align: None,
                                            ty: DataType::U32 { span: span!(0..0) },
                                            ptr: false,
                                            space: None,
                                            name: variable_symbol("param_placeholder"),
                                            array: vec![],
                                            span: span!(0..0),
                                        }],
                                        noreturn: false,
                                        abi_preserve: Some(4),
                                        abi_preserve_control: Some(2),
                                        span: span!(0..0),
                                    },
                                    span: span!(0..0),
                                },
                                span: span!(0..0),
                            },
                            instruction("ret;"),
                        ],
                        span: span!(0..0),
                    }),
                    span: span!(0..0),
                },
                span: span!(0..0),
            },
        ],
        span: span!(0..0),
    }
}

#[allow(dead_code)]
fn reg(name: &str, ty: &str) -> FunctionStatement {
    FunctionStatement::Directive {
        directive: StatementDirective::Reg {
            directive: RegisterDirective {
                ty: data_type(ty),
                registers: vec![RegisterTarget {
                    name: variable_symbol(name),
                    range: None,
                    span: span!(0..0),
                }],
                span: span!(0..0),
            },
            span: span!(0..0),
        },
        span: span!(0..0),
    }
}

#[allow(dead_code)]
fn instruction(source: &str) -> FunctionStatement {
    let tokens = tokenize(source).expect("tokenize instruction");
    let mut stream = PtxTokenStream::new(&tokens);
    let (inst, _) = Instruction::parse()(&mut stream).expect("parse instruction");
    FunctionStatement::Instruction {
        instruction: inst,
        span: span!(0..0),
    }
}

#[allow(dead_code)]
fn label(name: &str) -> FunctionStatement {
    FunctionStatement::Label {
        label: label_symbol(name),
        span: span!(0..0),
    }
}

fn label_symbol(name: &str) -> Label {
    Label {
        val: name.into(),
        span: span!(0..0),
    }
}

fn function_symbol(name: &str) -> FunctionSymbol {
    FunctionSymbol {
        val: name.into(),
        span: span!(0..0),
    }
}

fn variable_symbol(name: &str) -> VariableSymbol {
    VariableSymbol {
        val: name.into(),
        span: span!(0..0),
    }
}

fn data_type(name: &str) -> DataType {
    match name {
        "b32" => DataType::B32 { span: span!(0..0) },
        "b64" => DataType::B64 { span: span!(0..0) },
        "pred" => DataType::Pred { span: span!(0..0) },
        other => panic!("unsupported register type: {other}"),
    }
}
