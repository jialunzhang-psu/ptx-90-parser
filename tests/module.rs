use std::fs;
use std::path::{Path, PathBuf};

use ptx_parser::lexer::tokenize;
use ptx_parser::parser::{PtxParser, PtxTokenStream};
use ptx_parser::r#type::module::Module;
use ptx_parser::r#type::function::{FunctionKernelDirective, FunctionStatement};

#[test]
fn parse_sample_ptx_files() {
    let samples_dir = Path::new("tests/sample");
    let mut entries: Vec<PathBuf> = fs::read_dir(samples_dir)
        .expect("samples directory missing")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().map(|ext| ext == "ptx").unwrap_or(false))
        .collect();

    entries.sort();

    let mut total_files = 0;
    let mut successful_files = 0;

    for path in entries {
        total_files += 1;
        if parse_sample_file(&path) {
            successful_files += 1;
        }
    }

    eprintln!(
        "\nOverall: {}/{} files parsed successfully",
        successful_files, total_files
    );

    assert_eq!(
        successful_files, total_files,
        "All PTX files should parse successfully as modules"
    );
}

fn parse_sample_file(path: &Path) -> bool {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));

    let tokens = match tokenize(&source) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!(
                "✗ {}: Tokenization failed: {:?}",
                path.display(),
                err
            );
            return false;
        }
    };

    let mut stream = PtxTokenStream::new(&tokens);
    match Module::parse(&mut stream) {
        Ok(module) => {
            eprintln!(
                "✓ {}: Successfully parsed {} directives",
                path.display(),
                module.directives.len()
            );
            
            if !stream.is_at_end() {
                eprintln!(
                    "  Warning: {} tokens remaining after parsing",
                    tokens.len() - stream.position().index
                );
            }
            
            true
        }
        Err(err) => {
            eprintln!(
                "✗ {}: Module parsing failed: {}",
                path.display(),
                err
            );
            false
        }
    }
}

#[test]
fn test_parse_labels_and_predicates() {
    let source = r#"
        .version 8.5
        .target sm_90
        .address_size 64
        
        .entry test_kernel(.param .u64 param) {
            // Instruction with label
            loop_start: add.s32 %r1, %r2, %r3;
            
            // Instruction with predicate
            @%p0 sub.s32 %r4, %r5, %r6;
            
            // Instruction with negated predicate
            @!%p1 mul.lo.s32 %r7, %r8, %r9;
            
            // Instruction with both label and predicate
            continue_label: @%p2 mov.u32 %r10, %r11;
            
            // Instruction with label and negated predicate
            exit_label: @!%p3 bra exit_label;
            
            ret;
        }
    "#;

    let tokens = tokenize(source).expect("tokenization should succeed");
    let mut stream = PtxTokenStream::new(&tokens);
    let module = Module::parse(&mut stream).expect("module parsing should succeed");

    // Find the entry function
    let entry = module
        .directives
        .iter()
        .find_map(|d| match d {
            ptx_parser::r#type::module::ModuleDirective::FunctionKernel(
                FunctionKernelDirective::Entry(entry),
            ) => Some(entry),
            _ => None,
        })
        .expect("should find entry function");

    // Extract instructions from the function body
    let instructions: Vec<_> = entry
        .body
        .statements
        .iter()
        .filter_map(|stmt| match stmt {
            FunctionStatement::Instruction(inst) => Some(inst),
            _ => None,
        })
        .collect();

    // We should have 6 instructions (excluding ret which is also an instruction)
    assert!(
        instructions.len() >= 6,
        "should have at least 6 instructions, found {}",
        instructions.len()
    );

    // Test 1: Instruction with label only
    let inst_with_label = &instructions[0];
    assert_eq!(
        inst_with_label.label,
        Some("loop_start".to_string()),
        "first instruction should have label 'loop_start'"
    );
    assert!(
        inst_with_label.predicate.is_none(),
        "first instruction should not have predicate"
    );

    // Test 2: Instruction with predicate only
    let inst_with_pred = &instructions[1];
    assert!(
        inst_with_pred.label.is_none(),
        "second instruction should not have label"
    );
    let pred = inst_with_pred
        .predicate
        .as_ref()
        .expect("second instruction should have predicate");
    assert!(!pred.negated, "predicate should not be negated");

    // Test 3: Instruction with negated predicate
    let inst_with_neg_pred = &instructions[2];
    assert!(
        inst_with_neg_pred.label.is_none(),
        "third instruction should not have label"
    );
    let neg_pred = inst_with_neg_pred
        .predicate
        .as_ref()
        .expect("third instruction should have predicate");
    assert!(neg_pred.negated, "predicate should be negated");

    // Test 4: Instruction with both label and predicate
    let inst_with_both = &instructions[3];
    assert_eq!(
        inst_with_both.label,
        Some("continue_label".to_string()),
        "fourth instruction should have label 'continue_label'"
    );
    let both_pred = inst_with_both
        .predicate
        .as_ref()
        .expect("fourth instruction should have predicate");
    assert!(!both_pred.negated, "predicate should not be negated");

    // Test 5: Instruction with label and negated predicate
    let inst_with_label_neg = &instructions[4];
    assert_eq!(
        inst_with_label_neg.label,
        Some("exit_label".to_string()),
        "fifth instruction should have label 'exit_label'"
    );
    let label_neg_pred = inst_with_label_neg
        .predicate
        .as_ref()
        .expect("fifth instruction should have predicate");
    assert!(label_neg_pred.negated, "predicate should be negated");

}
