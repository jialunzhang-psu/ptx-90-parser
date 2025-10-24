use std::fmt;

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

impl ParameterQualifiers {
    pub fn is_empty(&self) -> bool {
        !self.is_const
            && !self.is_volatile
            && !self.is_restrict
            && !self.is_noalias
            && self.pointer.is_none()
    }
}

/// Raw specifier token captured while parsing a parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterSpecifier(pub String);

impl ParameterSpecifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

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
    Local(GenericFunctionDeclaration),
    Param(GenericFunctionDeclaration),
    Shared(GenericFunctionDeclaration),
    Pragma(PragmaDirective),
    Loc(LocationDirective),
    Dwarf(DwarfDirective),
}

/// Executable items that appear within a function body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionStatement {
    Label(String),
    Directive(StatementDirective),
    Instruction(Instruction),
}

/// Recognised declaration directive kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionDeclarationKind {
    AbiPreserve,
    AbiPreserveControl,
    Align,
    Attribute,
    CallTargets,
    CallPrototype,
    Local,
    Maxnreg,
    Maxsmem,
    Noreturn,
    Param,
    Pragma,
    Reg,
    Section,
    Shared,
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

/// Generic fallback for function declaration directives without dedicated parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericFunctionDeclaration {
    pub kind: FunctionDeclarationKind,
    pub keyword: String,
    pub arguments: Vec<String>,
    pub comment: Option<String>,
    pub raw: String,
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

impl AddressSign {
    pub fn negate(self) -> Self {
        match self {
            AddressSign::Positive => AddressSign::Negative,
            AddressSign::Negative => AddressSign::Positive,
        }
    }
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

impl VariableQualifier {
    pub fn width(&self) -> u32 {
        match self {
            VariableQualifier::Vector(width) => *width,
            VariableQualifier::Sampler => 1,
        }
    }
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

impl fmt::Display for StateSpaceModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateSpaceModifier::Param => write!(f, "param"),
            StateSpaceModifier::Global => write!(f, "global"),
            StateSpaceModifier::Local => write!(f, "local"),
            StateSpaceModifier::Shared => write!(f, "shared"),
            StateSpaceModifier::Const => write!(f, "const"),
            StateSpaceModifier::Generic => write!(f, "generic"),
        }
    }
}

