use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use ptx_parser::{parse_ptx, run_with_large_stack};
use ptx_parser::pretty_print::{TreeDisplay, TreeFormatter, print_compact_module};

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
    /// Parse a PTX file and dump the AST.
    PrintAst {
        /// Path to the PTX source file to parse and inspect.
        input_file: PathBuf,
        /// Emit the AST in a compact, single-line-per-instruction format.
        #[arg(long)]
        compact: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::ParseFile { input_file } => parse_file(&input_file)?,
        Command::PrintAst {
            input_file,
            compact,
        } => print_ast(&input_file, compact)?,
    }

    Ok(())
}

fn parse_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;
    let module = parse_with_large_stack(source)?;

    println!(
        "✓ {}: Successfully parsed PTX module with {} directives",
        path.display(),
        module.directives.len()
    );

    Ok(())
}

fn print_ast(path: &Path, compact: bool) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;
    let module = parse_with_large_stack(source.clone())?;

    if compact {
        print_compact_module(&module);
    } else {
        print_ast_with_large_stack(module, source)?;
    }

    Ok(())
}

fn print_ast_with_large_stack(module: ptx_parser::r#type::Module, source: String) -> Result<(), Box<dyn std::error::Error>> {
    run_with_large_stack(move || {
        let mut formatter = TreeFormatter::new();
        module
            .tree_display(&mut formatter, &source)
            .expect("Failed to display tree");
        println!("{}", formatter.finish());
    });
    Ok(())
}


fn parse_with_large_stack(source: String) -> Result<ptx_parser::r#type::Module, ptx_parser::PtxParseError> {
    run_with_large_stack(move || parse_ptx(&source))
}
