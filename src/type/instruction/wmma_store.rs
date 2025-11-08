//! Original PTX specification:
//!
//! wmma.store.d.sync.aligned.layout.shape{.ss}.type [p], r {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .ss     = {.global, .shared, .shared::cta};
//! .type   = {.f16, .f32, .s32};
//! ----------------------------------------------------------------
//! wmma.store.d.sync.aligned.layout.shape{.ss}.type [p], r {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k32, .m8n8k128};
//! .ss     = {.global, .shared, .shared::cta};
//! .type   = {.s32};
//! ----------------------------------------------------------------
//! wmma.store.d.sync.aligned.layout.shape{.ss}.type [p], r {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k8};
//! .ss     = {.global, .shared, .shared::cta};
//! .type   = {.f32};
//! ----------------------------------------------------------------
//! wmma.store.d.sync.aligned.layout.shape{.ss}.type [p], r {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k4 };
//! .ss     = {.global, .shared, .shared::cta};
//! .type   = {.f64};

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Layout {
        Row, // .row
        Col, // .col
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape {
        M16n16k16, // .m16n16k16
        M8n32k16,  // .m8n32k16
        M32n8k16,  // .m32n8k16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCta, // .shared::cta
        Global,    // .global
        Shared,    // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        F16, // .f16
        F32, // .f32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaStoreDSyncAlignedLayoutShapeSsType {
        pub store: (),                      // .store
        pub d: (),                          // .d
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub type_: Type,                    // .type
        pub p: AddressOperand,              // [p]
        pub r: GeneralOperand,              // r
        pub stride: Option<GeneralOperand>, // {, stride}
    }
}

pub mod section_1 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Layout {
        Row, // .row
        Col, // .col
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape {
        M8n8k128, // .m8n8k128
        M8n8k32,  // .m8n8k32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCta, // .shared::cta
        Global,    // .global
        Shared,    // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaStoreDSyncAlignedLayoutShapeSsType1 {
        pub store: (),                      // .store
        pub d: (),                          // .d
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub type_: Type,                    // .type
        pub p: AddressOperand,              // [p]
        pub r: GeneralOperand,              // r
        pub stride: Option<GeneralOperand>, // {, stride}
    }
}

pub mod section_2 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Layout {
        Row, // .row
        Col, // .col
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape {
        M16n16k8, // .m16n16k8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCta, // .shared::cta
        Global,    // .global
        Shared,    // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaStoreDSyncAlignedLayoutShapeSsType2 {
        pub store: (),                      // .store
        pub d: (),                          // .d
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub type_: Type,                    // .type
        pub p: AddressOperand,              // [p]
        pub r: GeneralOperand,              // r
        pub stride: Option<GeneralOperand>, // {, stride}
    }
}

pub mod section_3 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Layout {
        Row, // .row
        Col, // .col
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape {
        M8n8k4, // .m8n8k4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCta, // .shared::cta
        Global,    // .global
        Shared,    // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaStoreDSyncAlignedLayoutShapeSsType3 {
        pub store: (),                      // .store
        pub d: (),                          // .d
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub type_: Type,                    // .type
        pub p: AddressOperand,              // [p]
        pub r: GeneralOperand,              // r
        pub stride: Option<GeneralOperand>, // {, stride}
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Layout as Layout0;
pub use section_0::Shape as Shape0;
pub use section_0::Ss as Ss0;
pub use section_0::Type as Type0;
pub use section_0::WmmaStoreDSyncAlignedLayoutShapeSsType;
pub use section_1::Layout as Layout1;
pub use section_1::Shape as Shape1;
pub use section_1::Ss as Ss1;
pub use section_1::Type as Type1;
pub use section_1::WmmaStoreDSyncAlignedLayoutShapeSsType1;
pub use section_2::Layout as Layout2;
pub use section_2::Shape as Shape2;
pub use section_2::Ss as Ss2;
pub use section_2::Type as Type2;
pub use section_2::WmmaStoreDSyncAlignedLayoutShapeSsType2;
pub use section_3::Layout as Layout3;
pub use section_3::Shape as Shape3;
pub use section_3::Ss as Ss3;
pub use section_3::Type as Type3;
pub use section_3::WmmaStoreDSyncAlignedLayoutShapeSsType3;
