use std::fs;
use std::path::{Path, PathBuf};

use ptx_parser::tokenize;
use ptx_parser::r#type::{
    instruction::Inst, FunctionKernelDirective, FunctionStatement, Instruction, Module,
    ModuleDirective, ModuleInfoDirectiveKind,
};
use ptx_parser::{PtxParser, PtxTokenStream};

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
            eprintln!("✗ {}: Tokenization failed: {:?}", path.display(), err);
            return false;
        }
    };

    let mut stream = PtxTokenStream::new(&tokens);
    match Module::parse(&mut stream) {
        Ok(module) => {
            validate_module_ast(&module, path);
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
            eprintln!("✗ {}: Module parsing failed: {}", path.display(), err);
            false
        }
    }
}

fn validate_module_ast(module: &Module, path: &Path) {
    let mut seen_function = false;
    let mut seen_version = false;
    let mut seen_target = false;
    let mut seen_address = false;

    for (idx, directive) in module.directives.iter().enumerate() {
        match directive {
            ModuleDirective::ModuleInfo { directive: info, .. } => {
                assert!(
                    !seen_function,
                    "{}: module info directive found after function directive at index {}",
                    path.display(),
                    idx
                );

                match info {
                    ModuleInfoDirectiveKind::Version { directive: version, .. } => {
                        seen_version = true;
                        assert!(
                            version.major > 0,
                            "{}: version major should be > 0",
                            path.display()
                        );
                    }
                    ModuleInfoDirectiveKind::Target { directive: target, .. } => {
                        seen_target = true;
                        assert!(
                            !target.entries.is_empty(),
                            "{}: target directive should contain entries",
                            path.display()
                        );
                    }
                    ModuleInfoDirectiveKind::AddressSize { directive: address, .. } => {
                        seen_address = true;
                        assert!(
                            address.size == 32 || address.size == 64,
                            "{}: address_size must be 32 or 64",
                            path.display()
                        );
                    }
                }
            }
            ModuleDirective::FunctionKernel { directive: function, .. } => {
                seen_function = true;
                if let Some((name, instruction_count)) = instruction_count_for_function(function) {
                    if instruction_count == 1 {
                        let only_ret = first_instruction_in_function(function)
                            .map(|inst| matches!(inst.inst, Inst::RetUni(_)))
                            .unwrap_or(false);
                        if !only_ret {
                            panic!(
                                "{}: kernel/function `{}` contains exactly 1 executable statement",
                                path.display(),
                                name
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }

    assert!(
        seen_version && seen_target && seen_address,
        "{}: module missing required module info directives (version: {}, target: {}, address_size: {})",
        path.display(),
        seen_version,
        seen_target,
        seen_address
    );
    assert!(
        seen_function,
        "{}: module should contain at least one function or kernel directive",
        path.display()
    );
}

fn instruction_count_for_function<'a>(
    function: &'a FunctionKernelDirective,
) -> Option<(&'a str, usize)> {
    match function {
        FunctionKernelDirective::Entry { function: entry, .. } => {
            Some((entry.name.as_str(), instruction_count_in_body(&entry.body)))
        }
        FunctionKernelDirective::Func { function: func, .. } => {
            Some((func.name.as_str(), instruction_count_in_body(&func.body)))
        }
        FunctionKernelDirective::Alias { .. } => None,
    }
}

fn instruction_count_in_body(body: &ptx_parser::r#type::FunctionBody) -> usize {
    body.statements
        .iter()
        .map(instruction_count_in_statement)
        .sum()
}

fn instruction_count_in_statement(statement: &FunctionStatement) -> usize {
    match statement {
        FunctionStatement::Label { .. } => 0,
        FunctionStatement::Instruction { .. } => 1,
        FunctionStatement::Directive { .. } => 0,
        FunctionStatement::Block { statements, .. } => {
            statements.iter().map(instruction_count_in_statement).sum()
        }
    }
}

fn first_instruction_in_function(function: &FunctionKernelDirective) -> Option<&Instruction> {
    match function {
        FunctionKernelDirective::Entry { function: entry, .. } => first_instruction_in_statements(&entry.body.statements),
        FunctionKernelDirective::Func { function: func, .. } => first_instruction_in_statements(&func.body.statements),
        FunctionKernelDirective::Alias { .. } => None,
    }
}

fn first_instruction_in_statements(statements: &[FunctionStatement]) -> Option<&Instruction> {
    for statement in statements {
        match statement {
            FunctionStatement::Instruction { instruction: inst, .. } => return Some(inst),
            FunctionStatement::Block { statements: block, .. } => {
                if let Some(inst) = first_instruction_in_statements(block) {
                    return Some(inst);
                }
            }
            _ => {}
        }
    }
    None
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
            ptx_parser::r#type::ModuleDirective::FunctionKernel {
                directive: FunctionKernelDirective::Entry { function: entry, .. },
                ..
            } => Some(entry),
            _ => None,
        })
        .expect("should find entry function");

    // Extract instructions with their preceding labels
    let mut labeled_instructions = Vec::new();
    let mut pending_label: Option<String> = None;
    for statement in &entry.body.statements {
        match statement {
            FunctionStatement::Label { name, .. } => pending_label = Some(name.clone()),
            FunctionStatement::Instruction { instruction: inst, .. } => {
                labeled_instructions.push((pending_label.take(), inst));
            }
            _ => {}
        }
    }

    assert!(
        labeled_instructions.len() >= 6,
        "should have at least 6 instructions, found {}",
        labeled_instructions.len()
    );

    // Test 1: Instruction with label only
    let (first_label, inst_with_label) = &labeled_instructions[0];
    assert_eq!(
        first_label.as_deref(),
        Some("loop_start"),
        "first instruction should have label 'loop_start'"
    );
    assert!(
        inst_with_label.predicate.is_none(),
        "first instruction should not have predicate"
    );

    // Test 2: Instruction with predicate only
    let (second_label, inst_with_pred) = &labeled_instructions[1];
    assert!(
        second_label.is_none(),
        "second instruction should not have label"
    );
    let pred = inst_with_pred
        .predicate
        .as_ref()
        .expect("second instruction should have predicate");
    assert!(!pred.negated, "predicate should not be negated");

    // Test 3: Instruction with negated predicate
    let (third_label, inst_with_neg_pred) = &labeled_instructions[2];
    assert!(
        third_label.is_none(),
        "third instruction should not have label"
    );
    let neg_pred = inst_with_neg_pred
        .predicate
        .as_ref()
        .expect("third instruction should have predicate");
    assert!(neg_pred.negated, "predicate should be negated");

    // Test 4: Instruction with both label and predicate
    let (fourth_label, inst_with_both) = &labeled_instructions[3];
    assert_eq!(
        fourth_label.as_deref(),
        Some("continue_label"),
        "fourth instruction should have label 'continue_label'"
    );
    let both_pred = inst_with_both
        .predicate
        .as_ref()
        .expect("fourth instruction should have predicate");
    assert!(!both_pred.negated, "predicate should not be negated");

    // Test 5: Instruction with label and negated predicate
    let (fifth_label, inst_with_label_neg) = &labeled_instructions[4];
    assert_eq!(
        fifth_label.as_deref(),
        Some("exit_label"),
        "fifth instruction should have label 'exit_label'"
    );
    let label_neg_pred = inst_with_label_neg
        .predicate
        .as_ref()
        .expect("fifth instruction should have predicate");
    assert!(label_neg_pred.negated, "predicate should be negated");
}
