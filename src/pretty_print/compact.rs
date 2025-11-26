use crate::r#type::{
    AddressSize, Module, ModuleDirective, ModuleInfoDirectiveKind, ModuleVariableDirective,
    TargetString,
};

/// Print a compact summary of a PTX module, similar to nvdisasm headers.
pub fn print_compact_module(module: &Module) {
    for directive in &module.directives {
        match directive {
            ModuleDirective::ModuleInfo {
                directive: info, ..
            } => match info {
                ModuleInfoDirectiveKind::Version {
                    directive: version, ..
                } => {
                    println!(".version {}.{}", version.major, version.minor);
                }
                ModuleInfoDirectiveKind::Target {
                    directive: target, ..
                } => {
                    let values: Vec<&'static str> =
                        target.entries.iter().map(target_string_name).collect();
                    println!(".target {}", values.join(", "));
                }
                ModuleInfoDirectiveKind::AddressSize {
                    directive: addr, ..
                } => {
                    let value = match addr.size {
                        AddressSize::Size32 { .. } => 32,
                        AddressSize::Size64 { .. } => 64,
                    };
                    println!(".address_size {}", value);
                }
            },
            ModuleDirective::EntryFunction {
                linkage,
                directive: function,
                ..
            } => {
                if let Some(link) = linkage {
                    println!("{link:?}");
                }
                println!(".entry {}", function.name.val);
            }
            ModuleDirective::FuncFunction {
                linkage,
                directive: function,
                ..
            } => {
                if let Some(link) = linkage {
                    println!("{link:?}");
                }
                println!(".func {}", function.name.val);
            }
            ModuleDirective::AliasFunction {
                directive: alias, ..
            } => {
                println!(".alias {} {}", alias.alias.val, alias.target.val);
            }
            ModuleDirective::ModuleVariable {
                linkage,
                directive: var,
                ..
            } => {
                if let Some(link) = linkage {
                    println!("{link:?}");
                }
                print_module_variable(var);
            }
            other => println!("{other:?}"),
        }
    }
}

fn print_module_variable(var: &ModuleVariableDirective) {
    match var {
        ModuleVariableDirective::Global {
            directive: decl, ..
        } => {
            println!(".global {}", describe_variable_decl(decl))
        }
        ModuleVariableDirective::Shared {
            directive: decl, ..
        } => {
            println!(".shared {}", describe_variable_decl(decl))
        }
        ModuleVariableDirective::Const {
            directive: decl, ..
        } => {
            println!(".const {}", describe_variable_decl(decl))
        }
        ModuleVariableDirective::Tex {
            directive: decl, ..
        } => {
            println!(".tex {}", describe_variable_decl(decl))
        }
    }
}

fn describe_variable_decl(decl: &crate::r#type::VariableDirective) -> String {
    let ty = format!("{:?}", decl.ty);
    format!("{} {}", ty, decl.name.val)
}

fn target_string_name(entry: &TargetString) -> &'static str {
    use TargetString::*;
    match entry {
        Sm120a { .. } => "sm_120a",
        Sm120f { .. } => "sm_120f",
        Sm120 { .. } => "sm_120",
        Sm121a { .. } => "sm_121a",
        Sm121f { .. } => "sm_121f",
        Sm121 { .. } => "sm_121",
        Sm110a { .. } => "sm_110a",
        Sm110f { .. } => "sm_110f",
        Sm110 { .. } => "sm_110",
        Sm100a { .. } => "sm_100a",
        Sm100f { .. } => "sm_100f",
        Sm100 { .. } => "sm_100",
        Sm101a { .. } => "sm_101a",
        Sm101f { .. } => "sm_101f",
        Sm101 { .. } => "sm_101",
        Sm103a { .. } => "sm_103a",
        Sm103f { .. } => "sm_103f",
        Sm103 { .. } => "sm_103",
        Sm90a { .. } => "sm_90a",
        Sm90 { .. } => "sm_90",
        Sm80 { .. } => "sm_80",
        Sm86 { .. } => "sm_86",
        Sm87 { .. } => "sm_87",
        Sm88 { .. } => "sm_88",
        Sm89 { .. } => "sm_89",
        Sm70 { .. } => "sm_70",
        Sm72 { .. } => "sm_72",
        Sm75 { .. } => "sm_75",
        Sm60 { .. } => "sm_60",
        Sm61 { .. } => "sm_61",
        Sm62 { .. } => "sm_62",
        Sm50 { .. } => "sm_50",
        Sm52 { .. } => "sm_52",
        Sm53 { .. } => "sm_53",
        Sm30 { .. } => "sm_30",
        Sm32 { .. } => "sm_32",
        Sm35 { .. } => "sm_35",
        Sm37 { .. } => "sm_37",
        Sm20 { .. } => "sm_20",
        Sm10 { .. } => "sm_10",
        Sm11 { .. } => "sm_11",
        Sm12 { .. } => "sm_12",
        Sm13 { .. } => "sm_13",
        TexmodeUnified { .. } => "texmode_unified",
        TexmodeIndependent { .. } => "texmode_independent",
        Debug { .. } => "debug",
        MapF64ToF32 { .. } => "map_f64_to_f32",
    }
}
