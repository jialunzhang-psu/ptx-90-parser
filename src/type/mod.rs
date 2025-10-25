mod impls;

use thiserror::Error;

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
    Module(ModuleDirectiveKind),
    Debug(ModuleDebugDirective),
    Linking(LinkingDirective),
}

/// All directives that describe kernel/function entities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionKernelDirective {
    Entry(EntryFunction),
    Func(FuncFunction),
    Alias(FunctionAlias),
}

/// Module-level declarations that reserve storage in a specific address space.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleVariableDirective {
    Tex(VariableDirective),
    Shared(VariableDirective),
    Global(VariableDirective),
    Const(VariableDirective),
}

/// Directives that apply to the PTX module as a whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDirectiveKind {
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

/// Linking directives that influence symbol visibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkingDirective {
    pub kind: LinkingDirectiveKind,
    pub prototype: String,
    pub raw: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkingDirectiveKind {
    Extern,
    Visible,
    Weak,
    Common,
}

/// Cluster dimension directives used for cooperative kernels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClusterDirective {
    RequireCtasPerCluster(ClusterSizeDirective),
    ExplicitCluster(ClusterSizeDirective),
    MaxClusterRank { count: u32, raw: String },
}

/// Miscellaneous directives defined by the PTX ISA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiscDirective {
    BlocksAreClusters { raw: String },
}

/// Raw representation of the `.b8/.b16/...` module-level data directives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleDataDirective {
    pub kind: DataDirectiveKind,
    pub values: Vec<String>,
    pub raw: String,
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

/// Shared representation for cluster dimension directives that specify extents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClusterSizeDirective {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub raw: String,
}

/// Alias directive relating one function symbol to another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionAlias {
    pub alias: String,
    pub target: String,
    pub raw: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataDirectiveKind {
    B8,
    B16,
    B32,
    B64,
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
/// Module-scoped variable declaration shared by `.tex`, `.shared`, `.global`, and `.const`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDirective {
    pub visibility: Option<GlobalVisibility>,
    pub linkages: Vec<GlobalLinkage>,
    pub address_space: Option<GlobalAddressSpace>,
    pub mutability: Option<GlobalMutability>,
    pub alignment: Option<u32>,
    pub ty: Option<ScalarType>,
    pub qualifiers: Vec<VariableQualifier>,
    pub name: String,
    pub array: Option<ArraySpecifier>,
    pub initializer: Option<GlobalInitializer>,
    pub raw: String,
}

/// Qualifiers left on module variable declarations (e.g. `.v4`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableQualifier {
    Vector(u32),
    Sampler,
}

/// Visibility markers for global variables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalVisibility {
    Visible,
    Hidden,
}

/// Linkage specifiers for global variables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalLinkage {
    Extern,
    Weak,
    WeakExtern,
}

/// Memory spaces addressable by global declarations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalAddressSpace {
    Global,
    Const,
    Shared,
    Local,
    Param,
}

/// Mutability qualifiers applicable to globals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalMutability {
    Const,
    Volatile,
}

/// Scalar data types encountered in global declarations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarType {
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
    F16,
    F32,
    F64,
    Pred,
    TexRef,
    SamplerRef,
    SurfRef,
}

/// Optional array specification attached to a global variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArraySpecifier {
    pub dimensions: Vec<Option<u64>>,
}

/// Numeric literal kinds allowed inside initialisers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericLiteral {
    Signed(i64),
    Unsigned(u64),
    Float64(u64),
    Float32(u32),
}

/// Values that can appear in global initialiser lists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitializerValue {
    Numeric(NumericLiteral),
    Symbol(String),
    StringLiteral(String),
}

/// Structured representation of a global variable initialiser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlobalInitializer {
    Scalar(InitializerValue),
    Aggregate(Vec<GlobalInitializer>),
}

/// Errors that can occur while parsing PTX source text.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum PtxParseError {
    #[error("unexpected end of input while parsing {context} starting at line {line}")]
    UnexpectedEof { context: &'static str, line: usize },

    #[error("invalid directive at line {line}: {message}")]
    InvalidDirective { line: usize, message: String },

    #[error("invalid function header at line {line}: {message}")]
    InvalidFunctionHeader { line: usize, message: String },

    #[error("invalid instruction at line {line}: {message}")]
    InvalidInstruction { line: usize, message: String },

    #[error("invalid global declaration at line {line}: {message}")]
    InvalidGlobal { line: usize, message: String },
}