impl fmt::Display for OpcodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpcodeKind::Abs => write!(f, "abs"),
            OpcodeKind::Activemask => write!(f, "activemask"),
            OpcodeKind::Add => write!(f, "add"),
            OpcodeKind::Addc => write!(f, "addc"),
            OpcodeKind::Alloca => write!(f, "alloca"),
            OpcodeKind::And => write!(f, "and"),
            OpcodeKind::Applypriority => write!(f, "applypriority"),
            OpcodeKind::Atom => write!(f, "atom"),
            OpcodeKind::Bar => write!(f, "bar"),
            OpcodeKind::Barrier => write!(f, "barrier"),
            OpcodeKind::Bfe => write!(f, "bfe"),
            OpcodeKind::Bfi => write!(f, "bfi"),
            OpcodeKind::Bfind => write!(f, "bfind"),
            OpcodeKind::Bmsk => write!(f, "bmsk"),
            OpcodeKind::Brev => write!(f, "brev"),
            OpcodeKind::Bra => write!(f, "bra"),
            OpcodeKind::Brkpt => write!(f, "brkpt"),
            OpcodeKind::Brx => write!(f, "brx"),
            OpcodeKind::Call => write!(f, "call"),
            OpcodeKind::Clz => write!(f, "clz"),
            OpcodeKind::Clusterlaunchcontrol => write!(f, "clusterlaunchcontrol"),
            OpcodeKind::Cnot => write!(f, "cnot"),
            OpcodeKind::Copysign => write!(f, "copysign"),
            OpcodeKind::Cos => write!(f, "cos"),
            OpcodeKind::Cp => write!(f, "cp"),
            OpcodeKind::Createpolicy => write!(f, "createpolicy"),
            OpcodeKind::Cvt => write!(f, "cvt"),
            OpcodeKind::Cvta => write!(f, "cvta"),
            OpcodeKind::Div => write!(f, "div"),
            OpcodeKind::Discard => write!(f, "discard"),
            OpcodeKind::Dp2a => write!(f, "dp2a"),
            OpcodeKind::Dp4a => write!(f, "dp4a"),
            OpcodeKind::Elect => write!(f, "elect"),
            OpcodeKind::Ex2 => write!(f, "ex2"),
            OpcodeKind::Exit => write!(f, "exit"),
            OpcodeKind::Fence => write!(f, "fence"),
            OpcodeKind::Fma => write!(f, "fma"),
            OpcodeKind::Fns => write!(f, "fns"),
            OpcodeKind::Getctarank => write!(f, "getctarank"),
            OpcodeKind::Griddepcontrol => write!(f, "griddepcontrol"),
            OpcodeKind::Isspacep => write!(f, "isspacep"),
            OpcodeKind::Istypep => write!(f, "istypep"),
            OpcodeKind::Ld => write!(f, "ld"),
            OpcodeKind::Ldmatrix => write!(f, "ldmatrix"),
            OpcodeKind::Ldu => write!(f, "ldu"),
            OpcodeKind::Lg2 => write!(f, "lg2"),
            OpcodeKind::Lop3 => write!(f, "lop3"),
            OpcodeKind::Mad => write!(f, "mad"),
            OpcodeKind::Mad24 => write!(f, "mad24"),
            OpcodeKind::Madc => write!(f, "madc"),
            OpcodeKind::Mapa => write!(f, "mapa"),
            OpcodeKind::Match => write!(f, "match"),
            OpcodeKind::Max => write!(f, "max"),
            OpcodeKind::Mbarrier => write!(f, "mbarrier"),
            OpcodeKind::Membar => write!(f, "membar"),
            OpcodeKind::Min => write!(f, "min"),
            OpcodeKind::Mov => write!(f, "mov"),
            OpcodeKind::Movmatrix => write!(f, "movmatrix"),
            OpcodeKind::Mma => write!(f, "mma"),
            OpcodeKind::Mul => write!(f, "mul"),
            OpcodeKind::Mul24 => write!(f, "mul24"),
            OpcodeKind::Multimem => write!(f, "multimem"),
            OpcodeKind::Nanosleep => write!(f, "nanosleep"),
            OpcodeKind::Neg => write!(f, "neg"),
            OpcodeKind::Not => write!(f, "not"),
            OpcodeKind::Or => write!(f, "or"),
            OpcodeKind::Pmevent => write!(f, "pmevent"),
            OpcodeKind::Popc => write!(f, "popc"),
            OpcodeKind::Prefetch => write!(f, "prefetch"),
            OpcodeKind::Prefetchu => write!(f, "prefetchu"),
            OpcodeKind::Prmt => write!(f, "prmt"),
            OpcodeKind::Rcp => write!(f, "rcp"),
            OpcodeKind::Red => write!(f, "red"),
            OpcodeKind::Redux => write!(f, "redux"),
            OpcodeKind::Rem => write!(f, "rem"),
            OpcodeKind::Rsqrt => write!(f, "rsqrt"),
            OpcodeKind::Sad => write!(f, "sad"),
            OpcodeKind::Selp => write!(f, "selp"),
            OpcodeKind::Set => write!(f, "set"),
            OpcodeKind::Setmaxnreg => write!(f, "setmaxnreg"),
            OpcodeKind::Setp => write!(f, "setp"),
            OpcodeKind::Shf => write!(f, "shf"),
            OpcodeKind::Shfl => write!(f, "shfl"),
            OpcodeKind::Shl => write!(f, "shl"),
            OpcodeKind::Shr => write!(f, "shr"),
            OpcodeKind::Sin => write!(f, "sin"),
            OpcodeKind::Slct => write!(f, "slct"),
            OpcodeKind::Sqrt => write!(f, "sqrt"),
            OpcodeKind::Stackrestore => write!(f, "stackrestore"),
            OpcodeKind::Stacksave => write!(f, "stacksave"),
            OpcodeKind::St => write!(f, "st"),
            OpcodeKind::Stmatrix => write!(f, "stmatrix"),
            OpcodeKind::Sub => write!(f, "sub"),
            OpcodeKind::Subc => write!(f, "subc"),
            OpcodeKind::Suq => write!(f, "suq"),
            OpcodeKind::Suld => write!(f, "suld"),
            OpcodeKind::Sured => write!(f, "sured"),
            OpcodeKind::Sust => write!(f, "sust"),
            OpcodeKind::Szext => write!(f, "szext"),
            OpcodeKind::Tanh => write!(f, "tanh"),
            OpcodeKind::Tcgen05 => write!(f, "tcgen05"),
            OpcodeKind::Tensormap => write!(f, "tensormap"),
            OpcodeKind::Tex => write!(f, "tex"),
            OpcodeKind::Testp => write!(f, "testp"),
            OpcodeKind::Tld4 => write!(f, "tld4"),
            OpcodeKind::Trap => write!(f, "trap"),
            OpcodeKind::Txq => write!(f, "txq"),
            OpcodeKind::Vabsdiff => write!(f, "vabsdiff"),
            OpcodeKind::Vabsdiff2 => write!(f, "vabsdiff2"),
            OpcodeKind::Vabsdiff4 => write!(f, "vabsdiff4"),
            OpcodeKind::Vadd => write!(f, "vadd"),
            OpcodeKind::Vadd2 => write!(f, "vadd2"),
            OpcodeKind::Vadd4 => write!(f, "vadd4"),
            OpcodeKind::Vavrg2 => write!(f, "vavrg2"),
            OpcodeKind::Vavrg4 => write!(f, "vavrg4"),
            OpcodeKind::Vmad => write!(f, "vmad"),
            OpcodeKind::Vmax => write!(f, "vmax"),
            OpcodeKind::Vmax2 => write!(f, "vmax2"),
            OpcodeKind::Vmax4 => write!(f, "vmax4"),
            OpcodeKind::Vmin => write!(f, "vmin"),
            OpcodeKind::Vmin2 => write!(f, "vmin2"),
            OpcodeKind::Vmin4 => write!(f, "vmin4"),
            OpcodeKind::Vset => write!(f, "vset"),
            OpcodeKind::Vset2 => write!(f, "vset2"),
            OpcodeKind::Vset4 => write!(f, "vset4"),
            OpcodeKind::Vshl => write!(f, "vshl"),
            OpcodeKind::Vshr => write!(f, "vshr"),
            OpcodeKind::Vsub => write!(f, "vsub"),
            OpcodeKind::Vsub2 => write!(f, "vsub2"),
            OpcodeKind::Vsub4 => write!(f, "vsub4"),
            OpcodeKind::Vote => write!(f, "vote"),
            OpcodeKind::Wgmma => write!(f, "wgmma"),
            OpcodeKind::Wmma => write!(f, "wmma"),
            OpcodeKind::Xor => write!(f, "xor"),
            OpcodeKind::Ret => write!(f, "ret"),
        }
    }
}

