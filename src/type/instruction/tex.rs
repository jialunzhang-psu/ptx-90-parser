use crate::r#type::common::{PredicateRegister, RegisterOperand, VariableSymbol};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tex {
    /// `tex.geom.v4.dtype.ctype  d, [a, c] {, e} {, f};`
    Vector4ImplicitSampler(Vector4ImplicitSampler),
    /// `tex.geom.v4.dtype.ctype  d[|p], [a, b, c] {, e} {, f};`
    Vector4ExplicitSampler(Vector4ExplicitSampler),
    /// `tex.geom.v2.f16x2.ctype  d[|p], [a, c] {, e} {, f};`
    Vector2F16x2ImplicitSampler(Vector2F16x2ImplicitSampler),
    /// `tex.geom.v2.f16x2.ctype  d[|p], [a, b, c] {, e} {, f};`
    Vector2F16x2ExplicitSampler(Vector2F16x2ExplicitSampler),
    /// `tex.base.geom.v4.dtype.ctype   d[|p], [a, {b,} c] {, e} {, f};`
    Vector4MipBase(Vector4MipBase),
    /// `tex.level.geom.v4.dtype.ctype  d[|p], [a, {b,} c], lod {, e} {, f};`
    Vector4MipLevel(Vector4MipLevel),
    /// `tex.grad.geom.v4.dtype.ctype   d[|p], [a, {b,} c], dPdx, dPdy {, e} {, f};`
    Vector4MipGradient(Vector4MipGradient),
    /// `tex.base.geom.v2.f16x2.ctype   d[|p], [a, {b,} c] {, e} {, f};`
    Vector2F16x2MipBase(Vector2F16x2MipBase),
    /// `tex.level.geom.v2.f16x2.ctype  d[|p], [a, {b,} c], lod {, e} {, f};`
    Vector2F16x2MipLevel(Vector2F16x2MipLevel),
    /// `tex.grad.geom.v2.f16x2.ctype   d[|p], [a, {b,} c], dPdx, dPdy {, e} {, f};`
    Vector2F16x2MipGradient(Vector2F16x2MipGradient),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector4ImplicitSampler {
    /// `.geom`
    pub geometry: Geometry,
    /// `.dtype`
    pub data_type: Vector4DataType,
    /// `.ctype`
    pub coordinate_type: CoordinateType,
    /// `d`
    pub destination: RegisterOperand,
    /// `p`
    pub predicate: Option<PredicateRegister>,
    /// `a`
    pub texture: TextureOperand,
    /// `c`
    pub coordinates: RegisterOperand,
    /// `e`
    pub offset: Option<Offset>,
    /// `f`
    pub depth_compare: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector4ExplicitSampler {
    /// `.geom`
    pub geometry: Geometry,
    /// `.dtype`
    pub data_type: Vector4DataType,
    /// `.ctype`
    pub coordinate_type: CoordinateType,
    /// `d`
    pub destination: RegisterOperand,
    /// `p`
    pub predicate: Option<PredicateRegister>,
    /// `a`
    pub texture: TextureOperand,
    /// `b`
    pub sampler: SamplerOperand,
    /// `c`
    pub coordinates: RegisterOperand,
    /// `e`
    pub offset: Option<Offset>,
    /// `f`
    pub depth_compare: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector2F16x2ImplicitSampler {
    /// `.geom`
    pub geometry: Geometry,
    /// `.ctype`
    pub coordinate_type: CoordinateType,
    /// `d`
    pub destination: RegisterOperand,
    /// `p`
    pub predicate: Option<PredicateRegister>,
    /// `a`
    pub texture: TextureOperand,
    /// `c`
    pub coordinates: RegisterOperand,
    /// `e`
    pub offset: Option<Offset>,
    /// `f`
    pub depth_compare: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector2F16x2ExplicitSampler {
    /// `.geom`
    pub geometry: Geometry,
    /// `.ctype`
    pub coordinate_type: CoordinateType,
    /// `d`
    pub destination: RegisterOperand,
    /// `p`
    pub predicate: Option<PredicateRegister>,
    /// `a`
    pub texture: TextureOperand,
    /// `b`
    pub sampler: SamplerOperand,
    /// `c`
    pub coordinates: RegisterOperand,
    /// `e`
    pub offset: Option<Offset>,
    /// `f`
    pub depth_compare: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector4MipBase {
    /// `.geom`
    pub geometry: Geometry,
    /// `.dtype`
    pub data_type: Vector4DataType,
    /// `.ctype`
    pub coordinate_type: CoordinateType,
    /// `d`
    pub destination: RegisterOperand,
    /// `p`
    pub predicate: Option<PredicateRegister>,
    /// `a`
    pub texture: TextureOperand,
    /// `b`
    pub sampler: Option<SamplerOperand>,
    /// `c`
    pub coordinates: RegisterOperand,
    /// `e`
    pub offset: Option<Offset>,
    /// `f`
    pub depth_compare: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector4MipLevel {
    /// `.geom`
    pub geometry: Geometry,
    /// `.dtype`
    pub data_type: Vector4DataType,
    /// `.ctype`
    pub coordinate_type: CoordinateType,
    /// `d`
    pub destination: RegisterOperand,
    /// `p`
    pub predicate: Option<PredicateRegister>,
    /// `a`
    pub texture: TextureOperand,
    /// `b`
    pub sampler: Option<SamplerOperand>,
    /// `c`
    pub coordinates: RegisterOperand,
    /// `lod`
    pub level_of_detail: LevelOfDetail,
    /// `e`
    pub offset: Option<Offset>,
    /// `f`
    pub depth_compare: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector4MipGradient {
    /// `.geom`
    pub geometry: Geometry,
    /// `.dtype`
    pub data_type: Vector4DataType,
    /// `.ctype`
    pub coordinate_type: CoordinateType,
    /// `d`
    pub destination: RegisterOperand,
    /// `p`
    pub predicate: Option<PredicateRegister>,
    /// `a`
    pub texture: TextureOperand,
    /// `b`
    pub sampler: Option<SamplerOperand>,
    /// `c`
    pub coordinates: RegisterOperand,
    /// `dPdx`, `dPdy`
    pub gradients: Gradients,
    /// `e`
    pub offset: Option<Offset>,
    /// `f`
    pub depth_compare: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector2F16x2MipBase {
    /// `.geom`
    pub geometry: Geometry,
    /// `.ctype`
    pub coordinate_type: CoordinateType,
    /// `d`
    pub destination: RegisterOperand,
    /// `p`
    pub predicate: Option<PredicateRegister>,
    /// `a`
    pub texture: TextureOperand,
    /// `b`
    pub sampler: Option<SamplerOperand>,
    /// `c`
    pub coordinates: RegisterOperand,
    /// `e`
    pub offset: Option<Offset>,
    /// `f`
    pub depth_compare: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector2F16x2MipLevel {
    /// `.geom`
    pub geometry: Geometry,
    /// `.ctype`
    pub coordinate_type: CoordinateType,
    /// `d`
    pub destination: RegisterOperand,
    /// `p`
    pub predicate: Option<PredicateRegister>,
    /// `a`
    pub texture: TextureOperand,
    /// `b`
    pub sampler: Option<SamplerOperand>,
    /// `c`
    pub coordinates: RegisterOperand,
    /// `lod`
    pub level_of_detail: LevelOfDetail,
    /// `e`
    pub offset: Option<Offset>,
    /// `f`
    pub depth_compare: Option<RegisterOperand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector2F16x2MipGradient {
    /// `.geom`
    pub geometry: Geometry,
    /// `.ctype`
    pub coordinate_type: CoordinateType,
    /// `d`
    pub destination: RegisterOperand,
    /// `p`
    pub predicate: Option<PredicateRegister>,
    /// `a`
    pub texture: TextureOperand,
    /// `b`
    pub sampler: Option<SamplerOperand>,
    /// `c`
    pub coordinates: RegisterOperand,
    /// `dPdx`, `dPdy`
    pub gradients: Gradients,
    /// `e`
    pub offset: Option<Offset>,
    /// `f`
    pub depth_compare: Option<RegisterOperand>,
}

/// `.geom = { .1d, .2d, .3d, .a1d, .a2d, .cube, .acube, .2dms, .a2dms }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Geometry {
    /// `.1d`
    OneD,
    /// `.2d`
    TwoD,
    /// `.3d`
    ThreeD,
    /// `.a1d`
    Array1D,
    /// `.a2d`
    Array2D,
    /// `.cube`
    Cube,
    /// `.acube`
    ArrayCube,
    /// `.2dms`
    TwoDMultisample,
    /// `.a2dms`
    Array2DMultisample,
}

/// `.dtype = { .u32, .s32, .f16,  .f32 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector4DataType {
    U32,
    S32,
    F16,
    F32,
}

/// `.ctype = { .s32, .f32 }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateType {
    S32,
    F32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offset {
    Scalar(RegisterOperand),
    Pair(RegisterOperand),
    Quad(RegisterOperand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LevelOfDetail {
    S32(RegisterOperand),
    F32(RegisterOperand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gradients {
    pub dpdx: GradientVector,
    pub dpdy: GradientVector,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GradientVector {
    Scalar(RegisterOperand),
    Pair(RegisterOperand),
    Quad(RegisterOperand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextureOperand {
    Symbol(VariableSymbol),
    Register(RegisterOperand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SamplerOperand {
    Symbol(VariableSymbol),
    Register(RegisterOperand),
}
