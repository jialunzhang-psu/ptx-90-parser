mod util;

use ptx_parser::{PtxUnlexer, PtxUnparser, r#type::Instruction};

#[test]
fn test_instruction_with_predicate_only() {
    ptx_parser::run_with_large_stack(|| {
        let input = "@%p0 bra exit_label;";
        let result: Instruction = util::parse(input);

        assert!(result.predicate.is_some());
        let predicate = result.predicate.unwrap();
        assert_eq!(predicate.negated, false);
    });
}

#[test]
fn test_instruction_with_negated_predicate() {
    ptx_parser::run_with_large_stack(|| {
        let input = "@!%p0 add.s32 %r1, %r2, %r3;";
        let result: Instruction = util::parse(input);

        let predicate = result.predicate.expect("predicate should exist");
        assert!(predicate.negated, "predicate should be negated");
    });
}

#[test]
fn test_instruction_without_label_or_predicate() {
    ptx_parser::run_with_large_stack(|| {
        let input = "mov.u32 %r0, %r1;";
        let result: Instruction = util::parse(input);

        assert!(result.predicate.is_none());
    });
}

#[test]
fn test_unparse_instruction_with_predicate() {
    ptx_parser::run_with_large_stack(|| {
        let input = "@%p0 bra target;";
        let parsed: Instruction = util::parse(input);
        let tokens = parsed.to_tokens();
        let unparsed = PtxUnlexer::to_string(&tokens).expect("unparsing failed");

        assert!(unparsed.contains("@"));
        assert!(unparsed.contains("p0"));
        assert!(unparsed.contains("bra"));
    });
}

#[test]
fn test_roundtrip_with_predicate() {
    ptx_parser::run_with_large_stack(|| {
        let input = "@%p1 mov.u32 %r0, 42;";
        let parsed: Instruction = util::parse(input);
        let tokens = parsed.to_tokens();
        let unparsed = PtxUnlexer::to_string(&tokens).expect("unparsing failed");

        let reparsed: Instruction = util::parse(&unparsed);
        assert_eq!(parsed.predicate.is_some(), reparsed.predicate.is_some());
    });
}
