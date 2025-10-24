use std::{fs, path::Path};

use ptx_parser::parse;

#[test]
fn parses_sample_files() {
    let samples_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/samples");
    let mut ptx_files = Vec::new();
    collect_ptx_files(&samples_root, &mut ptx_files);
    assert!(
        !ptx_files.is_empty(),
        "expected to find at least one PTX sample in {}",
        samples_root.display()
    );

    for file in ptx_files {
        let source = fs::read_to_string(&file)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", file.display()));
        parse(&source).unwrap_or_else(|err| panic!("failed to parse {}: {err:?}", file.display()));
    }
}

fn collect_ptx_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("failed to read sample directory {}: {err}", dir.display()))
    {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            collect_ptx_files(&path, out);
        } else if path
            .extension()
            .map(|ext| ext.eq_ignore_ascii_case("ptx"))
            .unwrap_or(false)
        {
            out.push(path);
        }
    }
}
