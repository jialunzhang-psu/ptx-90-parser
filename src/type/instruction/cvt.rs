use crate::r#type::common::RegisterOperand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cvt {
    /// `cvt{.irnd}{.ftz}{.sat}.dtype.atype  d, a;`
    /// `cvt{.frnd}{.ftz}{.sat}.dtype.atype  d, a;`
    Basic(Basic),
    /// `cvt.frnd2{.relu}{.satfinite}.f16.f32  d, a;`
    /// `cvt.frnd2{.relu}{.satfinite}.f16x2.f32  d, a, b;`
    /// `cvt.frnd2{.relu}{.satfinite}.bf16.f32  d, a;`
    /// `cvt.frnd2{.relu}{.satfinite}.bf16x2.f32  d, a, b;`
    /// `cvt.frnd2{.satfinite}{.relu}.tf32.f32  d, a;`
    Frnd2(Frnd2),
    /// `cvt.rs{.relu}{.satfinite}.f16x2.f32  d, a, b, rbits;`
    /// `cvt.rs{.relu}{.satfinite}.bf16x2.f32  d, a, b, rbits;`
    /// `cvt.rs{.relu}.satfinite.f8x4type.f32  d, {a, b, e, f}, rbits;`
    /// `cvt.rs{.relu}.satfinite.f4x4type.f32  d, {a, b, e, f}, rbits;`
    /// `cvt.rs{.relu}.satfinite.f6x4type.f32  d, {a, b, e, f}, rbits;`
    Rs(Rs),
    /// `cvt.rna{.satfinite}.tf32.f32  d, a;`
    Rna(Rna),
    /// `cvt.rn.satfinite{.relu}.f8x2type.f32  d, a, b;`
    /// `cvt.rn.satfinite{.relu}.f8x2type.f16x2  d, a;`
    /// `cvt.rn.{.relu}.f16x2.f8x2type  d, a;`
    /// `cvt.rn.satfinite{.relu}.f4x2type.f32  d, a, b;`
    /// `cvt.rn{.relu}.f16x2.f4x2type  d, a;`
    /// `cvt.rn.satfinite{.relu}.f6x2type.f32  d, a, b;`
    /// `cvt.rn{.relu}.f16x2.f6x2type  d, a;`
    /// `cvt.rn.bf16x2.ue8m0x2  d, a;`
    Rn(Rn),
    /// `cvt.frnd3{.satfinite}.ue8m0x2.f32  d, a, b;`
    /// `cvt.frnd3{.satfinite}.ue8m0x2.bf16x2  d, a;`
    Frnd3(Frnd3),
}

/// `cvt{.irnd}{.ftz}{.sat}.dtype.atype  d, a;`
/// `cvt{.frnd}{.ftz}{.sat}.dtype.atype  d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Basic {
    /// `{.irnd | .frnd}`
    pub rounding: Option<Rounding>,
    /// `.ftz`
    pub flush_to_zero: bool,
    /// `.sat`
    pub saturate: bool,
    /// `.dtype`
    pub destination_type: ScalarType,
    /// `.atype`
    pub source_type: ScalarType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rounding {
    /// `.irnd`
    Integer(IntegerRounding),
    /// `.frnd`
    Float(FloatRounding),
}

/// `.irnd = { .rni, .rzi, .rmi, .rpi };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerRounding {
    /// `.rni`
    Rni,
    /// `.rzi`
    Rzi,
    /// `.rmi`
    Rmi,
    /// `.rpi`
    Rpi,
}

/// `.frnd = { .rn, .rz, .rm, .rp };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatRounding {
    /// `.rn`
    Rn,
    /// `.rz`
    Rz,
    /// `.rm`
    Rm,
    /// `.rp`
    Rp,
}

/// `.dtype = .atype = { .u8, .u16, .u32, .u64, .s8, .s16, .s32, .s64, .bf16, .f16, .f32, .f64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarType {
    /// `.u8`
    U8,
    /// `.u16`
    U16,
    /// `.u32`
    U32,
    /// `.u64`
    U64,
    /// `.s8`
    S8,
    /// `.s16`
    S16,
    /// `.s32`
    S32,
    /// `.s64`
    S64,
    /// `.bf16`
    Bf16,
    /// `.f16`
    F16,
    /// `.f32`
    F32,
    /// `.f64`
    F64,
}

