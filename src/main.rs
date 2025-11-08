use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use ptx_parser::parse_ptx;

#[derive(Parser)]
#[command(name = "ptx-parser", about = "Utilities for parsing PTX assembly")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Parse a PTX source file as a complete module.
    ParseFile {
        /// Path to the PTX source file to parse.
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

    println!(
        "✓ {}: Successfully parsed PTX module with {} directives",
        path.display(),
        module.directives.len()
    );

    Ok(())
}
