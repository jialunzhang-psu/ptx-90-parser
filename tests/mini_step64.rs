use std::fs;

use ptx_parser::r#type::{
    AddressSizeDirective, AddressSpace, CodeLinkage, CodeOrDataLinkage, DataLinkage, DataType,
    EntryFunction, FileDirective, FuncFunction, FunctionBody, FunctionHeaderDirective,
    FunctionKernelDirective, FunctionStatement, GlobalInitializer, InitializerValue, Instruction,
    LinkingDirective, LocationDirective, Module, ModuleDebugDirective, ModuleDirective,
    ModuleInfoDirectiveKind, ModuleVariableDirective, NumericLiteral, PragmaDirective,
    RegisterDirective, StatementDirective, TargetDirective, VariableDirective, VariableModifier,
    VersionDirective,
};
use ptx_parser::{PtxParser, PtxTokenStream, PtxUnlexer, PtxUnparser, tokenize};

fn load_module() -> Module {
    let source = fs::read_to_string("tests/sample/mini_step64.ptx").expect("sample PTX missing");
    let tokens = tokenize(&source).expect("tokenization failed");
    let mut stream = PtxTokenStream::new(&tokens);
    let module = Module::parse(&mut stream).expect("module parsing failed");
    assert!(stream.is_at_end(), "parser should consume all tokens");
    module
}

