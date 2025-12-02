use super::common::{Instruction, Label};
use super::variable::{ParameterDirective, VariableDirective};
use crate::Spanned;
use crate::parser::Span;
use crate::r#type::{AttributeDirective, DataType, FunctionSymbol, VariableSymbol};
use serde::Serialize;

/// Alias directive relating one function symbol to another.
///
/// Syntax:
/// .alias fAlias, fAliasee;
///
/// Example:
/// .alias foo, bar;
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct AliasFunctionDirective {
    pub alias: FunctionSymbol,
    pub target: FunctionSymbol,
    pub span: Span,
}

/// A PTX kernel declared with the `.entry` directive.
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct FuncFunctionDirective {
    /// Example:
    /// .func .attribute(.unified(0xAB, 0xCD)) bar() { ... }
    pub attributes: Vec<AttributeDirective>,
    /// Optional return param.
    ///
    /// Example:
    /// .func (.param .u32 rval) bar(.param .u32 N, .param .align 4 .b8 numbers[]) { ... }
    pub return_param: Option<ParameterDirective>,
    /// Function name.
    pub name: FunctionSymbol,
    /// Function parameters.
    ///
    /// Example:
    /// .func (.param .u32 rval) bar(.param .u32 N, .param .align 4 .b8 numbers[])
    pub params: Vec<ParameterDirective>,
    /// Optional directives.
    ///
    /// Example:
    /// .func foo (.reg .b32 N, .reg .f64 dbl) .noreturn { ... }
    pub directives: Vec<FuncFunctionHeaderDirective>,
    /// Pre-body declarations (.reg, .local, .shared, .param) that appear between
    /// the function header and the body. These are allowed by PTX but must appear
    /// before the opening brace.
    ///
    /// Example:
    /// .func foo()
    ///     .reg .b32 %r0;
    ///     .local .b8 stack[16];
    /// { ... }
    pub pre_body_declarations: Vec<StatementDirective>,
    /// Optional function body. Without body represents a function prototype.
    pub body: Option<FunctionBody>,
    pub span: Span,
}

/// A PTX device function declared with the `.func` directive.
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct EntryFunctionDirective {
    /// Name of the entry function.
    pub name: FunctionSymbol,
    /// Function parameters.
    pub params: Vec<ParameterDirective>,
    /// Optional directives.
    pub directives: Vec<EntryFunctionHeaderDirective>,
    /// Optional function body. Without body represents a function prototype.
    pub body: Option<FunctionBody>,
    pub span: Span,
}

/// Directive tokens that may decorate a PTX function header.
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub enum FuncFunctionHeaderDirective {
    /// Syntax:
    /// .noreturn
    ///
    /// Example:
    /// .func foo .noreturn { ... }
    NoReturn { span: Span },
    /// Syntax:
    /// .pragma list-of-strings ;
    ///
    /// Example:
    ///.entry foo .pragma "nounroll"; { ... } // disable unrolling for current kernel
    Pragma { args: Vec<String>, span: Span },
    /// Syntax:
    /// .abi_preserve N
    ///
    /// Example:
    /// .entry foo .abi_preserve 8 { ... }
    AbiPreserve { value: u32, span: Span },
    /// Syntax:
    /// .abi_preserve_control N
    ///
    /// Example:
    /// .entry foo .abi_preserve_control 16 { ... }
    AbiPreserveControl { value: u32, span: Span },
}

