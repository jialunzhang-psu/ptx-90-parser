use crate::r#type::common::{RegisterOperand, VariableSymbol};

/// `suq.query.b32 d, [a];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suq {
    /// `.query`
    pub query: Query,
    /// `d`
    pub destination: RegisterOperand,
    /// `[a]`
    pub address: Operand,
}

/// `.query = { .width, .height, .depth, .channel_data_type, .channel_order, .array_size, .memory_layout };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Query {
    /// `.width`
    Width,
    /// `.height`
    Height,
    /// `.depth`
    Depth,
    /// `.channel_data_type`
    ChannelDataType,
    /// `.channel_order`
    ChannelOrder,
    /// `.array_size`
    ArraySize,
    /// `.memory_layout`
    MemoryLayout,
}

/// `a = { .surfref, .u64 }`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    /// `.surfref`
    Surface(VariableSymbol),
    /// `.u64`
    Register(RegisterOperand),
}
