//! Original PTX specification:
//!
//! // Floating point format .f16 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.f16, .s8, .u8};
//! .btype  = {.f16, .s8, .u8};
//! .ctype  = {.f16, .f32, .s32};
//! ----------------------------------------------------------------
//! // Alternate floating point format .bf16 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.bf16 };
//! .btype  = {.bf16 };
//! .ctype  = {.f32 };
//! ----------------------------------------------------------------
//! // Alternate floating point format .tf32 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m16n16k8 };
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.tf32 };
//! .btype  = {.tf32 };
//! .ctype  = {.f32 };
//! ----------------------------------------------------------------
//! // Double precision Floating point .f64 loads:
//! wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k4 };
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.f64 };
//! .btype  = {.f64 };
//! .ctype  = {.f64 };
//! ----------------------------------------------------------------
//! // Sub-byte loads:
//! wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k32};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.s4, .u4};
//! .btype  = {.s4, .u4};
//! .ctype  = {.s32};
//! ----------------------------------------------------------------
//! // Single-bit loads:
//! wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride};
//! wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride};
//! wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};
//! .layout = {.row, .col};
//! .shape  = {.m8n8k128};
//! .ss     = {.global, .shared, .shared::cta};
//! .atype  = {.b1};
//! .btype  = {.b1};
//! .ctype  = {.s32};

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
    pub enum Atype {
        F16, // .f16
        S8,  // .s8
        U8,  // .u8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Btype {
        F16, // .f16
        S8,  // .s8
        U8,  // .u8
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ctype {
        F16, // .f16
        F32, // .f32
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadASyncAlignedLayoutShapeSsAtype {
        pub load: (),                       // .load
        pub a: (),                          // .a
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub atype: Atype,                   // .atype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadBSyncAlignedLayoutShapeSsBtype {
        pub load: (),                       // .load
        pub b: (),                          // .b
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub btype: Btype,                   // .btype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadCSyncAlignedLayoutShapeSsCtype {
        pub load: (),                       // .load
        pub c: (),                          // .c
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub ctype: Ctype,                   // .ctype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
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
    pub enum Atype {
        Bf16, // .bf16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Btype {
        Bf16, // .bf16
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ctype {
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadASyncAlignedLayoutShapeSsAtype1 {
        pub load: (),                       // .load
        pub a: (),                          // .a
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub atype: Atype,                   // .atype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadBSyncAlignedLayoutShapeSsBtype1 {
        pub load: (),                       // .load
        pub b: (),                          // .b
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub btype: Btype,                   // .btype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadCSyncAlignedLayoutShapeSsCtype1 {
        pub load: (),                       // .load
        pub c: (),                          // .c
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub ctype: Ctype,                   // .ctype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
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
    pub enum Atype {
        Tf32, // .tf32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Btype {
        Tf32, // .tf32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ctype {
        F32, // .f32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadASyncAlignedLayoutShapeSsAtype2 {
        pub load: (),                       // .load
        pub a: (),                          // .a
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub atype: Atype,                   // .atype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadBSyncAlignedLayoutShapeSsBtype2 {
        pub load: (),                       // .load
        pub b: (),                          // .b
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub btype: Btype,                   // .btype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadCSyncAlignedLayoutShapeSsCtype2 {
        pub load: (),                       // .load
        pub c: (),                          // .c
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub ctype: Ctype,                   // .ctype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
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
    pub enum Atype {
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Btype {
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ctype {
        F64, // .f64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadASyncAlignedLayoutShapeSsAtype3 {
        pub load: (),                       // .load
        pub a: (),                          // .a
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub atype: Atype,                   // .atype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadBSyncAlignedLayoutShapeSsBtype3 {
        pub load: (),                       // .load
        pub b: (),                          // .b
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub btype: Btype,                   // .btype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadCSyncAlignedLayoutShapeSsCtype3 {
        pub load: (),                       // .load
        pub c: (),                          // .c
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub ctype: Ctype,                   // .ctype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }
}

pub mod section_4 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape {
        M8n8k32, // .m8n8k32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCta, // .shared::cta
        Global,    // .global
        Shared,    // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Atype {
        S4, // .s4
        U4, // .u4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Btype {
        S4, // .s4
        U4, // .u4
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Layout {
        Row, // .row
        Col, // .col
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ctype {
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadASyncAlignedRowShapeSsAtype {
        pub load: (),                       // .load
        pub a: (),                          // .a
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub row: (),                        // .row
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub atype: Atype,                   // .atype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadBSyncAlignedColShapeSsBtype {
        pub load: (),                       // .load
        pub b: (),                          // .b
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub col: (),                        // .col
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub btype: Btype,                   // .btype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadCSyncAlignedLayoutShapeSsCtype4 {
        pub load: (),                       // .load
        pub c: (),                          // .c
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub ctype: Ctype,                   // .ctype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }
}

pub mod section_5 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape {
        M8n8k128, // .m8n8k128
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        SharedCta, // .shared::cta
        Global,    // .global
        Shared,    // .shared
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Atype {
        B1, // .b1
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Btype {
        B1, // .b1
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Layout {
        Row, // .row
        Col, // .col
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ctype {
        S32, // .s32
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadASyncAlignedRowShapeSsAtype1 {
        pub load: (),                       // .load
        pub a: (),                          // .a
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub row: (),                        // .row
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub atype: Atype,                   // .atype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadBSyncAlignedColShapeSsBtype1 {
        pub load: (),                       // .load
        pub b: (),                          // .b
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub col: (),                        // .col
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub btype: Btype,                   // .btype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct WmmaLoadCSyncAlignedLayoutShapeSsCtype5 {
        pub load: (),                       // .load
        pub c: (),                          // .c
        pub sync: (),                       // .sync
        pub aligned: (),                    // .aligned
        pub layout: Layout,                 // .layout
        pub shape: Shape,                   // .shape
        pub ss: Option<Ss>,                 // {.ss}
        pub ctype: Ctype,                   // .ctype
        pub r: GeneralOperand,              // r
        pub p: AddressOperand,              // [p]
        pub stride: Option<GeneralOperand>, // {, stride}
    }
}

// Re-export types with section suffixes to avoid naming conflicts
// e.g., Type0 for section_0::Type, Type1 for section_1::Type
pub use section_0::Atype as Atype0;
pub use section_0::Btype as Btype0;
pub use section_0::Ctype as Ctype0;
pub use section_0::Layout as Layout0;
pub use section_0::Shape as Shape0;
pub use section_0::Ss as Ss0;
pub use section_0::WmmaLoadASyncAlignedLayoutShapeSsAtype;
pub use section_0::WmmaLoadBSyncAlignedLayoutShapeSsBtype;
pub use section_0::WmmaLoadCSyncAlignedLayoutShapeSsCtype;
pub use section_1::Atype as Atype1;
pub use section_1::Btype as Btype1;
pub use section_1::Ctype as Ctype1;
pub use section_1::Layout as Layout1;
pub use section_1::Shape as Shape1;
pub use section_1::Ss as Ss1;
pub use section_1::WmmaLoadASyncAlignedLayoutShapeSsAtype1;
pub use section_1::WmmaLoadBSyncAlignedLayoutShapeSsBtype1;
pub use section_1::WmmaLoadCSyncAlignedLayoutShapeSsCtype1;
pub use section_2::Atype as Atype2;
pub use section_2::Btype as Btype2;
pub use section_2::Ctype as Ctype2;
pub use section_2::Layout as Layout2;
pub use section_2::Shape as Shape2;
pub use section_2::Ss as Ss2;
pub use section_2::WmmaLoadASyncAlignedLayoutShapeSsAtype2;
pub use section_2::WmmaLoadBSyncAlignedLayoutShapeSsBtype2;
pub use section_2::WmmaLoadCSyncAlignedLayoutShapeSsCtype2;
pub use section_3::Atype as Atype3;
pub use section_3::Btype as Btype3;
pub use section_3::Ctype as Ctype3;
pub use section_3::Layout as Layout3;
pub use section_3::Shape as Shape3;
pub use section_3::Ss as Ss3;
pub use section_3::WmmaLoadASyncAlignedLayoutShapeSsAtype3;
pub use section_3::WmmaLoadBSyncAlignedLayoutShapeSsBtype3;
pub use section_3::WmmaLoadCSyncAlignedLayoutShapeSsCtype3;
pub use section_4::Atype as Atype4;
pub use section_4::Btype as Btype4;
pub use section_4::Ctype as Ctype4;
pub use section_4::Layout as Layout4;
pub use section_4::Shape as Shape4;
pub use section_4::Ss as Ss4;
pub use section_4::WmmaLoadASyncAlignedRowShapeSsAtype;
pub use section_4::WmmaLoadBSyncAlignedColShapeSsBtype;
pub use section_4::WmmaLoadCSyncAlignedLayoutShapeSsCtype4;
pub use section_5::Atype as Atype5;
pub use section_5::Btype as Btype5;
pub use section_5::Ctype as Ctype5;
pub use section_5::Layout as Layout5;
pub use section_5::Shape as Shape5;
pub use section_5::Ss as Ss5;
pub use section_5::WmmaLoadASyncAlignedRowShapeSsAtype1;
pub use section_5::WmmaLoadBSyncAlignedColShapeSsBtype1;
pub use section_5::WmmaLoadCSyncAlignedLayoutShapeSsCtype5;
