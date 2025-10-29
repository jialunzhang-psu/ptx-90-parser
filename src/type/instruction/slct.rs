use crate::r#type::common::RegisterOperand;

/// `slct.dtype.s32        d, a, b, c;`
/// `slct{.ftz}.dtype.f32  d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Slct {
    /// `slct.dtype.s32 d, a, b, c;`
    S32 {
        /// `.dtype`
        data_type: DataType,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        on_true: RegisterOperand,
        /// `b`
        on_false: RegisterOperand,
        /// `c`
        selector: RegisterOperand,
    },
    /// `slct{.ftz}.dtype.f32 d, a, b, c;`
    F32 {
        /// `.ftz`
        flush_to_zero: bool,
        /// `.dtype`
        data_type: DataType,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        on_true: RegisterOperand,
        /// `b`
        on_false: RegisterOperand,
        /// `c`
        selector: RegisterOperand,
    },
}

/// `.dtype = { .b16, .b32, .b64, .u16, .u32, .u64, .s16, .s32, .s64, .f32, .f64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
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
