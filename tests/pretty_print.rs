mod util;

use ptx_parser::{parse_ptx, pretty_print::{TreeDisplay, TreeFormatter}};

fn tree_display_with_large_stack(source: &str) -> String {
    let source_owned = source.to_string();

    ptx_parser::run_with_large_stack(move || {
        let module = parse_ptx(&source_owned).expect("Failed to parse PTX");
        let mut formatter = TreeFormatter::new();
        module.tree_display(&mut formatter, &source_owned).expect("Failed to display tree");
        formatter.finish()
    })
}

#[test]
fn test_pretty_print_simple_module() {
    let source = r#"
.version 7.0
.target sm_90
.address_size 64

.entry kernel() {
    ret;
}
"#;

    let output = tree_display_with_large_stack(source);

    // Check that output contains expected elements
    assert!(output.contains("Module"));
    assert!(output.contains("ModuleDirective"));
    assert!(output.contains("VersionDirective"));
    assert!(output.contains("TargetDirective"));
    assert!(output.contains("AddressSizeDirective"));
    assert!(output.contains("EntryFunctionDirective"));

    println!("Pretty-printed PTX module:\n{}", output);
}

#[test]
fn test_pretty_print_with_instructions() {
    let source = r#"
.version 7.0
.target sm_90
.address_size 64

.entry add_kernel(.param .u32 a, .param .u32 b) {
    .reg .u32 %r1, %r2, %r3;
    ld.param.u32 %r1, [a];
    ld.param.u32 %r2, [b];
    add.u32 %r3, %r1, %r2;
    ret;
}
"#;

    let output = tree_display_with_large_stack(source);

    // Verify structure
    assert!(output.contains("params: Vec"));
    assert!(output.contains("FunctionBody"));
    assert!(output.contains("FunctionStatement"));
    assert!(output.contains("Instruction"));

    println!("Pretty-printed PTX with instructions:\n{}", output);
}
