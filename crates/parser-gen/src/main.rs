//! Parser generator CLI tool (debug-oriented)

use clap::{Parser, Subcommand};
use parser_gen::analyzer::Analyzer;
use parser_gen::parser_generator::ParserGenerator;
use parser_gen::r#type::{Modifier, Rule, Section};
use parser_gen::type_generator::TypeGenerator;
use parser_gen::unparser_generator::UnparserGenerator;
use ptx_parser_gen as parser_gen;
use std::collections::BTreeSet;
use std::fs;
use std::panic;
use std::path::{Path, PathBuf};

/// Information about a module (instruction file)
#[derive(Debug, Clone)]
struct ModuleInfo {
    /// Module name (e.g., "add", "mul")
    module_name: String,
    /// List of instruction struct names defined in this module, with section info
    /// Format: (section_name, struct_name), e.g., ("section_0", "AddType")
    instruction_structs: Vec<(String, String)>,
}

/// Information about a generated parser module
#[derive(Debug, Clone)]
struct ParserModuleInfo {
    /// Module name (e.g., "add", "mul")
    module_name: String,
    /// Canonical opcode strings used to dispatch to this module
    /// Multiple opcodes needed when a file contains instructions with different opcodes
    /// (e.g., bar.txt has both "bar" and "barrier")
    opcodes: Vec<String>,
    /// Struct names for which parser implementations were emitted, with section info
    /// Format: (section_name, struct_name), e.g., ("section_0", "AddType")
    instruction_structs: Vec<(String, String)>,
}

#[derive(Parser)]
#[command(name = "parser-gen")]
#[command(about = "PTX syntax parser utilities", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Parse every file in the provided directory and print collected parameter names + modifiers.
    ParseDir {
        #[arg(value_name = "DIR")]
        dir: PathBuf,
    },
    /// Pretty-print the tree representation for an inline PTX specification string.
    PrettyString {
        #[arg(value_name = "SPEC")]
        spec: String,
    },
    /// Generate Rust type definitions from PTX specification files.
    GenerateType {
        #[arg(value_name = "INPUT_DIR")]
        input_dir: PathBuf,
        #[arg(value_name = "OUTPUT_DIR")]
        output_dir: PathBuf,
    },
    /// Generate Rust parser implementations from PTX specification files.
    GenerateParser {
        #[arg(value_name = "INPUT_DIR")]
        input_dir: PathBuf,
        #[arg(value_name = "OUTPUT_DIR")]
        output_dir: PathBuf,
    },
    /// Generate Rust unparser implementations from PTX specification files.
    GenerateUnparser {
        #[arg(value_name = "INPUT_DIR")]
        input_dir: PathBuf,
        #[arg(value_name = "OUTPUT_DIR")]
        output_dir: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::ParseDir { dir } => parse_directory(&dir)?,
        Command::PrettyString { spec } => {
            let pretty = parser_gen::parse_spec_to_treesitter(&spec)?;
            println!("{}", pretty.trim_end());
        }
        Command::GenerateType {
            input_dir,
            output_dir,
        } => {
            generate_types(&input_dir, &output_dir)?;
        }
        Command::GenerateParser {
            input_dir,
            output_dir,
        } => {
            generate_parsers(&input_dir, &output_dir)?;
        }
        Command::GenerateUnparser {
            input_dir,
            output_dir,
        } => {
            generate_unparsers(&input_dir, &output_dir)?;
        }
    }
    Ok(())
}

fn parse_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .collect();
    entries.sort_unstable_by(|a, b| a.path().cmp(&b.path()));

    let mut error_files = Vec::new();
    let mut panic_files = Vec::new();

    for entry in entries {
        let path = entry.path();
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let file_name = path.display().to_string();

        // Print file name before processing so we know which file caused a panic
        eprint!("Processing: {} ... ", file_name);

        // Catch panics to continue processing other files
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let tops = parser_gen::parse_spec_with_name(&content, &file_name)?;
            let (params, modifiers) = collect_debug_info(&tops);
            Ok::<_, Box<dyn std::error::Error>>((params, modifiers))
        }));

        match result {
            Ok(Ok((params, modifiers))) => {
                eprintln!("OK");
                println!(
                    "{} | params: {} | modifiers: {}",
                    file_name,
                    format_list(&params),
                    format_list(&modifiers)
                );
            }
            Ok(Err(e)) => {
                eprintln!("ERROR");
                eprintln!("  Error in {}: {}", file_name, e);
                error_files.push(file_name);
            }
            Err(panic_info) => {
                eprintln!("PANIC");
                eprintln!("  Panic in {}: {:?}", file_name, panic_info);
                panic_files.push(file_name);
            }
        }
    }

    let error_count = error_files.len();
    let panic_count = panic_files.len();

    if error_count > 0 || panic_count > 0 {
        eprintln!("\n{}", "=".repeat(80));
        eprintln!("Summary: {} errors, {} panics", error_count, panic_count);

        if !error_files.is_empty() {
            eprintln!("\nFiles with errors:");
            for file in &error_files {
                eprintln!("  - {}", file);
            }
        }

        if !panic_files.is_empty() {
            eprintln!("\nFiles with panics:");
            for file in &panic_files {
                eprintln!("  - {}", file);
            }
        }

        eprintln!("{}", "=".repeat(80));
        Err(format!(
            "Processing failed: {} errors, {} panics",
            error_count, panic_count
        )
        .into())
    } else {
        Ok(())
    }
}

