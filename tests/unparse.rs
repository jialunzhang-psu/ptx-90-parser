use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use ptx_parser::{Module, ModuleDirective, ModuleDirectiveKind, parse, unparse};
use tempfile::tempdir;

#[test]
fn unparse_roundtrip_validates_with_ptxas() {
    if !ptxas_available() {
        eprintln!("skipping unparse_roundtrip_validates_with_ptxas: ptxas not available");
        return;
    }

    let samples_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("samples");

    let mut ptx_files = Vec::new();
    collect_ptx_files(&samples_root, &mut ptx_files);
    assert!(
        !ptx_files.is_empty(),
        "expected PTX samples under {}",
        samples_root.display()
    );

    for file in ptx_files {
        run_roundtrip_for_file(&file);
    }
}

fn ptxas_available() -> bool {
    match Command::new("ptxas").arg("--version").output() {
        Ok(_) => true,
        Err(err) => err.kind() != std::io::ErrorKind::NotFound,
    }
}

fn find_target_arch(module: &Module) -> Option<String> {
    for directive in &module.directives {
        if let ModuleDirective::Module(ModuleDirectiveKind::Target(target)) = directive {
            for entry in &target.entries {
                if entry.starts_with("sm_") {
                    return Some(entry.clone());
                }
            }
        }
    }
    None
}

fn collect_ptx_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in
        fs::read_dir(dir).unwrap_or_else(|err| panic!("failed to read {}: {err}", dir.display()))
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

fn run_roundtrip_for_file(file: &Path) {
    let source = fs::read_to_string(file)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", file.display()));

    let module =
        parse(&source).unwrap_or_else(|err| panic!("failed to parse {}: {err:?}", file.display()));
    let regenerated = unparse(&module);
    let reparsed = match parse(&regenerated) {
        Ok(value) => value,
        Err(err) => {
            let debug_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target")
                .join("debug")
                .join("unparse_failures");
            fs::create_dir_all(&debug_dir).expect("failed to create target/debug/unparse_failures");
            let debug_path = debug_dir.join(
                file.file_name()
                    .unwrap_or_else(|| std::ffi::OsStr::new("unknown.ptx")),
            );
            fs::write(&debug_path, &regenerated)
                .expect("failed to persist regenerated PTX for debugging");
            panic!(
                "failed to parse regenerated PTX for {}: {err:?}\nwrote debug copy to {}",
                file.display(),
                debug_path.display()
            );
        }
    };

    assert_eq!(
        module,
        reparsed,
        "AST differs after unparse -> parse round trip for {}",
        file.display()
    );

    let temp_dir = tempdir().expect("failed to create temp directory for ptxas");
    let input_path = temp_dir.path().join("roundtrip.ptx");
    fs::write(&input_path, regenerated).expect("failed to write regenerated PTX");

    let cubin_path = temp_dir.path().join("roundtrip.cubin");
    let mut cmd = Command::new("ptxas");

    if let Some(arch) = find_target_arch(&module) {
        cmd.arg(format!("-arch={arch}"));
    }

    let output = cmd
        .arg(&input_path)
        .arg("-o")
        .arg(&cubin_path)
        .output()
        .unwrap_or_else(|err| panic!("failed to invoke ptxas on {}: {err}", file.display()));

    if !output.status.success() {
        let debug_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("unparse_failures");
        fs::create_dir_all(&debug_dir).expect("failed to create target/debug/unparse_failures");
        let debug_path = debug_dir.join(
            file.file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("unknown.ptx")),
        );
        fs::write(
            &debug_path,
            fs::read(&input_path).expect("failed to read regenerated PTX"),
        )
        .expect("failed to persist regenerated PTX for debugging");
        panic!(
            "ptxas failed for {}\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
            file.display(),
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
