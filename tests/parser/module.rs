use crate::util::{parse, parse_result};
use ptx_parser::r#type::module::{
    AddressSizeDirective, FileDirective, Module, ModuleDebugDirective, ModuleDirective,
    ModuleInfoDirectiveKind, TargetDirective, VersionDirective,
};

fn expect_version(directive: &ModuleDirective, major: u32, minor: u32) {
    match directive {
        ModuleDirective::ModuleInfo(ModuleInfoDirectiveKind::Version(VersionDirective {
            major: m,
            minor: n,
        })) => {
            assert_eq!((*m, *n), (major, minor));
        }
        other => panic!("expected version directive, got {other:?}"),
    }
}

fn expect_target(directive: &ModuleDirective, entries: &[&str]) {
    match directive {
        ModuleDirective::ModuleInfo(ModuleInfoDirectiveKind::Target(TargetDirective {
            entries: values,
            ..
        })) => {
            let expected: Vec<_> = entries.iter().map(|s| s.to_string()).collect();
            assert_eq!(values, &expected);
        }
        other => panic!("expected target directive, got {other:?}"),
    }
}

fn expect_address_size(directive: &ModuleDirective, size: u32) {
    match directive {
        ModuleDirective::ModuleInfo(ModuleInfoDirectiveKind::AddressSize(
            AddressSizeDirective { size: value },
        )) => {
            assert_eq!(*value, size);
        }
        other => panic!("expected address_size directive, got {other:?}"),
    }
}

fn expect_file(directive: &ModuleDirective, index: u32, path: &str) {
    match directive {
        ModuleDirective::Debug(ModuleDebugDirective::File(FileDirective {
            index: value,
            path: p,
        })) => {
            assert_eq!((*value, p.as_str()), (index, path));
        }
        other => panic!("expected file directive, got {other:?}"),
    }
}

#[test]
fn parses_basic_module() {
    let module = parse::<Module>(
        ".version 8.8\n\
         .target sm_80, texmode_independent\n\
         .address_size 64\n\
         .visible .global .align 4 .b8 data[4];\n\
         .file 1 \"dummy.ptx\"\n\
         .section .debug_info { }\n\
         .visible .entry kernel() { }\n",
    );
    assert_eq!(module.directives.len(), 7);
    expect_version(&module.directives[0], 8, 8);
    expect_target(&module.directives[1], &["sm_80", "texmode_independent"]);
    expect_address_size(&module.directives[2], 64);
    expect_file(&module.directives[4], 1, "dummy.ptx");

    let ModuleDirective::FunctionKernel(_) = module.directives[6] else {
        panic!("expected function kernel directive");
    };
}

#[test]
fn rejects_invalid_version_literal() {
    let err = parse_result::<Module>(".version eight.eight\n")
        .expect_err("parsing should fail for invalid version literal");
    assert!(matches!(
        err.kind,
        ptx_parser::parser::ParseErrorKind::UnexpectedToken { .. }
    ));
}

#[test]
fn rejects_target_without_arch() {
    let err =
        parse_result::<Module>(".target\n").expect_err("target must include at least one entry");
    assert!(matches!(
        err.kind,
        ptx_parser::parser::ParseErrorKind::UnexpectedToken { .. }
    ));
}
