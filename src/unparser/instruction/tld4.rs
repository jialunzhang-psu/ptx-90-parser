//! Original PTX specification:
//!
//! tld4.comp.2d.v4.dtype.f32    d{|p}, [a, c] {, e} {, f};
//! tld4.comp.geom.v4.dtype.f32  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
//! .comp  = { .r, .g, .b, .a };
//! .geom  = { .2d, .a2d, .cube, .acube };
//! .dtype = { .u32, .s32, .f32 };

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tld4::section_0::*;

    impl PtxUnparser for Tld4Comp2dV4DtypeF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tld4");
            match &self.comp {
                Comp::R => {
                    push_directive(tokens, "r");
                }
                Comp::G => {
                    push_directive(tokens, "g");
                }
                Comp::B => {
                    push_directive(tokens, "b");
                }
                Comp::A => {
                    push_directive(tokens, "a");
                }
            }
            push_directive(tokens, "2d");
            push_directive(tokens, "v4");
            match &self.dtype {
                Dtype::U32 => {
                    push_directive(tokens, "u32");
                }
                Dtype::S32 => {
                    push_directive(tokens, "s32");
                }
                Dtype::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            push_directive(tokens, "f32");
            self.d.unparse_tokens(tokens);
            if let Some(p_0) = self.p.as_ref() {
                tokens.push(PtxToken::Pipe);
                p_0.unparse_tokens(tokens);
            }
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            if self.e.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_1) = self.e.as_ref() {
                opt_1.unparse_tokens(tokens);
            }
            if self.f.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_2) = self.f.as_ref() {
                opt_2.unparse_tokens(tokens);
            }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for Tld4CompGeomV4DtypeF32 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tld4");
            match &self.comp {
                Comp::R => {
                    push_directive(tokens, "r");
                }
                Comp::G => {
                    push_directive(tokens, "g");
                }
                Comp::B => {
                    push_directive(tokens, "b");
                }
                Comp::A => {
                    push_directive(tokens, "a");
                }
            }
            match &self.geom {
                Geom::Acube => {
                    push_directive(tokens, "acube");
                }
                Geom::Cube => {
                    push_directive(tokens, "cube");
                }
                Geom::A2d => {
                    push_directive(tokens, "a2d");
                }
                Geom::_2d => {
                    push_directive(tokens, "2d");
                }
            }
            push_directive(tokens, "v4");
            match &self.dtype {
                Dtype::U32 => {
                    push_directive(tokens, "u32");
                }
                Dtype::S32 => {
                    push_directive(tokens, "s32");
                }
                Dtype::F32 => {
                    push_directive(tokens, "f32");
                }
            }
            push_directive(tokens, "f32");
            self.d.unparse_tokens(tokens);
            if let Some(p_3) = self.p.as_ref() {
                tokens.push(PtxToken::Pipe);
                p_3.unparse_tokens(tokens);
            }
            tokens.push(PtxToken::Comma);
            self.a.unparse_tokens(tokens);
            if self.e.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_4) = self.e.as_ref() {
                opt_4.unparse_tokens(tokens);
            }
            if self.f.is_some() {
                tokens.push(PtxToken::Comma);
            }
            if let Some(opt_5) = self.f.as_ref() {
                opt_5.unparse_tokens(tokens);
            }
            tokens.push(PtxToken::Semicolon);
        }
    }
}
