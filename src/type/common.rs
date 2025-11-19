/* --------------------------------------------------- */
/* -------------------- Basic Directives ------------- */
/* --------------------------------------------------- */

use crate::Spanned;
use crate::parser::Span;

#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum CodeLinkage {
    /// `.visible`
    Visible { span: Span },
    /// `.extern`
    Extern { span: Span },
    /// `.weak`
    Weak { span: Span },
}

#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum DataLinkage {
    /// `.visible`
    Visible { span: Span },
    /// `.extern`
    Extern { span: Span },
    /// `.weak`
    Weak { span: Span },
    /// `.common`
    Common { span: Span },
}

#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum AttributeDirective {
    /// `.unified(uuid1, uuid2)`
    Unified { uuid1: u64, uuid2: u64, span: Span },
    /// `.managed`
    Managed { span: Span },
}

#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum DataType {
    /// `.u8`
    U8 { span: Span },
    /// `.u16`
    U16 { span: Span },
    /// `.u32`
    U32 { span: Span },
    /// `.u64`
    U64 { span: Span },
    /// `.s8`
    S8 { span: Span },
    /// `.s16`
    S16 { span: Span },
    /// `.s32`
    S32 { span: Span },
    /// `.s64`
    S64 { span: Span },
    /// `.f16`
    F16 { span: Span },
    /// `.f16x2`
    F16x2 { span: Span },
    /// `.f32`
    F32 { span: Span },
    /// `.f64`
    F64 { span: Span },
    /// `.b8`
    B8 { span: Span },
    /// `.b16`
    B16 { span: Span },
    /// `.b32`
    B32 { span: Span },
    /// `.b64`
    B64 { span: Span },
    /// `.b128`
    B128 { span: Span },
    /// `.pred`
    Pred { span: Span },
    /// `.texref`
    TexRef { span: Span },
    /// `.samplerref`
    SamplerRef { span: Span },
    /// `.surfref`
    SurfRef { span: Span },
}

/* -------------------------------------------------- */
/* -------------------- Math Basics ----------------- */
/* -------------------------------------------------- */

#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum Sign {
    Negative { span: Span },
    Positive { span: Span },
}

/// Axis component for 3-component special registers (x/y/z)
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum Axis {
    /// No axis component present
    None {
        span: Span,
    },
    X {
        span: Span,
    },
    Y {
        span: Span,
    },
    Z {
        span: Span,
    },
}

/* --------------------------------------------------- */
/* -------------------- Special Registers ------------ */
/* --------------------------------------------------- */

#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum SpecialRegister {
    /// `%aggr_smem_size`
    AggrSmemSize { span: Span },
    /// `%dynamic_smem_size`
    DynamicSmemSize { span: Span },
    /// `%lanemask_gt`
    LanemaskGt { span: Span },
    /// `%reserved_smem_offset_begin`
    ReservedSmemOffsetBegin { span: Span },
    /// `%clock`
    Clock { span: Span },
    /// `%envreg0`, ..., `%envreg31`
    Envreg { index: u8, span: Span },
    /// `%lanemask_le`
    LanemaskLe { span: Span },
    /// `%reserved_smem_offset_cap`
    ReservedSmemOffsetCap { span: Span },
    /// `%clock64`
    Clock64 { span: Span },
    /// `%globaltimer`
    Globaltimer { span: Span },
    /// `%lanemask_lt`
    LanemaskLt { span: Span },
    /// `%reserved_smem_offset_end`
    ReservedSmemOffsetEnd { span: Span },
    /// `%cluster_ctaid` (optionally `.x/.y/.z`)
    ClusterCtaid { axis: Axis, span: Span },
    /// `%globaltimer_hi`
    GlobaltimerHi { span: Span },
    /// `%nclusterid`
    Nclusterid { span: Span },
    /// `%smid`
    Smid { span: Span },
    /// `%cluster_ctarank` (optionally `.x/.y/.z`)
    ClusterCtarank { axis: Axis, span: Span },
    /// `%globaltimer_lo`
    GlobaltimerLo { span: Span },
    /// `%nctaid` (optionally `.x/.y/.z`)
    Nctaid { axis: Axis, span: Span },
    /// `%tid` (optionally `.x/.y/.z`)
    Tid { axis: Axis, span: Span },
    /// `%cluster_nctaid` (optionally `.x/.y/.z`)
    ClusterNctaid { axis: Axis, span: Span },
    /// `%gridid`
    Gridid { span: Span },
    /// `%nsmid`
    Nsmid { span: Span },
    /// `%total_smem_size`
    TotalSmemSize { span: Span },
    /// `%cluster_nctarank` (optionally `.x/.y/.z`)
    ClusterNctarank { axis: Axis, span: Span },
    /// `%is_explicit_cluster`
    IsExplicitCluster { span: Span },
    /// `%ntid` (optionally `.x/.y/.z`)
    Ntid { axis: Axis, span: Span },
    /// `%warpid`
    Warpid { span: Span },
    /// `%clusterid`
    Clusterid { span: Span },
    /// `%laneid`
    Laneid { span: Span },
    /// `%nwarpid`
    Nwarpid { span: Span },
    /// `%WARPSZ`
    WARPSZ { span: Span },
    /// `%ctaid` (optionally `.x/.y/.z`)
    Ctaid { axis: Axis, span: Span },
    /// `%lanemask_eq`
    LanemaskEq { span: Span },
    /// `%pm0`, ..., `%pm7`
    Pm { index: u8, span: Span },
    /// `%pm0_64`, ..., `%pm7_64`
    Pm64 { index: u8, span: Span },
    /// `%current_graph_exec`
    CurrentGraphExec { span: Span },
    /// `%lanemask_ge`
    LanemaskGe { span: Span },
    /// `%reserved_smem_offset_0`, `%reserved_smem_offset_1`
    ReservedSmemOffset { index: u8, span: Span },
}