/// `cvt.frnd2{.relu}{.satfinite}.f16.f32  d, a;`
/// `cvt.frnd2{.relu}{.satfinite}.f16x2.f32  d, a, b;`
/// `cvt.frnd2{.relu}{.satfinite}.bf16.f32  d, a;`
/// `cvt.frnd2{.relu}{.satfinite}.bf16x2.f32  d, a, b;`
/// `cvt.frnd2{.satfinite}{.relu}.tf32.f32  d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frnd2 {
    /// `.frnd2`
    pub rounding: Frnd2Rounding,
    /// `.relu`
    pub relu: bool,
    /// `.satfinite`
    pub satfinite: bool,
    /// `.f16.f32 | .f16x2.f32 | .bf16.f32 | .bf16x2.f32 | .tf32.f32`
    pub kind: Frnd2Kind,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: Option<RegisterOperand>,
}

/// `.frnd2 = { .rn, .rz };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frnd2Rounding {
    /// `.rn`
    Rn,
    /// `.rz`
    Rz,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frnd2Kind {
    /// `.f16.f32`
    F16FromF32,
    /// `.f16x2.f32`
    F16x2FromF32,
    /// `.bf16.f32`
    Bf16FromF32,
    /// `.bf16x2.f32`
    Bf16x2FromF32,
    /// `.tf32.f32`
    Tf32FromF32,
}

/// `cvt.rs{.relu}{.satfinite}.f16x2.f32  d, a, b, rbits;`
/// `cvt.rs{.relu}{.satfinite}.bf16x2.f32  d, a, b, rbits;`
/// `cvt.rs{.relu}.satfinite.f8x4type.f32  d, {a, b, e, f}, rbits;`
/// `cvt.rs{.relu}.satfinite.f4x4type.f32  d, {a, b, e, f}, rbits;`
/// `cvt.rs{.relu}.satfinite.f6x4type.f32  d, {a, b, e, f}, rbits;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rs {
    /// `.relu`
    pub relu: bool,
    /// `.satfinite`
    pub satfinite: bool,
    /// `.f16x2.f32 | .bf16x2.f32 | .f8x4type.f32 | .f4x4type.f32 | .f6x4type.f32`
    pub kind: RsKind,
    /// `d`
    pub destination: RegisterOperand,
    /// `rbits`
    pub rbits: RegisterOperand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RsKind {
    /// `.f16x2.f32`
    F16x2FromF32 {
        /// `a`
        a: RegisterOperand,
        /// `b`
        b: RegisterOperand,
    },
    /// `.bf16x2.f32`
    Bf16x2FromF32 {
        /// `a`
        a: RegisterOperand,
        /// `b`
        b: RegisterOperand,
    },
    /// `.f8x4type.f32`
    F8x4FromF32 {
        /// `.f8x4type`
        data_type: F8x4Type,
        /// `a`
        a: RegisterOperand,
        /// `b`
        b: RegisterOperand,
        /// `e`
        e: RegisterOperand,
        /// `f`
        f: RegisterOperand,
    },
    /// `.f4x4type.f32`
    F4x4FromF32 {
        /// `.f4x4type`
        data_type: F4x4Type,
        /// `a`
        a: RegisterOperand,
        /// `b`
        b: RegisterOperand,
        /// `e`
        e: RegisterOperand,
        /// `f`
        f: RegisterOperand,
    },
    /// `.f6x4type.f32`
    F6x4FromF32 {
        /// `.f6x4type`
        data_type: F6x4Type,
        /// `a`
        a: RegisterOperand,
        /// `b`
        b: RegisterOperand,
        /// `e`
        e: RegisterOperand,
        /// `f`
        f: RegisterOperand,
    },
}

/// `cvt.rna{.satfinite}.tf32.f32  d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rna {
    /// `.satfinite`
    pub satfinite: bool,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

