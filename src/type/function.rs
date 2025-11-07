use super::common::{CodeLinkage, Instruction};
use super::variable::VariableDirective;

/// All directives that describe kernel/function entities.
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionKernelDirective {
    Entry(EntryFunction),
    Func(FuncFunction),
    Alias(FunctionAlias),
}

/// Alias directive relating one function symbol to another.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionAlias {
    pub alias: String,
    pub target: String,
    pub raw: String,
}

/// A PTX kernel declared with the `.entry` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct EntryFunction {
    pub name: String,
    pub directives: Vec<FunctionHeaderDirective>,
    pub params: Vec<VariableDirective>,
    pub body: FunctionBody,
}

/// A PTX device function declared with the `.func` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct FuncFunction {
    pub name: String,
    pub directives: Vec<FunctionHeaderDirective>,
    pub return_param: Option<VariableDirective>,
    pub params: Vec<VariableDirective>,
    pub body: FunctionBody,
}

/// Statements contained within a PTX function body.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FunctionBody {
    pub entry_directives: Vec<FunctionEntryDirective>,
    pub statements: Vec<FunctionStatement>,
}

/// Directive tokens that may decorate a PTX function header.
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionHeaderDirective {
    Linkage(CodeLinkage),
    NoReturn,
    AbiPreserve(u32),
    AbiPreserveControl(u32),
    MaxClusterRank(u32),
    BlocksAreClusters,
    ExplicitCluster(FunctionDim3),
    ReqNctaPerCluster(FunctionDim3),
    MaxNReg(u32),
    MaxNTid(FunctionDim3),
    MinNCtaPerSm(u32),
    ReqNTid(FunctionDim3),
    MaxNCtaPerSm(u32),
    Pragma(Vec<String>),
}

/// Dimension triplet used by several function header directives.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDim3 {
    pub x: u32,
    pub y: Option<u32>,
    pub z: Option<u32>,
}

/// Entry directives that appear before executable statements in a function body.
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionEntryDirective {
    Reg(RegisterDirective),
    Local(VariableDirective),
    Param(VariableDirective),
    Shared(VariableDirective),
    Pragma(PragmaDirective),
    Loc(LocationDirective),
    Dwarf(DwarfDirective),
}

/// Nested statement block enclosed in braces.
/// Executable items that appear within a function body.
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionStatement {
    Directive(StatementDirective),
    Instruction(Instruction),
    ExternCallBlock(ExternCallBlock),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternCallBlock {
    pub declarations: Vec<FunctionEntryDirective>,
    pub setup: Vec<ExternCallSetup>,
    pub call: Instruction,
    pub post_call: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExternCallSetup {
    Param(VariableDirective),
    Store(Instruction),
}

/// .reg .ty name<range>
#[derive(Debug, Clone, PartialEq)]
pub struct RegisterDirective {
    pub name: String,
    pub ty: Option<String>,
    pub range: Option<u32>,
    pub comment: Option<String>,
    pub raw: String,
}

/// Directive that applies to individual statements.
#[derive(Debug, Clone, PartialEq)]
pub enum StatementDirective {
    Dwarf(DwarfDirective),
    Loc(LocationDirective),
    Pragma(PragmaDirective),
    Section(StatementSectionDirective),
}

/// Raw dwarf directive emitted by the compiler (e.g. @@dwarf).
#[derive(Debug, Clone, PartialEq)]
pub struct DwarfDirective {
    pub keyword: String,
    pub arguments: Vec<String>,
    pub comment: Option<String>,
    pub raw: String,
}

/// Structured representation of a `.loc` directive inside a PTX function.
#[derive(Debug, Clone, PartialEq)]
pub struct LocationDirective {
    pub file_index: u32,
    pub line: u32,
    pub column: u32,
    pub options: Vec<String>,
    pub comment: Option<String>,
    pub raw: String,
}

/// Structured representation of a `.pragma` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct PragmaDirective {
    pub arguments: Vec<String>,
    pub comment: Option<String>,
    pub raw: String,
}

/// Structured representation of a `.section` directive inside a function body.
#[derive(Debug, Clone, PartialEq)]
pub struct StatementSectionDirective {
    pub name: String,
    pub arguments: Vec<String>,
    pub comment: Option<String>,
    pub raw: String,
}
