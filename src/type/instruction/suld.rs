use crate::r#type::common::{RegisterOperand, VariableSymbol};

/// `suld.b.geom{.cop}.vec.dtype.clamp  d, [a, b];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Suld {
    /// `suld.b.1d{.cop}.vec.dtype.clamp  d, [a, b];`
    OneD(Descriptor<Coordinate1d>),
    /// `suld.b.2d{.cop}.vec.dtype.clamp  d, [a, b];`
    TwoD(Descriptor<Coordinate2d>),
    /// `suld.b.3d{.cop}.vec.dtype.clamp  d, [a, b];`
    ThreeD(Descriptor<Coordinate3d>),
    /// `suld.b.a1d{.cop}.vec.dtype.clamp  d, [a, b];`
    Array1D(Descriptor<Array1dCoordinates>),
    /// `suld.b.a2d{.cop}.vec.dtype.clamp  d, [a, b];`
    Array2D(Descriptor<Array2dCoordinates>),
}

/// `d, [a, b];`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Descriptor<TCoordinates> {
    /// `.cop`
    pub cache_operator: Option<CacheOperator>,
    /// `.vec`
    pub vector: Vector,
    /// `.dtype`
    pub data_type: DataType,
    /// `.clamp`
    pub clamp: Clamp,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub surface: Surface,
    /// `b`
    pub coordinates: TCoordinates,
}

/// `.vec`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector {
    /// `none`
    None,
    /// `.v2`
    V2,
    /// `.v4`
    V4,
}

/// `.dtype`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// `.b8`
    B8,
    /// `.b16`
    B16,
    /// `.b32`
    B32,
    /// `.b64`
    B64,
}

/// `.clamp`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clamp {
    /// `.trap`
    Trap,
    /// `.clamp`
    Clamp,
    /// `.zero`
    Zero,
}

/// `.cop`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheOperator {
    /// `.ca`
    Ca,
    /// `.cg`
    Cg,
    /// `.cs`
    Cs,
    /// `.cv`
    Cv,
}

/// `a`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Surface {
    /// `.surfref`
    Reference(VariableSymbol),
    /// `.u64`
    Register(RegisterOperand),
}

/// `.1d`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordinate1d {
    /// `x`
    pub x: RegisterOperand,
}

/// `.2d`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordinate2d {
    /// `x`
    pub x: RegisterOperand,
    /// `y`
    pub y: RegisterOperand,
}

/// `.3d`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordinate3d {
    /// `x`
    pub x: RegisterOperand,
    /// `y`
    pub y: RegisterOperand,
    /// `z`
    pub z: RegisterOperand,
    /// `w` ignored
    pub w: RegisterOperand,
}

/// `.a1d`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array1dCoordinates {
    /// `idx`
    pub index: RegisterOperand,
    /// `x`
    pub x: RegisterOperand,
}

/// `.a2d`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array2dCoordinates {
    /// `idx`
    pub index: RegisterOperand,
    /// `x`
    pub x: RegisterOperand,
    /// `y`
    pub y: RegisterOperand,
    /// `z` ignored
    pub z: RegisterOperand,
}
