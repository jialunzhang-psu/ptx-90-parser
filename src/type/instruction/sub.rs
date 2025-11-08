//! Original PTX specification:
//!
//! sub.type       d, a, b;
//! sub{.sat}.s32  d, a, b;     // .sat applies only to .s32
//! .type = { .u16, .u32, .u64,
//! .s16, .s32, .s64 };
//! --------------------------------------------
//! sub{.rnd}{.ftz}{.sat}.f32  d, a, b;
//! sub{.rnd}{.ftz}.f32x2      d, a, b;
//! sub{.rnd}.f64              d, a, b;
//! .rnd = { .rn, .rz, .rm, .rp };
//! --------------------------------------------
//! sub{.rnd}{.ftz}{.sat}.f16   d, a, b;
//! sub{.rnd}{.ftz}{.sat}.f16x2 d, a, b;
//! sub{.rnd}.bf16   d, a, b;
//! sub{.rnd}.bf16x2 d, a, b;
//! .rnd = { .rn };
//! --------------------------------------------
//! sub{.rnd}{.sat}.f32.atype  d, a, c;
//! .atype = { .f16, .bf16};
//! .rnd   = { .rn, .rz, .rm, .rp };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        U16, // .u16
        U32, // .u32
        U64, // .u64
        S16, // .s16
        S32, // .s32
        S64, // .s64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubType {
        pub type_: Type,       // .type
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubSatS32 {
        pub sat: bool,         // {.sat}
        pub s32: (),           // .s32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }
}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndFtzSatF32 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub ftz: bool,         // {.ftz}
        pub sat: bool,         // {.sat}
        pub f32: (),           // .f32
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndFtzF32x2 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub ftz: bool,         // {.ftz}
        pub f32x2: (),         // .f32x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndF64 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub f64: (),           // .f64
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }
}

pub mod section_2 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndFtzSatF16 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub ftz: bool,         // {.ftz}
        pub sat: bool,         // {.sat}
        pub f16: (),           // .f16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndFtzSatF16x2 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub ftz: bool,         // {.ftz}
        pub sat: bool,         // {.sat}
        pub f16x2: (),         // .f16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndBf16 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub bf16: (),          // .bf16
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndBf16x2 {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub bf16x2: (),        // .bf16x2
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub b: GeneralOperand, // b
    }
}

pub mod section_3 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Rnd {
        Rn, // .rn
        Rz, // .rz
        Rm, // .rm
        Rp, // .rp
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Atype {
        Bf16, // .bf16
        F16,  // .f16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SubRndSatF32Atype {
        pub rnd: Option<Rnd>,  // {.rnd}
        pub sat: bool,         // {.sat}
        pub f32: (),           // .f32
        pub atype: Atype,      // .atype
        pub d: GeneralOperand, // d
        pub a: GeneralOperand, // a
        pub c: GeneralOperand, // c
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::SubSatS32;
pub use section_0::SubType;
pub use section_0::Type as Type0;
pub use section_1::Rnd as Rnd1;
pub use section_1::SubRndF64;
pub use section_1::SubRndFtzF32x2;
pub use section_1::SubRndFtzSatF32;
pub use section_2::Rnd as Rnd2;
pub use section_2::SubRndBf16;
pub use section_2::SubRndBf16x2;
pub use section_2::SubRndFtzSatF16;
pub use section_2::SubRndFtzSatF16x2;
pub use section_3::Atype as Atype3;
pub use section_3::Rnd as Rnd3;
pub use section_3::SubRndSatF32Atype;