fn collect_debug_info(tops: &[Section]) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut params = BTreeSet::new();
    let mut modifiers = BTreeSet::new();
    for top in tops {
        for rule in &top.rules {
            match rule {
                Rule::Parameter(param) => {
                    params.insert(param.name.clone());
                }
                Rule::Instruction(instr) => {
                    for modifier in &instr.head.modifiers {
                        modifiers.insert(modifier_label(modifier));
                    }
                }
            }
        }
    }
    (params, modifiers)
}

fn format_list(items: &BTreeSet<String>) -> String {
    if items.is_empty() {
        "EMPTY".to_string()
    } else {
        items.iter().cloned().collect::<Vec<_>>().join(", ")
    }
}

fn modifier_label(modifier: &Modifier) -> String {
    match modifier {
        Modifier::Atom(id) => id.clone(),
        Modifier::Optional(id) => format!("?{}", id),
        Modifier::Sequence(items) => {
            let parts = items.join(" -> ");
            format!("seq({parts})")
        }
    }
}

fn generate_types(input_dir: &Path, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    // Get all .txt files from input directory
    let mut entries: Vec<_> = fs::read_dir(input_dir)?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().is_file() && entry.path().extension().map_or(false, |ext| ext == "txt")
        })
        .collect();
    entries.sort_unstable_by(|a, b| a.path().cmp(&b.path()));

    let mut success_count = 0;
    let mut error_count = 0;
    let mut module_info = Vec::new();

    for entry in entries {
        let path = entry.path();
        let file_name = path.file_stem().unwrap().to_string_lossy();

        eprint!("Processing: {} ... ", file_name);

        match process_file(&path, output_dir) {
            Ok(info) => {
                eprintln!("OK");
                success_count += 1;
                module_info.push(info);
            }
            Err(e) => {
                eprintln!("ERROR");
                eprintln!("  Error: {}", e);
                error_count += 1;
            }
        }
    }

    eprintln!(
        "\nSummary: {} succeeded, {} failed",
        success_count, error_count
    );

    if error_count > 0 {
        return Err(format!("Generation failed: {} errors", error_count).into());
    }

    // Generate the mod.rs file with Instruction enum
    eprintln!("Generating type mod.rs ...");
    // Convert ModuleInfo to the format expected by type_generator
    let modules: Vec<(String, Vec<(String, String)>)> = module_info
        .iter()
        .map(|info| (info.module_name.clone(), info.instruction_structs.clone()))
        .collect();
    let content = parser_gen::type_generator::generate_mod_rs_content_v2(&modules);
    let mod_path = output_dir.join("mod.rs");
    fs::write(&mod_path, content)?;
    eprintln!("type mod.rs generated successfully");

    Ok(())
}

fn process_file(
    input_path: &Path,
    output_dir: &Path,
) -> Result<ModuleInfo, Box<dyn std::error::Error>> {
    // Read the file
    let content = fs::read_to_string(input_path)?;
    let file_name = input_path.file_name().unwrap().to_string_lossy();

    // Parse the specification
    let sections = parser_gen::parse_spec_with_name(&content, &file_name)?;

    if sections.is_empty() {
        return Err("No sections found in file".into());
    }

    // Analyze all sections at once
    let mut analyzer = Analyzer::new();
    let analyzed_sections = analyzer.analyze_sections(&sections);

    if analyzed_sections.is_empty() {
        return Err("No instructions found".into());
    }

    // Determine module name based on the file name
    let module_name = input_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .replace('.', "_");

    // Generate type definitions section by section
    // Use a single TypeGenerator instance to track generated enums globally
    let mut all_outputs = Vec::new();
    let mut type_gen = TypeGenerator::new();

    for (section_idx, section) in analyzed_sections.iter().enumerate() {
        let generated = type_gen.generate(section, section_idx);

        if !generated.code.trim().is_empty() {
            all_outputs.push(generated);
        }
    }

    if all_outputs.is_empty() {
        return Err("No instructions found".into());
    }

    // Collect all instruction structs with their section info
    let all_instruction_structs: Vec<(String, String)> = all_outputs
        .iter()
        .flat_map(|output| {
            let section_name = output.module_name.clone();
            output
                .instruction_structs
                .iter()
                .map(move |struct_name| (section_name.clone(), struct_name.clone()))
        })
        .collect();

    // Create comment header with original specification
    let mut output = String::new();
    output.push_str("//! Original PTX specification:\n");
    output.push_str("//!\n");
    for line in content.lines() {
        output.push_str("//! ");
        output.push_str(line);
        output.push_str("\n");
    }
    output.push_str("\n");
    output.push_str("#![allow(unused)]\n");
    output.push_str("use crate::r#type::common::*;\n");
    output.push_str("\n");

    // Append generated code for each section
    for (idx, gen_output) in all_outputs.iter().enumerate() {
        if idx > 0 {
            output.push_str("\n");
        }
        output.push_str(&gen_output.code);
        if !gen_output.code.ends_with('\n') {
            output.push('\n');
        }
    }

    // Determine output file name based on the instruction name
    // Use the file stem (without .txt) as the base name
    let output_file_name = input_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .replace('.', "_"); // Replace dots with underscores for valid Rust module names

    let output_path = output_dir.join(format!("{}.rs", output_file_name));

    // Write the generated code to the output file
    fs::write(&output_path, output)?;

    Ok(ModuleInfo {
        module_name,
        instruction_structs: all_instruction_structs,
    })
}

