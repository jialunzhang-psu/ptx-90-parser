// TreeDisplay implementations for module types (src/type/module.rs)

use crate::r#type::module::*;
use super::{TreeDisplay, TreeFormatter};

impl TreeDisplay for Module {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!("Module [{}]", f.format_raw(self.span, source)))?;
        f.field_vec(true, "directives", &self.directives, source)
    }
}

impl TreeDisplay for ModuleDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            ModuleDirective::ModuleVariable { linkage, directive, span } => {
                f.root(&format!("ModuleDirective::ModuleVariable [{}]", f.format_raw(*span, source)))?;
                f.field_option(false, "linkage", linkage, source)?;
                f.field_with_child(true, "directive", directive, source)
            }
            ModuleDirective::EntryFunction { linkage, directive, span } => {
                f.root(&format!("ModuleDirective::EntryFunction [{}]", f.format_raw(*span, source)))?;
                f.field_option(false, "linkage", linkage, source)?;
                f.field_with_child(true, "directive", directive, source)
            }
            ModuleDirective::FuncFunction { linkage, directive, span } => {
                f.root(&format!("ModuleDirective::FuncFunction [{}]", f.format_raw(*span, source)))?;
                f.field_option(false, "linkage", linkage, source)?;
                f.field_with_child(true, "directive", directive, source)
            }
            ModuleDirective::AliasFunction { directive, span } => {
                f.root(&format!("ModuleDirective::AliasFunction [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "directive", directive, source)
            }
            ModuleDirective::ModuleInfo { directive, span } => {
                f.root(&format!("ModuleDirective::ModuleInfo [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "directive", directive, source)
            }
            ModuleDirective::Debug { directive, span } => {
                f.root(&format!("ModuleDirective::Debug [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "directive", directive, source)
            }
        }
    }
}

impl TreeDisplay for ModuleInfoDirectiveKind {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            ModuleInfoDirectiveKind::Version { directive, span } => {
                f.root(&format!("ModuleInfoDirectiveKind::Version [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "directive", directive, source)
            }
            ModuleInfoDirectiveKind::Target { directive, span } => {
                f.root(&format!("ModuleInfoDirectiveKind::Target [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "directive", directive, source)
            }
            ModuleInfoDirectiveKind::AddressSize { directive, span } => {
                f.root(&format!("ModuleInfoDirectiveKind::AddressSize [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "directive", directive, source)
            }
        }
    }
}

impl TreeDisplay for VersionDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!("VersionDirective [{}]", f.format_raw(self.span, source)))?;
        f.field(false, "major", &self.major.to_string())?;
        f.field(true, "minor", &self.minor.to_string())
    }
}

impl TreeDisplay for TargetDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!("TargetDirective [{}]", f.format_raw(self.span, source)))?;
        f.field_vec(true, "entries", &self.entries, source)
    }
}

impl TreeDisplay for AddressSizeDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!("AddressSizeDirective [{}]", f.format_raw(self.span, source)))?;
        f.field_with_child(true, "size", &self.size, source)
    }
}

impl TreeDisplay for ModuleDebugDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            ModuleDebugDirective::File { directive, span } => {
                f.root(&format!("ModuleDebugDirective::File [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "directive", directive, source)
            }
            ModuleDebugDirective::Section { directive, span } => {
                f.root(&format!("ModuleDebugDirective::Section [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "directive", directive, source)
            }
            ModuleDebugDirective::Dwarf { directive, span } => {
                f.root(&format!("ModuleDebugDirective::Dwarf [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "directive", directive, source)
            }
        }
    }
}

impl TreeDisplay for FileDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!("FileDirective [{}]", f.format_raw(self.span, source)))?;
        f.field(false, "index", &self.index.to_string())?;
        f.field(false, "path", &format!("\"{}\"", self.path))?;
        match self.timestamp {
            Some(t) => f.field(false, "timestamp", &format!("Some({})", t))?,
            None => f.field(false, "timestamp", "None")?,
        }
        match self.file_size {
            Some(s) => f.field(true, "file_size", &format!("Some({})", s)),
            None => f.field(true, "file_size", "None"),
        }
    }
}

impl TreeDisplay for TargetString {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        let (variant, span) = match self {
            TargetString::Sm120a { span } => ("Sm120a", span),
            TargetString::Sm120f { span } => ("Sm120f", span),
            TargetString::Sm120 { span } => ("Sm120", span),
            TargetString::Sm121a { span } => ("Sm121a", span),
            TargetString::Sm121f { span } => ("Sm121f", span),
            TargetString::Sm121 { span } => ("Sm121", span),
            TargetString::Sm110a { span } => ("Sm110a", span),
            TargetString::Sm110f { span } => ("Sm110f", span),
            TargetString::Sm110 { span } => ("Sm110", span),
            TargetString::Sm100a { span } => ("Sm100a", span),
            TargetString::Sm100f { span } => ("Sm100f", span),
            TargetString::Sm100 { span } => ("Sm100", span),
            TargetString::Sm101a { span } => ("Sm101a", span),
            TargetString::Sm101f { span } => ("Sm101f", span),
            TargetString::Sm101 { span } => ("Sm101", span),
            TargetString::Sm103a { span } => ("Sm103a", span),
            TargetString::Sm103f { span } => ("Sm103f", span),
            TargetString::Sm103 { span } => ("Sm103", span),
            TargetString::Sm90a { span } => ("Sm90a", span),
            TargetString::Sm90 { span } => ("Sm90", span),
            TargetString::Sm80 { span } => ("Sm80", span),
            TargetString::Sm86 { span } => ("Sm86", span),
            TargetString::Sm87 { span } => ("Sm87", span),
            TargetString::Sm88 { span } => ("Sm88", span),
            TargetString::Sm89 { span } => ("Sm89", span),
            TargetString::Sm70 { span } => ("Sm70", span),
            TargetString::Sm72 { span } => ("Sm72", span),
            TargetString::Sm75 { span } => ("Sm75", span),
            TargetString::Sm60 { span } => ("Sm60", span),
            TargetString::Sm61 { span } => ("Sm61", span),
            TargetString::Sm62 { span } => ("Sm62", span),
            TargetString::Sm50 { span } => ("Sm50", span),
            TargetString::Sm52 { span } => ("Sm52", span),
            TargetString::Sm53 { span } => ("Sm53", span),
            TargetString::Sm30 { span } => ("Sm30", span),
            TargetString::Sm32 { span } => ("Sm32", span),
            TargetString::Sm35 { span } => ("Sm35", span),
            TargetString::Sm37 { span } => ("Sm37", span),
            TargetString::Sm20 { span } => ("Sm20", span),
            TargetString::Sm10 { span } => ("Sm10", span),
            TargetString::Sm11 { span } => ("Sm11", span),
            TargetString::Sm12 { span } => ("Sm12", span),
            TargetString::Sm13 { span } => ("Sm13", span),
            TargetString::TexmodeUnified { span } => ("TexmodeUnified", span),
            TargetString::TexmodeIndependent { span } => ("TexmodeIndependent", span),
            TargetString::Debug { span } => ("Debug", span),
            TargetString::MapF64ToF32 { span } => ("MapF64ToF32", span),
        };
        f.root(&format!("TargetString::{} [{}]", variant, f.format_raw(*span, source)))
    }
}

impl TreeDisplay for AddressSize {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        let (variant, span) = match self {
            AddressSize::Size32 { span } => ("Size32", span),
            AddressSize::Size64 { span } => ("Size64", span),
        };
        f.root(&format!("AddressSize::{} [{}]", variant, f.format_raw(*span, source)))
    }
}
