use crate::r#type::common::{RegisterOperand, VariableSymbol};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sured {
    /// `sured.b.op.1d.ctype.clamp [a,b],c;`
    Byte1d(Byte<Coordinate1d>),
    /// `sured.b.op.2d.ctype.clamp [a,b],c;`
    Byte2d(Byte<Coordinate2d>),
    /// `sured.b.op.3d.ctype.clamp [a,b],c;`
    Byte3d(Byte<Coordinate3d>),
    /// `sured.p.op.1d.ctype.clamp [a,b],c;`
    Sample1d(Sample<Coordinate1d>),
    /// `sured.p.op.2d.ctype.clamp [a,b],c;`
    Sample2d(Sample<Coordinate2d>),
    /// `sured.p.op.3d.ctype.clamp [a,b],c;`
    Sample3d(Sample<Coordinate3d>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reduction<TDataType, TCoordinates> {
    /// `.op`
    pub operator: Operator,
    /// `.ctype`
    pub data_type: TDataType,
    /// `.clamp`
    pub clamp: Clamp,
    /// `a`
    pub surface: Surface,
    /// `b`
    pub coordinates: TCoordinates,
    /// `c`
    pub source: RegisterOperand,
}

pub type Byte<TCoordinates> = Reduction<ByteDataType, TCoordinates>;
pub type Sample<TCoordinates> = Reduction<SampleDataType, TCoordinates>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// `.add`
    Add,
    /// `.min`
    Min,
    /// `.max`
    Max,
    /// `.and`
    And,
    /// `.or`
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteDataType {
    /// `.u32`
    U32,
    /// `.u64`
    U64,
    /// `.s32`
    S32,
    /// `.b32`
    B32,
    /// `.s64`
    S64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleDataType {
    /// `.b32`
    B32,
    /// `.b64`
    B64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clamp {
    /// `.trap`
    Trap,
    /// `.clamp`
    Clamp,
    /// `.zero`
    Zero,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Surface {
    /// `.surfref`
    Reference(VariableSymbol),
    /// `.u64`
    Indirect(RegisterOperand),
}

/// `.1d`
pub type Coordinate1d = RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordinate2d {
    /// `.2d`
    pub x: RegisterOperand,
    /// `.2d`
    pub y: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordinate3d {
    /// `.3d`
    pub x: RegisterOperand,
    /// `.3d`
    pub y: RegisterOperand,
    /// `.3d`
    pub z: RegisterOperand,
    /// `.3d` (ignored)
    pub w: RegisterOperand,
}