impl fmt::Display for InstructionOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl fmt::Display for TypeModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeModifier::F16 => write!(f, "f16"),
            TypeModifier::F32 => write!(f, "f32"),
            TypeModifier::F64 => write!(f, "f64"),
            TypeModifier::F128 => write!(f, "f128"),
            TypeModifier::B8 => write!(f, "b8"),
            TypeModifier::B16 => write!(f, "b16"),
            TypeModifier::B32 => write!(f, "b32"),
            TypeModifier::B64 => write!(f, "b64"),
            TypeModifier::S8 => write!(f, "s8"),
            TypeModifier::S16 => write!(f, "s16"),
            TypeModifier::S32 => write!(f, "s32"),
            TypeModifier::S64 => write!(f, "s64"),
            TypeModifier::U8 => write!(f, "u8"),
            TypeModifier::U16 => write!(f, "u16"),
            TypeModifier::U32 => write!(f, "u32"),
            TypeModifier::U64 => write!(f, "u64"),
            TypeModifier::Pred => write!(f, "pred"),
        }
    }
}

impl fmt::Display for ConditionModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConditionModifier::Eq => write!(f, "eq"),
            ConditionModifier::Ne => write!(f, "ne"),
            ConditionModifier::Lt => write!(f, "lt"),
            ConditionModifier::Le => write!(f, "le"),
            ConditionModifier::Gt => write!(f, "gt"),
            ConditionModifier::Ge => write!(f, "ge"),
            ConditionModifier::Lo => write!(f, "lo"),
            ConditionModifier::Hi => write!(f, "hi"),
            ConditionModifier::Ls => write!(f, "ls"),
            ConditionModifier::Hs => write!(f, "hs"),
        }
    }
}

