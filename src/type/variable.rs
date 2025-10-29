use crate::r#type::common::{AddressSpace, AttributeDirective, DataLinkage, DataType};

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
    pub address_space: Option<AddressSpace>,
    pub attributes: Vec<AttributeDirective>,
    pub ty: Option<DataType>,
    pub modifiers: Vec<VariableModifier>,
    pub name: String,
    pub array: Vec<Option<u64>>,
    pub initializer: Option<GlobalInitializer>,
    pub raw: String,
}

/// Qualifiers left on module variable declarations (e.g. `.v4`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableModifier {
    Vector(u32),
    Alignment(u32),
    Linkage(DataLinkage),
    Ptr,
}

//// Numeric literal kinds allowed inside initialisers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericLiteral {
    Signed(i64),
    Unsigned(u64),
    Float64(u64),
    Float32(u32),
}

// Values that can appear in global initialiser lists.
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
