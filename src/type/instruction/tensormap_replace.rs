//! Original PTX specification:
//!
//! tensormap.replace.mode.field1{.ss}.b1024.type  [addr], new_val;
//! tensormap.replace.mode.field2{.ss}.b1024.type  [addr], ord, new_val;
//! tensormap.replace.mode.field3{.ss}.b1024.type  [addr], new_val;
//! .mode    = { .tile };
//! .field1  = { .global_address, .rank };
//! .field2  = { .box_dim, .global_dim, .global_stride, .element_stride  };
//! .field3  = { .elemtype,  .interleave_layout, .swizzle_mode, .swizzle_atomicity, .fill_mode };
//! .ss      = { .global, .shared::cta };
//! .type    = { .b32, .b64 };

#![allow(unused)]
use crate::r#type::common::*;

pub mod section_0 {
    use crate::r#type::common::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Mode {
        Tile, // .tile
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Field1 {
        GlobalAddress, // .global_address
        Rank, // .rank
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Ss {
        Global, // .global
        SharedCta, // .shared::cta
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        B32, // .b32
        B64, // .b64
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Field2 {
        BoxDim, // .box_dim
        GlobalDim, // .global_dim
        GlobalStride, // .global_stride
        ElementStride, // .element_stride
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Field3 {
        Elemtype, // .elemtype
        InterleaveLayout, // .interleave_layout
        SwizzleMode, // .swizzle_mode
        SwizzleAtomicity, // .swizzle_atomicity
        FillMode, // .fill_mode
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TensormapReplaceModeField1SsB1024Type {
        pub replace: (), // .replace
        pub mode: Mode, // .mode
        pub field1: Field1, // .field1
        pub ss: Option<Ss>, // {.ss}
        pub b1024: (), // .b1024
        pub type_: Type, // .type
        pub addr: AddressOperand, // [addr]
        pub new_val: Operand, // new_val
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TensormapReplaceModeField2SsB1024Type {
        pub replace: (), // .replace
        pub mode: Mode, // .mode
        pub field2: Field2, // .field2
        pub ss: Option<Ss>, // {.ss}
        pub b1024: (), // .b1024
        pub type_: Type, // .type
        pub addr: AddressOperand, // [addr]
        pub ord: Operand, // ord
        pub new_val: Operand, // new_val
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TensormapReplaceModeField3SsB1024Type {
        pub replace: (), // .replace
        pub mode: Mode, // .mode
        pub field3: Field3, // .field3
        pub ss: Option<Ss>, // {.ss}
        pub b1024: (), // .b1024
        pub type_: Type, // .type
        pub addr: AddressOperand, // [addr]
        pub new_val: Operand, // new_val
    }

}
