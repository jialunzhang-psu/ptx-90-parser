mod util;

use std::fs;
use std::path::{Path, PathBuf};

use ptx_parser::tokenize;
use ptx_parser::r#type::{
    AddressSize, FunctionStatement, Instruction, Module, ModuleDirective, ModuleInfoDirectiveKind,
    instruction::Inst,
};
use ptx_parser::{PtxParser, PtxTokenStream};

#[test]
fn parse_sample_ptx_files() {
    ptx_parser::run_with_large_stack(|| {
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
    });
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
    match Module::parse()(&mut stream) {
        Ok((module, _)) => {
            validate_module_ast(&module, path);
            eprintln!(
                "✓ {}: Successfully parsed {} directives",
                path.display(),
                module.directives.len()
            );

            if !stream.is_at_end() {
                eprintln!(
                    "  Warning: {} tokens remaining after parsing",
                    tokens.len() - stream.position().0
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
            ModuleDirective::ModuleInfo {
                directive: info, ..
            } => {
                assert!(
                    !seen_function,
                    "{}: module info directive found after function directive at index {}",
                    path.display(),
                    idx
                );

                match info {
                    ModuleInfoDirectiveKind::Version {
                        directive: version, ..
                    } => {
                        seen_version = true;
                        assert!(
                            version.major > 0,
                            "{}: version major should be > 0",
                            path.display()
                        );
                    }
                    ModuleInfoDirectiveKind::Target {
                        directive: target, ..
                    } => {
                        seen_target = true;
                        assert!(
                            !target.entries.is_empty(),
                            "{}: target directive should contain entries",
                            path.display()
                        );
                    }
                    ModuleInfoDirectiveKind::AddressSize {
                        directive: address, ..
                    } => {
                        seen_address = true;
                        assert!(
                            matches!(
                                address.size,
                                AddressSize::Size32 { .. } | AddressSize::Size64 { .. }
                            ),
                            "{}: address_size must be 32 or 64",
                            path.display()
                        );
                    }
                }
            }
            ModuleDirective::EntryFunction {
                directive: function,
                ..
            } => {
                seen_function = true;
                let name = &function.name.val;
                let instruction_count = function
                    .body
                    .as_ref()
                    .map(instruction_count_in_body)
                    .unwrap_or(0);
                if instruction_count == 1 {
                    let only_ret = function
                        .body
                        .as_ref()
                        .and_then(|body| first_instruction_in_statements(&body.statements))
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
            ModuleDirective::FuncFunction {
                directive: function,
                ..
            } => {
                seen_function = true;
                let name = &function.name.val;
                let instruction_count = function
                    .body
                    .as_ref()
                    .map(instruction_count_in_body)
                    .unwrap_or(0);
                if instruction_count == 1 {
                    let only_ret = function
                        .body
                        .as_ref()
                        .and_then(|body| first_instruction_in_statements(&body.statements))
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
            ModuleDirective::AliasFunction { .. } => {
                seen_function = true;
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

fn first_instruction_in_statements(statements: &[FunctionStatement]) -> Option<&Instruction> {
    for statement in statements {
        match statement {
            FunctionStatement::Instruction {
                instruction: inst, ..
            } => return Some(inst),
            FunctionStatement::Block {
                statements: block, ..
            } => {
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
fn test_entry_function_forward_declaration() {
    ptx_parser::run_with_large_stack(|| {
        // Test parsing entry function forward declarations (prototypes)
        let source = r#".version 8.0
.target sm_80
.address_size 64

.visible .entry test();

.visible .entry test()
{
    ret;
}
"#;

        let tokens = tokenize(source).expect("tokenization should succeed");
        let mut stream = PtxTokenStream::new(&tokens);
        let (module, _) = Module::parse()(&mut stream).expect("module parsing should succeed");

        // Should have 5 directives: version, target, address_size, entry prototype, entry definition
        assert_eq!(module.directives.len(), 5, "should have 5 directives");

        // Find entry functions
        let entries: Vec<_> = module
            .directives
            .iter()
            .filter_map(|d| match d {
                ModuleDirective::EntryFunction { directive, .. } => Some(directive),
                _ => None,
            })
            .collect();

        assert_eq!(entries.len(), 2, "should have 2 entry function directives");

        // First entry should be a forward declaration (no body)
        assert!(
            entries[0].body.is_none(),
            "first entry should be a forward declaration (no body)"
        );
        assert_eq!(entries[0].name.val, "test", "first entry should be named 'test'");

        // Second entry should have a body
        assert!(
            entries[1].body.is_some(),
            "second entry should have a body"
        );
        assert_eq!(entries[1].name.val, "test", "second entry should be named 'test'");

        // Test roundtrip
        util::assert_roundtrip::<Module>(source);
    });
}

#[test]
fn test_parse_labels_and_predicates() {
    ptx_parser::run_with_large_stack(|| {
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
        let (module, _) = Module::parse()(&mut stream).expect("module parsing should succeed");

        let entry = module
            .directives
            .iter()
            .find_map(|d| match d {
                ptx_parser::r#type::ModuleDirective::EntryFunction {
                    directive: entry, ..
                } => Some(entry),
                _ => None,
            })
            .expect("should find entry function");

        let mut labeled_instructions = Vec::new();
        let mut pending_label: Option<String> = None;
        let body = entry
            .body
            .as_ref()
            .expect("entry function should have a body");
        for statement in &body.statements {
            match statement {
                FunctionStatement::Label { label, .. } => pending_label = Some(label.val.clone()),
                FunctionStatement::Instruction {
                    instruction: inst, ..
                } => {
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
    });
}
