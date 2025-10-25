mod functions;
mod instructions;
mod variables;
mod impls;

pub use functions::*;
pub use instructions::*;
pub use variables::*;

use thiserror::Error;

/// A full PTX module containing directives and function definitions.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Module {
    pub directives: Vec<ModuleDirective>,
}

/// Module-level directives recognised by the parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDirective {
    ModuleVariable(ModuleVariableDirective),
    FunctionKernel(FunctionKernelDirective),
    Module(ModuleDirectiveKind),
    Debug(ModuleDebugDirective),
    Linking(LinkingDirective),
}

/// Directives that apply to the PTX module as a whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDirectiveKind {
    Version(VersionDirective),
    Target(TargetDirective),
    AddressSize(AddressSizeDirective),
}

/// Structured representation of the `.version` directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionDirective {
    pub major: u32,
    pub minor: u32,
}

/// Structured representation of the `.target` directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetDirective {
    pub entries: Vec<String>,
    pub raw: String,
}

/// Structured representation of the `.address_size` directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressSizeDirective {
    pub size: u32,
}

/// Debugging directives defined by the PTX ISA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDebugDirective {
    File(FileDirective),
    Section(SectionDirective),
    Dwarf(DwarfDirective),
}

/// Linking directives that influence symbol visibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkingDirective {
    pub kind: LinkingDirectiveKind,
    pub prototype: String,
    pub raw: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkingDirectiveKind {
    Extern,
    Visible,
    Weak,
    Common,
}

/// Cluster dimension directives used for cooperative kernels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClusterDirective {
    RequireCtasPerCluster(ClusterSizeDirective),
    ExplicitCluster(ClusterSizeDirective),
    MaxClusterRank { count: u32, raw: String },
}

/// Miscellaneous directives defined by the PTX ISA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiscDirective {
    BlocksAreClusters { raw: String },
}

/// Raw representation of the `.b8/.b16/...` module-level data directives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleDataDirective {
    pub kind: DataDirectiveKind,
    pub values: Vec<String>,
    pub raw: String,
}

/// Structured representation of the `.file` directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileDirective {
    pub index: u32,
    pub path: String,
}

/// Structured representation of the `.section` directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionDirective {
    pub name: String,
    pub attributes: Vec<String>,
}

/// Shared representation for cluster dimension directives that specify extents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClusterSizeDirective {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub raw: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataDirectiveKind {
    B8,
    B16,
    B32,
    B64,
}


/// Errors that can occur while parsing PTX source text.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum PtxParseError {
    #[error("unexpected end of input while parsing {context} starting at line {line}")]
    UnexpectedEof { context: &'static str, line: usize },

    #[error("invalid directive at line {line}: {message}")]
    InvalidDirective { line: usize, message: String },

    #[error("invalid function header at line {line}: {message}")]
    InvalidFunctionHeader { line: usize, message: String },

    #[error("invalid instruction at line {line}: {message}")]
    InvalidInstruction { line: usize, message: String },

    #[error("invalid global declaration at line {line}: {message}")]
    InvalidGlobal { line: usize, message: String },
}
