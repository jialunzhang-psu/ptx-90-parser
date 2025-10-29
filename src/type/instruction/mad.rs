use crate::r#type::common::RegisterOperand;

/// `mad.mode.type  d, a, b, c;`
/// `mad.hi.sat.s32 d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mad {
    /// `mad.mode.type  d, a, b, c;`
    Mode {
        mode: Mode,
        data_type: DataType,
        destination: RegisterOperand,
        a: RegisterOperand,
        b: RegisterOperand,
        c: RegisterOperand,
    },
    /// `mad.hi.sat.s32 d, a, b, c;`
    HiSatS32 {
        destination: RegisterOperand,
        a: RegisterOperand,
        b: RegisterOperand,
        c: RegisterOperand,
    },
}

/// `.mode = { .hi, .lo, .wide };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `.hi`
    Hi,
    /// `.lo`
    Lo,
    /// `.wide`
    Wide,
}

/// `.type = { .u16, .u32, .u64, .s16, .s32, .s64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
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
}
