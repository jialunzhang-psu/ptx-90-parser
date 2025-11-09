//! PTX type definitions - Abstract Syntax Tree (AST) nodes.
//!
//! All types are re-exported at the top level for easy access:
//! - `ptx_parser::r#type::Module` - Root module type
//! - `ptx_parser::r#type::Instruction` - Instruction with label/predicate
//! - `ptx_parser::r#type::Operand` - Operand types
//! - etc.
//!
//! Instruction variants are under a separate namespace:
//! - `ptx_parser::r#type::instruction::Inst` - Enum of all instruction variants
//! - Individual instruction modules under `ptx_parser::r#type::instruction::*`

// Internal modules - accessible within crate but not publicly exposed
// Types are re-exported at top level for public API
pub(crate) mod common;
pub(crate) mod function;
pub(crate) mod module;
pub(crate) mod variable;

// Only instruction module is public
pub mod instruction;

// Re-export all common types at the top level (explicit list)
pub use common::{
    AddressBase, AddressOffset, AddressOperand, AddressSpace, AttributeDirective, Axis,
    CodeLinkage, CodeOrDataLinkage, DataLinkage, DataType, FunctionSymbol, GeneralOperand,
    Immediate, Instruction, Label, Operand, Predicate, PredicateRegister, RegisterOperand, Sign,
    SpecialRegister, TexHandler2, TexHandler3, TexHandler3Optional, TexType, VariableSymbol,
    VectorOperand,
};

// Re-export module types
pub use module::{
    AddressSizeDirective, FileDirective, LinkingDirective, Module, ModuleDebugDirective,
    ModuleDirective, ModuleInfoDirectiveKind, SectionDirective, TargetDirective, VersionDirective,
};

// Re-export function types
pub use function::{
    DwarfDirective, EntryFunction, FuncFunction, FunctionAlias, FunctionBody, FunctionDim3,
    FunctionHeaderDirective, FunctionKernelDirective, FunctionStatement, LocationDirective,
    PragmaDirective, RegisterDirective, StatementDirective, StatementSectionDirective,
};

// Re-export variable types
pub use variable::{
    GlobalInitializer, InitializerValue, ModuleVariableDirective, NumericLiteral,
    VariableDirective, VariableModifier,
};
