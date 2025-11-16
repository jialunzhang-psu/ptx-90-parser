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
            ptx_parser::r#type::ModuleDirective::ModuleInfo {
                directive: info, ..
            } => match info {
                ptx_parser::r#type::ModuleInfoDirectiveKind::Version {
                    directive: version, ..
                } => {
                    println!(".version {}.{}", version.major, version.minor);
                }
                ptx_parser::r#type::ModuleInfoDirectiveKind::Target {
                    directive: target, ..
                } => {
                    let values: Vec<&'static str> =
                        target.entries.iter().map(target_string_name).collect();
                    println!(".target {}", values.join(", "));
                }
                ptx_parser::r#type::ModuleInfoDirectiveKind::AddressSize {
                    directive: addr,
                    ..
                } => {
                    let value = match addr.size {
                        ptx_parser::r#type::AddressSize::Size32 { .. } => 32,
                        ptx_parser::r#type::AddressSize::Size64 { .. } => 64,
                    };
                    println!(".address_size {}", value);
                }
            },
            ptx_parser::r#type::ModuleDirective::EntryFunction {
                linkage,
                directive: function,
                ..
            } => {
                if let Some(link) = linkage {
                    println!("{:?}", link);
                }
                println!(".entry {}", function.name.val);
            }
            ptx_parser::r#type::ModuleDirective::FuncFunction {
                linkage,
                directive: function,
                ..
            } => {
                if let Some(link) = linkage {
                    println!("{:?}", link);
                }
                println!(".func {}", function.name.val);
            }
            ptx_parser::r#type::ModuleDirective::AliasFunction {
                directive: alias, ..
            } => {
                println!(".alias {} {}", alias.alias.val, alias.target.val);
            }
            ptx_parser::r#type::ModuleDirective::ModuleVariable {
                linkage,
                directive: var,
                ..
            } => {
                if let Some(link) = linkage {
                    println!("{:?}", link);
                }
                print_module_variable(var);
            }
            other => println!("{:?}", other),
        }
    }
}

fn print_module_variable(var: &ptx_parser::r#type::ModuleVariableDirective) {
    use ptx_parser::r#type::ModuleVariableDirective::*;
    match var {
        Global {
            directive: decl, ..
        } => println!(".global {}", describe_variable_decl(decl)),
        Shared {
            directive: decl, ..
        } => println!(".shared {}", describe_variable_decl(decl)),
        Const {
            directive: decl, ..
        } => println!(".const {}", describe_variable_decl(decl)),
        Tex {
            directive: decl, ..
        } => println!(".tex {}", describe_variable_decl(decl)),
    }
}

fn describe_variable_decl(decl: &ptx_parser::r#type::VariableDirective) -> String {
    let ty = format!("{:?}", decl.ty);
    format!("{} {}", ty, decl.name.val)
}

fn target_string_name(entry: &ptx_parser::r#type::TargetString) -> &'static str {
    use ptx_parser::r#type::TargetString::*;
    match entry {
        Sm120a { .. } => "sm_120a",
        Sm120f { .. } => "sm_120f",
        Sm120 { .. } => "sm_120",
        Sm121a { .. } => "sm_121a",
        Sm121f { .. } => "sm_121f",
        Sm121 { .. } => "sm_121",
        Sm110a { .. } => "sm_110a",
        Sm110f { .. } => "sm_110f",
        Sm110 { .. } => "sm_110",
        Sm100a { .. } => "sm_100a",
        Sm100f { .. } => "sm_100f",
        Sm100 { .. } => "sm_100",
        Sm101a { .. } => "sm_101a",
        Sm101f { .. } => "sm_101f",
        Sm101 { .. } => "sm_101",
        Sm103a { .. } => "sm_103a",
        Sm103f { .. } => "sm_103f",
        Sm103 { .. } => "sm_103",
        Sm90a { .. } => "sm_90a",
        Sm90 { .. } => "sm_90",
        Sm80 { .. } => "sm_80",
        Sm86 { .. } => "sm_86",
        Sm87 { .. } => "sm_87",
        Sm88 { .. } => "sm_88",
        Sm89 { .. } => "sm_89",
        Sm70 { .. } => "sm_70",
        Sm72 { .. } => "sm_72",
        Sm75 { .. } => "sm_75",
        Sm60 { .. } => "sm_60",
        Sm61 { .. } => "sm_61",
        Sm62 { .. } => "sm_62",
        Sm50 { .. } => "sm_50",
        Sm52 { .. } => "sm_52",
        Sm53 { .. } => "sm_53",
        Sm30 { .. } => "sm_30",
        Sm32 { .. } => "sm_32",
        Sm35 { .. } => "sm_35",
        Sm37 { .. } => "sm_37",
        Sm20 { .. } => "sm_20",
        Sm10 { .. } => "sm_10",
        Sm11 { .. } => "sm_11",
        Sm12 { .. } => "sm_12",
        Sm13 { .. } => "sm_13",
        TexmodeUnified { .. } => "texmode_unified",
        TexmodeIndependent { .. } => "texmode_independent",
        Debug { .. } => "debug",
        MapF64ToF32 { .. } => "map_f64_to_f32",
    }
}

fn print_function_body(body: &Option<ptx_parser::r#type::FunctionBody>, indent: usize) {
    match body {
        Some(body) => {
            for statement in &body.statements {
                print_function_statement(statement, indent);
            }
        }
        None => println!("{:indent$};", "", indent = indent),
    }
}

fn print_function_statement(statement: &ptx_parser::r#type::FunctionStatement, indent: usize) {
    let indent_str = " ".repeat(indent);
    match statement {
        ptx_parser::r#type::FunctionStatement::Label { label, .. } => {
            println!("{indent_str}{}:", label.val);
        }
        ptx_parser::r#type::FunctionStatement::Instruction {
            instruction: inst, ..
        } => {
            println!("{indent_str}{:?}", inst);
        }
        ptx_parser::r#type::FunctionStatement::Directive { directive: dir, .. } => {
            println!("{indent_str}{:?}", dir);
        }
        ptx_parser::r#type::FunctionStatement::Block {
            statements: block, ..
        } => {
            println!("{indent_str}{{");
            for stmt in block {
                print_function_statement(stmt, indent + 2);
            }
            println!("{indent_str}}}");
        }
    }
}
