/* --------------------------------------------------- */
/* -------------------- Basic Directives ------------- */
/* --------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeLinkage {
    /// `.visible`
    Visible,
    /// `.extern`
    Extern,
    /// `.weak`
    Weak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataLinkage {
    /// `.visible`
    Visible,
    /// `.extern`
    Extern,
    /// `.weak`
    Weak,
    /// `.common`
    Common,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeOrDataLinkage {
    /// `.visible`
    Visible,
    /// `.extern`
    Extern,
    /// `.weak`
    Weak,
    /// `.common`
    Common,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TexType {
    /// `.texref`
    TexRef,
    /// `.samplerref`
    SamplerRef,
    /// `.surfref`
    SurfRef,
}

/// Memory spaces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressSpace {
    Global,
    Const,
    Shared,
    Local,
    Param,
    Reg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeDirective {
    /// `.unified(uuid1, uuid2)`
    Unified(u64, u64),
    /// `.managed`
    Managed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u8`
    U8,
    /// `.u16`
    U16,
    /// `.u32`
    U32,
    /// `.u64`
    U64,
    /// `.s8`
    S8,
    /// `.s16`
    S16,
    /// `.s32`
    S32,
    /// `.s64`
    S64,
    /// `.f16`
    F16,
    /// `.f16x2`
    F16x2,
    /// `.f32`
    F32,
    /// `.f64`
    F64,
    /// `.b8`
    B8,
    /// `.b16`
    B16,
    /// `.b32`
    B32,
    /// `.b64`
    B64,
    /// `.b128`
    B128,
    /// `.pred`
    Pred,
}

/* -------------------------------------------------- */
/* -------------------- Math Basics ----------------- */
/* -------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Negative,
    Positive,
}

/// Axis component for 3-component special registers (x/y/z)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// No axis component present
    None,
    X,
    Y,
    Z,
}

/* --------------------------------------------------- */
/* -------------------- Label ------------------------ */
/* --------------------------------------------------- */

/// Label name (e.g., `L__label_1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label(pub String);

/* --------------------------------------------------- */
/* -------------------- Special Registers ------------ */
/* --------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialRegister {
    /// `%aggr_smem_size`
    AggrSmemSize,
    /// `%dynamic_smem_size`
    DynamicSmemSize,
    /// `%lanemask_gt`
    LanemaskGt,
    /// `%reserved_smem_offset_begin`
    ReservedSmemOffsetBegin,
    /// `%clock`
    Clock,
    /// `%envreg0`, ..., `%envreg31`
    Envreg(u8),
    /// `%lanemask_le`
    LanemaskLe,
    /// `%reserved_smem_offset_cap`
    ReservedSmemOffsetCap,
    /// `%clock64`
    Clock64,
    /// `%globaltimer`
    Globaltimer,
    /// `%lanemask_lt`
    LanemaskLt,
    /// `%reserved_smem_offset_end`
    ReservedSmemOffsetEnd,
    /// `%cluster_ctaid` (optionally `.x/.y/.z`)
    ClusterCtaid(Axis),
    /// `%globaltimer_hi`
    GlobaltimerHi,
    /// `%nclusterid`
    Nclusterid,
    /// `%smid`
    Smid,
    /// `%cluster_ctarank` (optionally `.x/.y/.z`)
    ClusterCtarank(Axis),
    /// `%globaltimer_lo`
    GlobaltimerLo,
    /// `%nctaid` (optionally `.x/.y/.z`)
    Nctaid(Axis),
    /// `%tid` (optionally `.x/.y/.z`)
    Tid(Axis),
    /// `%cluster_nctaid` (optionally `.x/.y/.z`)
    ClusterNctaid(Axis),
    /// `%gridid`
    Gridid,
    /// `%nsmid`
    Nsmid,
    /// `%total_smem_size`
    TotalSmemSize,
    /// `%cluster_nctarank` (optionally `.x/.y/.z`)
    ClusterNctarank(Axis),
    /// `%is_explicit_cluster`
    IsExplicitCluster,
    /// `%ntid` (optionally `.x/.y/.z`)
    Ntid(Axis),
    /// `%warpid`
    Warpid,
    /// `%clusterid`
    Clusterid,
    /// `%laneid`
    Laneid,
    /// `%nwarpid`
    Nwarpid,
    /// `%WARPSZ`
    WARPSZ,
    /// `%ctaid` (optionally `.x/.y/.z`)
    Ctaid(Axis),
    /// `%lanemask_eq`
    LanemaskEq,
    /// `%pm0`, ..., `%pm7`
    Pm(u8),
    /// `%pm0_64`, ..., `%pm7_64`
    Pm64(u8),
    /// `%current_graph_exec`
    CurrentGraphExec,
    /// `%lanemask_ge`
    LanemaskGe,
    /// `%reserved_smem_offset_0`, `%reserved_smem_offset_1`
    ReservedSmemOffset(u8),
}

/* --------------------------------------------------- */
/* ------------------- Operands ---------------------- */
/* --------------------------------------------------- */

/// Texture handler with 2 operands, e.g. [%r1, %r2]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TexHandler2([GeneralOperand; 2]);

/// Texture handler with 3 operands, e.g. [%r1{, %r2}, %r3] and [%r1, %r2, %r3]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TexHandler3([GeneralOperand; 3]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneralOperand{
    Vec(VectorOperand),
    Single(Operand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorOperand {
    /// {%r1}
    Vector1(Operand),
    /// {%r1, %r2}
    Vector2([Operand; 2]),
    /// {%r1, %r2, %r3}
    Vector3([Operand; 3]),
    /// {%r1, %r2, %r3, %r4}
    Vector4([Operand; 4]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    /// %r1
    Register(RegisterOperand),
    /// 0xffff
    Immediate(Immediate),
    /// foo
    Symbol(String),
}

/// Register operand starting with % (e.g., `%r1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterOperand(pub String);
/// Predicate register names (e.g., `%p0`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateRegister(pub String);

/// Representation of an address operand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressOperand {
    /// base[immIndex]
    Array(VariableSymbol, Immediate),
    /// Immediate address value, e.g., [0xffff], unsigned, 32-bit
    ImmediateAddress(Immediate),
    /// Offset address with optional displacement, e.g., [base + offset] and [base]
    Offset(AddressBase, Option<AddressOffset>),
}

/// Base location referenced by an address expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressBase {
    Register(RegisterOperand),
    Variable(VariableSymbol),
}

/// Specific adjustment applied within a displacement term.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressOffset {
    Register(RegisterOperand),
    Immediate(Sign, Immediate),
}

/* --------------------------------------------------- */
/* -------------------- Immediate -------------------- */
/* --------------------------------------------------- */

/// Immediate value represented as a string.
/// TODO: Replace with appropriate numeric type later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Immediate(pub String);

/* --------------------------------------------------- */
/* -------------------- Symbols ---------------------- */
/* --------------------------------------------------- */

/// Function symbol
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSymbol(pub String);

/// Variable symbol
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableSymbol(pub String);