/* --------------------------------------------------- */
/* ------------------- Operands ---------------------- */
/* --------------------------------------------------- */

/// Texture handler with 2 operands, e.g. [%r1, %r2]
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub struct TexHandler2 {
    pub operands: [GeneralOperand; 2],
    pub span: Span,
}

/// Texture handler with optional sampler operand, e.g. `[tex, coords]` or `[tex, sampler, coords]`
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub struct TexHandler3Optional {
    pub handle: GeneralOperand,
    pub sampler: Option<GeneralOperand>,
    pub coords: GeneralOperand,
    pub span: Span,
}

/// Texture handler with optional sampler operand, e.g. `[tex, coords]` or `[tex, sampler, coords]`
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub struct TexHandler3 {
    pub handle: GeneralOperand,
    pub sampler: GeneralOperand,
    pub coords: GeneralOperand,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum GeneralOperand {
    Vec { operand: VectorOperand, span: Span },
    Single { operand: Operand, span: Span },
}

#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum VectorOperand {
    /// {%r1}
    Vector1 { operand: Operand, span: Span },
    /// {%r1, %r2}
    Vector2 { operands: [Operand; 2], span: Span },
    /// {%r1, %r2, %r3}
    Vector3 { operands: [Operand; 3], span: Span },
    /// {%r1, %r2, %r3, %r4}
    Vector4 { operands: [Operand; 4], span: Span },
    /// {%r1, %r2, %r3, %r4, %r5, %r6, %r7, %r8}
    Vector8 { operands: [Operand; 8], span: Span },
}

#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum Operand {
    /// %r1
    Register {
        operand: RegisterOperand,
        span: Span,
    },
    /// 0xffff
    Immediate { operand: Immediate, span: Span },
    /// foo
    Symbol { name: String, span: Span },
    /// foo + 4 (symbol + immediate offset)
    SymbolOffset {
        symbol: String,
        offset: Immediate,
        span: Span,
    },
}

/// Register operand starting with % (e.g., `%r1`).
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub struct RegisterOperand {
    pub name: String,
    /// Optional component suffix (e.g., `.x`, `.y`, `.z`, `.w`)
    pub component: Option<String>,
    pub span: Span,
}

/// Predicate register names (e.g., `%p0`).
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub struct PredicateRegister {
    pub name: String,
    pub span: Span,
}

/// Representation of an address operand.
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum AddressOperand {
    /// base[immIndex]
    Array {
        base: VariableSymbol,
        index: Immediate,
        span: Span,
    },
    /// Immediate address value, e.g., [0xffff], unsigned, 32-bit
    ImmediateAddress { addr: Immediate, span: Span },
    /// Offset address with optional displacement, e.g., [base + offset] and [base]
    Offset {
        base: AddressBase,
        offset: Option<AddressOffset>,
        span: Span,
    },
}

/// Base location referenced by an address expression.
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum AddressBase {
    Register {
        operand: RegisterOperand,
        span: Span,
    },
    Variable {
        symbol: VariableSymbol,
        span: Span,
    },
}

/// Specific adjustment applied within a displacement term.
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum AddressOffset {
    Register {
        operand: RegisterOperand,
        span: Span,
    },
    Immediate {
        sign: Sign,
        value: Immediate,
        span: Span,
    },
}

/* --------------------------------------------------- */
/* -------------------- Immediate -------------------- */
/* --------------------------------------------------- */

/// Immediate value representing a constant literal in PTX assembly.
///
/// Supports all PTX constant formats:
///
/// **Integer literals** (with optional `U` suffix for unsigned):
/// - Hexadecimal: `0x1234`, `0x1234U`, `0XABCD`
/// - Octal: `0777`, `0777U`, `0123`
/// - Binary: `0b1010`, `0b1010U`, `0B0011`
/// - Decimal: `42`, `42U`, `0`, `0U`, `123`
///
/// **Floating-point literals**:
/// - Decimal float: `3.14`, `2.5`
/// - Scientific notation: `1.5e10`, `3.2E-5`, `1e3`
/// - Hex float (single-precision, 32-bit): `0f3f800000` (8 hex digits after `0f`)
/// - Hex float (double-precision, 64-bit): `0d3ff0000000000000` (16 hex digits after `0d`)
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub struct Immediate {
    /// The literal value as a string (includes prefix, digits, and optional 'U' suffix)
    pub value: String,
    pub span: Span,
}

/* --------------------------------------------------- */
/* -------------------- Symbols ---------------------- */
/* --------------------------------------------------- */

/// Function symbol
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub struct FunctionSymbol {
    pub val: String,
    pub span: Span,
}

/// Variable symbol
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub struct VariableSymbol {
    pub val: String,
    pub span: Span,
}

/// Label name (e.g., `L__label_1`).
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub struct Label {
    pub val: String,
    pub span: Span,
}

/* --------------------------------------------------- */
/* -------------------- Predicate -------------------- */
/* --------------------------------------------------- */

/// Predicate guard for conditional instruction execution
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Predicate {
    pub negated: bool,
    pub operand: Operand,
    pub span: Span,
}

/* --------------------------------------------------- */
/* -------------------- Instruction ------------------ */
/* --------------------------------------------------- */

/// Represents a complete instruction with optional label and predicate guard
/// Format: [label:] [@{!}pred] instruction
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct Instruction {
    pub predicate: Option<Predicate>,
    pub inst: crate::r#type::instruction::Inst,
    pub span: Span,
}
