/// Module-level declarations that reserve storage in a specific address space.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleVariableDirective {
    Tex(VariableDirective),
    Shared(VariableDirective),
    Global(VariableDirective),
    Const(VariableDirective),
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
