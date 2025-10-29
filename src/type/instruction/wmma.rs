use crate::r#type::common::{AddressOperand, Operand, RegisterOperand};

/// `wmma.load.{a|b|c}.sync.aligned.layout.shape{.ss}.(atype|btype|ctype) r, [p] {, stride};`
/// `wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride}`
/// `wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride}`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    /// `wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};`
    /// `wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride}`
    /// `wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride}`
    LoadA(LoadA),
    /// `wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};`
    /// `wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride}`
    /// `wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride}`
    LoadB(LoadB),
    /// `wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};`
    /// `wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride}`
    LoadC(LoadC),
}

/// `wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};`
/// `wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride}`
/// `wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride}`
/// `wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride}`
/// `wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride}`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadA {
    /// `.layout` / `.row`
    pub layout: Layout,
    /// `.shape`
    pub shape: Shape,
    /// `.ss`
    pub state_space: Option<StateSpace>,
    /// `.atype`
    pub data_type: AType,
    /// `r`
    pub destination: RegisterOperand,
    /// `[p]`
    pub address: AddressOperand,
    /// `stride`
    pub stride: Option<Operand>,
}

/// `wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};`
/// `wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride}`
/// `wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride}`
/// `wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride}`
/// `wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride}`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadB {
    /// `.layout` / `.col`
    pub layout: Layout,
    /// `.shape`
    pub shape: Shape,
    /// `.ss`
    pub state_space: Option<StateSpace>,
    /// `.btype`
    pub data_type: BType,
    /// `r`
    pub destination: RegisterOperand,
    /// `[p]`
    pub address: AddressOperand,
    /// `stride`
    pub stride: Option<Operand>,
}

/// `wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};`
/// `wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride}`
/// `wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride}`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadC {
    /// `.layout`
    pub layout: Layout,
    /// `.shape`
    pub shape: Shape,
    /// `.ss`
    pub state_space: Option<StateSpace>,
    /// `.ctype`
    pub data_type: CType,
    /// `r`
    pub destination: RegisterOperand,
    /// `[p]`
    pub address: AddressOperand,
    /// `stride`
    pub stride: Option<Operand>,
}

/// `.layout = {.row, .col};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    /// `.row`
    Row,
    /// `.col`
    Col,
}

/// `.shape = {.m16n16k16, .m8n32k16, .m32n8k16, .m16n16k8, .m8n8k4, .m8n8k32, .m8n8k128};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// `.m16n16k16`
    M16N16K16,
    /// `.m8n32k16`
    M8N32K16,
    /// `.m32n8k16`
    M32N8K16,
    /// `.m16n16k8`
    M16N16K8,
    /// `.m8n8k4`
    M8N8K4,
    /// `.m8n8k32`
    M8N8K32,
    /// `.m8n8k128`
    M8N8K128,
}

/// `.ss = {.global, .shared{::cta}};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSpace {
    /// `.global`
    Global,
    /// `.shared`
    Shared,
    /// `.shared::cta`
    SharedCta,
}

/// `.atype = {.f16, .s8, .u8, .bf16, .tf32, .f64, .s4, .u4, .b1};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AType {
    /// `.f16`
    F16,
    /// `.s8`
    S8,
    /// `.u8`
    U8,
    /// `.bf16`
    Bf16,
    /// `.tf32`
    Tf32,
    /// `.f64`
    F64,
    /// `.s4`
    S4,
    /// `.u4`
    U4,
    /// `.b1`
    B1,
}

/// `.btype = {.f16, .s8, .u8, .bf16, .tf32, .f64, .s4, .u4, .b1};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BType {
    /// `.f16`
    F16,
    /// `.s8`
    S8,
    /// `.u8`
    U8,
    /// `.bf16`
    Bf16,
    /// `.tf32`
    Tf32,
    /// `.f64`
    F64,
    /// `.s4`
    S4,
    /// `.u4`
    U4,
    /// `.b1`
    B1,
}

/// `.ctype = {.f16, .f32, .s32, .f64};`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CType {
    /// `.f16`
    F16,
    /// `.f32`
    F32,
    /// `.s32`
    S32,
    /// `.f64`
    F64,
}
