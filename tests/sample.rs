mod util;

#[test]
fn parses_sample_ptx_files() {
    let mut paths: Vec<_> = std::fs::read_dir("tests/sample")
        .expect("sample directory should exist")
        .map(|entry| entry.expect("read_dir entry").path())
        .collect();
    paths.sort();

    for path in paths {
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("failed to read {}", path.display()));
        let module = ptx_parser::parser::parse_ptx(&source)
            .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()));
        assert!(
            !module.directives.is_empty(),
            "parsed module should contain directives for {}",
            path.display()
        );

        let mut has_version = false;
        let mut has_target = false;
        let mut has_address_size = false;
        let mut has_function = false;

        for directive in &module.directives {
            use ptx_parser::r#type::module::{ModuleDirective, ModuleInfoDirectiveKind};
            match directive {
                ModuleDirective::ModuleInfo(ModuleInfoDirectiveKind::Version(_)) => {
                    has_version = true
                }
                ModuleDirective::ModuleInfo(ModuleInfoDirectiveKind::Target(_)) => {
                    has_target = true
                }
                ModuleDirective::ModuleInfo(ModuleInfoDirectiveKind::AddressSize(_)) => {
                    has_address_size = true
                }
                ModuleDirective::FunctionKernel(_) => has_function = true,
                _ => {}
            }
        }

        assert!(
            has_version && has_target && has_address_size,
            "module {} should declare version/target/address_size directives",
            path.display()
        );
        assert!(
            has_function,
            "module {} should contain at least one function/kernel directive",
            path.display()
        );
    }
}
