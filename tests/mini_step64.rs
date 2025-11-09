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
use ptx_parser::{PtxParser, PtxTokenStream, tokenize};

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
    assert_eq!(load_module(), expected_module());
}

fn expected_module() -> Module {
    Module {
        directives: vec![
            ModuleDirective::ModuleInfo(ModuleInfoDirectiveKind::Version(VersionDirective {
                major: 8,
                minor: 5,
            })),
            ModuleDirective::ModuleInfo(ModuleInfoDirectiveKind::Target(TargetDirective {
                entries: vec!["sm_80".into()],
                raw: "sm_80".into(),
            })),
            ModuleDirective::ModuleInfo(ModuleInfoDirectiveKind::AddressSize(
                AddressSizeDirective { size: 64 },
            )),
            ModuleDirective::Debug(ModuleDebugDirective::File(FileDirective {
                index: 1,
                path: "mini_step64.cu".into(),
            })),
            ModuleDirective::Linking(LinkingDirective {
                kind: CodeOrDataLinkage::Visible,
                prototype: "symbol_linkage".into(),
                raw: "symbol_linkage".into(),
            }),
            ModuleDirective::ModuleVariable(ModuleVariableDirective::Global(VariableDirective {
                address_space: Some(AddressSpace::Global),
                attributes: vec![],
                ty: Some(DataType::U64),
                modifiers: vec![
                    VariableModifier::Linkage(DataLinkage::Visible),
                    VariableModifier::Alignment(8),
                ],
                name: "g_data".into(),
                array: vec![Some(2)],
                initializer: Some(GlobalInitializer::Aggregate(vec![
                    GlobalInitializer::Scalar(InitializerValue::Numeric(NumericLiteral::Unsigned(
                        1,
                    ))),
                    GlobalInitializer::Scalar(InitializerValue::Numeric(NumericLiteral::Unsigned(
                        2,
                    ))),
                ])),
                raw: String::new(),
            })),
            ModuleDirective::ModuleVariable(ModuleVariableDirective::Const(VariableDirective {
                address_space: Some(AddressSpace::Const),
                attributes: vec![],
                ty: Some(DataType::B32),
                modifiers: vec![
                    VariableModifier::Linkage(DataLinkage::Visible),
                    VariableModifier::Alignment(4),
                ],
                name: "const_values".into(),
                array: vec![Some(2)],
                initializer: Some(GlobalInitializer::Aggregate(vec![
                    GlobalInitializer::Scalar(InitializerValue::Numeric(NumericLiteral::Unsigned(
                        3,
                    ))),
                    GlobalInitializer::Scalar(InitializerValue::Numeric(NumericLiteral::Unsigned(
                        4,
                    ))),
                ])),
                raw: String::new(),
            })),
            ModuleDirective::FunctionKernel(FunctionKernelDirective::Func(FuncFunction {
                name: "helper".into(),
                directives: vec![FunctionHeaderDirective::Linkage(CodeLinkage::Visible)],
                return_param: None,
                params: vec![VariableDirective {
                    address_space: Some(AddressSpace::Param),
                    attributes: vec![],
                    ty: Some(DataType::B32),
                    modifiers: vec![],
                    name: "helper_param".into(),
                    array: vec![],
                    initializer: None,
                    raw: ".param.b32 helper_param".into(),
                }],
                body: FunctionBody {
                    statements: vec![
                        reg("%r0", "b32"),
                        instruction("mov.u32 %r0, %r0;"),
                        instruction("ret;"),
                    ],
                },
            })),
            ModuleDirective::FunctionKernel(FunctionKernelDirective::Entry(EntryFunction {
                name: "step64_kernel".into(),
                directives: vec![FunctionHeaderDirective::Linkage(CodeLinkage::Visible)],
                params: vec![
                    VariableDirective {
                        address_space: Some(AddressSpace::Param),
                        attributes: vec![],
                        ty: Some(DataType::U64),
                        modifiers: vec![],
                        name: "param0".into(),
                        array: vec![],
                        initializer: None,
                        raw: ".param.u64 param0".into(),
                    },
                    VariableDirective {
                        address_space: Some(AddressSpace::Param),
                        attributes: vec![],
                        ty: Some(DataType::U32),
                        modifiers: vec![],
                        name: "param1".into(),
                        array: vec![],
                        initializer: None,
                        raw: ".param.u32 param1".into(),
                    },
                ],
                body: FunctionBody {
                    statements: vec![
                        reg("%p1", "pred"),
                        reg("%r1", "b32"),
                        reg("%r2", "b32"),
                        reg("%rd1", "b64"),
                        FunctionStatement::Directive(StatementDirective::Loc(LocationDirective {
                            file_index: 1,
                            line: 42,
                            column: 3,
                            options: vec![],
                            comment: None,
                            raw: ".loc 1 42 3".into(),
                        })),
                        instruction("mov.u32 %r1, %tid.x;"),
                        instruction("mov.u32 %r2, %ntid.x;"),
                        instruction("add.s32 %r1, %r1, %r2;"),
                        FunctionStatement::Directive(StatementDirective::Pragma(PragmaDirective {
                            arguments: vec!["nounroll".into()],
                            comment: None,
                            raw: ".pragma nounroll;".into(),
                        })),
                        label("$L_loop"),
                        FunctionStatement::Directive(StatementDirective::Loc(LocationDirective {
                            file_index: 1,
                            line: 42,
                            column: 3,
                            options: vec![],
                            comment: None,
                            raw: ".loc 1 42 3".into(),
                        })),
                        instruction("setp.eq.s32 %p1, %r1, 0;"),
                        instruction("@%p1 bra $L_exit;"),
                        FunctionStatement::Block(vec![
                            reg("%r3", "b32"),
                            instruction("mov.u32 %r3, %r1;"),
                        ]),
                        instruction("add.s32 %r1, %r1, -1;"),
                        instruction("bra $L_loop;"),
                        label("$L_exit"),
                        instruction("ret;"),
                    ],
                },
            })),
        ],
    }
}

fn reg(name: &str, ty: &str) -> FunctionStatement {
    FunctionStatement::Directive(StatementDirective::Reg(RegisterDirective {
        name: name.into(),
        ty: Some(ty.into()),
        range: None,
        comment: None,
        raw: String::new(),
    }))
}

fn instruction(source: &str) -> FunctionStatement {
    let tokens = tokenize(source).expect("tokenize instruction");
    let mut stream = PtxTokenStream::new(&tokens);
    let inst = Instruction::parse(&mut stream).expect("parse instruction");
    FunctionStatement::Instruction(inst)
}

fn label(name: &str) -> FunctionStatement {
    FunctionStatement::Label(name.into())
}
