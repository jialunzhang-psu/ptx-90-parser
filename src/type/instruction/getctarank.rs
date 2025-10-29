use crate::r#type::common::{Immediate, RegisterOperand, VariableSymbol};

/// `getctarank{.space}.type d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Getctarank {
    /// `getctarank.type d, a;`
    Generic {
        /// `.type`
        data_type: DataType,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        source: RegisterOperand,
    },
    /// `getctarank.shared::cluster.type d, a;`
    SharedRegister {
        /// `.type`
        data_type: DataType,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        source: RegisterOperand,
    },
    /// `getctarank.shared::cluster.type d, var;`
    SharedVariable {
        /// `.type`
        data_type: DataType,
        /// `d`
        destination: RegisterOperand,
        /// `var`
        symbol: VariableSymbol,
    },
    /// `getctarank.shared::cluster.type d, var + imm;`
    SharedVariableWithImmediate {
        /// `.type`
        data_type: DataType,
        /// `d`
        destination: RegisterOperand,
        /// `var`
        symbol: VariableSymbol,
        /// `imm`
        immediate: Immediate,
    },
}

/// `.type  = { .u32, .u64 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.u64`
    U64,
}
