use super::instructions::Instruction;
use super::variables::{ArraySpecifier, ScalarType, VariableDirective};

/// All directives that describe kernel/function entities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionKernelDirective {
    Entry(EntryFunction),
    Func(FuncFunction),
    Alias(FunctionAlias),
}

/// Alias directive relating one function symbol to another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionAlias {
    pub alias: String,
    pub target: String,
    pub raw: String,
}

/// Statements contained within a PTX function body.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FunctionBody {
    pub entry_directives: Vec<FunctionEntryDirective>,
    pub statements: Vec<FunctionStatement>,
}

/// A PTX kernel declared with the `.entry` directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryFunction {
    pub name: String,
    pub directives: Vec<FunctionHeaderDirective>,
    pub params: Vec<Parameter>,
    pub body: FunctionBody,
}

/// A PTX device function declared with the `.func` directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncFunction {
    pub name: String,
    pub directives: Vec<FunctionHeaderDirective>,
    pub return_param: Option<Parameter>,
    pub params: Vec<Parameter>,
    pub body: FunctionBody,
}

/// Directive tokens that may decorate a PTX function header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionHeaderDirective {
    Visibility(FunctionVisibility),
    Linkage(FunctionLinkage),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDim3 {
    pub x: u32,
    pub y: Option<u32>,
    pub z: Option<u32>,
}

/// Visibility markers usable on functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionVisibility {
    Visible,
    Hidden,
}

/// Linkage modifiers for PTX functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionLinkage {
    Extern,
    Weak,
    WeakExtern,
}

/// Parameter declaration inside a PTX function signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub storage: Option<ParameterStorage>,
    pub alignment: Option<u32>,
    pub ty: Option<ScalarType>,
    pub qualifiers: ParameterQualifiers,
    pub array: Option<ArraySpecifier>,
    pub specifiers: Vec<ParameterSpecifier>,
    pub raw: String,
}

/// Qualifiers attached to a function parameter.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParameterQualifiers {
    pub is_const: bool,
    pub is_volatile: bool,
    pub is_restrict: bool,
    pub is_noalias: bool,
    pub pointer: Option<PointerQualifier>,
}

/// Raw specifier token captured while parsing a parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterSpecifier(pub String);

/// Pointer specific qualifiers that can decorate parameters.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PointerQualifier {
    pub address_space: Option<PointerAddressSpace>,
}

/// Address spaces that a pointer parameter can target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerAddressSpace {
    Generic,
    Global,
    Shared,
    Local,
    Const,
}

/// Storage classes that can prefix a function parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterStorage {
    Param,
}

/// Structured representation of a `.loc` directive inside a PTX function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocationDirective {
    pub file_index: u32,
    pub line: u32,
    pub column: u32,
    pub options: Vec<String>,
    pub comment: Option<String>,
    pub raw: String,
}

/// Structured representation of a `.pragma` directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PragmaDirective {
    pub arguments: Vec<String>,
    pub comment: Option<String>,
    pub raw: String,
}

/// Entry directives that appear before executable statements in a function body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionEntryDirective {
    Reg(RegisterDeclaration),
    Local(VariableDirective),
    Param(VariableDirective),
    Shared(VariableDirective),
    Pragma(PragmaDirective),
    Loc(LocationDirective),
    Dwarf(DwarfDirective),
}

/// Nested statement block enclosed in braces.
/// Executable items that appear within a function body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionStatement {
    Label(String),
    Directive(StatementDirective),
    Instruction(Instruction),
    ExternCallBlock(ExternCallBlock),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternCallSetup {
    Param(VariableDirective),
    Store(Instruction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternCallBlock {
    pub declarations: Vec<FunctionEntryDirective>,
    pub setup: Vec<ExternCallSetup>,
    pub call: Instruction,
    pub post_call: Vec<Instruction>,
}

/// Concretely parsed `.reg` directive inside a function body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterDeclaration {
    pub keyword: String,
    pub ty: RegisterType,
    pub registers: Vec<RegisterSpecifier>,
    pub comment: Option<String>,
    pub raw: String,
}

/// Normalised representation of the register type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterType {
    pub scalar: Option<ScalarType>,
    pub raw: String,
}

/// Individual register binding described by a `.reg` directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterSpecifier {
    Named(String),
    Range { prefix: String, count: u32 },
}

/// Directive that applies to individual statements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementDirective {
    Dwarf(DwarfDirective),
    Loc(LocationDirective),
    Pragma(PragmaDirective),
    Section(StatementSectionDirective),
}

/// Raw dwarf directive emitted by the compiler (e.g. @@dwarf).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DwarfDirective {
    pub keyword: String,
    pub arguments: Vec<String>,
    pub comment: Option<String>,
    pub raw: String,
}

/// Structured representation of a `.section` directive inside a function body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementSectionDirective {
    pub name: String,
    pub arguments: Vec<String>,
    pub comment: Option<String>,
    pub raw: String,
}
