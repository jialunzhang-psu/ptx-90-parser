use crate::r#type::common::RegisterOperand;

/// `mad24.mode.type d, a, b, c;`
/// `mad24.hi.sat.s32 d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mad24 {
    /// `mad24.mode.type d, a, b, c;`
    Mode {
        mode: Mode,
        data_type: DataType,
        destination: RegisterOperand,
        a: RegisterOperand,
        b: RegisterOperand,
        c: RegisterOperand,
    },
    /// `mad24.hi.sat.s32 d, a, b, c;`
    HiSatS32 {
        destination: RegisterOperand,
        a: RegisterOperand,
        b: RegisterOperand,
        c: RegisterOperand,
    },
}

/// `.mode = { .hi, .lo };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `.hi`
    Hi,
    /// `.lo`
    Lo,
}

/// `.type = { .u32, .s32 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.u32`
    U32,
    /// `.s32`
    S32,
}
