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
    AddressBase, AddressOffset, AddressOperand, AttributeDirective, Axis, CodeLinkage, DataLinkage,
    DataType, FunctionSymbol, GeneralOperand, Immediate, Instruction, Label, Operand, Predicate,
    PredicateRegister, RegisterOperand, Sign, SpecialRegister, TexHandler2, TexHandler3,
    TexHandler3Optional, VariableSymbol, VectorOperand,
};

// Re-export module types
pub use module::{
    AddressSize, AddressSizeDirective, FileDirective, Module, ModuleDebugDirective,
    ModuleDirective, ModuleInfoDirectiveKind, TargetDirective, TargetString, VersionDirective,
};

// Re-export function types
pub use function::{
    AliasFunctionDirective, BranchTargetsDirective, CallPrototypeDirective,
    CallPrototypeReturnSpec, CallTargetsDirective, DwarfDirective, DwarfDirectiveKind,
    EntryFunctionDirective, EntryFunctionHeaderDirective, FuncFunctionDirective,
    FuncFunctionHeaderDirective, FunctionBody, FunctionDim, FunctionStatement, LocationDirective,
    LocationInlinedAt, PragmaDirective, PragmaDirectiveKind, RegisterDirective, RegisterTarget,
    SectionDirective, SectionEntry, StatementDirective, StatementSectionDirectiveLine,
};

// Re-export variable types
pub use variable::{
    GlobalInitializer, InitializerValue, ModuleVariableDirective, ParamStateSpace,
    ParameterDirective, VariableDirective, VariableModifier,
};
