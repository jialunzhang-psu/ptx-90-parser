use std::fs;
use std::path::{Path, PathBuf};

use ptx_parser::lexer::tokenize;
use ptx_parser::parser::{PtxParser, PtxTokenStream};
use ptx_parser::r#type::module::Module;

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
