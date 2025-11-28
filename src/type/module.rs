use super::common::*;
use super::function::{DwarfDirective, SectionDirective};
use super::variable::ModuleVariableDirective;
use crate::Spanned;
use crate::parser::Span;
use crate::r#type::{AliasFunctionDirective, EntryFunctionDirective, FuncFunctionDirective};
use serde::Serialize;

/// A full PTX module containing directives and function definitions.
#[derive(Debug, Clone, PartialEq, Spanned, Default, Serialize)]
pub struct Module {
    pub directives: Vec<ModuleDirective>,
    pub span: Span,
}

/// Module-level directives recognised by the parser.
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub enum ModuleDirective {
    ModuleVariable {
        linkage: Option<DataLinkage>,
        directive: ModuleVariableDirective,
        span: Span,
    },
    EntryFunction {
        linkage: Option<CodeLinkage>,
        directive: EntryFunctionDirective,
        span: Span,
    },
    FuncFunction {
        linkage: Option<CodeLinkage>,
        directive: FuncFunctionDirective,
        span: Span,
    },
    AliasFunction {
        directive: AliasFunctionDirective,
        span: Span,
    },
    ModuleInfo {
        directive: ModuleInfoDirectiveKind,
        span: Span,
    },
    Debug {
        directive: ModuleDebugDirective,
        span: Span,
    },
}

/// Directives that apply to the PTX module as a whole.
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub enum ModuleInfoDirectiveKind {
    Version {
        directive: VersionDirective,
        span: Span,
    },
    Target {
        directive: TargetDirective,
        span: Span,
    },
    AddressSize {
        directive: AddressSizeDirective,
        span: Span,
    },
}

/// Structured representation of the `.version` directive.
///
/// Syntax:
/// .version  major.minor    // major, minor are integers
///
/// Example:
/// .version 3.1
/// .version 3.0
/// .version 2.3
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct VersionDirective {
    pub major: u32,
    pub minor: u32,
    pub span: Span,
}

/// Structured representation of the `.target` directive.
///
/// Syntax:
/// .target stringlist         // comma separated list of target specifiers
/// string = { sm_120a, sm_120f, sm_120,          // sm_12x target architectures
/// sm_121a, sm_121f, sm_121,          // sm_12x target architectures
/// sm_110a, sm_110f, sm_110,          // sm_11x target architectures
/// sm_100a, sm_100f, sm_100,          // sm_10x target architectures
/// sm_101a, sm_101f, sm_101,          // sm_10x target architectures
/// sm_103a, sm_103f, sm_103           // sm_10x target architectures
/// sm_90a, sm_90,                     // sm_9x target architectures
/// sm_80, sm_86, sm_87, sm_88, sm_89, // sm_8x target architectures
/// sm_70, sm_72, sm_75,               // sm_7x target architectures
/// sm_60, sm_61, sm_62,               // sm_6x target architectures
/// sm_50, sm_52, sm_53,               // sm_5x target architectures
/// sm_30, sm_32, sm_35, sm_37,        // sm_3x target architectures
/// sm_20,                             // sm_2x target architectures
/// sm_10, sm_11, sm_12, sm_13,        // sm_1x target architectures
/// texmode_unified, texmode_independent,   // texturing mode
/// debug,                                  // platform option
/// map_f64_to_f32 };                       // platform option
///
/// Example:
/// .target sm_10       // baseline target architecture
/// .target sm_13       // supports double-precision
/// .target sm_20, texmode_independent
/// .target sm_90       // baseline target architecture
/// .target sm_90a      // PTX using architecture-specific features
/// .target sm_100f     // PTX using family-specific features
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct TargetDirective {
    pub entries: Vec<TargetString>,
    pub span: Span,
}

/// Structured representation of the `.address_size` directive.
///
/// Syntax:
/// .address_size  address-size
/// address-size = { 32, 64 };
///
/// Example:
/// .address_size 64
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct AddressSizeDirective {
    pub size: AddressSize,
    pub span: Span,
}

/// Debugging directives defined by the PTX ISA.
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub enum ModuleDebugDirective {
    File {
        directive: FileDirective,
        span: Span,
    },
    Section {
        directive: SectionDirective,
        span: Span,
    },
    Dwarf {
        directive: DwarfDirective,
        span: Span,
    },
}

/// Structured representation of the `.file` directive.
///
/// Syntax:
/// .file file_index "filename" {, timestamp, file_size}
///
/// Example:
/// .file 1 "example.cu"
/// .file 2 "kernel.cu"
/// .file 1 "kernel.cu", 1339013327, 64118
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct FileDirective {
    pub index: u32,
    pub path: String,
    pub timestamp: Option<u64>,
    pub file_size: Option<u64>,
    pub span: Span,
}

/// Target specifiers used in the `.target` directive.
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub enum TargetString {
    Sm120a { span: Span },
    Sm120f { span: Span },
    Sm120 { span: Span },
    Sm121a { span: Span },
    Sm121f { span: Span },
    Sm121 { span: Span },
    Sm110a { span: Span },
    Sm110f { span: Span },
    Sm110 { span: Span },
    Sm100a { span: Span },
    Sm100f { span: Span },
    Sm100 { span: Span },
    Sm101a { span: Span },
    Sm101f { span: Span },
    Sm101 { span: Span },
    Sm103a { span: Span },
    Sm103f { span: Span },
    Sm103 { span: Span },
    Sm90a { span: Span },
    Sm90 { span: Span },
    Sm80 { span: Span },
    Sm86 { span: Span },
    Sm87 { span: Span },
    Sm88 { span: Span },
    Sm89 { span: Span },
    Sm70 { span: Span },
    Sm72 { span: Span },
    Sm75 { span: Span },
    Sm60 { span: Span },
    Sm61 { span: Span },
    Sm62 { span: Span },
    Sm50 { span: Span },
    Sm52 { span: Span },
    Sm53 { span: Span },
    Sm30 { span: Span },
    Sm32 { span: Span },
    Sm35 { span: Span },
    Sm37 { span: Span },
    Sm20 { span: Span },
    Sm10 { span: Span },
    Sm11 { span: Span },
    Sm12 { span: Span },
    Sm13 { span: Span },
    TexmodeUnified { span: Span },
    TexmodeIndependent { span: Span },
    Debug { span: Span },
    MapF64ToF32 { span: Span },
}

/// Address size options for the `.address_size` directive.
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub enum AddressSize {
    Size32 { span: Span },
    Size64 { span: Span },
}
