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
        M8n32k16, // .m8n32k16
        M32n8k16, // .m32n8k16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCta, // .shared::cta
        Global, // .global
        Shared, // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        F16, // .f16
        F32, // .f32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaStoreDSyncAlignedLayoutShapeSsType {
        pub store: (), // .store
        pub d: (), // .d
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub layout: Layout, // .layout
        pub shape: Shape, // .shape
        pub ss: Option<Ss>, // {.ss}
        pub type_: Type, // .type
        pub p: AddressOperand, // [p]
        pub r: GeneralOperand, // r
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
        M8n8k32, // .m8n8k32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCta, // .shared::cta
        Global, // .global
        Shared, // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaStoreDSyncAlignedLayoutShapeSsType1 {
        pub store: (), // .store
        pub d: (), // .d
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub layout: Layout, // .layout
        pub shape: Shape, // .shape
        pub ss: Option<Ss>, // {.ss}
        pub type_: Type, // .type
        pub p: AddressOperand, // [p]
        pub r: GeneralOperand, // r
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
        Global, // .global
        Shared, // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaStoreDSyncAlignedLayoutShapeSsType2 {
        pub store: (), // .store
        pub d: (), // .d
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub layout: Layout, // .layout
        pub shape: Shape, // .shape
        pub ss: Option<Ss>, // {.ss}
        pub type_: Type, // .type
        pub p: AddressOperand, // [p]
        pub r: GeneralOperand, // r
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
        Global, // .global
        Shared, // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaStoreDSyncAlignedLayoutShapeSsType3 {
        pub store: (), // .store
        pub d: (), // .d
        pub sync: (), // .sync
        pub aligned: (), // .aligned
        pub layout: Layout, // .layout
        pub shape: Shape, // .shape
        pub ss: Option<Ss>, // {.ss}
        pub type_: Type, // .type
        pub p: AddressOperand, // [p]
        pub r: GeneralOperand, // r
        pub stride: Option<GeneralOperand>, // {, stride}
    }

}
