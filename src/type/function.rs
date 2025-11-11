use super::common::{CodeLinkage, Instruction};
use super::variable::VariableDirective;
use crate::parser::Span;

/// All directives that describe kernel/function entities.
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionKernelDirective {
    Entry { function: EntryFunction, span: Span },
    Func { function: FuncFunction, span: Span },
    Alias { alias: FunctionAlias, span: Span },
}

impl FunctionKernelDirective {
    pub fn span(&self) -> Span {
        match self {
            FunctionKernelDirective::Entry { span, .. } => span.clone(),
            FunctionKernelDirective::Func { span, .. } => span.clone(),
            FunctionKernelDirective::Alias { span, .. } => span.clone(),
        }
    }
}

/// Alias directive relating one function symbol to another.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionAlias {
    pub alias: String,
    pub target: String,
    pub span: Span,
}

impl FunctionAlias {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// A PTX kernel declared with the `.entry` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct EntryFunction {
    pub name: String,
    pub directives: Vec<FunctionHeaderDirective>,
    pub params: Vec<VariableDirective>,
    pub body: FunctionBody,
    pub span: Span,
}

impl EntryFunction {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// A PTX device function declared with the `.func` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct FuncFunction {
    pub name: String,
    pub directives: Vec<FunctionHeaderDirective>,
    pub return_param: Option<VariableDirective>,
    pub params: Vec<VariableDirective>,
    pub body: FunctionBody,
    pub span: Span,
}

impl FuncFunction {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Statements contained within a PTX function body.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    pub statements: Vec<FunctionStatement>,
    pub span: Span,
}

impl FunctionBody {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

impl Default for FunctionBody {
    fn default() -> Self {
        Self {
            statements: Vec::new(),
            span: 0..0,
        }
    }
}

/// Directive tokens that may decorate a PTX function header.
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionHeaderDirective {
    Linkage { linkage: CodeLinkage, span: Span },
    NoReturn { span: Span },
    AbiPreserve { value: u32, span: Span },
    AbiPreserveControl { value: u32, span: Span },
    MaxClusterRank { value: u32, span: Span },
    BlocksAreClusters { span: Span },
    ExplicitCluster { dim: FunctionDim3, span: Span },
    ReqNctaPerCluster { dim: FunctionDim3, span: Span },
    MaxNReg { value: u32, span: Span },
    MaxNTid { dim: FunctionDim3, span: Span },
    MinNCtaPerSm { value: u32, span: Span },
    ReqNTid { dim: FunctionDim3, span: Span },
    MaxNCtaPerSm { value: u32, span: Span },
    Pragma { args: Vec<String>, span: Span },
}

impl FunctionHeaderDirective {
    pub fn span(&self) -> Span {
        match self {
            FunctionHeaderDirective::Linkage { span, .. } => span.clone(),
            FunctionHeaderDirective::NoReturn { span } => span.clone(),
            FunctionHeaderDirective::AbiPreserve { span, .. } => span.clone(),
            FunctionHeaderDirective::AbiPreserveControl { span, .. } => span.clone(),
            FunctionHeaderDirective::MaxClusterRank { span, .. } => span.clone(),
            FunctionHeaderDirective::BlocksAreClusters { span } => span.clone(),
            FunctionHeaderDirective::ExplicitCluster { span, .. } => span.clone(),
            FunctionHeaderDirective::ReqNctaPerCluster { span, .. } => span.clone(),
            FunctionHeaderDirective::MaxNReg { span, .. } => span.clone(),
            FunctionHeaderDirective::MaxNTid { span, .. } => span.clone(),
            FunctionHeaderDirective::MinNCtaPerSm { span, .. } => span.clone(),
            FunctionHeaderDirective::ReqNTid { span, .. } => span.clone(),
            FunctionHeaderDirective::MaxNCtaPerSm { span, .. } => span.clone(),
            FunctionHeaderDirective::Pragma { span, .. } => span.clone(),
        }
    }
}

/// Dimension triplet used by several function header directives.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDim3 {
    pub x: u32,
    pub y: Option<u32>,
    pub z: Option<u32>,
    pub span: Span,
}

impl FunctionDim3 {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Nested statement block enclosed in braces.
/// Executable items that appear within a function body.
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionStatement {
    Label { name: String, span: Span },
    Directive { directive: StatementDirective, span: Span },
    Instruction { instruction: Instruction, span: Span },
    Block { statements: Vec<FunctionStatement>, span: Span },
}

impl FunctionStatement {
    pub fn span(&self) -> Span {
        match self {
            FunctionStatement::Label { span, .. } => span.clone(),
            FunctionStatement::Directive { span, .. } => span.clone(),
            FunctionStatement::Instruction { span, .. } => span.clone(),
            FunctionStatement::Block { span, .. } => span.clone(),
        }
    }
}

/// .reg .ty name<range>
#[derive(Debug, Clone, PartialEq)]
pub struct RegisterDirective {
    pub name: String,
    pub ty: Option<String>,
    pub range: Option<u32>,
    pub comment: Option<String>,
    pub span: Span,
}

impl RegisterDirective {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Directive that applies to individual statements.
#[derive(Debug, Clone, PartialEq)]
pub enum StatementDirective {
    Loc { directive: LocationDirective, span: Span },
    Pragma { directive: PragmaDirective, span: Span },
    Section { directive: StatementSectionDirective, span: Span },
    Reg { directive: RegisterDirective, span: Span },
    Local { directive: VariableDirective, span: Span },
    Param { directive: VariableDirective, span: Span },
    Shared { directive: VariableDirective, span: Span },
    Dwarf { directive: DwarfDirective, span: Span },
}

impl StatementDirective {
    pub fn span(&self) -> Span {
        match self {
            StatementDirective::Loc { span, .. } => span.clone(),
            StatementDirective::Pragma { span, .. } => span.clone(),
            StatementDirective::Section { span, .. } => span.clone(),
            StatementDirective::Reg { span, .. } => span.clone(),
            StatementDirective::Local { span, .. } => span.clone(),
            StatementDirective::Param { span, .. } => span.clone(),
            StatementDirective::Shared { span, .. } => span.clone(),
            StatementDirective::Dwarf { span, .. } => span.clone(),
        }
    }
}

/// Raw dwarf directive emitted by the compiler (e.g. @@dwarf).
#[derive(Debug, Clone, PartialEq)]
pub struct DwarfDirective {
    pub keyword: String,
    pub arguments: Vec<String>,
    pub comment: Option<String>,
    pub span: Span,
}

impl DwarfDirective {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Structured representation of a `.loc` directive inside a PTX function.
#[derive(Debug, Clone, PartialEq)]
pub struct LocationDirective {
    pub file_index: u32,
    pub line: u32,
    pub column: u32,
    pub options: Vec<String>,
    pub comment: Option<String>,
    pub span: Span,
}

impl LocationDirective {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Structured representation of a `.pragma` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct PragmaDirective {
    pub arguments: Vec<String>,
    pub comment: Option<String>,
    pub span: Span,
}

impl PragmaDirective {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Structured representation of a `.section` directive inside a function body.
#[derive(Debug, Clone, PartialEq)]
pub struct StatementSectionDirective {
    pub name: String,
    pub arguments: Vec<String>,
    pub comment: Option<String>,
    pub span: Span,
}

impl StatementSectionDirective {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}
