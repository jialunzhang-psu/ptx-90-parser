use crate::Spanned;
use crate::parser::Span;
use crate::r#type::common::{AttributeDirective, DataType};
use crate::r#type::{FunctionSymbol, Immediate, VariableSymbol};

/// Module-level declarations that reserve storage in a specific address space.
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum ModuleVariableDirective {
    /// Deprecated `.tex` variable declaration.
    ///
    /// Example:
    /// .tex .u32 tex_a; // is equivalent to .global .texref tex_a;
    Tex {
        directive: VariableDirective,
        span: Span,
    },
    Shared {
        directive: VariableDirective,
        span: Span,
    },
    Global {
        directive: VariableDirective,
        span: Span,
    },
    Const {
        directive: VariableDirective,
        span: Span,
    },
}

/// Module-scoped variable declaration shared by `.tex`, `.shared`, `.global`, and `.const`.
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub struct VariableDirective {
    /// Example:
    /// .global .attribute(.managed) .s32 g;
    /// .global .attribute(.managed) .u64 x;
    /// .global .attribute(.unified(19,95)) .f32 f;
    pub attributes: Vec<AttributeDirective>,
    /// The data type of the variable (e.g., `.s32`, `.f64`).
    pub ty: DataType,
    /// Modifiers applied to the variable (e.g., `.v4`, `.align 16`, `.ptr`).
    pub modifiers: Vec<VariableModifier>,
    /// The variable name.
    pub name: VariableSymbol,
    /// The array dimensions, if any.
    /// Example:
    /// .global .s32 offset[][2] = { {-1, 0}, {0, -1}, {1, 0}, {0, 1} };
    pub array_dims: Vec<Option<u64>>,
    /// Optional global initializer for the variable.
    pub initializer: Option<GlobalInitializer>,
    pub span: Span,
}

/// Qualifiers left on module variable declarations (e.g. `.v4`).
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum VariableModifier {
    Vector { value: u32, span: Span },
    Alignment { value: u32, span: Span },
    Ptr { span: Span },
}

/// Parameters, used in function declarations and calls, e.g., `.param .b32 p`.
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum ParameterDirective {
    Register {
        ty: DataType,
        name: VariableSymbol,
        span: Span,
    },
    Parameter {
        align: Option<u32>,
        ty: DataType,
        ptr: bool,
        space: Option<ParamStateSpace>,
        name: VariableSymbol,
        array: Vec<Option<u64>>,
        span: Span,
    },
}

/// Address space qualifiers for parameters.
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum ParamStateSpace {
    /// .const
    Const { span: Span },
    /// .global
    Global { span: Span },
    /// .local
    Local { span: Span },
    /// .shared
    Shared { span: Span },
}

// Values that can appear in global initialiser lists.
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum InitializerValue {
    NumericLiteral { value: Immediate, span: Span },
    FunctionSymbol { name: FunctionSymbol, span: Span },
    StringLiteral { value: String, span: Span },
}

/// Structured representation of a global variable initialiser.
#[derive(Debug, Clone, PartialEq, Eq, Spanned)]
pub enum GlobalInitializer {
    Scalar {
        value: InitializerValue,
        span: Span,
    },
    Aggregate {
        values: Vec<GlobalInitializer>,
        span: Span,
    },
}