#[test]
fn parse_mini_step64_ast() {
    // Compare by unparsing both ASTs to text and comparing the results
    // This ignores span differences while still verifying structural equality
    let parsed = load_module();
    let expected = expected_module();

    let parsed_text = PtxUnlexer::to_string(&parsed.to_tokens()).expect("unparse parsed");
    let expected_text = PtxUnlexer::to_string(&expected.to_tokens()).expect("unparse expected");

    assert_eq!(parsed_text, expected_text, "Unparsed ASTs should match");
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
                        span: 0..0,
                    },
                    span: 0..0,
                },
                span: 0..0,
            },
            ModuleDirective::ModuleInfo {
                directive: ModuleInfoDirectiveKind::Target {
                    directive: TargetDirective {
                        entries: vec!["sm_80".into()],
                        span: 0..0,
                    },
                    span: 0..0,
                },
                span: 0..0,
            },
            ModuleDirective::ModuleInfo {
                directive: ModuleInfoDirectiveKind::AddressSize {
                    directive: AddressSizeDirective {
                        size: 64,
                        span: 0..0,
                    },
                    span: 0..0,
                },
                span: 0..0,
            },
            ModuleDirective::Debug {
                directive: ModuleDebugDirective::File {
                    directive: FileDirective {
                        index: 1,
                        path: "mini_step64.cu".into(),
                        span: 0..0,
                    },
                    span: 0..0,
                },
                span: 0..0,
            },
            ModuleDirective::Linking {
                directive: LinkingDirective {
                    kind: CodeOrDataLinkage::Visible { span: 0..0 },
                    prototype: "symbol_linkage".into(),
                    span: 0..0,
                },
                span: 0..0,
            },
            ModuleDirective::ModuleVariable {
                directive: ModuleVariableDirective::Global {
                    directive: VariableDirective {
                        address_space: Some(AddressSpace::Global { span: 0..0 }),
                        attributes: vec![],
                        ty: Some(DataType::U64 { span: 0..0 }),
                        modifiers: vec![
                            VariableModifier::Linkage {
                                linkage: DataLinkage::Visible { span: 0..0 },
                                span: 0..0,
                            },
                            VariableModifier::Alignment {
                                value: 8,
                                span: 0..0,
                            },
                        ],
                        name: "g_data".into(),
                        array: vec![Some(2)],
                        initializer: Some(GlobalInitializer::Aggregate {
                            values: vec![
                                GlobalInitializer::Scalar {
                                    value: InitializerValue::Numeric {
                                        value: NumericLiteral::Unsigned {
                                            value: 1,
                                            span: 0..0,
                                        },
                                        span: 0..0,
                                    },
                                    span: 0..0,
                                },
                                GlobalInitializer::Scalar {
                                    value: InitializerValue::Numeric {
                                        value: NumericLiteral::Unsigned {
                                            value: 2,
                                            span: 0..0,
                                        },
                                        span: 0..0,
                                    },
                                    span: 0..0,
                                },
                            ],
                            span: 0..0,
                        }),
                        span: 0..0,
                    },
                    span: 0..0,
                },
                span: 0..0,
            },
            ModuleDirective::ModuleVariable {
                directive: ModuleVariableDirective::Const {
                    directive: VariableDirective {
                        address_space: Some(AddressSpace::Const { span: 0..0 }),
                        attributes: vec![],
                        ty: Some(DataType::B32 { span: 0..0 }),
                        modifiers: vec![
                            VariableModifier::Linkage {
                                linkage: DataLinkage::Visible { span: 0..0 },
                                span: 0..0,
                            },
                            VariableModifier::Alignment {
                                value: 4,
                                span: 0..0,
                            },
                        ],
                        name: "const_values".into(),
                        array: vec![Some(2)],
                        initializer: Some(GlobalInitializer::Aggregate {
                            values: vec![
                                GlobalInitializer::Scalar {
                                    value: InitializerValue::Numeric {
                                        value: NumericLiteral::Unsigned {
                                            value: 3,
                                            span: 0..0,
                                        },
                                        span: 0..0,
                                    },
                                    span: 0..0,
                                },
                                GlobalInitializer::Scalar {
                                    value: InitializerValue::Numeric {
                                        value: NumericLiteral::Unsigned {
                                            value: 4,
                                            span: 0..0,
                                        },
                                        span: 0..0,
                                    },
                                    span: 0..0,
                                },
                            ],
                            span: 0..0,
                        }),
                        span: 0..0,
                    },
                    span: 0..0,
                },
                span: 0..0,
            },
            ModuleDirective::FunctionKernel {
                directive: FunctionKernelDirective::Func {
                    function: FuncFunction {
                        name: "helper".into(),
                        directives: vec![FunctionHeaderDirective::Linkage {
                            linkage: CodeLinkage::Visible { span: 0..0 },
                            span: 0..0,
                        }],
                        return_param: None,
                        params: vec![VariableDirective {
                            address_space: Some(AddressSpace::Param { span: 0..0 }),
                            attributes: vec![],
                            ty: Some(DataType::B32 { span: 0..0 }),
                            modifiers: vec![],
                            name: "helper_param".into(),
                            array: vec![],
                            initializer: None,
                            span: 0..0,
                        }],
                        body: FunctionBody {
                            statements: vec![
                                reg("%r0", "b32"),
                                instruction("mov.u32 %r0, %r0;"),
                                instruction("ret;"),
                            ],
                            span: 0..0,
                        },
                        span: 0..0,
                    },
                    span: 0..0,
                },
                span: 0..0,
            },
            ModuleDirective::FunctionKernel {
                directive: FunctionKernelDirective::Entry {
                    function: EntryFunction {
                        name: "step64_kernel".into(),
                        directives: vec![FunctionHeaderDirective::Linkage {
                            linkage: CodeLinkage::Visible { span: 0..0 },
                            span: 0..0,
                        }],
                        params: vec![
                            VariableDirective {
                                address_space: Some(AddressSpace::Param { span: 0..0 }),
                                attributes: vec![],
                                ty: Some(DataType::U64 { span: 0..0 }),
                                modifiers: vec![],
                                name: "param0".into(),
                                array: vec![],
                                initializer: None,
                                span: 0..0,
                            },
                            VariableDirective {
                                address_space: Some(AddressSpace::Param { span: 0..0 }),
                                attributes: vec![],
                                ty: Some(DataType::U32 { span: 0..0 }),
                                modifiers: vec![],
                                name: "param1".into(),
                                array: vec![],
                                initializer: None,
                                span: 0..0,
                            },
                        ],
                        body: FunctionBody {
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
                                            options: vec![],
                                            comment: None,
                                            span: 0..0,
                                        },
                                        span: 0..0,
                                    },
                                    span: 0..0,
                                },
                                instruction("mov.u32 %r1, %tid.x;"),
                                instruction("mov.u32 %r2, %ntid.x;"),
                                instruction("add.s32 %r1, %r1, %r2;"),
                                FunctionStatement::Directive {
                                    directive: StatementDirective::Pragma {
                                        directive: PragmaDirective {
                                            arguments: vec!["nounroll".into()],
                                            comment: None,
                                            span: 0..0,
                                        },
                                        span: 0..0,
                                    },
                                    span: 0..0,
                                },
                                label("$L_loop"),
                                FunctionStatement::Directive {
                                    directive: StatementDirective::Loc {
                                        directive: LocationDirective {
                                            file_index: 1,
                                            line: 42,
                                            column: 3,
                                            options: vec![],
                                            comment: None,
                                            span: 0..0,
                                        },
                                        span: 0..0,
                                    },
                                    span: 0..0,
                                },
                                instruction("setp.eq.s32 %p1, %r1, 0;"),
                                instruction("@%p1 bra $L_exit;"),
                                FunctionStatement::Block {
                                    statements: vec![
                                        reg("%r3", "b32"),
                                        instruction("mov.u32 %r3, %r1;"),
                                    ],
                                    span: 0..0,
                                },
                                instruction("add.s32 %r1, %r1, -1;"),
                                instruction("bra $L_loop;"),
                                label("$L_exit"),
                                instruction("ret;"),
                            ],
                            span: 0..0,
                        },
                        span: 0..0,
                    },
                    span: 0..0,
                },
                span: 0..0,
            },
        ],
        span: 0..0,
    }
}

#[allow(dead_code)]
fn reg(name: &str, ty: &str) -> FunctionStatement {
    FunctionStatement::Directive {
        directive: StatementDirective::Reg {
            directive: RegisterDirective {
                name: name.into(),
                ty: Some(ty.into()),
                range: None,
                comment: None,
                span: 0..0,
            },
            span: 0..0,
        },
        span: 0..0,
    }
}

#[allow(dead_code)]
fn instruction(source: &str) -> FunctionStatement {
    let tokens = tokenize(source).expect("tokenize instruction");
    let mut stream = PtxTokenStream::new(&tokens);
    let inst = Instruction::parse(&mut stream).expect("parse instruction");
    FunctionStatement::Instruction {
        instruction: inst,
        span: 0..0,
    }
}

#[allow(dead_code)]
fn label(name: &str) -> FunctionStatement {
    FunctionStatement::Label {
        name: name.into(),
        span: 0..0,
    }
}