/// Directive tokens that may decorate a PTX function header.
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub enum EntryFunctionHeaderDirective {
    /// Syntax:
    /// .maxnreg n
    ///
    /// Example:
    /// .entry foo .maxnreg 16 { ... }  // max regs per thread = 16
    MaxNReg { value: u32, span: Span },
    /// Syntax:
    /// .maxntid nx
    /// .maxntid nx, ny
    /// .maxntid nx, ny, nz
    ///
    /// Example:
    /// .entry foo .maxntid 256       { ... }  // max threads = 256
    /// .entry bar .maxntid 16,16,4   { ... }  // max threads = 1024
    MaxNTid { dim: FunctionDim, span: Span },
    /// Syntax:
    /// .reqntid nx
    /// .reqntid nx, ny
    /// .reqntid nx, ny, nz
    ///
    /// Example:
    /// .entry foo .reqntid 256       { ... }  // num threads = 256
    /// .entry bar .reqntid 16,16,4   { ... }  // num threads = 1024
    ReqNTid { dim: FunctionDim, span: Span },
    /// Syntax:
    /// .minnctapersm ncta
    ///
    /// Example:
    /// .entry foo .maxntid 256 .minnctapersm 4 { ... }
    MinNCtaPerSm { value: u32, span: Span },
    /// Syntax:
    /// .maxnctapersm ncta
    ///
    /// Example:
    /// .entry foo .maxntid 256 .maxnctapersm 4 { ... }
    MaxNCtaPerSm { value: u32, span: Span },
    /// Syntax:
    /// .pragma list-of-strings ;
    ///
    /// Example:
    ///.entry foo .pragma "nounroll"; { ... } // disable unrolling for current kernel
    Pragma { args: Vec<String>, span: Span },
    /// Syntax:
    /// .reqnctapercluster nx
    /// .reqnctapercluster nx, ny
    /// .reqnctapercluster nx, ny, nz
    ///
    /// Example:
    /// .entry foo .reqnctapercluster 2         { . . . }
    /// .entry bar .reqnctapercluster 2, 2, 1   { . . . }
    /// .entry ker .reqnctapercluster 3, 2      { . . . }
    ReqNctaPerCluster { dim: FunctionDim, span: Span },
    /// Syntax:
    /// .explicitcluster
    ///
    /// Example:
    /// .entry foo .explicitcluster         { . . . }
    ExplicitCluster { span: Span },
    /// Syntax:
    /// .maxclusterrank n
    ///
    /// Example:
    /// .entry foo ..maxclusterrank 8         { . . . }
    MaxClusterRank { value: u32, span: Span },
    /// Syntax:
    ///.blocksareclusters
    ///
    /// Example:
    /// .entry foo .reqntid 32, 32, 1 .reqnctapercluster 32, 32, 1 .blocksareclusters { ... } // only allowed when with .reqnctapercluster and .reqntid
    BlocksAreClusters { span: Span },
}

/// Statements contained within a PTX function body.
#[derive(Debug, Clone, Default, PartialEq, Spanned, Serialize)]
pub struct FunctionBody {
    pub statements: Vec<FunctionStatement>,
    pub span: Span,
}

/// Nested statement block enclosed in braces.
/// Executable items that appear within a function body.
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub enum FunctionStatement {
    Label {
        label: Label,
        span: Span,
    },
    Directive {
        directive: StatementDirective,
        span: Span,
    },
    Instruction {
        instruction: Instruction,
        span: Span,
    },
    Block {
        statements: Vec<FunctionStatement>,
        span: Span,
    },
}