fn generate_parsers(input_dir: &Path, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(output_dir)?;

    let mut entries: Vec<_> = fs::read_dir(input_dir)?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().is_file() && entry.path().extension().map_or(false, |ext| ext == "txt")
        })
        .collect();
    entries.sort_unstable_by(|a, b| a.path().cmp(&b.path()));

    let mut success_count = 0;
    let mut error_count = 0;
    let mut module_info = Vec::new();

    for entry in entries {
        let path = entry.path();
        let file_name = path.file_stem().unwrap().to_string_lossy();

        eprint!("Processing: {} ... ", file_name);

        match process_parser_file(&path, output_dir) {
            Ok(info) => {
                eprintln!("OK");
                success_count += 1;
                module_info.push(info);
            }
            Err(e) => {
                eprintln!("ERROR");
                eprintln!("  Error: {}", e);
                error_count += 1;
            }
        }
    }

    eprintln!(
        "\nSummary: {} succeeded, {} failed",
        success_count, error_count
    );

    if error_count > 0 {
        return Err(format!("Generation failed: {} errors", error_count).into());
    }

    eprintln!("Generating parser mod.rs ...");
    // Extract module info with opcodes and structs
    let modules: Vec<(String, Vec<String>, Vec<(String, String)>)> = module_info
        .iter()
        .map(|info| {
            (
                info.module_name.clone(),
                info.opcodes.clone(),
                info.instruction_structs.clone(),
            )
        })
        .collect();
    let content = parser_gen::parser_generator::generate_parser_mod_rs_content(&modules);
    let mod_path = output_dir.join("mod.rs");
    fs::write(&mod_path, content)?;
    eprintln!("parser mod.rs generated successfully");

    Ok(())
}

fn process_parser_file(
    input_path: &Path,
    output_dir: &Path,
) -> Result<ParserModuleInfo, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let file_name = input_path.file_name().unwrap().to_string_lossy();

    let sections = parser_gen::parse_spec_with_name(&content, &file_name)?;

    if sections.is_empty() {
        return Err("No sections found in file".into());
    }

    // Analyze all sections at once
    let mut analyzer = Analyzer::new();
    let analyzed_sections = analyzer.analyze_sections(&sections);

    if analyzed_sections.is_empty() {
        return Err("No instructions found".into());
    }

    // Collect ALL unique opcodes from ALL instructions in the file
    // This is needed for files like bar.txt which contain both "bar" and "barrier" instructions
    use std::collections::BTreeSet;
    let opcodes: Vec<String> = analyzed_sections
        .iter()
        .flat_map(|section| section.opcodes.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    if opcodes.is_empty() {
        return Err("No instructions found".into());
    }

    let module_name = input_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .replace('.', "_");

    // Generate parser definitions section by section
    // Use a single ParserGenerator instance to track generated enums globally
    let mut all_outputs = Vec::new();
    let mut parser_gen = ParserGenerator::new();

    for (section_idx, section) in analyzed_sections.iter().enumerate() {
        let generated = parser_gen.generate(section, section_idx, &module_name);

        if !generated.code.trim().is_empty() {
            all_outputs.push(generated);
        }
    }

    if all_outputs.is_empty() {
        return Err("No instructions found".into());
    }

    // Collect all instruction structs with their section info
    let all_instruction_structs: Vec<(String, String)> = analyzed_sections
        .iter()
        .flat_map(|section| {
            let section_name = format!("section_{}", analyzed_sections.iter().position(|s| std::ptr::eq(s, section)).unwrap());
            section.instructions.iter().map(move |instr| {
                (section_name.clone(), instr.rust_name.clone())
            })
        })
        .collect();

    let mut output = String::new();
    output.push_str("//! Original PTX specification:\n");
    output.push_str("//!\n");
    for line in content.lines() {
        output.push_str("//! ");
        output.push_str(line);
        output.push_str("\n");
    }
    output.push_str("\n");
    output.push_str("#![allow(unused)]\n");
    output.push_str("\n");

    // Add imports once
    output.push_str(&ParserGenerator::generate_imports());
    output.push_str("\n\n");

    // Append generated code for each section
    for gen_output in all_outputs.iter() {
        output.push_str(&gen_output.code);
        output.push_str("\n");
    }

    let output_file_name = input_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .replace('.', "_");

    let output_path = output_dir.join(format!("{}.rs", output_file_name));
    fs::write(&output_path, output)?;

    Ok(ParserModuleInfo {
        module_name,
        opcodes,
        instruction_structs: all_instruction_structs,
    })
}

