mod util;

use ptx_parser::{
    PtxParser, PtxToken, PtxTokenStream, PtxUnlexer, PtxUnparser, tokenize,
    r#type::{FunctionStatement, Instruction},
};

fn parse_statements(source: &str) -> Vec<FunctionStatement> {
    let tokens = tokenize(source).expect("tokenization should succeed");
    let mut stream = PtxTokenStream::new(&tokens);
    let mut statements = Vec::new();
    while !stream.is_at_end() {
        statements
            .push(FunctionStatement::parse(&mut stream).expect("statement parse should succeed"));
    }
    statements
}

fn parse_label_and_instruction(source: &str) -> (String, Instruction) {
    let mut statements = parse_statements(source).into_iter();
    let label_stmt = statements.next().expect("expected label statement");
    let label = match label_stmt {
        FunctionStatement::Label(name) => name,
        other => panic!("expected label statement, got {:?}", other),
    };

    let inst_stmt = statements.next().expect("expected instruction statement");
    let inst = match inst_stmt {
        FunctionStatement::Instruction(inst) => inst,
        other => panic!("expected instruction after label, got {:?}", other),
    };

    (label, inst)
}

fn unparse_statements(statements: &[FunctionStatement]) -> Vec<PtxToken> {
    let mut tokens = Vec::new();
    for statement in statements {
        tokens.extend(statement.to_tokens());
    }
    tokens
}

#[test]
fn test_instruction_with_label_only() {
    let (label, inst) = parse_label_and_instruction("loop_start: add.s32 %r1, %r2, %r3;");
    assert_eq!(label, "loop_start");
    assert!(inst.predicate.is_none());
}

#[test]
fn test_instruction_with_predicate_only() {
    let input = "@%p0 bra exit_label;";
    let result: Instruction = util::parse(input);

    assert!(result.predicate.is_some());
    let predicate = result.predicate.unwrap();
    assert_eq!(predicate.negated, false);
}

#[test]
fn test_instruction_with_negated_predicate() {
    let input = "@!%p0 add.s32 %r1, %r2, %r3;";
    let result: Instruction = util::parse(input);

    let predicate = result.predicate.expect("predicate should exist");
    assert!(predicate.negated, "predicate should be negated");
}

#[test]
fn test_instruction_with_label_and_predicate() {
    let (label, inst) = parse_label_and_instruction("my_label: @%p1 setp.eq.s32 %p0, %r1, %r2;");
    assert_eq!(label, "my_label");

    let predicate = inst.predicate.expect("predicate should exist");
    assert!(!predicate.negated);
}

#[test]
fn test_instruction_with_label_and_negated_predicate() {
    let (label, inst) = parse_label_and_instruction("exit_point: @!%p0 ret;");
    assert_eq!(label, "exit_point");

    let predicate = inst.predicate.expect("predicate should exist");
    assert!(predicate.negated);
}

#[test]
fn test_instruction_without_label_or_predicate() {
    let input = "mov.u32 %r0, %r1;";
    let result: Instruction = util::parse(input);

    assert!(result.predicate.is_none());
}

#[test]
fn test_complex_label_names() {
    let statements = parse_statements("loop_start_123: add.s32 %r1, %r2, %r3;");
    match &statements[0] {
        FunctionStatement::Label(name) => assert_eq!(name, "loop_start_123"),
        other => panic!("expected label statement, got {:?}", other),
    }
}

#[test]
fn test_multiple_instructions_in_sequence() {
    let inputs = vec![
        "start: mov.u32 %r0, 1;",
        "@%p0 add.s32 %r1, %r1, 1;",
        "loop: @!%p0 bra start;",
        "ret;",
    ];

    for input in inputs {
        let statements = parse_statements(input);
        assert!(
            statements
                .iter()
                .any(|stmt| matches!(stmt, FunctionStatement::Instruction(_))),
            "expected at least one instruction in {input}"
        );
    }
}

#[test]
fn test_unparse_instruction_with_label() {
    let statements = parse_statements("my_label: add.s32 %r1, %r2, %r3;");
    let tokens = unparse_statements(&statements);
    let unparsed = PtxUnlexer::to_string(&tokens).expect("unparse failed");

    assert!(unparsed.contains("my_label:"));
    assert!(unparsed.contains("add.s32"));
}

#[test]
fn test_unparse_instruction_with_predicate() {
    let input = "@%p0 bra target;";
    let parsed: Instruction = util::parse(input);
    let tokens = parsed.to_tokens();
    let unparsed = PtxUnlexer::to_string(&tokens).expect("unparsing failed");

    assert!(unparsed.contains("@"));
    assert!(unparsed.contains("p0"));
    assert!(unparsed.contains("bra"));
}

#[test]
fn test_unparse_instruction_with_label_and_predicate() {
    let statements = parse_statements("check: @!%p0 ret;");
    let tokens = unparse_statements(&statements);
    assert!(tokens.contains(&PtxToken::Identifier("check".into())));
    assert!(tokens.contains(&PtxToken::At));
    assert!(tokens.contains(&PtxToken::Exclaim));
    assert!(tokens.contains(&PtxToken::Register("%p0".into())));
}

#[test]
fn test_roundtrip_with_label() {
    let statements = parse_statements("loop: add.s32 %r1, %r2, %r3;");
    let tokens = unparse_statements(&statements);
    let reparsed = parse_statements(&PtxUnlexer::to_string(&tokens).expect("unparse failed"));
    assert_eq!(statements, reparsed);
}

#[test]
fn test_roundtrip_with_predicate() {
    let input = "@%p1 mov.u32 %r0, 42;";
    let parsed: Instruction = util::parse(input);
    let tokens = parsed.to_tokens();
    let unparsed = PtxUnlexer::to_string(&tokens).expect("unparsing failed");

    let reparsed: Instruction = util::parse(&unparsed);
    assert_eq!(parsed.predicate.is_some(), reparsed.predicate.is_some());
}

#[test]
fn test_roundtrip_with_label_and_predicate() {
    let statements = parse_statements("branch_target: @!%p0 bra done;");
    let tokens = unparse_statements(&statements);
    let unparsed = PtxUnlexer::to_string(&tokens).expect("unparsing failed");
    let reparsed = parse_statements(&unparsed);
    assert_eq!(statements, reparsed);
}
