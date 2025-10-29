use crate::r#type::common::{RegisterOperand, VariableSymbol};

/// `sust.b.{1d,2d,3d}{.cop}.vec.ctype.clamp [a, b], c;`
/// `sust.p.{1d,2d,3d}.vec.b32.clamp [a, b], c;`
/// `sust.b.{a1d,a2d}{.cop}.vec.ctype.clamp [a, b], c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sust {
    /// `sust.b.1d{.cop}.vec.ctype.clamp [a, b], c;`
    Block1d(Block<Coordinate1d>),
    /// `sust.b.2d{.cop}.vec.ctype.clamp [a, b], c;`
    Block2d(Block<Coordinate2d>),
    /// `sust.b.3d{.cop}.vec.ctype.clamp [a, b], c;`
    Block3d(Block<Coordinate3d>),
    /// `sust.b.a1d{.cop}.vec.ctype.clamp [a, b], c;`
    BlockArray1d(Block<Array1dCoordinates>),
    /// `sust.b.a2d{.cop}.vec.ctype.clamp [a, b], c;`
    BlockArray2d(Block<Array2dCoordinates>),
    /// `sust.p.1d.vec.b32.clamp [a, b], c;`
    Formatted1d(Formatted<Coordinate1d>),
    /// `sust.p.2d.vec.b32.clamp [a, b], c;`
    Formatted2d(Formatted<Coordinate2d>),
    /// `sust.p.3d.vec.b32.clamp [a, b], c;`
    Formatted3d(Formatted<Coordinate3d>),
}

/// `[a, b], c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<TCoordinates> {
    /// `.cop`
    pub cache_operator: Option<CacheOperator>,
    /// `.vec`
    pub vector: Vector,
    /// `.ctype`
    pub component_type: ComponentType,
    /// `.clamp`
    pub clamp: Clamp,
    /// `a`
    pub surface: Surface,
    /// `b`
    pub coordinates: TCoordinates,
    /// `c`
    pub value: RegisterOperand,
}

/// `[a, b], c;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Formatted<TCoordinates> {
    /// `.vec`
    pub vector: Vector,
    /// `.b32`
    pub component_type: FormattedComponentType,
    /// `.clamp`
    pub clamp: Clamp,
    /// `a`
    pub surface: Surface,
    /// `b`
    pub coordinates: TCoordinates,
    /// `c`
    pub value: RegisterOperand,
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

/// `.ctype`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    /// `.b8`
    B8,
    /// `.b16`
    B16,
    /// `.b32`
    B32,
    /// `.b64`
    B64,
}

/// `.b32`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormattedComponentType {
    /// `.b32`
    B32,
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
    /// `.wb`
    Wb,
    /// `.cg`
    Cg,
    /// `.cs`
    Cs,
    /// `.wt`
    Wt,
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