impl fmt::Display for DataDirectiveKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataDirectiveKind::B8 => write!(f, "b8"),
            DataDirectiveKind::B16 => write!(f, "b16"),
            DataDirectiveKind::B32 => write!(f, "b32"),
            DataDirectiveKind::B64 => write!(f, "b64"),
        }
    }
}

impl fmt::Display for FunctionVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionVisibility::Visible => write!(f, "visible"),
            FunctionVisibility::Hidden => write!(f, "hidden"),
        }
    }
}

impl fmt::Display for FunctionLinkage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionLinkage::Extern => write!(f, "extern"),
            FunctionLinkage::Weak => write!(f, "weak"),
            FunctionLinkage::WeakExtern => write!(f, "weak_extern"),
        }
    }
}

impl fmt::Display for PointerAddressSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PointerAddressSpace::Generic => write!(f, "generic"),
            PointerAddressSpace::Global => write!(f, "global"),
            PointerAddressSpace::Shared => write!(f, "shared"),
            PointerAddressSpace::Local => write!(f, "local"),
            PointerAddressSpace::Const => write!(f, "const"),
        }
    }
}

impl fmt::Display for RoundingModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoundingModifier::Rn => write!(f, "rn"),
            RoundingModifier::Rz => write!(f, "rz"),
            RoundingModifier::Rm => write!(f, "rm"),
            RoundingModifier::Rp => write!(f, "rp"),
        }
    }
}

impl fmt::Display for GlobalVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlobalVisibility::Visible => write!(f, "visible"),
            GlobalVisibility::Hidden => write!(f, "hidden"),
        }
    }
}

impl fmt::Display for GlobalLinkage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlobalLinkage::Extern => write!(f, "extern"),
            GlobalLinkage::Weak => write!(f, "weak"),
            GlobalLinkage::WeakExtern => write!(f, "weak_extern"),
        }
    }
}

impl fmt::Display for GlobalAddressSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlobalAddressSpace::Global => write!(f, "global"),
            GlobalAddressSpace::Const => write!(f, "const"),
            GlobalAddressSpace::Shared => write!(f, "shared"),
            GlobalAddressSpace::Local => write!(f, "local"),
        }
    }
}

impl fmt::Display for GlobalMutability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlobalMutability::Const => write!(f, "const"),
            GlobalMutability::Volatile => write!(f, "volatile"),
        }
    }
}

impl fmt::Display for FunctionDeclarationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            FunctionDeclarationKind::AbiPreserve => "abi_preserve",
            FunctionDeclarationKind::AbiPreserveControl => "abi_preserve_control",
            FunctionDeclarationKind::Align => "align",
            FunctionDeclarationKind::Attribute => "attribute",
            FunctionDeclarationKind::CallTargets => "calltargets",
            FunctionDeclarationKind::CallPrototype => "callprototype",
            FunctionDeclarationKind::Local => "local",
            FunctionDeclarationKind::Maxnreg => "maxnreg",
            FunctionDeclarationKind::Maxsmem => "maxsmem",
            FunctionDeclarationKind::Noreturn => "noreturn",
            FunctionDeclarationKind::Param => "param",
            FunctionDeclarationKind::Pragma => "pragma",
            FunctionDeclarationKind::Reg => "reg",
            FunctionDeclarationKind::Section => "section",
            FunctionDeclarationKind::Shared => "shared",
        };
        write!(f, "{}", label)
    }
}

impl fmt::Display for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalarType::B8 => write!(f, "b8"),
            ScalarType::B16 => write!(f, "b16"),
            ScalarType::B32 => write!(f, "b32"),
            ScalarType::B64 => write!(f, "b64"),
            ScalarType::S8 => write!(f, "s8"),
            ScalarType::S16 => write!(f, "s16"),
            ScalarType::S32 => write!(f, "s32"),
            ScalarType::S64 => write!(f, "s64"),
            ScalarType::U8 => write!(f, "u8"),
            ScalarType::U16 => write!(f, "u16"),
            ScalarType::U32 => write!(f, "u32"),
            ScalarType::U64 => write!(f, "u64"),
            ScalarType::F16 => write!(f, "f16"),
            ScalarType::F32 => write!(f, "f32"),
            ScalarType::F64 => write!(f, "f64"),
            ScalarType::Pred => write!(f, "pred"),
            ScalarType::TexRef => write!(f, "texref"),
            ScalarType::SamplerRef => write!(f, "samplerref"),
            ScalarType::SurfRef => write!(f, "surfref"),
        }
    }
}

