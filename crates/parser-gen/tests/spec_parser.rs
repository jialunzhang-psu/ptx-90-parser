use ptx_parser_gen::parse_spec;
use ptx_parser_gen::r#type::{
    InstructionHead, InstructionRule, Modifier, Operand, OperandElement, OperatorToken,
    ParameterRule, Rule,
};

#[test]
fn parses_parameter_sequence() {
    assert_eq!(
        parse_parameter_rules(
            "
.collector_usage = { .collector::buffer::op };
::buffer         = { ::a };
::op             = { ::fill, ::use, ::lastuse, ::discard* };
.list = { aa::item3 };
",
        ),
        vec![
            ParameterRule {
                name: ".collector_usage".into(),
                choices: vec![Modifier::Sequence(vec![
                    ".collector".into(),
                    "::buffer".into(),
                    "::op".into()
                ])],
            },
            ParameterRule {
                name: ".list".into(),
                choices: vec![Modifier::Atom("aa::item3".into())],
            },
            ParameterRule {
                name: "::buffer".into(),
                choices: vec![Modifier::Atom("::a".into())],
            },
            ParameterRule {
                name: "::op".into(),
                choices: vec![
                    Modifier::Atom("::fill".into()),
                    Modifier::Atom("::use".into()),
                    Modifier::Atom("::lastuse".into()),
                    Modifier::Atom("::discard*".into())
                ],
            },
        ]
    );
}

#[test]
fn parses_sequence_chioces() {
    assert_eq!(
        parse_parameter_rules(
            "
.asel = .bsel = { .b.n.n.n.n };
.n = { 0, 1, 2, 3, 4, 5, 6, 7};",
        ),
        vec![
            ParameterRule {
                name: ".asel".into(),
                choices: vec![Modifier::Sequence(vec![
                    ".b".into(),
                    ".n".into(),
                    ".n".into(),
                    ".n".into(),
                    ".n".into(),
                ])],
            },
            ParameterRule {
                name: ".bsel".into(),
                choices: vec![Modifier::Sequence(vec![
                    ".b".into(),
                    ".n".into(),
                    ".n".into(),
                    ".n".into(),
                    ".n".into(),
                ])],
            },
            ParameterRule {
                name: ".n".into(),
                choices: vec![
                    Modifier::Atom("0".into()),
                    Modifier::Atom("1".into()),
                    Modifier::Atom("2".into()),
                    Modifier::Atom("3".into()),
                    Modifier::Atom("4".into()),
                    Modifier::Atom("5".into()),
                    Modifier::Atom("6".into()),
                    Modifier::Atom("7".into()),
                ],
            },
        ]
    );
}

#[test]
fn duplicates_rules_for_aliases() {
    assert_eq!(
        parse_parameter_rules(".dtype = .atype = .btype = { .u32, .s32 };"),
        vec![
            ParameterRule {
                name: ".atype".into(),
                choices: vec![Modifier::Atom(".u32".into()), Modifier::Atom(".s32".into())],
            },
            ParameterRule {
                name: ".btype".into(),
                choices: vec![Modifier::Atom(".u32".into()), Modifier::Atom(".s32".into())],
            },
            ParameterRule {
                name: ".dtype".into(),
                choices: vec![Modifier::Atom(".u32".into()), Modifier::Atom(".s32".into())],
            }
        ]
    );
}

fn parse_instruction_rules(x: &str) -> Vec<InstructionRule> {
    let instructions: Vec<InstructionRule> = parse_spec(x)
        .expect("parse")
        .into_iter()
        .flat_map(|top| top.rules)
        .filter_map(|rule| match rule {
            Rule::Instruction(instr) => Some(instr),
            _ => None,
        })
        .collect();
    instructions
}

#[test]
fn parses_operand_modifier_suffixes() {
    assert_instructions(
        parse_instruction_rules("vmad.dtype.atype.btype{.sat}{.scale}  d, a{.asel}, b{.bsel}, c;"),
        vec![InstructionRule {
            head: InstructionHead {
                opcode: "vmad".into(),
                modifiers: vec![
                    Modifier::Atom(".dtype".into()),
                    Modifier::Atom(".atype".into()),
                    Modifier::Atom(".btype".into()),
                    Modifier::Optional(".sat".into()),
                    Modifier::Optional(".scale".into()),
                ],
            },
            operands: vec![
                Operand {
                    operator: None,
                    operand: (OperandElement::Item("d".into())),
                    modifier: None,
                },
                Operand {
                    operator: None,
                    operand: (OperandElement::Item("a".into())),
                    modifier: Some(Modifier::Optional(".asel".into())),
                },
                Operand {
                    operator: None,
                    operand: (OperandElement::Item("b".into())),
                    modifier: Some(Modifier::Optional(".bsel".into())),
                },
                Operand {
                    operator: None,
                    operand: (OperandElement::Item("c".into())),
                    modifier: None,
                },
            ],
            ..InstructionRule::default()
        }],
    );
}

#[test]
fn parses_operand_modifiers() {
    assert_instructions(
        parse_instruction_rules("vmad.dtype.atype.btype{.sat}{.scale}  d, a.asel, b.bsel, c;"),
        vec![InstructionRule {
            head: InstructionHead {
                opcode: "vmad".into(),
                modifiers: vec![
                    Modifier::Atom(".dtype".into()),
                    Modifier::Atom(".atype".into()),
                    Modifier::Atom(".btype".into()),
                    Modifier::Optional(".sat".into()),
                    Modifier::Optional(".scale".into()),
                ],
            },
            operands: vec![
                Operand {
                    operator: None,
                    operand: (OperandElement::Item("d".into())),
                    modifier: None,
                },
                Operand {
                    operator: None,
                    operand: (OperandElement::Item("a".into())),
                    modifier: Some(Modifier::Atom(".asel".into())),
                },
                Operand {
                    operator: None,
                    operand: (OperandElement::Item("b".into())),
                    modifier: Some(Modifier::Atom(".bsel".into())),
                },
                Operand {
                    operator: None,
                    operand: (OperandElement::Item("c".into())),
                    modifier: None,
                },
            ],
            ..InstructionRule::default()
        }],
    );
}

#[test]
fn parses_call() {
    let head = InstructionHead {
        opcode: "call".into(),
        modifiers: vec![Modifier::Optional(".uni".into())],
    };

    assert_instructions(
        parse_instruction_rules(
            "call{.uni} (ret-param), func, (param-list);
call{.uni} func, (param-list);
call{.uni} func;
call{.uni} (ret-param), fptr, (param-list), flist;
call{.uni} fptr, (param-list), flist;
call{.uni} fptr, flist;
call{.uni} (ret-param), fptr, (param-list), fproto;
call{.uni} fptr, (param-list), fproto;
call{.uni} fptr, fproto;
",
        ),
        vec![
            InstructionRule {
                head: head.clone(),
                operands: vec![
                    new_simple_operand(OperandElement::ParenthesizedOperand("ret-param".into())),
                    new_simple_operand(OperandElement::Item("func".into())),
                    new_simple_operand(OperandElement::ParamList),
                ],
                ..InstructionRule::default()
            },
            InstructionRule {
                head: head.clone(),
                operands: vec![
                    new_simple_operand(OperandElement::Item("func".into())),
                    new_simple_operand(OperandElement::ParamList),
                ],
                ..InstructionRule::default()
            },
            InstructionRule {
                head: head.clone(),
                operands: vec![new_simple_operand(OperandElement::Item("func".into()))],
                ..InstructionRule::default()
            },
            InstructionRule {
                head: head.clone(),
                operands: vec![
                    new_simple_operand(OperandElement::ParenthesizedOperand("ret-param".into())),
                    new_simple_operand(OperandElement::Item("fptr".into())),
                    new_simple_operand(OperandElement::ParamList),
                    new_simple_operand(OperandElement::Item("flist".into())),
                ],
                ..InstructionRule::default()
            },
            InstructionRule {
                head: head.clone(),
                operands: vec![
                    new_simple_operand(OperandElement::Item("fptr".into())),
                    new_simple_operand(OperandElement::ParamList),
                    new_simple_operand(OperandElement::Item("flist".into())),
                ],
                ..InstructionRule::default()
            },
            InstructionRule {
                head: head.clone(),
                operands: vec![
                    new_simple_operand(OperandElement::Item("fptr".into())),
                    new_simple_operand(OperandElement::Item("flist".into())),
                ],
                ..InstructionRule::default()
            },
            InstructionRule {
                head: head.clone(),
                operands: vec![
                    new_simple_operand(OperandElement::ParenthesizedOperand("ret-param".into())),
                    new_simple_operand(OperandElement::Item("fptr".into())),
                    new_simple_operand(OperandElement::ParamList),
                    new_simple_operand(OperandElement::Item("fproto".into())),
                ],
                ..InstructionRule::default()
            },
            InstructionRule {
                head: head.clone(),
                operands: vec![
                    new_simple_operand(OperandElement::Item("fptr".into())),
                    new_simple_operand(OperandElement::ParamList),
                    new_simple_operand(OperandElement::Item("fproto".into())),
                ],
                ..InstructionRule::default()
            },
            InstructionRule {
                head: head.clone(),
                operands: vec![
                    new_simple_operand(OperandElement::Item("fptr".into())),
                    new_simple_operand(OperandElement::Item("fproto".into())),
                ],
                ..InstructionRule::default()
            },
        ],
    );
}

