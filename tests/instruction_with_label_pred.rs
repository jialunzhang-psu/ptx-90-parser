mod util;

use ptx_parser::{
    r#type::common::Instruction,
    unparser::PtxUnparser,
    unlexer::PtxUnlexer,
};

#[test]
fn test_instruction_with_label_only() {
    let input = "loop_start: add.s32 %r1, %r2, %r3;";
    let result: Instruction = util::parse(input);
    
    assert_eq!(result.label, Some("loop_start".to_string()));
    assert_eq!(result.predicate, None);
}

#[test]
fn test_instruction_with_predicate_only() {
    let input = "@%p0 bra exit_label;";
    let result: Instruction = util::parse(input);
    
    assert_eq!(result.label, None);
    assert!(result.predicate.is_some());
    
    let predicate = result.predicate.unwrap();
    assert_eq!(predicate.negated, false);
}

#[test]
fn test_instruction_with_negated_predicate() {
    let input = "@!%p0 add.s32 %r1, %r2, %r3;";
    let result: Instruction = util::parse(input);
    
    assert_eq!(result.label, None);
    assert!(result.predicate.is_some());
    
    let predicate = result.predicate.unwrap();
    assert_eq!(predicate.negated, true);
}

#[test]
fn test_instruction_with_label_and_predicate() {
    let input = "my_label: @%p1 setp.eq.s32 %p0, %r1, %r2;";
    let result: Instruction = util::parse(input);
    
    assert_eq!(result.label, Some("my_label".to_string()));
    assert!(result.predicate.is_some());
    
    let predicate = result.predicate.unwrap();
    assert_eq!(predicate.negated, false);
}

#[test]
fn test_instruction_with_label_and_negated_predicate() {
    let input = "exit_point: @!%p0 ret;";
    let result: Instruction = util::parse(input);
    
    assert_eq!(result.label, Some("exit_point".to_string()));
    assert!(result.predicate.is_some());
    
    let predicate = result.predicate.unwrap();
    assert_eq!(predicate.negated, true);
}

#[test]
fn test_instruction_without_label_or_predicate() {
    let input = "mov.u32 %r0, %r1;";
    let result: Instruction = util::parse(input);
    
    assert_eq!(result.label, None);
    assert_eq!(result.predicate, None);
}

#[test]
fn test_complex_label_names() {
    let input = "loop_start_123: add.s32 %r1, %r2, %r3;";
    let result: Instruction = util::parse(input);
    
    assert_eq!(result.label, Some("loop_start_123".to_string()));
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
        let result: Instruction = util::parse(input);
        // Just verify they all parse successfully without panicking
        let _ = result.inst;
    }
}

#[test]
fn test_unparse_instruction_with_label() {
    let input = "my_label: add.s32 %r1, %r2, %r3;";
    let parsed: Instruction = util::parse(input);
    let tokens = parsed.to_tokens();
    let unparsed = PtxUnlexer::to_string(&tokens).expect("unparsing failed");
    
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
    let input = "check: @!%p0 ret;";
    let parsed: Instruction = util::parse(input);
    let tokens = parsed.to_tokens();
    let unparsed = PtxUnlexer::to_string(&tokens).expect("unparsing failed");
    
    eprintln!("Unparsed: {}", unparsed);
    assert!(unparsed.contains("check:"));
    assert!(unparsed.contains("@"));
    assert!(unparsed.contains("!"));
    assert!(unparsed.contains("p0"));
    assert!(unparsed.contains("ret"));
}

#[test]
fn test_roundtrip_with_label() {
    let input = "loop: add.s32 %r1, %r2, %r3;";
    let parsed: Instruction = util::parse(input);
    let tokens = parsed.to_tokens();
    let unparsed = PtxUnlexer::to_string(&tokens).expect("unparsing failed");
    
    // Parse the unparsed output again
    let reparsed: Instruction = util::parse(&unparsed);
    assert_eq!(parsed.label, reparsed.label);
    assert_eq!(parsed.predicate.is_some(), reparsed.predicate.is_some());
}

#[test]
fn test_roundtrip_with_predicate() {
    let input = "@%p1 mov.u32 %r0, 42;";
    let parsed: Instruction = util::parse(input);
    let tokens = parsed.to_tokens();
    let unparsed = PtxUnlexer::to_string(&tokens).expect("unparsing failed");
    
    // Parse the unparsed output again
    let reparsed: Instruction = util::parse(&unparsed);
    assert_eq!(parsed.label, reparsed.label);
    assert_eq!(parsed.predicate.is_some(), reparsed.predicate.is_some());
}

#[test]
fn test_roundtrip_with_label_and_predicate() {
    let input = "branch_target: @!%p0 bra done;";
    let parsed: Instruction = util::parse(input);
    let tokens = parsed.to_tokens();
    let unparsed = PtxUnlexer::to_string(&tokens).expect("unparsing failed");
    
    // Parse the unparsed output again
    let reparsed: Instruction = util::parse(&unparsed);
    assert_eq!(parsed.label, reparsed.label);
    assert_eq!(parsed.predicate.is_some(), reparsed.predicate.is_some());
    if let (Some(pred1), Some(pred2)) = (&parsed.predicate, &reparsed.predicate) {
        assert_eq!(pred1.negated, pred2.negated);
    }
}

