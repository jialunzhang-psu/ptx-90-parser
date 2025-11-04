use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use ptx_parser::parser::{InstructionParseSummary, analyze_instruction_stream, parse_ptx};

#[derive(Parser)]
#[command(name = "ptx-parser", about = "Utilities for parsing PTX assembly")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Parse instructions from a PTX source file and report successes/failures.
    ParseFile {
        /// Path to the PTX source file to analyse.
        input_file: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::ParseFile { input_file } => parse_file(&input_file)?,
    }

    Ok(())
}

fn parse_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;
    let module = parse_ptx(&source)?;
    let summary = analyze_instruction_stream(&source);

    print_report(path, &summary);

    if summary.failures.is_empty() {
        println!(
            "Successfully parsed PTX module with {} directives",
            module.directives.len()
        );
    }

    if summary.failures.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{} instruction parse failures encountered",
            summary.failures.len()
        )
        .into())
    }
}

fn print_report(path: &Path, summary: &InstructionParseSummary) {
    println!(
        "{}: parsed {} statements, {} failed",
        path.display(),
        summary.parsed,
        summary.failures.len()
    );

    let max_reported = 5;
    for failure in summary.failures.iter().take(max_reported) {
        println!(
            "  statement {} failed: {}\n    error: {}",
            failure.index, failure.statement, failure.error
        );
    }

    if summary.failures.len() > max_reported {
        println!(
            "  ... and {} more failures",
            summary.failures.len() - max_reported
        );
    }
}
