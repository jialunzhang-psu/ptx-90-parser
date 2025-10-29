use crate::r#type::common::RegisterOperand;

/// Type-safe representation of every `mma` syntax variant described in the cache specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MmaInstruction {
    /// `mma.sync.aligned.m8n8k4.alayout.blayout.dtype.f16.f16.ctype d, a, b, c;`
    SyncAlignedM8N8K4(SyncAlignedM8N8K4),
    /// `mma.sync.aligned.m16n8k8.row.col.dtype.f16.f16.ctype d, a, b, c;`
    SyncAlignedM16N8K8(SyncAlignedM16N8K8),
    /// `mma.sync.aligned.m16n8k16.row.col.dtype.f16.f16.ctype d, a, b, c;`
    SyncAlignedM16N8K16(SyncAlignedM16N8K16),
    /// `mma.sync.aligned.m16n8k4.row.col.f32.tf32.tf32.f32 d, a, b, c;`
    SyncAlignedM16N8K4Tf32(SyncAlignedM16N8K4Tf32),
    /// `mma.sync.aligned.m16n8k8.row.col.f32.atype.btype.f32 d, a, b, c;`
    SyncAlignedM16N8K8Mixed(SyncAlignedM16N8K8Mixed),
    /// `mma.sync.aligned.m16n8k16.row.col.f32.bf16.bf16.f32 d, a, b, c;`
    SyncAlignedM16N8K16Bf16(SyncAlignedM16N8K16Bf16),
    /// `mma.sync.aligned.shape.row.col.dtype.f8type.f8type.ctype d, a, b, c;`
    SyncAlignedF8(SyncAlignedF8),
    /// `mma.sync.aligned.m16n8k32.row.col.kind.dtype.f8f6f4type.f8f6f4type.ctype d, a, b, c;`
    SyncAlignedM16N8K32F8F6F4(SyncAlignedM16N8K32F8F6F4),
    /// `mma.sync.aligned.m16n8k64.row.col.kind.block_scale{.scale_vec_size}.f32.e2m1.e2m1.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};`
    SyncAlignedBlockScaleM16N8K64MxF4(SyncAlignedBlockScaleM16N8K64MxF4),
    /// `mma.sync.aligned.m16n8k64.row.col.kind.block_scale.scale_vec_size.f32.e2m1.e2m1.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};`
    SyncAlignedBlockScaleM16N8K64MxF4NvF4(SyncAlignedBlockScaleM16N8K64MxF4NvF4),
    /// `mma.sync.aligned.m16n8k32.row.col.kind.block_scale{.scale_vec_size}.f32.f8f6f4type.f8f6f4type.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};`
    SyncAlignedBlockScaleM16N8K32MxF8F6F4(SyncAlignedBlockScaleM16N8K32MxF8F6F4),
    /// `mma.sync.aligned.shape.row.col.f64.f64.f64.f64 d, a, b, c;`
    SyncAlignedF64(SyncAlignedF64),
    /// `mma.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c;` with `.atype`/`.btype` in `{.u8, .s8}`
    SyncAlignedInteger8Bit(SyncAlignedInteger8Bit),
    /// `mma.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c;` with `.atype`/`.btype` in `{.u4, .s4}`
    SyncAlignedInteger4Bit(SyncAlignedInteger4Bit),
    /// `mma.sync.aligned.shape.row.col.s32.b1.b1.s32.bitOp.popc d, a, b, c;`
    SyncAlignedSingleBit(SyncAlignedSingleBit),
}

/// `mma.sync.aligned.m8n8k4.alayout.blayout.dtype.f16.f16.ctype d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedM8N8K4 {
    pub a_layout: Layout,
    pub b_layout: Layout,
    pub d_type: DataType,
    pub c_type: DataType,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
}

/// `mma.sync.aligned.m16n8k8.row.col.dtype.f16.f16.ctype d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedM16N8K8 {
    pub d_type: DataType,
    pub c_type: DataType,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
}

/// `mma.sync.aligned.m16n8k16.row.col.dtype.f16.f16.ctype d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedM16N8K16 {
    pub d_type: DataType,
    pub c_type: DataType,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
}

/// `mma.sync.aligned.m16n8k4.row.col.f32.tf32.tf32.f32 d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedM16N8K4Tf32 {
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
}

/// `mma.sync.aligned.m16n8k8.row.col.f32.atype.btype.f32 d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedM16N8K8Mixed {
    pub a_type: AlternateMatrixType,
    pub b_type: AlternateMatrixType,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
}

/// `mma.sync.aligned.m16n8k16.row.col.f32.bf16.bf16.f32 d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedM16N8K16Bf16 {
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
}

/// `mma.sync.aligned.shape.row.col.dtype.f8type.f8type.ctype d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedF8 {
    pub shape: F8Shape,
    pub d_type: DataType,
    pub a_type: F8Type,
    pub b_type: F8Type,
    pub c_type: DataType,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
}

/// `mma.sync.aligned.m16n8k32.row.col.kind.dtype.f8f6f4type.f8f6f4type.ctype d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedM16N8K32F8F6F4 {
    pub d_type: DataType,
    pub a_type: F8F6F4Type,
    pub b_type: F8F6F4Type,
    pub c_type: DataType,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
}

/// `mma.sync.aligned.m16n8k64.row.col.kind.block_scale{.scale_vec_size}.f32.e2m1.e2m1.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedBlockScaleM16N8K64MxF4 {
    /// `.scale_vec_size`
    pub scale_vec_size: Option<MxF4ScaleVecSize>,
    /// `.stype`
    pub stype: MxF4ScaleDataType,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
    pub block_scale: BlockScaleOperands,
}

