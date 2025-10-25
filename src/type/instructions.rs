/// A PTX instruction with optional predicate and modifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub predicate: Option<String>,
    pub opcode: InstructionOpcode,
    pub operands: Vec<Operand>,
    pub comment: Option<String>,
    pub raw: String,
}

/// An opcode annotated with its parsed modifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionOpcode {
    pub kind: OpcodeKind,
    pub modifiers: Vec<ModifierKind>,
}

/// Categorisation of PTX opcodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpcodeKind {
    Abs,
    Activemask,
    Add,
    Addc,
    Alloca,
    And,
    Applypriority,
    Atom,
    Bar,
    Barrier,
    Bfe,
    Bfi,
    Bfind,
    Bmsk,
    Brev,
    Bra,
    Brkpt,
    Brx,
    Call,
    Clz,
    Clusterlaunchcontrol,
    Cnot,
    Copysign,
    Cos,
    Cp,
    Createpolicy,
    Cvt,
    Cvta,
    Div,
    Discard,
    Dp2a,
    Dp4a,
    Elect,
    Ex2,
    Exit,
    Fence,
    Fma,
    Fns,
    Getctarank,
    Griddepcontrol,
    Isspacep,
    Istypep,
    Ld,
    Ldmatrix,
    Ldu,
    Lg2,
    Lop3,
    Mad,
    Mad24,
    Madc,
    Mapa,
    Match,
    Max,
    Mbarrier,
    Membar,
    Min,
    Mov,
    Movmatrix,
    Mma,
    Mul,
    Mul24,
    Multimem,
    Nanosleep,
    Neg,
    Not,
    Or,
    Pmevent,
    Popc,
    Prefetch,
    Prefetchu,
    Prmt,
    Rcp,
    Red,
    Redux,
    Rem,
    Rsqrt,
    Sad,
    Selp,
    Set,
    Setmaxnreg,
    Setp,
    Shf,
    Shfl,
    Shl,
    Shr,
    Sin,
    Slct,
    Sqrt,
    Stackrestore,
    Stacksave,
    St,
    Stmatrix,
    Sub,
    Subc,
    Suq,
    Suld,
    Sured,
    Sust,
    Szext,
    Tanh,
    Tcgen05,
    Tensormap,
    Tex,
    Testp,
    Tld4,
    Trap,
    Txq,
    Vabsdiff,
    Vabsdiff2,
    Vabsdiff4,
    Vadd,
    Vadd2,
    Vadd4,
    Vavrg2,
    Vavrg4,
    Vmad,
    Vmax,
    Vmax2,
    Vmax4,
    Vmin,
    Vmin2,
    Vmin4,
    Vset,
    Vset2,
    Vset4,
    Vshl,
    Vshr,
    Vsub,
    Vsub2,
    Vsub4,
    Vote,
    Wgmma,
    Wmma,
    Xor,
    Ret,
}

/// Recognised modifiers encountered on instructions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifierKind {
    Type(TypeModifier),
    Condition(ConditionModifier),
    AddressSpace(StateSpaceModifier),
    Conversion(StateSpaceModifier),
    Rounding(RoundingModifier),
    VectorWidth(u32),
    MathMode(MathModeModifier),
    Synchronization(SynchronizationModifier),
    AsyncGroup(AsyncGroupModifier),
    Shuffle(ShuffleModifier),
    Cache(CacheModifier),
    Scope(MemoryScopeModifier),
    Atomic(AtomicOperationModifier),
    Call(CallModifier),
    MemoryOrder(MemoryOrderModifier),
    Wide,
}

/// Operand that appears in a PTX instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Register(String),
    Immediate(String),
    Symbol(String),
    Memory(MemoryOperand),
    CallTarget {
        name: String,
        arguments: Vec<String>,
    },
    Parenthesized(Vec<String>),
}

/// Recognised type modifier tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeModifier {
    F16,
    F32,
    F64,
    F128,
    B8,
    B16,
    B32,
    B64,
    S8,
    S16,
    S32,
    S64,
    U8,
    U16,
    U32,
    U64,
    Pred,
}

/// Recognised condition codes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionModifier {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Lo,
    Hi,
    Ls,
    Hs,
}

/// Structured representation of a memory operand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryOperand {
    pub base: Option<AddressBase>,
    pub displacements: Vec<AddressDisplacement>,
}

/// Base location referenced by an address expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressBase {
    Register(String),
    Symbol(String),
}

/// Additional components that adjust an address expression relative to its base.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressDisplacement {
    pub sign: AddressSign,
    pub kind: AddressDisplacementKind,
}

/// Sign attached to a displacement term.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressSign {
    Positive,
    Negative,
}

/// Specific adjustment applied within a displacement term.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressDisplacementKind {
    Register {
        register: String,
        scale: Option<String>,
    },
    Symbol(String),
    Immediate(String),
}

/// Recognised address spaces for instruction modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSpaceModifier {
    Param,
    Global,
    Local,
    Shared,
    Const,
    Generic,
}

/// Rounding modes applied to arithmetic instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingModifier {
    Rn,
    Rz,
    Rm,
    Rp,
}

/// Precision modes for transcendental math instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathModeModifier {
    Approx,
    Full,
}

/// Synchronisation style required by the instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynchronizationModifier {
    Sync,
    Async,
}

/// Pipeline group modifiers for asynchronous copy instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncGroupModifier {
    CommitGroup,
    WaitGroup,
}

/// Shuffle operation modes for warp shuffle instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShuffleModifier {
    Bfly,
    Down,
    Up,
    Idx,
}

/// Cache modifiers for memory operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheModifier {
    Nc,
    Ca,
    Cg,
    Cs,
    Lu,
}

/// Memory scope specifiers for barriers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryScopeModifier {
    Cta,
    Gl,
    Gpu,
    Sys,
}

/// Atomic operation selectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomicOperationModifier {
    Cas,
    Add,
    Inc,
    Dec,
    Exch,
    Min,
    Max,
    And,
    Or,
    Xor,
}

/// Call instruction qualifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallModifier {
    Uni,
}

/// Memory ordering qualifiers for atomics or reductions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryOrderModifier {
    Relaxed,
    Acquire,
    Release,
    AcqRel,
    Sc,
}
