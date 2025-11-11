use super::common::*;
use super::function::{DwarfDirective, FunctionKernelDirective};
use super::variable::ModuleVariableDirective;
use crate::parser::Span;

/// A full PTX module containing directives and function definitions.
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub directives: Vec<ModuleDirective>,
    pub span: Span,
}

impl Module {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

impl Default for Module {
    fn default() -> Self {
        Self {
            directives: Vec::new(),
            span: 0..0,
        }
    }
}

/// Module-level directives recognised by the parser.
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleDirective {
    ModuleVariable { directive: ModuleVariableDirective, span: Span },
    FunctionKernel { directive: FunctionKernelDirective, span: Span },
    ModuleInfo { directive: ModuleInfoDirectiveKind, span: Span },
    Debug { directive: ModuleDebugDirective, span: Span },
    Linking { directive: LinkingDirective, span: Span },
}

impl ModuleDirective {
    pub fn span(&self) -> Span {
        match self {
            ModuleDirective::ModuleVariable { span, .. } => span.clone(),
            ModuleDirective::FunctionKernel { span, .. } => span.clone(),
            ModuleDirective::ModuleInfo { span, .. } => span.clone(),
            ModuleDirective::Debug { span, .. } => span.clone(),
            ModuleDirective::Linking { span, .. } => span.clone(),
        }
    }
}

/// Directives that apply to the PTX module as a whole.
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleInfoDirectiveKind {
    Version { directive: VersionDirective, span: Span },
    Target { directive: TargetDirective, span: Span },
    AddressSize { directive: AddressSizeDirective, span: Span },
}

impl ModuleInfoDirectiveKind {
    pub fn span(&self) -> Span {
        match self {
            ModuleInfoDirectiveKind::Version { span, .. } => span.clone(),
            ModuleInfoDirectiveKind::Target { span, .. } => span.clone(),
            ModuleInfoDirectiveKind::AddressSize { span, .. } => span.clone(),
        }
    }
}

/// Structured representation of the `.version` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct VersionDirective {
    pub major: u32,
    pub minor: u32,
    pub span: Span,
}

impl VersionDirective {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Structured representation of the `.target` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct TargetDirective {
    pub entries: Vec<String>,
    pub span: Span,
}

impl TargetDirective {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Structured representation of the `.address_size` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct AddressSizeDirective {
    pub size: u32,
    pub span: Span,
}

impl AddressSizeDirective {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Debugging directives defined by the PTX ISA.
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleDebugDirective {
    File { directive: FileDirective, span: Span },
    Section { directive: SectionDirective, span: Span },
    Dwarf { directive: DwarfDirective, span: Span },
}

impl ModuleDebugDirective {
    pub fn span(&self) -> Span {
        match self {
            ModuleDebugDirective::File { span, .. } => span.clone(),
            ModuleDebugDirective::Section { span, .. } => span.clone(),
            ModuleDebugDirective::Dwarf { span, .. } => span.clone(),
        }
    }
}

/// Structured representation of the `.file` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct FileDirective {
    pub index: u32,
    pub path: String,
    pub span: Span,
}

impl FileDirective {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Structured representation of the `.section` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct SectionDirective {
    pub name: String,
    pub attributes: Vec<String>,
    pub span: Span,
}

impl SectionDirective {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Linking directives that influence symbol visibility. TODO: further parse the
/// prototype, which should be a function signature.
#[derive(Debug, Clone, PartialEq)]
pub struct LinkingDirective {
    pub kind: CodeOrDataLinkage,
    pub prototype: String,
    pub span: Span,
}

impl LinkingDirective {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}
