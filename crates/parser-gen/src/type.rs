//! Abstract syntax tree for PTX syntax specifications.
//!
//! The AST models both instruction rules and auxiliary parameter rules that
//! appear in `docs/ptx_syntax`. It captures modifiers, operands, grouping,
//! and punctuation so downstream generators can reason about optional pieces
//! and structural choices that are encoded with braces, brackets, pipes, etc.

//! Important assumptions
//! - {.x} means optional modifier
//! - d{|p} means Pipe choice between d or p, and p is optional
//! - {!}c means prefix operator ! applied to c, ! is always optional
//! - {-}c means prefix operator - applied to c, - is always optional
//! - Parameter Rules should not recurse

/// Root node for a parsed syntax specification file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    /// Ordered list of rules as they appear in the source file.
    pub rules: Vec<Rule>,
}

/// Top-level rule that can either be an instruction syntax or a parameter set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    Instruction(InstructionRule),
    Parameter(ParameterRule),
}

/// Represents a single instruction form (one line terminated by a semicolon).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionRule {
    /// Opcode plus chained modifiers in the instruction head.
    pub head: InstructionHead,
    /// Ordered sequence of operands, separators, and optional groups.
    pub operands: Vec<Operand>,
    /// Original raw specification string
    pub raw: String,
}

impl Default for InstructionRule {
    fn default() -> Self {
        InstructionRule {
            head: InstructionHead {
                opcode: String::new(),
                modifiers: Vec::new(),
            },
            operands: Vec::new(),
            raw: String::new(),
        }
    }
}

/// Parameter definition of the form `.name = { choices };`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterRule {
    pub name: IdentifierToken,
    pub choices: Vec<Modifier>,
}

/// Instruction head: base opcode and its modifier chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionHead {
    pub opcode: IdentifierToken,
    pub modifiers: Vec<Modifier>,
}

/// Modifiers that decorate opcodes or parameter choices.
///
/// The recursive variants allow modeling constructs like
/// `.shared{::cta, ::cluster}` and `{.sem}{.scope}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modifier {
    /// Literal modifier segment (e.g. `.f32`, `::cache_hint`, `16`).
    Atom(IdentifierToken),
    /// Ordered sequence of identifier tokens (e.g., `::buffer`, `::op`).
    Sequence(Vec<IdentifierToken>),
    /// Optional modifier wrapped in `{ ... }`. Always contains a simple identifier.
    Optional(IdentifierToken),
}

/// Operand or separator node inside an instruction body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operand {
    pub operator: Option<OperatorToken>,
    pub operand: OperandElement,
    pub modifier: Option<Modifier>,
}

/// Underlying operand element (identifier or nested group).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperandElement {
    /// a::b, ::n, etc.
    Item(IdentifierToken),
    /// {xdim, ydim, zdim, ...}
    CurlyGroup(Vec<Operand>),
    /// tex-related
    SquareGroup(Vec<Operand>),
    /// (param-list)
    ParamList,
    /// (ret-param) - Always contains a simple identifier.
    ParenthesizedOperand(IdentifierToken),
    /// a|p
    PipeChoice((IdentifierToken, IdentifierToken)),
    /// a{|p}
    PipeOptionalChoice((IdentifierToken, IdentifierToken)),
    /// [a] - Always contains a simple identifier.
    Address(IdentifierToken),
    /// {, cache-policy} - Always contains a simple identifier.
    Optional(IdentifierToken),
    /// Immediate number
    ImmediateNumber(String),
}

/// Prefix operators that can appear before operands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorToken {
    Negate,
    LogicalNot,
}

pub type IdentifierToken = String;

/// Supported unary operator kinds (e.g. `!a`, `-a`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefixOperator {
    LogicalNot,
    Negate,
}
