use ptx_parser::{parse, ModuleDirective, ModuleDirectiveKind, ModuleVariableDirective};

#[test]
fn parses_module_headers() {
    let source = ".version 7.1\n.target sm_80\n.address_size 64\n";
    let module = parse(source).expect("module headers should parse");

    assert!(matches!(
        module.directives.as_slice(),
        [
            ModuleDirective::Module(ModuleDirectiveKind::Version(version)),
            ModuleDirective::Module(ModuleDirectiveKind::Target(_)),
            ModuleDirective::Module(ModuleDirectiveKind::AddressSize(size))
        ] if version.major == 7 && version.minor == 1 && size.size == 64
    ));
}

#[test]
fn parses_multiple_module_variables() {
    let source = ".global .u32 g0;\n.shared .align 16 .u8 s0[4];\n";
    let module = parse(source).expect("module variables should parse");

    assert!(module.directives.iter().any(|directive| matches!(
        directive,
        ModuleDirective::ModuleVariable(ModuleVariableDirective::Global(var))
            if var.name == "g0"
    )));
    assert!(module.directives.iter().any(|directive| matches!(
        directive,
        ModuleDirective::ModuleVariable(ModuleVariableDirective::Shared(var))
            if var.name == "s0"
    )));
}

#[test]
fn rejects_empty_module() {
    let module = parse("").expect("empty module should parse");
    assert!(module.directives.is_empty());
}
