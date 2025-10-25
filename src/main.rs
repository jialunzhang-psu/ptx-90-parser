mod format;

use std::path::PathBuf;

use clap::Parser;
use format::module_to_tree;
use ptx_parser::{
    ExternCallSetup, FunctionBody, FunctionKernelDirective, FunctionStatement, ModuleDirective,
    parse,
};
#[derive(Parser, Debug)]
#[command(name = "ptx-parser-bin", about = "Parse PTX files and emit a summary")]
struct Cli {
    /// Path to the PTX source file to parse.
    #[arg(short, long)]
    input: PathBuf,
    /// Optional path to write the summary; stdout when omitted.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let source = match std::fs::read_to_string(&cli.input) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Failed to read {}: {err}", cli.input.display());
            std::process::exit(1);
        }
    };

    let module = match parse(&source) {
        Ok(module) => module,
        Err(err) => {
            eprintln!("Failed to parse {}: {err}", cli.input.display());
            std::process::exit(1);
        }
    };

    let function_count = module
        .directives
        .iter()
        .filter(|directive| {
            matches!(
                directive,
                ModuleDirective::FunctionKernel(FunctionKernelDirective::Entry(_))
                    | ModuleDirective::FunctionKernel(FunctionKernelDirective::Func(_))
            )
        })
        .count();
    println!("functions {}", function_count);

    println!("Parsed {}", cli.input.display());
    println!("  directives: {}", module.directives.len());
    println!("  functions : {}", function_count);
    for directive in &module.directives {
        match directive {
            ModuleDirective::FunctionKernel(FunctionKernelDirective::Entry(function)) => {
                log_function_summary(function.name.as_str(), &function.body);
            }
            ModuleDirective::FunctionKernel(FunctionKernelDirective::Func(function)) => {
                log_function_summary(function.name.as_str(), &function.body);
            }
            _ => continue,
        };
    }

    if let Some(output_path) = cli.output {
        let ast_repr = module_to_tree(&module);
        if let Err(err) = std::fs::write(&output_path, ast_repr) {
            eprintln!("Failed to write {}: {err}", output_path.display());
            std::process::exit(1);
        }
    }
}

fn log_function_summary(name: &str, body: &FunctionBody) {
    let directive_count = body.entry_directives.len();
    let (statement_count, instruction_count) = count_statements_and_instructions(&body.statements);
    println!(
        "    {}: {} entry directives, {} statement directives, {} instructions",
        name, directive_count, statement_count, instruction_count
    );
}

fn count_statements_and_instructions(statements: &[FunctionStatement]) -> (usize, usize) {
    statements.iter().fold(
        (0usize, 0usize),
        |(stmt_total, instr_total), stmt| match stmt {
            FunctionStatement::Directive(_) => (stmt_total + 1, instr_total),
            FunctionStatement::Instruction(_) => (stmt_total, instr_total + 1),
            FunctionStatement::Label(_) => (stmt_total, instr_total),
            FunctionStatement::ExternCallBlock(block) => {
                let mut block_statements = block.declarations.len();
                let mut block_instructions = 0usize;

                for step in &block.setup {
                    match step {
                        ExternCallSetup::Param(_) => block_statements += 1,
                        ExternCallSetup::Store(_) => block_instructions += 1,
                    }
                }

                block_instructions += 1; // the call itself
                block_instructions += block.post_call.len();

                (
                    stmt_total + block_statements,
                    instr_total + block_instructions,
                )
            }
        },
    )
}
