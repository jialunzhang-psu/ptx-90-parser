use crate::r#type::common::{AddressSpace, AttributeDirective, DataLinkage, DataType};
use crate::parser::Span;

/// Module-level declarations that reserve storage in a specific address space.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleVariableDirective {
    Tex { directive: VariableDirective, span: Span },
    Shared { directive: VariableDirective, span: Span },
    Global { directive: VariableDirective, span: Span },
    Const { directive: VariableDirective, span: Span },
}

impl ModuleVariableDirective {
    pub fn span(&self) -> Span {
        match self {
            ModuleVariableDirective::Tex { span, .. } => span.clone(),
            ModuleVariableDirective::Shared { span, .. } => span.clone(),
            ModuleVariableDirective::Global { span, .. } => span.clone(),
            ModuleVariableDirective::Const { span, .. } => span.clone(),
        }
    }
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
    pub span: Span,
}

impl VariableDirective {
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

/// Qualifiers left on module variable declarations (e.g. `.v4`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableModifier {
    Vector { value: u32, span: Span },
    Alignment { value: u32, span: Span },
    Linkage { linkage: DataLinkage, span: Span },
    Ptr { span: Span },
}

impl VariableModifier {
    pub fn span(&self) -> Span {
        match self {
            VariableModifier::Vector { span, .. } => span.clone(),
            VariableModifier::Alignment { span, .. } => span.clone(),
            VariableModifier::Linkage { span, .. } => span.clone(),
            VariableModifier::Ptr { span } => span.clone(),
        }
    }
}

//// Numeric literal kinds allowed inside initialisers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumericLiteral {
    Signed { value: i64, span: Span },
    Unsigned { value: u64, span: Span },
    Float64 { value: u64, span: Span },
    Float32 { value: u32, span: Span },
}

impl NumericLiteral {
    pub fn span(&self) -> Span {
        match self {
            NumericLiteral::Signed { span, .. } => span.clone(),
            NumericLiteral::Unsigned { span, .. } => span.clone(),
            NumericLiteral::Float64 { span, .. } => span.clone(),
            NumericLiteral::Float32 { span, .. } => span.clone(),
        }
    }
}

// Values that can appear in global initialiser lists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitializerValue {
    Numeric { value: NumericLiteral, span: Span },
    Symbol { name: String, span: Span },
    StringLiteral { value: String, span: Span },
}

impl InitializerValue {
    pub fn span(&self) -> Span {
        match self {
            InitializerValue::Numeric { span, .. } => span.clone(),
            InitializerValue::Symbol { span, .. } => span.clone(),
            InitializerValue::StringLiteral { span, .. } => span.clone(),
        }
    }
}

/// Structured representation of a global variable initialiser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlobalInitializer {
    Scalar { value: InitializerValue, span: Span },
    Aggregate { values: Vec<GlobalInitializer>, span: Span },
}

impl GlobalInitializer {
    pub fn span(&self) -> Span {
        match self {
            GlobalInitializer::Scalar { span, .. } => span.clone(),
            GlobalInitializer::Aggregate { span, .. } => span.clone(),
        }
    }
}
