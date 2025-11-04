// #[path = "instruction/mod.rs"]
// mod instruction;
mod util;
// use ptx_parser::{
//     parser::ParseErrorKind,
//     r#type::{
//         common::{Immediate, Operand, PredicateRegister, RegisterOperand},
//         instruction::{
//             Instruction, InstructionOpcode,
//             add::Add,
//             barrier::{Barrier, BarrierSync, Scope},
//             multimem::{Instruction as MultimemInstruction, Red},
//         },
//     },
//     unparser::PtxUnparser,
// };
// use util::{assert_roundtrip, parse, parse_result, tokenize_only};

// #[test]
// fn parses_add_opcode_dispatch() {
//     let InstructionOpcode::Add(Add {
//         data_type,
//         destination,
//         a,
//         b,
//     }) = parse::<InstructionOpcode>("add.s32 %r0, %r1, %r2;")
//     else {
//         panic!("expected add opcode");
//     };

//     assert!(matches!(
//         data_type,
//         ptx_parser::r#type::instruction::add::DataType::S32 { .. }
//     ));
//     assert_eq!(destination, RegisterOperand::Single("%r0".into()));
//     assert_eq!(a, RegisterOperand::Single("%r1".into()));
//     assert_eq!(b, Operand::Register(RegisterOperand::Single("%r2".into())));
//     assert_roundtrip::<InstructionOpcode>("add.s32 %r0, %r1, %r2;");
// }

// #[test]
// fn parses_barrier_opcode_dispatch() {
//     let InstructionOpcode::Barrier(Barrier::Sync(BarrierSync {
//         scope,
//         aligned,
//         barrier,
//         expected_count,
//     })) = parse::<InstructionOpcode>("barrier.cta.sync.aligned %r1, 4;")
//     else {
//         panic!("expected barrier.sync opcode");
//     };

//     assert_eq!(scope, Scope::Cta);
//     assert!(aligned);
//     assert_eq!(
//         barrier,
//         Operand::Register(RegisterOperand::Single("%r1".into()))
//     );
//     assert_eq!(
//         expected_count,
//         Some(Operand::Immediate(Immediate("4".into())))
//     );
//     assert_roundtrip::<InstructionOpcode>("barrier.cta.sync.aligned %r1, 4;");
// }

// #[test]
// fn parses_multimem_opcode_dispatch() {
//     let InstructionOpcode::Multimem(MultimemInstruction::Red(Red::Int(_))) =
//         parse::<InstructionOpcode>("multimem.red.release.cta.global.add.u64 [%rd3], %rd4;")
//     else {
//         panic!("expected multimem.red opcode");
//     };
//     assert_roundtrip::<InstructionOpcode>("multimem.red.release.cta.global.add.u64 [%rd3], %rd4;");
// }

// #[test]
// fn instruction_without_predicate_unparses_add() {
//     let source = "add.s32 %r0, %r1, %r2;";
//     let opcode = parse::<InstructionOpcode>(source);
//     let instruction = Instruction {
//         predicate: None,
//         opcode,
//         comment: None,
//         raw: source.into(),
//     };
//     assert_eq!(instruction.to_tokens(), tokenize_only(source));
// }

// #[test]
// fn instruction_with_predicate_unparses_add() {
//     let opcode = parse::<InstructionOpcode>("add.s32 %r0, %r1, %r2;");
//     let source = "@%p1 add.s32 %r0, %r1, %r2;";
//     let instruction = Instruction {
//         predicate: Some(PredicateRegister("%p1".into())),
//         opcode,
//         comment: None,
//         raw: source.into(),
//     };
//     assert_eq!(instruction.to_tokens(), tokenize_only(source));
// }

// #[test]
// fn rejects_unknown_instruction_name() {
//     let err = parse_result::<InstructionOpcode>("foo;")
//         .expect_err("parsing should fail for unknown opcode");

//     let ParseErrorKind::UnexpectedToken { expected, found } = err.kind else {
//         panic!("expected unexpected token error");
//     };

//     assert_eq!(found, "foo");
//     assert!(
//         expected.contains(&"abs".to_string()),
//         "expected list should include known opcodes"
//     );
// }
