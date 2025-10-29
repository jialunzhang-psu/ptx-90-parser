use crate::r#type::common::{AddressOperand, RegisterOperand};

/// `tensormap.replace.mode.field1{.ss}.b1024.type  [addr], new_val;`
/// `tensormap.replace.mode.field2{.ss}.b1024.type  [addr], ord, new_val;`
/// `tensormap.replace.mode.field3{.ss}.b1024.type  [addr], new_val;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TensormapOpcode {
    /// `tensormap.replace.mode.field1{.ss}.b1024.type  [addr], new_val;`
    Field1(Field1),
    /// `tensormap.replace.mode.field2{.ss}.b1024.type  [addr], ord, new_val;`
    Field2(Field2),
    /// `tensormap.replace.mode.field3{.ss}.b1024.type  [addr], new_val;`
    Field3(Field3),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field1 {
    /// `.mode`
    pub mode: Mode,
    /// `.ss`
    pub state_space: Option<StateSpace>,
    /// `.b1024`
    pub object_size: ObjectSize,
    /// `.type`
    pub data_type: DataType,
    /// `[addr]`
    pub address: AddressOperand,
    /// `new_val`
    pub field: Field1Field,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field2 {
    /// `.mode`
    pub mode: Mode,
    /// `.ss`
    pub state_space: Option<StateSpace>,
    /// `.b1024`
    pub object_size: ObjectSize,
    /// `.type`
    pub data_type: DataType,
    /// `[addr]`
    pub address: AddressOperand,
    /// `ord`
    pub ordinal: u32,
    /// `new_val`
    pub field: Field2Field,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field3 {
    /// `.mode`
    pub mode: Mode,
    /// `.ss`
    pub state_space: Option<StateSpace>,
    /// `.b1024`
    pub object_size: ObjectSize,
    /// `.type`
    pub data_type: DataType,
    /// `[addr]`
    pub address: AddressOperand,
    /// `new_val`
    pub field: Field3Field,
}

/// `.mode = { .tile }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `.tile`
    Tile,
}

/// `.ss = { .global, .shared::cta }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSpace {
    /// `.global`
    Global,
    /// `.shared::cta`
    SharedCta,
}

/// `.b1024`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectSize {
    /// `.b1024`
    B1024,
}

/// `.type = { .b32, .b64 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b32`
    B32,
    /// `.b64`
    B64,
}

/// `.field1 = { .global_address, .rank }`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field1Field {
    /// `.global_address`
    GlobalAddress(RegisterOperand),
    /// `.rank`
    Rank(RegisterOperand),
}

/// `.field2 = { .box_dim, .global_dim, .global_stride, .element_stride }`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field2Field {
    /// `.box_dim`
    BoxDim(RegisterOperand),
    /// `.global_dim`
    GlobalDim(RegisterOperand),
    /// `.global_stride`
    GlobalStride(RegisterOperand),
    /// `.element_stride`
    ElementStride(RegisterOperand),
}

/// `.field3 = { .elemtype, .interleave_layout, .swizzle_mode, .swizzle_atomicity, .fill_mode }`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field3Field {
    /// `.elemtype`
    ElemType(ElementType),
    /// `.interleave_layout`
    InterleaveLayout(InterleaveLayout),
    /// `.swizzle_mode`
    SwizzleMode(SwizzleMode),
    /// `.swizzle_atomicity`
    SwizzleAtomicity(SwizzleAtomicity),
    /// `.fill_mode`
    FillMode(FillMode),
}

/// `new_val`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    U8,
    U16,
    U32,
    S32,
    U64,
    S64,
    F16,
    F32,
    F32Ftz,
    F64,
    Bf16,
    Tf32,
    Tf32Ftz,
    B4x16,
    B4x16P64,
    B6x16P32,
}

/// `new_val`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterleaveLayout {
    None,
    Interleave16B,
    Interleave32B,
}

/// `new_val`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwizzleMode {
    None,
    Swizzle32B,
    Swizzle64B,
    Swizzle128B,
    Swizzle96B,
}

/// `new_val`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwizzleAtomicity {
    Bytes16,
    Bytes32,
    Bytes32With8ByteFlip,
    Bytes64,
}

/// `new_val`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillMode {
    Zero,
    OobNan,
}