/// `mma.sync.aligned.m16n8k64.row.col.kind.block_scale.scale_vec_size.f32.e2m1.e2m1.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedBlockScaleM16N8K64MxF4NvF4 {
    /// `.scale_vec_size`
    pub scale_vec_size: MxF4NvF4ScaleVecSize,
    pub stype: MxF4NvF4ScaleDataType,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
    pub block_scale: BlockScaleOperands,
}

/// `mma.sync.aligned.m16n8k32.row.col.kind.block_scale{.scale_vec_size}.f32.f8f6f4type.f8f6f4type.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedBlockScaleM16N8K32MxF8F6F4 {
    /// `.scale_vec_size`
    pub scale_vec_size: Option<MxF8F6F4ScaleVecSize>,
    pub a_type: F8F6F4Type,
    pub b_type: F8F6F4Type,
    /// `.stype`
    pub stype: MxF8F6F4ScaleDataType,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
    pub block_scale: BlockScaleOperands,
}

/// `mma.sync.aligned.shape.row.col.f64.f64.f64.f64 d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedF64 {
    pub shape: F64Shape,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
}

/// `mma.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedInteger8Bit {
    pub shape: Integer8Shape,
    /// `true` when `.satfinite` is present
    pub satfinite: bool,
    pub a_type: Integer8Type,
    pub b_type: Integer8Type,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
}

/// `mma.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedInteger4Bit {
    pub shape: Integer4Shape,
    /// `true` when `.satfinite` is present
    pub satfinite: bool,
    pub a_type: Integer4Type,
    pub b_type: Integer4Type,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
}

/// `mma.sync.aligned.shape.row.col.s32.b1.b1.s32.bitOp.popc d, a, b, c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncAlignedSingleBit {
    pub shape: SingleBitShape,
    pub bit_op: BitOp,
    pub destination: RegisterOperand,
    pub operand_a: RegisterOperand,
    pub operand_b: RegisterOperand,
    pub operand_c: RegisterOperand,
}

/// `.alayout` / `.blayout`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    /// `.row`
    Row,
    /// `.col`
    Col,
}

/// `.dtype` / `.ctype`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.f16`
    F16,
    /// `.f32`
    F32,
}

/// `.atype` / `.btype` with alternate floating point inputs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlternateMatrixType {
    /// `.bf16`
    Bf16,
    /// `.tf32`
    Tf32,
}

/// `.f8type`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F8Type {
    /// `.e4m3`
    E4M3,
    /// `.e5m2`
    E5M2,
}

/// `.f8f6f4type`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F8F6F4Type {
    /// `.e4m3`
    E4M3,
    /// `.e5m2`
    E5M2,
    /// `.e3m2`
    E3M2,
    /// `.e2m3`
    E2M3,
    /// `.e2m1`
    E2M1,
}

/// `.shape`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F8Shape {
    /// `.m16n8k16`
    M16N8K16,
    /// `.m16n8k32`
    M16N8K32,
}

/// `.shape`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F64Shape {
    /// `.m8n8k4`
    M8N8K4,
    /// `.m16n8k4`
    M16N8K4,
    /// `.m16n8k8`
    M16N8K8,
    /// `.m16n8k16`
    M16N8K16,
}

/// `.shape`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Integer8Shape {
    /// `.m8n8k16`
    M8N8K16,
    /// `.m16n8k16`
    M16N8K16,
    /// `.m16n8k32`
    M16N8K32,
}

/// `.shape`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Integer4Shape {
    /// `.m8n8k32`
    M8N8K32,
    /// `.m16n8k32`
    M16N8K32,
    /// `.m16n8k64`
    M16N8K64,
}

/// `.atype` / `.btype`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Integer8Type {
    /// `.u8`
    U8,
    /// `.s8`
    S8,
}

/// `.atype` / `.btype`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Integer4Type {
    /// `.u4`
    U4,
    /// `.s4`
    S4,
}

/// `.shape`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleBitShape {
    /// `.m8n8k128`
    M8N8K128,
    /// `.m16n8k128`
    M16N8K128,
    /// `.m16n8k256`
    M16N8K256,
}

/// `.bitOp`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitOp {
    /// `.xor`
    Xor,
    /// `.and`
    And,
}

/// `scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b}`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockScaleOperands {
    pub scale_a_data: RegisterOperand,
    /// `byte-id-a`
    pub scale_a_byte_id: u16,
    /// `thread-id-a`
    pub scale_a_thread_id: u16,
    pub scale_b_data: RegisterOperand,
    /// `byte-id-b`
    pub scale_b_byte_id: u16,
    /// `thread-id-b`
    pub scale_b_thread_id: u16,
}

/// `.scale_vec_size`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MxF4ScaleVecSize {
    /// `.scale_vec::2X`
    TwoX,
}

/// `.scale_vec_size`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MxF4NvF4ScaleVecSize {
    /// `.scale_vec::2X`
    TwoX,
    /// `.scale_vec::4X`
    FourX,
}

/// `.scale_vec_size`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MxF8F6F4ScaleVecSize {
    /// `.scale_vec::1X`
    OneX,
}

/// `.stype = {.ue8m0}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MxF4ScaleDataType {
    /// `.ue8m0`
    UE8M0,
}

/// `.stype = {.ue8m0, .ue4m3}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MxF4NvF4ScaleDataType {
    /// `.ue8m0`
    UE8M0,
    /// `.ue4m3`
    UE4M3,
}

/// `.stype = {.ue8m0}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MxF8F6F4ScaleDataType {
    /// `.ue8m0`
    UE8M0,
}
