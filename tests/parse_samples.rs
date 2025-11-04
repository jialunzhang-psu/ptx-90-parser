use std::fs;
use std::path::{Path, PathBuf};

use ptx_parser::parser::{InstructionParseSummary, analyze_instruction_stream};

#[test]
fn parse_sample_ptx_files() {
    let samples_dir = Path::new("tests/sample");
    let mut entries: Vec<PathBuf> = fs::read_dir(samples_dir)
        .expect("samples directory missing")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().map(|ext| ext == "ptx").unwrap_or(false))
        .collect();

    entries.sort();

    for path in entries {
        parse_sample_file(&path);
    }
}

fn parse_sample_file(path: &Path) {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));

    let summary = analyze_instruction_stream(&source);

    eprintln!(
        "{}: parsed {} statements, {} failed",
        path.display(),
        summary.parsed,
        summary.failures.len()
    );

    report_failures(&summary);

    assert!(
        summary.parsed > 0,
        "no statements parsed successfully in {}",
        path.display()
    );
}

fn report_failures(summary: &InstructionParseSummary) {
    let max_reported = 5;
    for failure in summary.failures.iter().take(max_reported) {
        eprintln!(
            "  statement {} failed: {}\n    error: {}",
            failure.index, failure.statement, failure.error
        );
    }
    if summary.failures.len() > max_reported {
        eprintln!(
            "  ... and {} more failures",
            summary.failures.len() - max_reported
        );
    }
}