/// `cvt.rn.satfinite{.relu}.f8x2type.f32  d, a, b;`
/// `cvt.rn.satfinite{.relu}.f8x2type.f16x2  d, a;`
/// `cvt.rn.{.relu}.f16x2.f8x2type  d, a;`
/// `cvt.rn.satfinite{.relu}.f4x2type.f32  d, a, b;`
/// `cvt.rn{.relu}.f16x2.f4x2type  d, a;`
/// `cvt.rn.satfinite{.relu}.f6x2type.f32  d, a, b;`
/// `cvt.rn{.relu}.f16x2.f6x2type  d, a;`
/// `cvt.rn.bf16x2.ue8m0x2  d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rn {
    /// `.satfinite`
    pub satfinite: bool,
    /// `.relu`
    pub relu: bool,
    /// `.f8x2type | .f4x2type | .f6x2type | .bf16x2.ue8m0x2`
    pub kind: RnKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RnKind {
    /// `.f8x2type.f32  d, a, b;`
    F8x2FromF32 {
        /// `.f8x2type`
        data_type: F8x2Type,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        a: RegisterOperand,
        /// `b`
        b: RegisterOperand,
    },
    /// `.f8x2type.f16x2  d, a;`
    F8x2FromF16x2 {
        /// `.f8x2type`
        data_type: F8x2Type,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        a: RegisterOperand,
    },
    /// `.f16x2.f8x2type  d, a;`
    F16x2FromF8x2 {
        /// `.f8x2type`
        data_type: F8x2Type,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        a: RegisterOperand,
    },
    /// `.f4x2type.f32  d, a, b;`
    F4x2FromF32 {
        /// `.f4x2type`
        data_type: F4x2Type,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        a: RegisterOperand,
        /// `b`
        b: RegisterOperand,
    },
    /// `.f16x2.f4x2type  d, a;`
    F16x2FromF4x2 {
        /// `.f4x2type`
        data_type: F4x2Type,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        a: RegisterOperand,
    },
    /// `.f6x2type.f32  d, a, b;`
    F6x2FromF32 {
        /// `.f6x2type`
        data_type: F6x2Type,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        a: RegisterOperand,
        /// `b`
        b: RegisterOperand,
    },
    /// `.f16x2.f6x2type  d, a;`
    F16x2FromF6x2 {
        /// `.f6x2type`
        data_type: F6x2Type,
        /// `d`
        destination: RegisterOperand,
        /// `a`
        a: RegisterOperand,
    },
    /// `.bf16x2.ue8m0x2  d, a;`
    Bf16x2FromUe8m0x2 {
        /// `d`
        destination: RegisterOperand,
        /// `a`
        a: RegisterOperand,
    },
}

/// `cvt.frnd3{.satfinite}.ue8m0x2.f32  d, a, b;`
/// `cvt.frnd3{.satfinite}.ue8m0x2.bf16x2  d, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frnd3 {
    /// `.frnd3`
    pub rounding: Frnd3Rounding,
    /// `.satfinite`
    pub satfinite: bool,
    /// `.ue8m0x2.f32 | .ue8m0x2.bf16x2`
    pub kind: Frnd3Kind,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a: RegisterOperand,
    /// `b`
    pub b: Option<RegisterOperand>,
}

/// `.frnd3 = { .rz, .rp };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frnd3Rounding {
    /// `.rz`
    Rz,
    /// `.rp`
    Rp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frnd3Kind {
    /// `.ue8m0x2.f32`
    Ue8m0x2FromF32,
    /// `.ue8m0x2.bf16x2`
    Ue8m0x2FromBf16x2,
}

/// `.f8x2type = { .e4m3x2, .e5m2x2 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F8x2Type {
    /// `.e4m3x2`
    E4m3x2,
    /// `.e5m2x2`
    E5m2x2,
}

/// `.f4x2type = { .e2m1x2 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F4x2Type {
    /// `.e2m1x2`
    E2m1x2,
}

/// `.f6x2type = { .e2m3x2, .e3m2x2 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F6x2Type {
    /// `.e2m3x2`
    E2m3x2,
    /// `.e3m2x2`
    E3m2x2,
}

/// `.f8x4type = { .e4m3x4, .e5m2x4 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F8x4Type {
    /// `.e4m3x4`
    E4m3x4,
    /// `.e5m2x4`
    E5m2x4,
}

/// `.f4x4type = { .e2m1x4 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F4x4Type {
    /// `.e2m1x4`
    E2m1x4,
}

/// `.f6x4type = { .e2m3x4, .e3m2x4 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F6x4Type {
    /// `.e2m3x4`
    E2m3x4,
    /// `.e3m2x4`
    E3m2x4,
}
