use super::common::*;
use super::function::{DwarfDirective, FunctionKernelDirective};
use super::variable::ModuleVariableDirective;

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
    ModuleInfo(ModuleInfoDirectiveKind),
    Debug(ModuleDebugDirective),
    Linking(LinkingDirective),
}

/// Directives that apply to the PTX module as a whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleInfoDirectiveKind {
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

/// Linking directives that influence symbol visibility. TODO: further parse the
/// prototype, which should be a function signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkingDirective {
    pub kind: CodeOrDataLinkage,
    pub prototype: String,
    pub raw: String,
}
