use crate::r#type::common::{Immediate, Operand, RegisterOperand, VariableSymbol};

/// `mapa{.space}.type d, a, b;`
/// `mapa.shared::cluster.type d, a, b;`
/// `mapa.shared::cluster.type d, sh, b;`
/// `mapa.shared::cluster.type d, sh + imm, b;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mapa {
    /// `mapa.type d, a, b;`
    Generic {
        /// `.type`
        data_type: DataType,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        address: RegisterOperand,
        /// `b`
        cta: Operand,
    },
    /// `mapa.shared::cluster.type d, a, b;`
    SharedRegister {
        /// `.type`
        data_type: DataType,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        address: RegisterOperand,
        /// `b`
        cta: Operand,
    },
    /// `mapa.shared::cluster.type d, sh, b;`
    SharedVariable {
        /// `.type`
        data_type: DataType,
        /// `d`
        destination: RegisterOperand,
        /// `sh`
        variable: VariableSymbol,
        /// `b`
        cta: Operand,
    },
    /// `mapa.shared::cluster.type d, sh + imm, b;`
    SharedVariableWithImmediate {
        /// `.type`
        data_type: DataType,
        /// `d`
        destination: RegisterOperand,
        /// `sh`
        variable: VariableSymbol,
        /// `imm`
        immediate: Immediate,
        /// `b`
        cta: Operand,
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
