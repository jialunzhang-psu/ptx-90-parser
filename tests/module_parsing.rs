use std::fs;
use std::path::Path;

use ptx_parser::lexer::tokenize;
use ptx_parser::parser::{PtxParser, PtxTokenStream};
use ptx_parser::r#type::module::Module;

#[test]
fn test_hello_ptx_module() {
    let source = fs::read_to_string("tests/sample/hello.ptx").expect("Failed to read hello.ptx");

    let tokens = tokenize(&source).expect("Tokenization failed");
    let mut stream = PtxTokenStream::new(&tokens);
    let module = Module::parse(&mut stream).expect("Module parsing failed");

    // Verify we parsed directives
    assert!(
        !module.directives.is_empty(),
        "Module should have directives"
    );
    eprintln!(
        "hello.ptx: Successfully parsed {} directives",
        module.directives.len()
    );
}

#[test]
fn test_matrix_ptx_module() {
    let source = fs::read_to_string("tests/sample/matrix_free_device_matrix_vector_01.1.sm_80.ptx")
        .expect("Failed to read matrix file");

    let tokens = tokenize(&source).expect("Tokenization failed");
    let mut stream = PtxTokenStream::new(&tokens);
    let module = Module::parse(&mut stream).expect("Module parsing failed");

    // Verify we parsed directives
    assert!(
        !module.directives.is_empty(),
        "Module should have directives"
    );
    eprintln!(
        "matrix file: Successfully parsed {} directives",
        module.directives.len()
    );
}

#[test]
fn test_all_samples_parse() {
    let samples_dir = Path::new("tests/sample");
    let mut count = 0;
    let mut success = 0;

    for entry in fs::read_dir(samples_dir).expect("samples directory missing") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.extension().map(|e| e == "ptx").unwrap_or(false) {
            count += 1;
            let source = fs::read_to_string(&path).expect("Failed to read file");

            match tokenize(&source) {
                Ok(tokens) => {
                    let mut stream = PtxTokenStream::new(&tokens);
                    match Module::parse(&mut stream) {
                        Ok(module) => {
                            success += 1;
                            eprintln!(
                                "✓ {}: {} directives",
                                path.file_name().unwrap().to_string_lossy(),
                                module.directives.len()
                            );
                        }
                        Err(e) => {
                            eprintln!(
                                "✗ {}: Parse error: {:?}",
                                path.file_name().unwrap().to_string_lossy(),
                                e
                            );
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "✗ {}: Tokenization error: {:?}",
                        path.file_name().unwrap().to_string_lossy(),
                        e
                    );
                }
            }
        }
    }

    eprintln!("\nModule parsing: {}/{} files successful", success, count);
    assert_eq!(success, count, "All PTX files should parse successfully");
}