#[test]
fn parse_cluster_launch() {
    let spec = "
clusterlaunchcontrol.query_cancel.is_canceled.pred.b128 pred, try_cancel_response;
clusterlaunchcontrol.query_cancel.get_first_ctaid.v4.b32.b128 {xdim, ydim, zdim, _},  try_cancel_response;
clusterlaunchcontrol.query_cancel{.get_first_ctaid::dimension}.b32.b128 reg, try_cancel_response;
.get_first_ctaid::dimension = { .get_first_ctaid::x, .get_first_ctaid::y, .get_first_ctaid::z };";

    let instructions = parse_instruction_rules(spec);
    let params = parse_parameter_rules(spec);

    let expected_instructions = vec![
        InstructionRule {
            head: InstructionHead {
                opcode: "clusterlaunchcontrol".into(),
                modifiers: vec![
                    Modifier::Atom(".query_cancel".into()),
                    Modifier::Atom(".is_canceled".into()),
                    Modifier::Atom(".pred".into()),
                    Modifier::Atom(".b128".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("pred".into())),
                new_simple_operand(OperandElement::Item("try_cancel_response".into())),
            ],
            ..InstructionRule::default()
        },
        InstructionRule {
            head: InstructionHead {
                opcode: "clusterlaunchcontrol".into(),
                modifiers: vec![
                    Modifier::Atom(".query_cancel".into()),
                    Modifier::Atom(".get_first_ctaid".into()),
                    Modifier::Atom(".v4".into()),
                    Modifier::Atom(".b32".into()),
                    Modifier::Atom(".b128".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::CurlyGroup(vec![
                    new_simple_operand(OperandElement::Item("xdim".into())),
                    new_simple_operand(OperandElement::Item("ydim".into())),
                    new_simple_operand(OperandElement::Item("zdim".into())),
                    new_simple_operand(OperandElement::Item("_".into())),
                ])),
                new_simple_operand(OperandElement::Item("try_cancel_response".into())),
            ],
            ..InstructionRule::default()
        },
        InstructionRule {
            head: InstructionHead {
                opcode: "clusterlaunchcontrol".into(),
                modifiers: vec![
                    Modifier::Atom(".query_cancel".into()),
                    Modifier::Optional(".get_first_ctaid::dimension".into()),
                    Modifier::Atom(".b32".into()),
                    Modifier::Atom(".b128".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("reg".into())),
                new_simple_operand(OperandElement::Item("try_cancel_response".into())),
            ],
            ..InstructionRule::default()
        },
    ];

    assert_instructions(instructions, expected_instructions);

    let expected_params = vec![ParameterRule {
        name: ".get_first_ctaid::dimension".into(),
        choices: vec![
            Modifier::Atom(".get_first_ctaid::x".into()),
            Modifier::Atom(".get_first_ctaid::y".into()),
            Modifier::Atom(".get_first_ctaid::z".into()),
        ],
    }];
    assert_eq!(params, expected_params);
}

#[test]
fn parse_tcgen05_cp() {
    let spec = "
tcgen05.cp.cta_group.shape{.multicast}{.dst_src_fmt} [taddr], s-desc;
.cta_group = { .cta_group::1, .cta_group::2 };
.dst_src_fmt   = { .b8x16.b6x16_p32, .b8x16.b4x16_p64 };
.shape     = { .128x256b, .4x256b, .128x128b, .64x128b**, .32x128b*** };
.multicast = { .warpx2::02_13** , .warpx2::01_23**, .warpx4*** };
";

    let instructions = parse_instruction_rules(spec);
    let mut params = parse_parameter_rules(spec);

    let expected_instructions = vec![InstructionRule {
        head: InstructionHead {
            opcode: "tcgen05".into(),
            modifiers: vec![
                Modifier::Atom(".cp".into()),
                Modifier::Atom(".cta_group".into()),
                Modifier::Atom(".shape".into()),
                Modifier::Optional(".multicast".into()),
                Modifier::Optional(".dst_src_fmt".into()),
            ],
        },
        operands: vec![
            new_simple_operand(OperandElement::Address("taddr".into())),
            new_simple_operand(OperandElement::Item("s-desc".into())),
        ],
        ..InstructionRule::default()
    }];

    assert_instructions(instructions, expected_instructions);

    params.sort_by(|a, b| a.name.cmp(&b.name));
    let expected_params = vec![
        ParameterRule {
            name: ".cta_group".into(),
            choices: vec![
                Modifier::Atom(".cta_group::1".into()),
                Modifier::Atom(".cta_group::2".into()),
            ],
        },
        ParameterRule {
            name: ".dst_src_fmt".into(),
            choices: vec![
                Modifier::Atom(".b8x16.b6x16_p32".into()),
                Modifier::Atom(".b8x16.b4x16_p64".into()),
            ],
        },
        ParameterRule {
            name: ".multicast".into(),
            choices: vec![
                Modifier::Atom(".warpx2::02_13**".into()),
                Modifier::Atom(".warpx2::01_23**".into()),
                Modifier::Atom(".warpx4***".into()),
            ],
        },
        ParameterRule {
            name: ".shape".into(),
            choices: vec![
                Modifier::Atom(".128x256b".into()),
                Modifier::Atom(".4x256b".into()),
                Modifier::Atom(".128x128b".into()),
                Modifier::Atom(".64x128b**".into()),
                Modifier::Atom(".32x128b***".into()),
            ],
        },
    ];
    assert_eq!(params, expected_params);
}

#[test]
fn parse_bar() {
    let spec = "
barrier{.cta}.sync{.aligned}      a{, b};
barrier{.cta}.arrive{.aligned}    a, b;
barrier{.cta}.red.popc{.aligned}.u32  d, a{, b}, {!}c;
barrier{.cta}.red.op{.aligned}.pred   p, a{, b}, {!}c;
bar{.cta}.sync      a{, b};
bar{.cta}.arrive    a, b;
bar{.cta}.red.popc.u32  d, a{, b}, {!}c;
bar{.cta}.red.op.pred   p, a{, b}, {!}c;
.op = { .and, .or };";

    let instructions = parse_instruction_rules(spec);
    let params = parse_parameter_rules(spec);

    let expected_instructions = vec![
        // barrier{.cta}.sync{.aligned} a{, b};
        InstructionRule {
            head: InstructionHead {
                opcode: "barrier".into(),
                modifiers: vec![
                    Modifier::Optional(".cta".into()),
                    Modifier::Atom(".sync".into()),
                    Modifier::Optional(".aligned".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Optional("b".into())),
            ],
            ..InstructionRule::default()
        },
        // barrier{.cta}.arrive{.aligned} a, b;
        InstructionRule {
            head: InstructionHead {
                opcode: "barrier".into(),
                modifiers: vec![
                    Modifier::Optional(".cta".into()),
                    Modifier::Atom(".arrive".into()),
                    Modifier::Optional(".aligned".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
            ],
            ..InstructionRule::default()
        },
        // barrier{.cta}.red.popc{.aligned}.u32 d, a{, b}, {!}c;
        InstructionRule {
            head: InstructionHead {
                opcode: "barrier".into(),
                modifiers: vec![
                    Modifier::Optional(".cta".into()),
                    Modifier::Atom(".red".into()),
                    Modifier::Atom(".popc".into()),
                    Modifier::Optional(".aligned".into()),
                    Modifier::Atom(".u32".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Optional("b".into())),
                Operand {
                    operator: Some(OperatorToken::LogicalNot),
                    operand: OperandElement::Item("c".into()),
                    modifier: None,
                },
            ],
            ..InstructionRule::default()
        },
        // barrier{.cta}.red.op{.aligned}.pred p, a{, b}, {!}c;
        InstructionRule {
            head: InstructionHead {
                opcode: "barrier".into(),
                modifiers: vec![
                    Modifier::Optional(".cta".into()),
                    Modifier::Atom(".red".into()),
                    Modifier::Atom(".op".into()),
                    Modifier::Optional(".aligned".into()),
                    Modifier::Atom(".pred".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("p".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Optional("b".into())),
                Operand {
                    operator: Some(OperatorToken::LogicalNot),
                    operand: OperandElement::Item("c".into()),
                    modifier: None,
                },
            ],
            ..InstructionRule::default()
        },
        // bar{.cta}.sync a{, b};
        InstructionRule {
            head: InstructionHead {
                opcode: "bar".into(),
                modifiers: vec![
                    Modifier::Optional(".cta".into()),
                    Modifier::Atom(".sync".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Optional("b".into())),
            ],
            ..InstructionRule::default()
        },
        // bar{.cta}.arrive a, b;
        InstructionRule {
            head: InstructionHead {
                opcode: "bar".into(),
                modifiers: vec![
                    Modifier::Optional(".cta".into()),
                    Modifier::Atom(".arrive".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
            ],
            ..InstructionRule::default()
        },
        // bar{.cta}.red.popc.u32 d, a{, b}, {!}c;
        InstructionRule {
            head: InstructionHead {
                opcode: "bar".into(),
                modifiers: vec![
                    Modifier::Optional(".cta".into()),
                    Modifier::Atom(".red".into()),
                    Modifier::Atom(".popc".into()),
                    Modifier::Atom(".u32".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Optional("b".into())),
                Operand {
                    operator: Some(OperatorToken::LogicalNot),
                    operand: OperandElement::Item("c".into()),
                    modifier: None,
                },
            ],
            ..InstructionRule::default()
        },
        // bar{.cta}.red.op.pred p, a{, b}, {!}c;
        InstructionRule {
            head: InstructionHead {
                opcode: "bar".into(),
                modifiers: vec![
                    Modifier::Optional(".cta".into()),
                    Modifier::Atom(".red".into()),
                    Modifier::Atom(".op".into()),
                    Modifier::Atom(".pred".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("p".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Optional("b".into())),
                Operand {
                    operator: Some(OperatorToken::LogicalNot),
                    operand: OperandElement::Item("c".into()),
                    modifier: None,
                },
            ],
            ..InstructionRule::default()
        },
    ];

    assert_instructions(instructions, expected_instructions);

    let expected_params = vec![ParameterRule {
        name: ".op".into(),
        choices: vec![Modifier::Atom(".and".into()), Modifier::Atom(".or".into())],
    }];
    assert_eq!(params, expected_params);
}
#[test]
fn parse_tex() {
    let spec = "
tex.geom.v4.dtype.ctype  d, [a, c] {, e} {, f};
tex.geom.v4.dtype.ctype  d{|p}, [a, b, c] {, e} {, f};
tex.geom.v2.f16x2.ctype  d{|p}, [a, c] {, e} {, f};
tex.geom.v2.f16x2.ctype  d{|p}, [a, b, c] {, e} {, f};
tex.base.geom.v4.dtype.ctype   d{|p}, [a, {b,} c] {, e} {, f};
tex.level.geom.v4.dtype.ctype  d{|p}, [a, {b,} c], lod {, e} {, f};
tex.grad.geom.v4.dtype.ctype   d{|p}, [a, {b,} c], dPdx, dPdy {, e} {, f};
tex.base.geom.v2.f16x2.ctype   d{|p}, [a, {b,} c] {, e} {, f};
tex.level.geom.v2.f16x2.ctype  d{|p}, [a, {b,} c], lod {, e} {, f};
tex.grad.geom.v2.f16x2.ctype   d{|p}, [a, {b,} c], dPdx, dPdy {, e} {, f};
.geom  = { .1d, .2d, .3d, .a1d, .a2d, .cube, .acube, .2dms, .a2dms };
.dtype = { .u32, .s32, .f16,  .f32 };
.ctype = {       .s32, .f32 };
";

    let instructions = parse_instruction_rules(spec);
    let mut params = parse_parameter_rules(spec);

    let expected_instructions = vec![
        // tex.geom.v4.dtype.ctype  d, [a, c] {, e} {, f};
        InstructionRule {
            head: InstructionHead {
                opcode: "tex".into(),
                modifiers: vec![
                    Modifier::Atom(".geom".into()),
                    Modifier::Atom(".v4".into()),
                    Modifier::Atom(".dtype".into()),
                    Modifier::Atom(".ctype".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::SquareGroup(vec![
                    new_simple_operand(OperandElement::Item("a".into())),
                    new_simple_operand(OperandElement::Item("c".into())),
                ])),
                new_simple_operand(OperandElement::Optional("e".into())),
                new_simple_operand(OperandElement::Optional("f".into())),
            ],
            ..InstructionRule::default()
        },
        // tex.geom.v4.dtype.ctype  d{|p}, [a, b, c] {, e} {, f};
        InstructionRule {
            head: InstructionHead {
                opcode: "tex".into(),
                modifiers: vec![
                    Modifier::Atom(".geom".into()),
                    Modifier::Atom(".v4".into()),
                    Modifier::Atom(".dtype".into()),
                    Modifier::Atom(".ctype".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::PipeOptionalChoice(("d".into(), "p".into()))),
                new_simple_operand(OperandElement::SquareGroup(vec![
                    new_simple_operand(OperandElement::Item("a".into())),
                    new_simple_operand(OperandElement::Item("b".into())),
                    new_simple_operand(OperandElement::Item("c".into())),
                ])),
                new_simple_operand(OperandElement::Optional("e".into())),
                new_simple_operand(OperandElement::Optional("f".into())),
            ],
            ..InstructionRule::default()
        },
        // tex.geom.v2.f16x2.ctype  d{|p}, [a, c] {, e} {, f};
        InstructionRule {
            head: InstructionHead {
                opcode: "tex".into(),
                modifiers: vec![
                    Modifier::Atom(".geom".into()),
                    Modifier::Atom(".v2".into()),
                    Modifier::Atom(".f16x2".into()),
                    Modifier::Atom(".ctype".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::PipeOptionalChoice(("d".into(), "p".into()))),
                new_simple_operand(OperandElement::SquareGroup(vec![
                    new_simple_operand(OperandElement::Item("a".into())),
                    new_simple_operand(OperandElement::Item("c".into())),
                ])),
                new_simple_operand(OperandElement::Optional("e".into())),
                new_simple_operand(OperandElement::Optional("f".into())),
            ],
            ..InstructionRule::default()
        },
        // tex.geom.v2.f16x2.ctype  d{|p}, [a, b, c] {, e} {, f};
        InstructionRule {
            head: InstructionHead {
                opcode: "tex".into(),
                modifiers: vec![
                    Modifier::Atom(".geom".into()),
                    Modifier::Atom(".v2".into()),
                    Modifier::Atom(".f16x2".into()),
                    Modifier::Atom(".ctype".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::PipeOptionalChoice(("d".into(), "p".into()))),
                new_simple_operand(OperandElement::SquareGroup(vec![
                    new_simple_operand(OperandElement::Item("a".into())),
                    new_simple_operand(OperandElement::Item("b".into())),
                    new_simple_operand(OperandElement::Item("c".into())),
                ])),
                new_simple_operand(OperandElement::Optional("e".into())),
                new_simple_operand(OperandElement::Optional("f".into())),
            ],
            ..InstructionRule::default()
        },
        // tex.base.geom.v4.dtype.ctype   d{|p}, [a, {b,} c] {, e} {, f};
        InstructionRule {
            head: InstructionHead {
                opcode: "tex".into(),
                modifiers: vec![
                    Modifier::Atom(".base".into()),
                    Modifier::Atom(".geom".into()),
                    Modifier::Atom(".v4".into()),
                    Modifier::Atom(".dtype".into()),
                    Modifier::Atom(".ctype".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::PipeOptionalChoice(("d".into(), "p".into()))),
                new_simple_operand(OperandElement::SquareGroup(vec![
                    new_simple_operand(OperandElement::Item("a".into())),
                    new_simple_operand(OperandElement::Optional("b".into())),
                    new_simple_operand(OperandElement::Item("c".into())),
                ])),
                new_simple_operand(OperandElement::Optional("e".into())),
                new_simple_operand(OperandElement::Optional("f".into())),
            ],
            ..InstructionRule::default()
        },
        // tex.level.geom.v4.dtype.ctype  d{|p}, [a, {b,} c], lod {, e} {, f};
        InstructionRule {
            head: InstructionHead {
                opcode: "tex".into(),
                modifiers: vec![
                    Modifier::Atom(".level".into()),
                    Modifier::Atom(".geom".into()),
                    Modifier::Atom(".v4".into()),
                    Modifier::Atom(".dtype".into()),
                    Modifier::Atom(".ctype".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::PipeOptionalChoice(("d".into(), "p".into()))),
                new_simple_operand(OperandElement::SquareGroup(vec![
                    new_simple_operand(OperandElement::Item("a".into())),
                    new_simple_operand(OperandElement::Optional("b".into())),
                    new_simple_operand(OperandElement::Item("c".into())),
                ])),
                new_simple_operand(OperandElement::Item("lod".into())),
                new_simple_operand(OperandElement::Optional("e".into())),
                new_simple_operand(OperandElement::Optional("f".into())),
            ],
            ..InstructionRule::default()
        },
        // tex.grad.geom.v4.dtype.ctype   d{|p}, [a, {b,} c], dPdx, dPdy {, e} {, f};
        InstructionRule {
            head: InstructionHead {
                opcode: "tex".into(),
                modifiers: vec![
                    Modifier::Atom(".grad".into()),
                    Modifier::Atom(".geom".into()),
                    Modifier::Atom(".v4".into()),
                    Modifier::Atom(".dtype".into()),
                    Modifier::Atom(".ctype".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::PipeOptionalChoice(("d".into(), "p".into()))),
                new_simple_operand(OperandElement::SquareGroup(vec![
                    new_simple_operand(OperandElement::Item("a".into())),
                    new_simple_operand(OperandElement::Optional("b".into())),
                    new_simple_operand(OperandElement::Item("c".into())),
                ])),
                new_simple_operand(OperandElement::Item("dPdx".into())),
                new_simple_operand(OperandElement::Item("dPdy".into())),
                new_simple_operand(OperandElement::Optional("e".into())),
                new_simple_operand(OperandElement::Optional("f".into())),
            ],
            ..InstructionRule::default()
        },
        // tex.base.geom.v2.f16x2.ctype   d{|p}, [a, {b,} c] {, e} {, f};
        InstructionRule {
            head: InstructionHead {
                opcode: "tex".into(),
                modifiers: vec![
                    Modifier::Atom(".base".into()),
                    Modifier::Atom(".geom".into()),
                    Modifier::Atom(".v2".into()),
                    Modifier::Atom(".f16x2".into()),
                    Modifier::Atom(".ctype".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::PipeOptionalChoice(("d".into(), "p".into()))),
                new_simple_operand(OperandElement::SquareGroup(vec![
                    new_simple_operand(OperandElement::Item("a".into())),
                    new_simple_operand(OperandElement::Optional("b".into())),
                    new_simple_operand(OperandElement::Item("c".into())),
                ])),
                new_simple_operand(OperandElement::Optional("e".into())),
                new_simple_operand(OperandElement::Optional("f".into())),
            ],
            ..InstructionRule::default()
        },
        // tex.level.geom.v2.f16x2.ctype  d{|p}, [a, {b,} c], lod {, e} {, f};
        InstructionRule {
            head: InstructionHead {
                opcode: "tex".into(),
                modifiers: vec![
                    Modifier::Atom(".level".into()),
                    Modifier::Atom(".geom".into()),
                    Modifier::Atom(".v2".into()),
                    Modifier::Atom(".f16x2".into()),
                    Modifier::Atom(".ctype".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::PipeOptionalChoice(("d".into(), "p".into()))),
                new_simple_operand(OperandElement::SquareGroup(vec![
                    new_simple_operand(OperandElement::Item("a".into())),
                    new_simple_operand(OperandElement::Optional("b".into())),
                    new_simple_operand(OperandElement::Item("c".into())),
                ])),
                new_simple_operand(OperandElement::Item("lod".into())),
                new_simple_operand(OperandElement::Optional("e".into())),
                new_simple_operand(OperandElement::Optional("f".into())),
            ],
            ..InstructionRule::default()
        },
        // tex.grad.geom.v2.f16x2.ctype   d{|p}, [a, {b,} c], dPdx, dPdy {, e} {, f};
        InstructionRule {
            head: InstructionHead {
                opcode: "tex".into(),
                modifiers: vec![
                    Modifier::Atom(".grad".into()),
                    Modifier::Atom(".geom".into()),
                    Modifier::Atom(".v2".into()),
                    Modifier::Atom(".f16x2".into()),
                    Modifier::Atom(".ctype".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::PipeOptionalChoice(("d".into(), "p".into()))),
                new_simple_operand(OperandElement::SquareGroup(vec![
                    new_simple_operand(OperandElement::Item("a".into())),
                    new_simple_operand(OperandElement::Optional("b".into())),
                    new_simple_operand(OperandElement::Item("c".into())),
                ])),
                new_simple_operand(OperandElement::Item("dPdx".into())),
                new_simple_operand(OperandElement::Item("dPdy".into())),
                new_simple_operand(OperandElement::Optional("e".into())),
                new_simple_operand(OperandElement::Optional("f".into())),
            ],
            ..InstructionRule::default()
        },
    ];

    assert_instructions(instructions, expected_instructions);

    params.sort_by(|a, b| a.name.cmp(&b.name));
    let expected_params = vec![
            ParameterRule {
                name: ".ctype".into(),
                choices: vec![Modifier::Atom(".s32".into()), Modifier::Atom(".f32".into())],
            },
            ParameterRule {
                name: ".dtype".into(),
                choices: vec![
                    Modifier::Atom(".u32".into()),
                    Modifier::Atom(".s32".into()),
                    Modifier::Atom(".f16".into()),
                    Modifier::Atom(".f32".into()),
                ],
            },
            ParameterRule {
                name: ".geom".into(),
                choices: vec![
                    Modifier::Atom(".1d".into()),
                    Modifier::Atom(".2d".into()),
                    Modifier::Atom(".3d".into()),
                    Modifier::Atom(".a1d".into()),
                    Modifier::Atom(".a2d".into()),
                    Modifier::Atom(".cube".into()),
                    Modifier::Atom(".acube".into()),
                    Modifier::Atom(".2dms".into()),
                    Modifier::Atom(".a2dms".into()),
                ],
            },
        ];
    assert_eq!(params, expected_params);
}

#[test]
fn parse_atom() {
    let src = "// Atomic operation with scalar type:
atom{.sem}{.scope}{.space}.op{.level::cache_hint}.type d, [a], b{, cache-policy};
atom{.sem}{.scope}{.space}.op.type d, [a], b, c;
atom{.sem}{.scope}{.space}.cas.b16 d, [a], b, c;
atom{.sem}{.scope}{.space}.cas.b128 d, [a], b, c;
atom{.sem}{.scope}{.space}.exch{.level::cache_hint}.b128 d, [a], b {, cache-policy};
atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16     d, [a], b{, cache-policy};
atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16x2   d, [a], b{, cache-policy};
atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16    d, [a], b{, cache-policy};
atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16x2  d, [a], b{, cache-policy};
.space =              { .global, .shared, .shared::cta, .shared::cluster};
.sem =                { .relaxed, .acquire, .release, .acq_rel };
.scope =              { .cta, .cluster, .gpu, .sys };
.op =                 { .and, .or, .xor, .cas, .exch, .add, .inc, .dec, .min, .max };
.level::cache_hint =  { .L2::cache_hint };
.type =               { .b32, .b64, .u32, .u64, .s32, .s64, .f32, .f64 };
-------------------------------------------------------------
// Atomic operation with vector type:
atom{.sem}{.scope}{.global}.add{.level::cache_hint}.vec_32_bit.f32                  d, [a], b{, cache-policy};
atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_16_bit.half_word_type  d, [a], b{, cache-policy};
atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_32_bit.packed_type     d, [a], b{, cache-policy};
.sem =               { .relaxed, .acquire, .release, .acq_rel };
.scope =             { .cta, .cluster, .gpu, .sys };
.op =                { .add, .min, .max };
.half_word_type =    { .f16, .bf16 };
.packed_type =       { .f16x2, .bf16x2 };
.vec_16_bit =        { .v2, .v4, .v8 };
.vec_32_bit =        { .v2, .v4 };
.level::cache_hint = { .L2::cache_hint };";

    let parsed = parse_spec(src).expect("parse atom spec");
    assert_eq!(parsed.len(), 2, "should have two top-level sections");
    let rules1 = parsed[0].rules.clone();
    let rules2 = parsed[1].rules.clone();

    // Check section 1 instructions
    let instructions1: Vec<InstructionRule> = rules1
        .iter()
        .filter_map(|rule| match rule {
            Rule::Instruction(instr) => Some(instr.clone()),
            _ => None,
        })
        .collect();
    // Check section 2 instructions
    let instructions2: Vec<InstructionRule> = rules2
        .iter()
        .filter_map(|rule| match rule {
            Rule::Instruction(instr) => Some(instr.clone()),
            _ => None,
        })
        .collect();

    let expected = vec![
        // atom{.sem}{.scope}{.space}.op{.level::cache_hint}.type d, [a], b{, cache-policy};
        InstructionRule {
            head: InstructionHead {
                opcode: "atom".into(),
                modifiers: vec![
                    Modifier::Optional(".sem".into()),
                    Modifier::Optional(".scope".into()),
                    Modifier::Optional(".space".into()),
                    Modifier::Atom(".op".into()),
                    Modifier::Optional(".level::cache_hint".into()),
                    Modifier::Atom(".type".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Address("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Optional("cache-policy".into())),
            ],
            ..InstructionRule::default()
        },
        // atom{.sem}{.scope}{.space}.op.type d, [a], b, c;
        InstructionRule {
            head: InstructionHead {
                opcode: "atom".into(),
                modifiers: vec![
                    Modifier::Optional(".sem".into()),
                    Modifier::Optional(".scope".into()),
                    Modifier::Optional(".space".into()),
                    Modifier::Atom(".op".into()),
                    Modifier::Atom(".type".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Address("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Item("c".into())),
            ],
            ..InstructionRule::default()
        },
        // atom{.sem}{.scope}{.space}.cas.b16 d, [a], b, c;
        InstructionRule {
            head: InstructionHead {
                opcode: "atom".into(),
                modifiers: vec![
                    Modifier::Optional(".sem".into()),
                    Modifier::Optional(".scope".into()),
                    Modifier::Optional(".space".into()),
                    Modifier::Atom(".cas".into()),
                    Modifier::Atom(".b16".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Address("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Item("c".into())),
            ],
            ..InstructionRule::default()
        },
        // atom{.sem}{.scope}{.space}.cas.b128 d, [a], b, c;
        InstructionRule {
            head: InstructionHead {
                opcode: "atom".into(),
                modifiers: vec![
                    Modifier::Optional(".sem".into()),
                    Modifier::Optional(".scope".into()),
                    Modifier::Optional(".space".into()),
                    Modifier::Atom(".cas".into()),
                    Modifier::Atom(".b128".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Address("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Item("c".into())),
            ],
            ..InstructionRule::default()
        },
        // atom{.sem}{.scope}{.space}.exch{.level::cache_hint}.b128 d, [a], b {, cache-policy};
        InstructionRule {
            head: InstructionHead {
                opcode: "atom".into(),
                modifiers: vec![
                    Modifier::Optional(".sem".into()),
                    Modifier::Optional(".scope".into()),
                    Modifier::Optional(".space".into()),
                    Modifier::Atom(".exch".into()),
                    Modifier::Optional(".level::cache_hint".into()),
                    Modifier::Atom(".b128".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Address("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Optional("cache-policy".into())),
            ],
            ..InstructionRule::default()
        },
        // atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16 d, [a], b{, cache-policy};
        InstructionRule {
            head: InstructionHead {
                opcode: "atom".into(),
                modifiers: vec![
                    Modifier::Optional(".sem".into()),
                    Modifier::Optional(".scope".into()),
                    Modifier::Optional(".space".into()),
                    Modifier::Atom(".add".into()),
                    Modifier::Atom(".noftz".into()),
                    Modifier::Optional(".level::cache_hint".into()),
                    Modifier::Atom(".f16".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Address("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Optional("cache-policy".into())),
            ],
            ..InstructionRule::default()
        },
        // atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16x2 d, [a], b{, cache-policy};
        InstructionRule {
            head: InstructionHead {
                opcode: "atom".into(),
                modifiers: vec![
                    Modifier::Optional(".sem".into()),
                    Modifier::Optional(".scope".into()),
                    Modifier::Optional(".space".into()),
                    Modifier::Atom(".add".into()),
                    Modifier::Atom(".noftz".into()),
                    Modifier::Optional(".level::cache_hint".into()),
                    Modifier::Atom(".f16x2".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Address("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Optional("cache-policy".into())),
            ],
            ..InstructionRule::default()
        },
        // atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16 d, [a], b{, cache-policy};
        InstructionRule {
            head: InstructionHead {
                opcode: "atom".into(),
                modifiers: vec![
                    Modifier::Optional(".sem".into()),
                    Modifier::Optional(".scope".into()),
                    Modifier::Optional(".space".into()),
                    Modifier::Atom(".add".into()),
                    Modifier::Atom(".noftz".into()),
                    Modifier::Optional(".level::cache_hint".into()),
                    Modifier::Atom(".bf16".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Address("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Optional("cache-policy".into())),
            ],
            ..InstructionRule::default()
        },
        // atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16x2 d, [a], b{, cache-policy};
        InstructionRule {
            head: InstructionHead {
                opcode: "atom".into(),
                modifiers: vec![
                    Modifier::Optional(".sem".into()),
                    Modifier::Optional(".scope".into()),
                    Modifier::Optional(".space".into()),
                    Modifier::Atom(".add".into()),
                    Modifier::Atom(".noftz".into()),
                    Modifier::Optional(".level::cache_hint".into()),
                    Modifier::Atom(".bf16x2".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Address("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Optional("cache-policy".into())),
            ],
            ..InstructionRule::default()
        },
        // atom{.sem}{.scope}{.global}.add{.level::cache_hint}.vec_32_bit.f32 d, [a], b{, cache-policy};
        InstructionRule {
            head: InstructionHead {
                opcode: "atom".into(),
                modifiers: vec![
                    Modifier::Optional(".sem".into()),
                    Modifier::Optional(".scope".into()),
                    Modifier::Optional(".global".into()),
                    Modifier::Atom(".add".into()),
                    Modifier::Optional(".level::cache_hint".into()),
                    Modifier::Atom(".vec_32_bit".into()),
                    Modifier::Atom(".f32".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Address("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Optional("cache-policy".into())),
            ],
            ..InstructionRule::default()
        },
        // atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_16_bit.half_word_type d, [a], b{, cache-policy};
        InstructionRule {
            head: InstructionHead {
                opcode: "atom".into(),
                modifiers: vec![
                    Modifier::Optional(".sem".into()),
                    Modifier::Optional(".scope".into()),
                    Modifier::Optional(".global".into()),
                    Modifier::Atom(".op".into()),
                    Modifier::Atom(".noftz".into()),
                    Modifier::Optional(".level::cache_hint".into()),
                    Modifier::Atom(".vec_16_bit".into()),
                    Modifier::Atom(".half_word_type".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Address("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Optional("cache-policy".into())),
            ],
            ..InstructionRule::default()
        },
        // atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_32_bit.packed_type d, [a], b{, cache-policy};
        InstructionRule {
            head: InstructionHead {
                opcode: "atom".into(),
                modifiers: vec![
                    Modifier::Optional(".sem".into()),
                    Modifier::Optional(".scope".into()),
                    Modifier::Optional(".global".into()),
                    Modifier::Atom(".op".into()),
                    Modifier::Atom(".noftz".into()),
                    Modifier::Optional(".level::cache_hint".into()),
                    Modifier::Atom(".vec_32_bit".into()),
                    Modifier::Atom(".packed_type".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Address("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Optional("cache-policy".into())),
            ],
            ..InstructionRule::default()
        },
    ];

    // Split expected into two sections: first 9 instructions are from section 1, last 3 are from section 2
    let expected1 = expected[0..9].to_vec();
    let expected2 = expected[9..12].to_vec();

    assert_instructions(instructions1, expected1);
    assert_instructions(instructions2, expected2);

    // Check section 1 parameters
    let mut params1: Vec<ParameterRule> = rules1
        .iter()
        .filter_map(|rule| match rule {
            Rule::Parameter(param) => Some(param.clone()),
            _ => None,
        })
        .collect();
    params1.sort_by(|a, b| a.name.cmp(&b.name));
    assert_eq!(
        params1,
        vec![
            ParameterRule {
                name: ".level::cache_hint".into(),
                choices: vec![Modifier::Atom(".L2::cache_hint".into())],
            },
            ParameterRule {
                name: ".op".into(),
                choices: vec![
                    Modifier::Atom(".and".into()),
                    Modifier::Atom(".or".into()),
                    Modifier::Atom(".xor".into()),
                    Modifier::Atom(".cas".into()),
                    Modifier::Atom(".exch".into()),
                    Modifier::Atom(".add".into()),
                    Modifier::Atom(".inc".into()),
                    Modifier::Atom(".dec".into()),
                    Modifier::Atom(".min".into()),
                    Modifier::Atom(".max".into()),
                ],
            },
            ParameterRule {
                name: ".scope".into(),
                choices: vec![
                    Modifier::Atom(".cta".into()),
                    Modifier::Atom(".cluster".into()),
                    Modifier::Atom(".gpu".into()),
                    Modifier::Atom(".sys".into()),
                ],
            },
            ParameterRule {
                name: ".sem".into(),
                choices: vec![
                    Modifier::Atom(".relaxed".into()),
                    Modifier::Atom(".acquire".into()),
                    Modifier::Atom(".release".into()),
                    Modifier::Atom(".acq_rel".into()),
                ],
            },
            ParameterRule {
                name: ".space".into(),
                choices: vec![
                    Modifier::Atom(".global".into()),
                    Modifier::Atom(".shared".into()),
                    Modifier::Atom(".shared::cta".into()),
                    Modifier::Atom(".shared::cluster".into()),
                ],
            },
            ParameterRule {
                name: ".type".into(),
                choices: vec![
                    Modifier::Atom(".b32".into()),
                    Modifier::Atom(".b64".into()),
                    Modifier::Atom(".u32".into()),
                    Modifier::Atom(".u64".into()),
                    Modifier::Atom(".s32".into()),
                    Modifier::Atom(".s64".into()),
                    Modifier::Atom(".f32".into()),
                    Modifier::Atom(".f64".into()),
                ],
            },
        ]
    );

    // Check section 2 parameters
    let mut params2: Vec<ParameterRule> = rules2
        .iter()
        .filter_map(|rule| match rule {
            Rule::Parameter(param) => Some(param.clone()),
            _ => None,
        })
        .collect();
    params2.sort_by(|a, b| a.name.cmp(&b.name));
    assert_eq!(
        params2,
        vec![
            ParameterRule {
                name: ".half_word_type".into(),
                choices: vec![
                    Modifier::Atom(".f16".into()),
                    Modifier::Atom(".bf16".into())
                ],
            },
            ParameterRule {
                name: ".level::cache_hint".into(),
                choices: vec![Modifier::Atom(".L2::cache_hint".into())],
            },
            ParameterRule {
                name: ".op".into(),
                choices: vec![
                    Modifier::Atom(".add".into()),
                    Modifier::Atom(".min".into()),
                    Modifier::Atom(".max".into()),
                ],
            },
            ParameterRule {
                name: ".packed_type".into(),
                choices: vec![
                    Modifier::Atom(".f16x2".into()),
                    Modifier::Atom(".bf16x2".into()),
                ],
            },
            ParameterRule {
                name: ".scope".into(),
                choices: vec![
                    Modifier::Atom(".cta".into()),
                    Modifier::Atom(".cluster".into()),
                    Modifier::Atom(".gpu".into()),
                    Modifier::Atom(".sys".into()),
                ],
            },
            ParameterRule {
                name: ".sem".into(),
                choices: vec![
                    Modifier::Atom(".relaxed".into()),
                    Modifier::Atom(".acquire".into()),
                    Modifier::Atom(".release".into()),
                    Modifier::Atom(".acq_rel".into()),
                ],
            },
            ParameterRule {
                name: ".vec_16_bit".into(),
                choices: vec![
                    Modifier::Atom(".v2".into()),
                    Modifier::Atom(".v4".into()),
                    Modifier::Atom(".v8".into()),
                ],
            },
            ParameterRule {
                name: ".vec_32_bit".into(),
                choices: vec![Modifier::Atom(".v2".into()), Modifier::Atom(".v4".into())],
            },
        ]
    );
}

#[test]
fn parse_vmad() {
    let spec = "// 32-bit scalar operation
vmad.dtype.atype.btype{.sat}{.scale}     d, {-}a{.asel}, {-}b{.bsel},
{-}c;
vmad.dtype.atype.btype.po{.sat}{.scale}  d, a{.asel}, b{.bsel}, c;
.dtype = .atype = .btype = { .u32, .s32 };
.asel  = .bsel  = { .b0, .b1, .b2, .b3, .h0, .h1 };
.scale = { .shr7, .shr15 };";

    let instructions = parse_instruction_rules(spec);

    let expected_instructions = vec![
        // vmad.dtype.atype.btype{.sat}{.scale} d, {-}a{.asel}, {-}b{.bsel}, {-}c;
        InstructionRule {
            head: InstructionHead {
                opcode: "vmad".into(),
                modifiers: vec![
                    Modifier::Atom(".dtype".into()),
                    Modifier::Atom(".atype".into()),
                    Modifier::Atom(".btype".into()),
                    Modifier::Optional(".sat".into()),
                    Modifier::Optional(".scale".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                Operand {
                    operator: Some(OperatorToken::Negate),
                    operand: OperandElement::Item("a".into()),
                    modifier: Some(Modifier::Optional(".asel".into())),
                },
                Operand {
                    operator: Some(OperatorToken::Negate),
                    operand: OperandElement::Item("b".into()),
                    modifier: Some(Modifier::Optional(".bsel".into())),
                },
                Operand {
                    operator: Some(OperatorToken::Negate),
                    operand: OperandElement::Item("c".into()),
                    modifier: None,
                },
            ],
            ..InstructionRule::default()
        },
        // vmad.dtype.atype.btype.po{.sat}{.scale} d, a{.asel}, b{.bsel}, c;
        InstructionRule {
            head: InstructionHead {
                opcode: "vmad".into(),
                modifiers: vec![
                    Modifier::Atom(".dtype".into()),
                    Modifier::Atom(".atype".into()),
                    Modifier::Atom(".btype".into()),
                    Modifier::Atom(".po".into()),
                    Modifier::Optional(".sat".into()),
                    Modifier::Optional(".scale".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                Operand {
                    operator: None,
                    operand: OperandElement::Item("a".into()),
                    modifier: Some(Modifier::Optional(".asel".into())),
                },
                Operand {
                    operator: None,
                    operand: OperandElement::Item("b".into()),
                    modifier: Some(Modifier::Optional(".bsel".into())),
                },
                new_simple_operand(OperandElement::Item("c".into())),
            ],
            ..InstructionRule::default()
        },
    ];

    assert_instructions(instructions, expected_instructions);

    let mut params = parse_parameter_rules(spec);
    params.sort_by(|a, b| a.name.cmp(&b.name));
    let expected_params = vec![
            ParameterRule {
                name: ".asel".into(),
                choices: vec![
                    Modifier::Atom(".b0".into()),
                    Modifier::Atom(".b1".into()),
                    Modifier::Atom(".b2".into()),
                    Modifier::Atom(".b3".into()),
                    Modifier::Atom(".h0".into()),
                    Modifier::Atom(".h1".into()),
                ],
            },
            ParameterRule {
                name: ".atype".into(),
                choices: vec![Modifier::Atom(".u32".into()), Modifier::Atom(".s32".into())],
            },
            ParameterRule {
                name: ".bsel".into(),
                choices: vec![
                    Modifier::Atom(".b0".into()),
                    Modifier::Atom(".b1".into()),
                    Modifier::Atom(".b2".into()),
                    Modifier::Atom(".b3".into()),
                    Modifier::Atom(".h0".into()),
                    Modifier::Atom(".h1".into()),
                ],
            },
            ParameterRule {
                name: ".btype".into(),
                choices: vec![Modifier::Atom(".u32".into()), Modifier::Atom(".s32".into())],
            },
            ParameterRule {
                name: ".dtype".into(),
                choices: vec![Modifier::Atom(".u32".into()), Modifier::Atom(".s32".into())],
            },
            ParameterRule {
                name: ".scale".into(),
                choices: vec![
                    Modifier::Atom(".shr7".into()),
                    Modifier::Atom(".shr15".into()),
                ],
            },
        ];
    assert_eq!(params, expected_params);
}

#[test]
fn parse_min() {
    let spec = "min.atype         d, a, b;
min{.relu}.btype  d, a, b;
.atype = { .u16, .u32, .u64, .u16x2, .s16, .s64 };
.btype = { .s16x2, .s32 };

min{.ftz}{.NaN}{.xorsign.abs}.f32  d, a, b;
min{.ftz}{.NaN}{.abs}.f32          d, a, b, c;
min.f64                            d, a, b;

min{.ftz}{.NaN}{.xorsign.abs}.f16      d, a, b;
min{.ftz}{.NaN}{.xorsign.abs}.f16x2    d, a, b;
min{.NaN}{.xorsign.abs}.bf16           d, a, b;
min{.NaN}{.xorsign.abs}.bf16x2         d, a, b;";

    let instructions = parse_instruction_rules(spec);

    let expected_instructions = vec![
        // min.atype d, a, b;
        InstructionRule {
            head: InstructionHead {
                opcode: "min".into(),
                modifiers: vec![Modifier::Atom(".atype".into())],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
            ],
            ..InstructionRule::default()
        },
        // min.btype d, a, b;
        InstructionRule {
            head: InstructionHead {
                opcode: "min".into(),
                modifiers: vec![
                    Modifier::Optional(".relu".into()),
                    Modifier::Atom(".btype".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
            ],
            ..InstructionRule::default()
        },
        // min{.ftz}{.NaN}{.xorsign.abs}.f32 d, a, b;
        InstructionRule {
            head: InstructionHead {
                opcode: "min".into(),
                modifiers: vec![
                    Modifier::Optional(".ftz".into()),
                    Modifier::Optional(".NaN".into()),
                    Modifier::Optional(".xorsign.abs".into()),
                    Modifier::Atom(".f32".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
            ],
            ..InstructionRule::default()
        },
        // min{.ftz}{.NaN}{.abs}.f32 d, a, b, c;
        InstructionRule {
            head: InstructionHead {
                opcode: "min".into(),
                modifiers: vec![
                    Modifier::Optional(".ftz".into()),
                    Modifier::Optional(".NaN".into()),
                    Modifier::Optional(".abs".into()),
                    Modifier::Atom(".f32".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
                new_simple_operand(OperandElement::Item("c".into())),
            ],
            ..InstructionRule::default()
        },
        // min.f64 d, a, b;
        InstructionRule {
            head: InstructionHead {
                opcode: "min".into(),
                modifiers: vec![Modifier::Atom(".f64".into())],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
            ],
            ..InstructionRule::default()
        },
        // min{.ftz}{.NaN}{.xorsign.abs}.f16 d, a, b;
        InstructionRule {
            head: InstructionHead {
                opcode: "min".into(),
                modifiers: vec![
                    Modifier::Optional(".ftz".into()),
                    Modifier::Optional(".NaN".into()),
                    Modifier::Optional(".xorsign.abs".into()),
                    Modifier::Atom(".f16".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
            ],
            ..InstructionRule::default()
        },
        // min{.ftz}{.NaN}{.xorsign.abs}.f16x2 d, a, b;
        InstructionRule {
            head: InstructionHead {
                opcode: "min".into(),
                modifiers: vec![
                    Modifier::Optional(".ftz".into()),
                    Modifier::Optional(".NaN".into()),
                    Modifier::Optional(".xorsign.abs".into()),
                    Modifier::Atom(".f16x2".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
            ],
            ..InstructionRule::default()
        },
        // min{.NaN}{.xorsign.abs}.bf16 d, a, b;
        InstructionRule {
            head: InstructionHead {
                opcode: "min".into(),
                modifiers: vec![
                    Modifier::Optional(".NaN".into()),
                    Modifier::Optional(".xorsign.abs".into()),
                    Modifier::Atom(".bf16".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
            ],
            ..InstructionRule::default()
        },
        // min{.NaN}{.xorsign.abs}.bf16x2 d, a, b;
        InstructionRule {
            head: InstructionHead {
                opcode: "min".into(),
                modifiers: vec![
                    Modifier::Optional(".NaN".into()),
                    Modifier::Optional(".xorsign.abs".into()),
                    Modifier::Atom(".bf16x2".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::Item("d".into())),
                new_simple_operand(OperandElement::Item("a".into())),
                new_simple_operand(OperandElement::Item("b".into())),
            ],
            ..InstructionRule::default()
        },
    ];

    assert_instructions(instructions, expected_instructions);

    let mut params = parse_parameter_rules(spec);
    params.sort_by(|a, b| a.name.cmp(&b.name));
    let expected_params = vec![
            ParameterRule {
                name: ".atype".into(),
                choices: vec![
                    Modifier::Atom(".u16".into()),
                    Modifier::Atom(".u32".into()),
                    Modifier::Atom(".u64".into()),
                    Modifier::Atom(".u16x2".into()),
                    Modifier::Atom(".s16".into()),
                    Modifier::Atom(".s64".into()),
                ],
            },
            ParameterRule {
                name: ".btype".into(),
                choices: vec![
                    Modifier::Atom(".s16x2".into()),
                    Modifier::Atom(".s32".into()),
                ],
            },
        ];
    assert_eq!(params, expected_params);
}

#[test]
fn parse_cp_async() {
    let spec = "cp.async.cg.state.global{.level::cache_hint}{.level::prefetch_size} [dst], [src], 16{, ignore-src}{, cache-policy}";

    let instructions = parse_instruction_rules(spec);
    let mut instruction = instructions.into_iter().next().expect("Expected instruction");
    instruction.raw.clear();

    let expected = InstructionRule {
        head: InstructionHead {
            opcode: "cp".into(),
            modifiers: vec![
                Modifier::Atom(".async".into()),
                Modifier::Atom(".cg".into()),
                Modifier::Atom(".state".into()),
                Modifier::Atom(".global".into()),
                Modifier::Optional(".level::cache_hint".into()),
                Modifier::Optional(".level::prefetch_size".into()),
            ],
        },
        operands: vec![
            new_simple_operand(OperandElement::Address("dst".into())),
            new_simple_operand(OperandElement::Address("src".into())),
            new_simple_operand(OperandElement::ImmediateNumber("16".into())),
            new_simple_operand(OperandElement::Optional("ignore-src".into())),
            new_simple_operand(OperandElement::Optional("cache-policy".into())),
        ],
        ..InstructionRule::default()
    };
    assert_eq!(instruction, expected);
}

#[test]
fn test_setp_cmp_op() {
    let spec = "setp.CmpOp{.ftz}.f16x2         p|q, a, b";

    let instructions = parse_instruction_rules(spec);
    let mut instruction = instructions.into_iter().next().expect("Expected instruction");
    instruction.raw.clear();

    let expected = InstructionRule {
        head: InstructionHead {
            opcode: "setp".into(),
            modifiers: vec![
                Modifier::Atom(".CmpOp".into()),
                Modifier::Optional(".ftz".into()),
                Modifier::Atom(".f16x2".into()),
            ],
        },
        operands: vec![
            new_simple_operand(OperandElement::PipeChoice(("p".into(), "q".into()))),
            new_simple_operand(OperandElement::Item("a".into())),
            new_simple_operand(OperandElement::Item("b".into())),
        ],
        ..InstructionRule::default()
    };

    assert_eq!(instruction, expected);
}

#[test]
fn test_imm_num() {
    let src = "cp-size = { 4, 8, 16 };";
    let rules: Vec<Rule> = parse_spec(src)
        .expect("parse")
        .into_iter()
        .flat_map(|top| top.rules)
        .collect();
    let parameter: ParameterRule = match &rules[0] {
        Rule::Parameter(param) => param.clone(),
        _ => panic!("Expected ParameterRule"),
    };

    let expected = ParameterRule {
        name: "cp-size".into(),
        choices: vec![
            Modifier::Atom("4".into()),
            Modifier::Atom("8".into()),
            Modifier::Atom("16".into()),
        ],
    };
    assert_eq!(parameter, expected);
}

#[test]
fn test_tld4(){
    let spec = "tld4.comp.2d.v4.dtype.f32    d{|p}, [a, c] {, e} {, f};
tld4.comp.geom.v4.dtype.f32  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
.comp  = { .r, .g, .b, .a };
.geom  = { .2d, .a2d, .cube, .acube };
.dtype = { .u32, .s32, .f32 };";

    let instructions = parse_instruction_rules(spec);
    let mut params = parse_parameter_rules(spec);

    let expected_instructions = vec![
        // tld4.comp.2d.v4.dtype.f32    d{|p}, [a, c] {, e} {, f};
        InstructionRule {
            head: InstructionHead {
                opcode: "tld4".into(),
                modifiers: vec![
                    Modifier::Atom(".comp".into()),
                    Modifier::Atom(".2d".into()),
                    Modifier::Atom(".v4".into()),
                    Modifier::Atom(".dtype".into()),
                    Modifier::Atom(".f32".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::PipeOptionalChoice(("d".into(), "p".into()))),
                new_simple_operand(OperandElement::SquareGroup(vec![
                    new_simple_operand(OperandElement::Item("a".into())),
                    new_simple_operand(OperandElement::Item("c".into())),
                ])),
                new_simple_operand(OperandElement::Optional("e".into())),
                new_simple_operand(OperandElement::Optional("f".into())),
            ],
            ..InstructionRule::default()
        },
        // tld4.comp.geom.v4.dtype.f32  d{|p}, [a, b, c] {, e} {, f};
        InstructionRule {
            head: InstructionHead {
                opcode: "tld4".into(),
                modifiers: vec![
                    Modifier::Atom(".comp".into()),
                    Modifier::Atom(".geom".into()),
                    Modifier::Atom(".v4".into()),
                    Modifier::Atom(".dtype".into()),
                    Modifier::Atom(".f32".into()),
                ],
            },
            operands: vec![
                new_simple_operand(OperandElement::PipeOptionalChoice(("d".into(), "p".into()))),
                new_simple_operand(OperandElement::SquareGroup(vec![
                    new_simple_operand(OperandElement::Item("a".into())),
                    new_simple_operand(OperandElement::Item("b".into())),
                    new_simple_operand(OperandElement::Item("c".into())),
                ])),
                new_simple_operand(OperandElement::Optional("e".into())),
                new_simple_operand(OperandElement::Optional("f".into())),
            ],
            ..InstructionRule::default()
        },
    ];

    assert_instructions(instructions, expected_instructions);

    params.sort_by(|a, b| a.name.cmp(&b.name));
    let expected_params = vec![
        ParameterRule {
            name: ".comp".into(),
            choices: vec![
                Modifier::Atom(".r".into()),
                Modifier::Atom(".g".into()),
                Modifier::Atom(".b".into()),
                Modifier::Atom(".a".into()),
            ],
        },
        ParameterRule {
            name: ".dtype".into(),
            choices: vec![
                Modifier::Atom(".u32".into()),
                Modifier::Atom(".s32".into()),
                Modifier::Atom(".f32".into()),
            ],
        },
        ParameterRule {
            name: ".geom".into(),
            choices: vec![
                Modifier::Atom(".2d".into()),
                Modifier::Atom(".a2d".into()),
                Modifier::Atom(".cube".into()),
                Modifier::Atom(".acube".into()),
            ],
        },
    ];
    assert_eq!(params, expected_params);
}

#[test]
fn test_ld() {
    let spec = "ld{.weak}{.ss}{.cop}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{.unified}{, cache-policy};";

    let instructions = parse_instruction_rules(spec);
    let instruction = instructions.into_iter().next().expect("Expected instruction");

    // Print the actual parsed structure for debugging
    println!("{:#?}", instruction);

    // Check that [a]{.unified} is parsed as Address with modifier
    assert_eq!(
        instruction.operands[1].operand,
        OperandElement::Address("a".into())
    );
    assert_eq!(
        instruction.operands[1].modifier,
        Some(Modifier::Optional(".unified".into()))
    );

    // Check that {, cache-policy} is parsed as Optional operand
    assert_eq!(
        instruction.operands[2].operand,
        OperandElement::Optional("cache-policy".into())
    );
    assert_eq!(instruction.operands[2].modifier, None);
}

fn normalize_instructions(mut instructions: Vec<InstructionRule>) -> Vec<InstructionRule> {
    for instr in &mut instructions {
        instr.raw.clear();
    }
    instructions
}

fn assert_instructions(actual: Vec<InstructionRule>, expected: Vec<InstructionRule>) {
    assert_eq!(
        normalize_instructions(actual),
        normalize_instructions(expected)
    );
}

fn new_simple_operand(operand: OperandElement) -> Operand {
    Operand {
        operator: None,
        operand,
        modifier: None,
    }
}

fn parse_parameter_rules(x: &str) -> Vec<ParameterRule> {
    let mut params: Vec<ParameterRule> = parse_spec(x)
        .expect("parse")
        .into_iter()
        .flat_map(|top| top.rules)
        .filter_map(|rule| match rule {
            Rule::Parameter(param) => Some(param),
            _ => None,
        })
        .collect();
    params.sort_by(|a, b| a.name.cmp(&b.name));
    params
}
