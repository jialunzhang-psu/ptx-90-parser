/* --------------------------------------------------- */
/* -------------------- Basic Directives ------------- */
/* --------------------------------------------------- */

use crate::parser::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeLinkage {
    /// `.visible`
    Visible { span: Span },
    /// `.extern`
    Extern { span: Span },
    /// `.weak`
    Weak { span: Span },
}

impl CodeLinkage {
    pub fn span(&self) -> Span {
        match self {
            CodeLinkage::Visible { span } => span.clone(),
            CodeLinkage::Extern { span } => span.clone(),
            CodeLinkage::Weak { span } => span.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl DataLinkage {
    pub fn span(&self) -> Span {
        match self {
            DataLinkage::Visible { span } => span.clone(),
            DataLinkage::Extern { span } => span.clone(),
            DataLinkage::Weak { span } => span.clone(),
            DataLinkage::Common { span } => span.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeOrDataLinkage {
    /// `.visible`
    Visible { span: Span },
    /// `.extern`
    Extern { span: Span },
    /// `.weak`
    Weak { span: Span },
    /// `.common`
    Common { span: Span },
}

impl CodeOrDataLinkage {
    pub fn span(&self) -> Span {
        match self {
            CodeOrDataLinkage::Visible { span } => span.clone(),
            CodeOrDataLinkage::Extern { span } => span.clone(),
            CodeOrDataLinkage::Weak { span } => span.clone(),
            CodeOrDataLinkage::Common { span } => span.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TexType {
    /// `.texref`
    TexRef { span: Span },
    /// `.samplerref`
    SamplerRef { span: Span },
    /// `.surfref`
    SurfRef { span: Span },
}

impl TexType {
    pub fn span(&self) -> Span {
        match self {
            TexType::TexRef { span } => span.clone(),
            TexType::SamplerRef { span } => span.clone(),
            TexType::SurfRef { span } => span.clone(),
        }
    }
}

/// Memory spaces
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressSpace {
    Global { span: Span },
    Const { span: Span },
    Shared { span: Span },
    Local { span: Span },
    Param { span: Span },
    Reg { span: Span },
}

impl AddressSpace {
    pub fn span(&self) -> Span {
        match self {
            AddressSpace::Global { span } => span.clone(),
            AddressSpace::Const { span } => span.clone(),
            AddressSpace::Shared { span } => span.clone(),
            AddressSpace::Local { span } => span.clone(),
            AddressSpace::Param { span } => span.clone(),
            AddressSpace::Reg { span } => span.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeDirective {
    /// `.unified(uuid1, uuid2)`
    Unified { uuid1: u64, uuid2: u64, span: Span },
    /// `.managed`
    Managed { span: Span },
}

impl AttributeDirective {
    pub fn span(&self) -> Span {
        match self {
            AttributeDirective::Unified { span, .. } => span.clone(),
            AttributeDirective::Managed { span } => span.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
}

impl DataType {
    pub fn span(&self) -> Span {
        match self {
            DataType::U8 { span } => span.clone(),
            DataType::U16 { span } => span.clone(),
            DataType::U32 { span } => span.clone(),
            DataType::U64 { span } => span.clone(),
            DataType::S8 { span } => span.clone(),
            DataType::S16 { span } => span.clone(),
            DataType::S32 { span } => span.clone(),
            DataType::S64 { span } => span.clone(),
            DataType::F16 { span } => span.clone(),
            DataType::F16x2 { span } => span.clone(),
            DataType::F32 { span } => span.clone(),
            DataType::F64 { span } => span.clone(),
            DataType::B8 { span } => span.clone(),
            DataType::B16 { span } => span.clone(),
            DataType::B32 { span } => span.clone(),
            DataType::B64 { span } => span.clone(),
            DataType::B128 { span } => span.clone(),
            DataType::Pred { span } => span.clone(),
        }
    }
}

/* -------------------------------------------------- */
/* -------------------- Math Basics ----------------- */
/* -------------------------------------------------- */

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    Negative { span: Span },
    Positive { span: Span },
}

impl Sign {
    pub fn span(&self) -> Span {
        match self {
            Sign::Negative { span } => span.clone(),
            Sign::Positive { span } => span.clone(),
        }
    }
}

/// Axis component for 3-component special registers (x/y/z)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Axis {
    /// No axis component present
    None { span: Span },
    X { span: Span },
    Y { span: Span },
    Z { span: Span },
}

impl Axis {
    pub fn span(&self) -> Span {
        match self {
            Axis::None { span } => span.clone(),
            Axis::X { span } => span.clone(),
            Axis::Y { span } => span.clone(),
            Axis::Z { span } => span.clone(),
        }
    }
}

/* --------------------------------------------------- */
/* -------------------- Label ------------------------ */
/* --------------------------------------------------- */

/// Label name (e.g., `L__label_1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub name: String,
    pub span: Span,
}

impl Label {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/* --------------------------------------------------- */
/* -------------------- Special Registers ------------ */
/* --------------------------------------------------- */

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl SpecialRegister {
    pub fn span(&self) -> Span {
        match self {
            SpecialRegister::AggrSmemSize { span } => span.clone(),
            SpecialRegister::DynamicSmemSize { span } => span.clone(),
            SpecialRegister::LanemaskGt { span } => span.clone(),
            SpecialRegister::ReservedSmemOffsetBegin { span } => span.clone(),
            SpecialRegister::Clock { span } => span.clone(),
            SpecialRegister::Envreg { span, .. } => span.clone(),
            SpecialRegister::LanemaskLe { span } => span.clone(),
            SpecialRegister::ReservedSmemOffsetCap { span } => span.clone(),
            SpecialRegister::Clock64 { span } => span.clone(),
            SpecialRegister::Globaltimer { span } => span.clone(),
            SpecialRegister::LanemaskLt { span } => span.clone(),
            SpecialRegister::ReservedSmemOffsetEnd { span } => span.clone(),
            SpecialRegister::ClusterCtaid { span, .. } => span.clone(),
            SpecialRegister::GlobaltimerHi { span } => span.clone(),
            SpecialRegister::Nclusterid { span } => span.clone(),
            SpecialRegister::Smid { span } => span.clone(),
            SpecialRegister::ClusterCtarank { span, .. } => span.clone(),
            SpecialRegister::GlobaltimerLo { span } => span.clone(),
            SpecialRegister::Nctaid { span, .. } => span.clone(),
            SpecialRegister::Tid { span, .. } => span.clone(),
            SpecialRegister::ClusterNctaid { span, .. } => span.clone(),
            SpecialRegister::Gridid { span } => span.clone(),
            SpecialRegister::Nsmid { span } => span.clone(),
            SpecialRegister::TotalSmemSize { span } => span.clone(),
            SpecialRegister::ClusterNctarank { span, .. } => span.clone(),
            SpecialRegister::IsExplicitCluster { span } => span.clone(),
            SpecialRegister::Ntid { span, .. } => span.clone(),
            SpecialRegister::Warpid { span } => span.clone(),
            SpecialRegister::Clusterid { span } => span.clone(),
            SpecialRegister::Laneid { span } => span.clone(),
            SpecialRegister::Nwarpid { span } => span.clone(),
            SpecialRegister::WARPSZ { span } => span.clone(),
            SpecialRegister::Ctaid { span, .. } => span.clone(),
            SpecialRegister::LanemaskEq { span } => span.clone(),
            SpecialRegister::Pm { span, .. } => span.clone(),
            SpecialRegister::Pm64 { span, .. } => span.clone(),
            SpecialRegister::CurrentGraphExec { span } => span.clone(),
            SpecialRegister::LanemaskGe { span } => span.clone(),
            SpecialRegister::ReservedSmemOffset { span, .. } => span.clone(),
        }
    }
}

/* --------------------------------------------------- */
/* ------------------- Operands ---------------------- */
/* --------------------------------------------------- */

/// Texture handler with 2 operands, e.g. [%r1, %r2]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TexHandler2 {
    pub operands: [GeneralOperand; 2],
    pub span: Span,
}

impl TexHandler2 {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Texture handler with optional sampler operand, e.g. `[tex, coords]` or `[tex, sampler, coords]`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TexHandler3Optional {
    pub handle: GeneralOperand,
    pub sampler: Option<GeneralOperand>,
    pub coords: GeneralOperand,
    pub span: Span,
}

impl TexHandler3Optional {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Texture handler with optional sampler operand, e.g. `[tex, coords]` or `[tex, sampler, coords]`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TexHandler3 {
    pub handle: GeneralOperand,
    pub sampler: GeneralOperand,
    pub coords: GeneralOperand,
    pub span: Span,
}

impl TexHandler3 {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneralOperand {
    Vec { operand: VectorOperand, span: Span },
    Single { operand: Operand, span: Span },
}

impl GeneralOperand {
    pub fn span(&self) -> Span {
        match self {
            GeneralOperand::Vec { span, .. } => span.clone(),
            GeneralOperand::Single { span, .. } => span.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl VectorOperand {
    pub fn span(&self) -> Span {
        match self {
            VectorOperand::Vector1 { span, .. } => span.clone(),
            VectorOperand::Vector2 { span, .. } => span.clone(),
            VectorOperand::Vector3 { span, .. } => span.clone(),
            VectorOperand::Vector4 { span, .. } => span.clone(),
            VectorOperand::Vector8 { span, .. } => span.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    /// %r1
    Register { operand: RegisterOperand, span: Span },
    /// 0xffff
    Immediate { operand: Immediate, span: Span },
    /// foo
    Symbol { name: String, span: Span },
    /// foo + 4 (symbol + immediate offset)
    SymbolOffset { symbol: String, offset: Immediate, span: Span },
}

impl Operand {
    pub fn span(&self) -> Span {
        match self {
            Operand::Register { span, .. } => span.clone(),
            Operand::Immediate { span, .. } => span.clone(),
            Operand::Symbol { span, .. } => span.clone(),
            Operand::SymbolOffset { span, .. } => span.clone(),
        }
    }
}

/// Register operand starting with % (e.g., `%r1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterOperand {
    pub name: String,
    pub span: Span,
}

impl RegisterOperand {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Predicate register names (e.g., `%p0`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateRegister {
    pub name: String,
    pub span: Span,
}

impl PredicateRegister {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Representation of an address operand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressOperand {
    /// base[immIndex]
    Array { base: VariableSymbol, index: Immediate, span: Span },
    /// Immediate address value, e.g., [0xffff], unsigned, 32-bit
    ImmediateAddress { addr: Immediate, span: Span },
    /// Offset address with optional displacement, e.g., [base + offset] and [base]
    Offset { base: AddressBase, offset: Option<AddressOffset>, span: Span },
}

impl AddressOperand {
    pub fn span(&self) -> Span {
        match self {
            AddressOperand::Array { span, .. } => span.clone(),
            AddressOperand::ImmediateAddress { span, .. } => span.clone(),
            AddressOperand::Offset { span, .. } => span.clone(),
        }
    }
}

/// Base location referenced by an address expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressBase {
    Register { operand: RegisterOperand, span: Span },
    Variable { symbol: VariableSymbol, span: Span },
}

impl AddressBase {
    pub fn span(&self) -> Span {
        match self {
            AddressBase::Register { span, .. } => span.clone(),
            AddressBase::Variable { span, .. } => span.clone(),
        }
    }
}

/// Specific adjustment applied within a displacement term.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressOffset {
    Register { operand: RegisterOperand, span: Span },
    Immediate { sign: Sign, value: Immediate, span: Span },
}

impl AddressOffset {
    pub fn span(&self) -> Span {
        match self {
            AddressOffset::Register { span, .. } => span.clone(),
            AddressOffset::Immediate { span, .. } => span.clone(),
        }
    }
}

/* --------------------------------------------------- */
/* -------------------- Immediate -------------------- */
/* --------------------------------------------------- */

/// Immediate value represented as a string.
/// TODO: Replace with appropriate numeric type later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Immediate {
    pub value: String,
    pub span: Span,
}

impl Immediate {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/* --------------------------------------------------- */
/* -------------------- Symbols ---------------------- */
/* --------------------------------------------------- */

/// Function symbol
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSymbol {
    pub name: String,
    pub span: Span,
}

impl FunctionSymbol {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Variable symbol
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableSymbol {
    pub name: String,
    pub span: Span,
}

impl VariableSymbol {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/* --------------------------------------------------- */
/* -------------------- Predicate -------------------- */
/* --------------------------------------------------- */

/// Predicate guard for conditional instruction execution
#[derive(Debug, Clone, PartialEq)]
pub struct Predicate {
    pub negated: bool,
    pub operand: Operand,
    pub span: Span,
}

impl Predicate {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/* --------------------------------------------------- */
/* -------------------- Instruction ------------------ */
/* --------------------------------------------------- */

/// Represents a complete instruction with optional label and predicate guard
/// Format: [label:] [@{!}pred] instruction
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub predicate: Option<Predicate>,
    pub inst: crate::r#type::instruction::Inst,
    pub span: Span,
}

impl Instruction {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}