impl fmt::Display for MathModeModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathModeModifier::Approx => write!(f, "approx"),
            MathModeModifier::Full => write!(f, "full"),
        }
    }
}

impl fmt::Display for SynchronizationModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SynchronizationModifier::Sync => write!(f, "sync"),
            SynchronizationModifier::Async => write!(f, "async"),
        }
    }
}

impl fmt::Display for AsyncGroupModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsyncGroupModifier::CommitGroup => write!(f, "commit_group"),
            AsyncGroupModifier::WaitGroup => write!(f, "wait_group"),
        }
    }
}

impl fmt::Display for ShuffleModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShuffleModifier::Bfly => write!(f, "bfly"),
            ShuffleModifier::Down => write!(f, "down"),
            ShuffleModifier::Up => write!(f, "up"),
            ShuffleModifier::Idx => write!(f, "idx"),
        }
    }
}

impl fmt::Display for CacheModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheModifier::Nc => write!(f, "nc"),
            CacheModifier::Ca => write!(f, "ca"),
            CacheModifier::Cg => write!(f, "cg"),
            CacheModifier::Cs => write!(f, "cs"),
            CacheModifier::Lu => write!(f, "lu"),
        }
    }
}

impl fmt::Display for MemoryScopeModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryScopeModifier::Cta => write!(f, "cta"),
            MemoryScopeModifier::Gl => write!(f, "gl"),
            MemoryScopeModifier::Gpu => write!(f, "gpu"),
            MemoryScopeModifier::Sys => write!(f, "sys"),
        }
    }
}

impl fmt::Display for AtomicOperationModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomicOperationModifier::Cas => write!(f, "cas"),
            AtomicOperationModifier::Add => write!(f, "add"),
            AtomicOperationModifier::Inc => write!(f, "inc"),
            AtomicOperationModifier::Dec => write!(f, "dec"),
            AtomicOperationModifier::Exch => write!(f, "exch"),
            AtomicOperationModifier::Min => write!(f, "min"),
            AtomicOperationModifier::Max => write!(f, "max"),
            AtomicOperationModifier::And => write!(f, "and"),
            AtomicOperationModifier::Or => write!(f, "or"),
            AtomicOperationModifier::Xor => write!(f, "xor"),
        }
    }
}

impl fmt::Display for CallModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallModifier::Uni => write!(f, "uni"),
        }
    }
}

impl fmt::Display for MemoryOrderModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryOrderModifier::Relaxed => write!(f, "relaxed"),
            MemoryOrderModifier::Acquire => write!(f, "acquire"),
            MemoryOrderModifier::Release => write!(f, "release"),
            MemoryOrderModifier::AcqRel => write!(f, "acq_rel"),
            MemoryOrderModifier::Sc => write!(f, "sc"),
        }
    }
}

impl FunctionHeaderDirective {
    pub fn name(&self) -> &'static str {
        match self {
            FunctionHeaderDirective::Visibility(_) => "visibility",
            FunctionHeaderDirective::Linkage(_) => "linkage",
            FunctionHeaderDirective::NoReturn => "noreturn",
            FunctionHeaderDirective::AbiPreserve(_) => "abi_preserve",
            FunctionHeaderDirective::AbiPreserveControl(_) => "abi_preserve_control",
            FunctionHeaderDirective::MaxClusterRank(_) => "maxclusterrank",
            FunctionHeaderDirective::BlocksAreClusters => "blocksareclusters",
            FunctionHeaderDirective::ExplicitCluster(_) => "explicitcluster",
            FunctionHeaderDirective::ReqNctaPerCluster(_) => "reqnctapercluster",
            FunctionHeaderDirective::MaxNReg(_) => "maxnreg",
            FunctionHeaderDirective::MaxNTid(_) => "maxntid",
            FunctionHeaderDirective::MinNCtaPerSm(_) => "minnctapersm",
            FunctionHeaderDirective::ReqNTid(_) => "reqntid",
            FunctionHeaderDirective::MaxNCtaPerSm(_) => "maxnctapersm",
            FunctionHeaderDirective::Pragma(_) => "pragma",
        }
    }
}
