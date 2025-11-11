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
    let module = parse_ptx(&source)?;

    println!(
        "✓ {}: Successfully parsed PTX module with {} directives",
        path.display(),
        module.directives.len()
    );

    Ok(())
}

fn print_ast(path: &Path, compact: bool) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;
    let module = parse_ptx(&source)?;

    if compact {
        print_compact_module(&module);
    } else {
        println!("{:#?}", module);
    }

    Ok(())
}

fn print_compact_module(module: &ptx_parser::r#type::Module) {
    for directive in &module.directives {
        match directive {
            ptx_parser::r#type::ModuleDirective::ModuleInfo { directive: info, .. } => match info {
                ptx_parser::r#type::ModuleInfoDirectiveKind::Version { directive: version, .. } => {
                    println!(".version {}.{}", version.major, version.minor);
                }
                ptx_parser::r#type::ModuleInfoDirectiveKind::Target { directive: target, .. } => {
                    println!(".target {}", target.entries.join(", "));
                }
                ptx_parser::r#type::ModuleInfoDirectiveKind::AddressSize { directive: addr, .. } => {
                    println!(".address_size {}", addr.size);
                }
            },
            ptx_parser::r#type::ModuleDirective::FunctionKernel { directive: function, .. } => {
                print_function_directive(function);
            }
            ptx_parser::r#type::ModuleDirective::ModuleVariable { directive: var, .. } => {
                print_module_variable(var);
            }
            other => println!("{:?}", other),
        }
    }
}

fn print_module_variable(var: &ptx_parser::r#type::ModuleVariableDirective) {
    use ptx_parser::r#type::ModuleVariableDirective::*;
    match var {
        Global { directive: decl, .. } => println!(".global {}", describe_variable_decl(decl)),
        Shared { directive: decl, .. } => println!(".shared {}", describe_variable_decl(decl)),
        Const { directive: decl, .. } => println!(".const {}", describe_variable_decl(decl)),
        Tex { directive: decl, .. } => println!(".tex {}", describe_variable_decl(decl)),
    }
}

fn describe_variable_decl(decl: &ptx_parser::r#type::VariableDirective) -> String {
    let ty = decl
        .ty
        .as_ref()
        .map(|t| format!("{:?}", t))
        .unwrap_or_else(|| "<?>".to_string());
    format!("{} {}", ty, decl.name)
}

fn print_function_directive(function: &ptx_parser::r#type::FunctionKernelDirective) {
    use ptx_parser::r#type::FunctionKernelDirective::*;
    match function {
        Entry { function: entry, .. } => {
            println!(".entry {} (params: {})", entry.name, entry.params.len());
            print_function_body(&entry.body, 2);
        }
        Func { function: func, .. } => {
            println!(".func {} (params: {})", func.name, func.params.len());
            print_function_body(&func.body, 2);
        }
        Alias { alias, .. } => {
            println!(".alias {} -> {}", alias.alias, alias.target);
        }
    }
}

fn print_function_body(body: &ptx_parser::r#type::FunctionBody, indent: usize) {
    for statement in &body.statements {
        print_function_statement(statement, indent);
    }
}

fn print_function_statement(statement: &ptx_parser::r#type::FunctionStatement, indent: usize) {
    let indent_str = " ".repeat(indent);
    match statement {
        ptx_parser::r#type::FunctionStatement::Label { name, .. } => {
            println!("{indent_str}{name}:");
        }
        ptx_parser::r#type::FunctionStatement::Instruction { instruction: inst, .. } => {
            println!("{indent_str}{:?}", inst);
        }
        ptx_parser::r#type::FunctionStatement::Directive { directive: dir, .. } => {
            println!("{indent_str}{:?}", dir);
        }
        ptx_parser::r#type::FunctionStatement::Block { statements: block, .. } => {
            println!("{indent_str}{{");
            for stmt in block {
                print_function_statement(stmt, indent + 2);
            }
            println!("{indent_str}}}");
        }
    }
}
