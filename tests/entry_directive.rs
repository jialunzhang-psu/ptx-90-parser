use ptx_parser::{
    FunctionDeclarationKind, FunctionEntryDirective, RegisterSpecifier, parse_entry_directive,
};

#[test]
fn parses_register_entry_directive() {
    for (source, expected) in [
        (".reg .s32 i;", vec!["i"]),
        (".reg .u32 %ptr, %n;", vec!["%ptr", "%n"]),
        (".reg    .pred p, q, r;", vec!["p", "q", "r"]),
    ] {
        let stmt = parse_entry_directive(source).expect(".reg directive should parse");
        match stmt {
            FunctionEntryDirective::Reg(reg) => {
                assert_eq!(reg.registers.len(), expected.len());
                for (actual, expected_name) in reg.registers.iter().zip(expected.iter()) {
                    assert!(
                        matches!(actual, RegisterSpecifier::Named(name) if name == expected_name)
                    );
                }
            }
            other => panic!("expected register declaration, got {:?}", other),
        }
    }
}

#[test]
fn parses_local_entry_directive() {
    let stmt = parse_entry_directive(".local .align 8 .b8 scratch[16];")
        .expect(".local directive should parse");
    let generic = match stmt {
        FunctionEntryDirective::Local(generic) => generic,
        other => panic!("expected local declaration, got {:?}", other),
    };
    assert_eq!(generic.kind, FunctionDeclarationKind::Local);
    assert!(generic.keyword.starts_with(".local"));
    assert!(generic.raw.contains("scratch"));
}

#[test]
fn parses_param_entry_directive() {
    let stmt = parse_entry_directive(".param .b32 param0;").expect(".param directive should parse");
    let generic = match stmt {
        FunctionEntryDirective::Param(generic) => generic,
        other => panic!("expected param declaration, got {:?}", other),
    };
    assert_eq!(generic.kind, FunctionDeclarationKind::Param);
    assert!(generic.raw.contains("param0"));
}

#[test]
fn parses_shared_entry_directive() {
    let stmt =
        parse_entry_directive(".shared .align 16 .b8 sdata[256];").expect(".shared should parse");
    let generic = match stmt {
        FunctionEntryDirective::Shared(generic) => generic,
        other => panic!("expected shared declaration, got {:?}", other),
    };
    assert_eq!(generic.kind, FunctionDeclarationKind::Shared);
    assert!(generic.raw.contains("sdata"));
}

#[test]
fn parses_loc_entry_directive() {
    let stmt = parse_entry_directive(".loc 1 2 3").expect(".loc directive should parse");
    match stmt {
        FunctionEntryDirective::Loc(loc) => {
            assert_eq!(loc.file_index, 1);
            assert_eq!(loc.line, 2);
            assert_eq!(loc.column, 3);
        }
        other => panic!("expected loc directive, got {:?}", other),
    }
}

#[test]
fn parses_pragma_entry_directive() {
    let stmt = parse_entry_directive(".pragma \"align\"").expect(".pragma directive should parse");
    match stmt {
        FunctionEntryDirective::Pragma(pragma) => {
            assert_eq!(pragma.arguments.len(), 1);
        }
        other => panic!("expected pragma directive, got {:?}", other),
    }
}

#[test]
fn parses_dwarf_entry_directive() {
    let stmt = parse_entry_directive("@@dwarf 1 2 3").expect("@@dwarf directive should parse");
    match stmt {
        FunctionEntryDirective::Dwarf(dwarf) => {
            assert_eq!(dwarf.arguments, vec!["1", "2", "3"]);
        }
        other => panic!("expected dwarf directive, got {:?}", other),
    }
}

#[test]
fn rejects_unsupported_entry_directive() {
    assert!(parse_entry_directive(".maxnreg 32;").is_err());
}