fn generate_unparsers(
    input_dir: &Path,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(output_dir)?;

    let mut entries: Vec<_> = fs::read_dir(input_dir)?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().is_file() && entry.path().extension().map_or(false, |ext| ext == "txt")
        })
        .collect();
    entries.sort_unstable_by(|a, b| a.path().cmp(&b.path()));

    let mut success_count = 0;
    let mut error_count = 0;
    let mut module_info = Vec::new();

    for entry in entries {
        let path = entry.path();
        let file_name = path.file_stem().unwrap().to_string_lossy();

        eprint!("Processing: {} ... ", file_name);

        match process_unparser_file(&path, output_dir) {
            Ok(info) => {
                eprintln!("OK");
                success_count += 1;
                module_info.push(info);
            }
            Err(e) => {
                eprintln!("ERROR");
                eprintln!("  Error: {}", e);
                error_count += 1;
            }
        }
    }

    eprintln!(
        "\nSummary: {} succeeded, {} failed",
        success_count, error_count
    );

    if error_count > 0 {
        return Err(format!("Generation failed: {} errors", error_count).into());
    }

    eprintln!("Generating unparser mod.rs ...");
    let modules: Vec<(String, Vec<(String, String)>)> = module_info
        .iter()
        .map(|info| (info.module_name.clone(), info.instruction_structs.clone()))
        .collect();
    let content = parser_gen::unparser_generator::generate_unparser_mod_rs_content(&modules);
    let mod_path = output_dir.join("mod.rs");
    fs::write(&mod_path, content)?;
    eprintln!("unparser mod.rs generated successfully");

    Ok(())
}

fn process_unparser_file(
    input_path: &Path,
    output_dir: &Path,
) -> Result<ModuleInfo, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let file_name = input_path.file_name().unwrap().to_string_lossy();

    let sections = parser_gen::parse_spec_with_name(&content, &file_name)?;

    if sections.is_empty() {
        return Err("No sections found in file".into());
    }

    let mut analyzer = Analyzer::new();
    let analyzed_sections = analyzer.analyze_sections(&sections);

    if analyzed_sections.is_empty() {
        return Err("No instructions found".into());
    }

    let module_name = input_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .replace('.', "_");

    let mut all_outputs = Vec::new();
    let mut unparser_gen = UnparserGenerator::new();

    for (section_idx, section) in analyzed_sections.iter().enumerate() {
        let generated = unparser_gen.generate(section, section_idx, &module_name);

        if !generated.code.trim().is_empty() {
            all_outputs.push(generated);
        }
    }

    if all_outputs.is_empty() {
        return Err("No instructions found".into());
    }

    let all_instruction_structs: Vec<(String, String)> = all_outputs
        .iter()
        .flat_map(|output| {
            let section_name = output.module_name.clone();
            output
                .instruction_structs
                .iter()
                .map(move |struct_name| (section_name.clone(), struct_name.clone()))
        })
        .collect();

    let mut output = String::new();
    output.push_str("//! Original PTX specification:\n");
    output.push_str("//!\n");
    for line in content.lines() {
        output.push_str("//! ");
        output.push_str(line);
        output.push_str("\n");
    }
    output.push_str("\n");
    output.push_str("#![allow(unused)]\n");
    output.push_str("\n");
    output.push_str(&UnparserGenerator::generate_imports());
    output.push_str("\n\n");

    for gen_output in all_outputs.iter() {
        output.push_str(&gen_output.code);
        output.push_str("\n");
    }

    let output_file_name = input_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .replace('.', "_");

    let output_path = output_dir.join(format!("{}.rs", output_file_name));
    fs::write(&output_path, output)?;

    Ok(ModuleInfo {
        module_name,
        instruction_structs: all_instruction_structs,
    })
}
