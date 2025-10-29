use crate::r#type::common::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tld4 {
    /// `tld4.comp.2d.v4.dtype.f32    d[|p], [a, c] {, e} {, f};`
    Implicit(ImplicitSampler),
    /// `tld4.comp.geom.v4.dtype.f32  d[|p], [a, b, c] {, e} {, f};`
    Explicit(ExplicitSampler),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplicitSampler {
    /// `.comp`
    pub component: Component,
    /// `.geom`
    pub geometry: Geometry,
    /// `.dtype`
    pub data_type: DataType,
    /// `d[|p]`
    pub destination: Destination,
    /// `a`
    pub texture: TextureOperand,
    /// `f`
    pub depth_compare: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplicitSampler {
    /// `.comp`
    pub component: Component,
    /// `.geom`
    pub geometry: Geometry,
    /// `.dtype`
    pub data_type: DataType,
    /// `d[|p]`
    pub destination: Destination,
    /// `a`
    pub texture: TextureOperand,
    /// `b`
    pub sampler: SamplerOperand,
    /// `f`
    pub depth_compare: Option<RegisterOperand>,
}

/// `d[|p]`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destination {
    pub vector: RegisterOperand,
    pub predicate: Option<PredicateRegister>,
}

/// `.comp = { .r, .g, .b, .a }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    R,
    G,
    B,
    A,
}

/// `.dtype = { .u32, .s32, .f32 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    U32,
    S32,
    F32,
}

/// `.geom = { .2d, .a2d, .cube, .acube }`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Geometry {
    /// `.2d`
    TwoD {
        /// `c`
        coordinates: RegisterOperand,
        /// `e`
        offset: Option<RegisterOperand>,
    },
    /// `.a2d`
    Array2D {
        /// `c`
        coordinates: RegisterOperand,
        /// `e`
        offset: Option<RegisterOperand>,
    },
    /// `.cube`
    Cube {
        /// `c`
        coordinates: RegisterOperand,
    },
    /// `.acube`
    ArrayCube {
        /// `c`
        coordinates: RegisterOperand,
    },
}

/// `a`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextureOperand {
    Symbol(VariableSymbol),
    Register(RegisterOperand),
}

/// `b`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SamplerOperand {
    Symbol(VariableSymbol),
    Register(RegisterOperand),
}