/// Directive that declares a register variable inside a function body.
///
/// Syntax:
/// .reg .ty name<range>
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct RegisterDirective {
    pub ty: DataType,
    pub registers: Vec<RegisterTarget>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct RegisterTarget {
    pub name: VariableSymbol,
    pub range: Option<u32>,
    pub span: Span,
}

/// Directive that applies to individual statements.
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub enum StatementDirective {
    Loc {
        directive: LocationDirective,
        span: Span,
    },
    Pragma {
        directive: PragmaDirective,
        span: Span,
    },
    Section {
        directive: SectionDirective,
        span: Span,
    },
    Reg {
        directive: RegisterDirective,
        span: Span,
    },
    Local {
        directive: VariableDirective,
        span: Span,
    },
    Param {
        directive: VariableDirective,
        span: Span,
    },
    Shared {
        directive: VariableDirective,
        span: Span,
    },
    Dwarf {
        directive: DwarfDirective,
        span: Span,
    },
    BranchTargets {
        directive: BranchTargetsDirective,
        span: Span,
    },
    CallTargets {
        directive: CallTargetsDirective,
        span: Span,
    },
    CallPrototype {
        directive: CallPrototypeDirective,
        span: Span,
    },
}

/// Raw dwarf directive emitted by the compiler (e.g. `@@dwarf`).
///
/// Syntax:
/// ```text
/// @@DWARF dwarf-string
///
/// dwarf-string may have one of the
/// .byte   byte-list   // comma-separated hexadecimal byte values
/// .4byte  int32-list  // comma-separated hexadecimal integers in range [0..2^32-1]
/// .quad   int64-list  // comma-separated hexadecimal integers in range [0..2^64-1]
/// .4byte  label
/// .quad   label
/// ```
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct DwarfDirective {
    pub kind: DwarfDirectiveKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum DwarfDirectiveKind {
    ByteValues(Vec<u8>),
    FourByteValues(Vec<u32>),
    QuadValues(Vec<u64>),
    FourByteLabel(Label),
    QuadLabel(Label),
}

/// Structured representation of a `.section` directive inside a function body.
///
/// Syntax:
/// ```text
/// .section section_name { dwarf-lines }
///
/// dwarf-lines have the following formats:
///   .b8    byte-list       // integers in [-128..255]
///   .b16   int16-list      // integers in [-2^15..2^16-1]
///   .b32   int32-list      // integers in [-2^31..2^32-1]
///   label:                 // define label inside the debug section
///   .b64   int64-list      // integers in [-2^63..2^64-1]
///   .b32   label
///   .b64   label
///   .b32   label+imm       // label plus constant integer byte offset (32-bit)
///   .b64   label+imm       // label plus constant integer byte offset (64-bit)
///   .b32   label1-label2   // difference between labels in same section (32-bit)
///   .b64   label3-label4   // difference between labels in same section (64-bit)
/// ```
///
/// Example:
/// ```text
///     .section .debug_str {
///    info_string0:
///     .b8 95  // _
///     .b8 90  // z
///     .b8 51  // 3
///     .b8 102 // f
///     .b8 111 // o
///     .b8 111 // o
///     .b8 118 // v
///     .b8 0
///    info_string1:
///     .b8 95  // _
///     .b8 90  // z
///     .b8 51  // 3
///     .b8 98  // b
///     .b8 97  // a
///     .b8 114 // r
///     .b8 118 // v
///     .b8 0
///     .b8 95  // _
///     .b8 90  // z
///     .b8 51  // 3
///     .b8 99  // c
///     .b8 97  // a
///     .b8 114 // r
///     .b8 118 // v
///     .b8 0
///    }
/// ```
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct SectionDirective {
    pub name: String,
    pub entries: Vec<SectionEntry>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum SectionEntry {
    Label { label: Label, span: Span },
    Directive(StatementSectionDirectiveLine),
}

#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub enum StatementSectionDirectiveLine {
    B8 { values: Vec<i16>, span: Span },
    B16 { values: Vec<i32>, span: Span },
    B32Immediate { values: Vec<i64>, span: Span },
    B64Immediate { values: Vec<i128>, span: Span },
    B32Label { labels: Label, span: Span },
    B64Label { labels: Label, span: Span },
    B32LabelPlusImm { entries: (Label, i32), span: Span },
    B64LabelPlusImm { entries: (Label, i64), span: Span },
    B32LabelDiff { entries: (Label, Label), span: Span },
    B64LabelDiff { entries: (Label, Label), span: Span },
}

/// Structured representation of a `.loc` directive inside a PTX function.
///
/// Syntax:
///     .loc file_index line_number column_position
///     .loc file_index line_number column_position,function_name label {+ immediate }, inlined_at file_index2 line_number2 column_position2
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct LocationDirective {
    pub file_index: u32,
    pub line: u32,
    pub column: u32,
    pub inlined_at: Option<LocationInlinedAt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct LocationInlinedAt {
    pub file_index: u32,
    pub line: u32,
    pub column: u32,
    pub function_name: FunctionSymbol,
    pub label: Label,
    pub label_offset: Option<i64>,
    pub span: Span,
}

/// Structured representation of a `.pragma` directive.
///
/// Syntax:
///     .pragma "nounroll";
///     .pragma "used_bytes_mask mask";
///     .pragma "enable_smem_spilling";
///     .pragma "frequency n";
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct PragmaDirective {
    pub kind: PragmaDirectiveKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PragmaDirectiveKind {
    Nounroll,
    UsedBytesMask { mask: String },
    EnableSmemSpilling,
    Frequency { value: u32 },
    Raw(String),
}

/// Structured representation of a `.branchtargets` directive.
///
/// Syntax:
///    .branchtargets label1, label2, label3, ...;
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct BranchTargetsDirective {
    pub labels: Vec<Label>,
    pub span: Span,
}

/// Structured representation of a `.calltargets` directive.
///
/// Syntax:
///     .calltargets func1, func2, func3, ...;
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct CallTargetsDirective {
    pub targets: Vec<FunctionSymbol>,
    pub span: Span,
}

/// Structured representation of a `.callprototype` directive.
///
/// Syntax:
///     // no input or return parameters
///     label: .callprototype _ .noreturn {.abi_preserve N} {.abi_preserve_control N};
///     // input params, no return params
///     label: .callprototype _ (param-list) .noreturn {.abi_preserve N} {.abi_preserve_control N};
///     // no input params, // return params
///     label: .callprototype (ret-param) _ {.abi_preserve N} {.abi_preserve_control N};
///     // input, return parameters
///     label: .callprototype (ret-param) _ (param-list) {.abi_preserve N} {.abi_preserve_control N};
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub struct CallPrototypeDirective {
    pub return_param: Option<ParameterDirective>,
    pub params: Vec<ParameterDirective>,
    pub noreturn: bool,
    pub abi_preserve: Option<u32>,
    pub abi_preserve_control: Option<u32>,
    pub span: Span,
}

/// Dimension triplet used by several function header directives.
#[derive(Debug, Clone, PartialEq, Spanned, Serialize)]
pub enum FunctionDim {
    X { x: u32, span: Span },
    XY { x: u32, y: u32, span: Span },
    XYZ { x: u32, y: u32, z: u32, span: Span },
}
