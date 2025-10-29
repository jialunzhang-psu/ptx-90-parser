use crate::r#type::common::*;

/// Syntax variants of the PTX `mov` instruction family.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mov {
    /// `mov.type  d, a;`
    Register(Register),
    /// `mov.type  d, sreg;`
    SpecialRegister(SpecialRegister),
    /// `mov.type  d, avar; // get address of variable`
    Variable(Variable),
    /// `mov.type  d, avar+imm; // get address of variable with offset`
    VariableWithImmediate(VariableWithImmediate),
    /// `mov.u32   d, fname; // get address of device function`
    /// `mov.u64   d, fname; // get address of device function`
    FunctionAddress(FunctionAddress),
    /// `mov.u32   d, kernel; // get address of entry function`
    /// `mov.u64   d, kernel; // get address of entry function`
    KernelAddress(KernelAddress),
}

/// `mov.type  d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: Destination,
    /// `a`
    pub source: RegisterSource,
}

/// `mov.type  d, sreg;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialRegister {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: Destination,
    /// `sreg`
    pub source: SpecialRegisterSource,
}

/// `mov.type  d, avar; // get address of variable`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `avar`
    pub variable: VariableSymbol,
}

/// `mov.type  d, avar+imm; // get address of variable with offset`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableWithImmediate {
    /// `.type`
    pub data_type: DataType,
    /// `d`
    pub destination: RegisterOperand,
    /// `avar`
    pub variable: VariableSymbol,
    /// `imm`
    pub immediate: Immediate,
}

/// `mov.u32   d, fname; // get address of device function`
/// `mov.u64   d, fname; // get address of device function`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionAddress {
    /// `.u32` / `.u64`
    pub data_type: AddressType,
    /// `d`
    pub destination: RegisterOperand,
    /// `fname`
    pub function: FunctionSymbol,
}

/// `mov.u32   d, kernel; // get address of entry function`
/// `mov.u64   d, kernel; // get address of entry function`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelAddress {
    /// `.u32` / `.u64`
    pub data_type: AddressType,
    /// `d`
    pub destination: RegisterOperand,
    /// `kernel`
    pub kernel: FunctionSymbol,
}

/// `.type = { .pred, .b16, .b32, .b64, .u16, .u32, .u64, .s16, .s32, .s64, .f32, .f64 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.pred`
    Pred,
    /// `.b16`
    B16,
    /// `.b32`
    B32,
    /// `.b64`
    B64,
    /// `.u16`
    U16,
    /// `.u32`
    U32,
    /// `.u64`
    U64,
    /// `.s16`
    S16,
    /// `.s32`
    S32,
    /// `.s64`
    S64,
    /// `.f32`
    F32,
    /// `.f64`
    F64,
}

/// `.type = { .u32, .u64 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    /// `.u32`
    U32,
    /// `.u64`
    U64,
}

/// `d`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Destination {
    /// `d`
    Register(RegisterOperand),
    /// `d` when `.pred`
    Predicate(PredicateRegister),
}

/// `a`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterSource {
    /// `a`
    Register(RegisterOperand),
    /// `a` when `.pred`
    Predicate(PredicateRegister),
    /// `imm`
    Immediate(Immediate),
}

/// `sreg`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialRegisterSource {
    /// `sreg`
    Register(crate::r#type::common::SpecialRegister),
    /// `sreg` when `.pred`
    Predicate(PredicateRegister),
}
