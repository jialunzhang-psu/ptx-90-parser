//! Original PTX specification:
//!
//! tex.geom.v4.dtype.ctype  d, [a, c] {, e} {, f};
//! tex.geom.v4.dtype.ctype  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
//! tex.geom.v2.f16x2.ctype  d{|p}, [a, c] {, e} {, f};
//! tex.geom.v2.f16x2.ctype  d{|p}, [a, b, c] {, e} {, f};  // explicit sampler
//! // mipmaps
//! tex.base.geom.v4.dtype.ctype   d{|p}, [a, {b,} c] {, e} {, f};
//! tex.level.geom.v4.dtype.ctype  d{|p}, [a, {b,} c], lod {, e} {, f};
//! tex.grad.geom.v4.dtype.ctype   d{|p}, [a, {b,} c], dPdx, dPdy {, e} {, f};
//! tex.base.geom.v2.f16x2.ctype   d{|p}, [a, {b,} c] {, e} {, f};
//! tex.level.geom.v2.f16x2.ctype  d{|p}, [a, {b,} c], lod {, e} {, f};
//! tex.grad.geom.v2.f16x2.ctype   d{|p}, [a, {b,} c], dPdx, dPdy {, e} {, f};
//! .geom  = { .1d, .2d, .3d, .a1d, .a2d, .cube, .acube, .2dms, .a2dms };
//! .dtype = { .u32, .s32, .f16,  .f32 };
//! .ctype = {       .s32, .f32 };          // .cube, .acube require .f32
//! // .2dms, .a2dms require .s32

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::tex::section_0::*;

    impl PtxUnparser for TexGeomV4DtypeCtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tex");
                    match &self.geom {
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
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
                            Dtype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    match &self.ctype {
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_0_0, ref group_0_1) = &self.a;
                    group_0_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_0_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_1) = self.e.as_ref() {
                        opt_1.unparse_tokens(tokens);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_2) = self.f.as_ref() {
                        opt_2.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for TexGeomV4DtypeCtype1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tex");
                    match &self.geom {
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
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
                            Dtype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    match &self.ctype {
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_3_0, ref group_3_1, ref group_3_2) = &self.a;
                    group_3_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_3_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_3_2.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_4) = self.e.as_ref() {
                        opt_4.unparse_tokens(tokens);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_5) = self.f.as_ref() {
                        opt_5.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for TexGeomV2F16x2Ctype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tex");
                    match &self.geom {
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                    }
                    push_directive(tokens, "v2");
                    push_directive(tokens, "f16x2");
                    match &self.ctype {
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_6_0, ref group_6_1) = &self.a;
                    group_6_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_6_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_7) = self.e.as_ref() {
                        opt_7.unparse_tokens(tokens);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_8) = self.f.as_ref() {
                        opt_8.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for TexGeomV2F16x2Ctype1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tex");
                    match &self.geom {
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                    }
                    push_directive(tokens, "v2");
                    push_directive(tokens, "f16x2");
                    match &self.ctype {
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_9_0, ref group_9_1, ref group_9_2) = &self.a;
                    group_9_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_9_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_9_2.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_10) = self.e.as_ref() {
                        opt_10.unparse_tokens(tokens);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_11) = self.f.as_ref() {
                        opt_11.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for TexBaseGeomV4DtypeCtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tex");
                    push_directive(tokens, "base");
                    match &self.geom {
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
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
                            Dtype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    match &self.ctype {
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_12_0, ref group_12_1, ref group_12_2) = &self.a;
                    group_12_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_12_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_12_2.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_13) = self.e.as_ref() {
                        opt_13.unparse_tokens(tokens);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_14) = self.f.as_ref() {
                        opt_14.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for TexLevelGeomV4DtypeCtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tex");
                    push_directive(tokens, "level");
                    match &self.geom {
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
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
                            Dtype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    match &self.ctype {
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_15_0, ref group_15_1, ref group_15_2) = &self.a;
                    group_15_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_15_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_15_2.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            tokens.push(PtxToken::Comma);
                    self.lod.unparse_tokens(tokens);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_16) = self.e.as_ref() {
                        opt_16.unparse_tokens(tokens);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_17) = self.f.as_ref() {
                        opt_17.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for TexGradGeomV4DtypeCtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tex");
                    push_directive(tokens, "grad");
                    match &self.geom {
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
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
                            Dtype::F16 => {
                                    push_directive(tokens, "f16");
                            }
                            Dtype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    match &self.ctype {
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_18_0, ref group_18_1, ref group_18_2) = &self.a;
                    group_18_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_18_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_18_2.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            tokens.push(PtxToken::Comma);
                    self.dpdx.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.dpdy.unparse_tokens(tokens);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_19) = self.e.as_ref() {
                        opt_19.unparse_tokens(tokens);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_20) = self.f.as_ref() {
                        opt_20.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for TexBaseGeomV2F16x2Ctype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tex");
                    push_directive(tokens, "base");
                    match &self.geom {
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                    }
                    push_directive(tokens, "v2");
                    push_directive(tokens, "f16x2");
                    match &self.ctype {
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_21_0, ref group_21_1, ref group_21_2) = &self.a;
                    group_21_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_21_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_21_2.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_22) = self.e.as_ref() {
                        opt_22.unparse_tokens(tokens);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_23) = self.f.as_ref() {
                        opt_23.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for TexLevelGeomV2F16x2Ctype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tex");
                    push_directive(tokens, "level");
                    match &self.geom {
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                    }
                    push_directive(tokens, "v2");
                    push_directive(tokens, "f16x2");
                    match &self.ctype {
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_24_0, ref group_24_1, ref group_24_2) = &self.a;
                    group_24_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_24_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_24_2.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            tokens.push(PtxToken::Comma);
                    self.lod.unparse_tokens(tokens);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_25) = self.e.as_ref() {
                        opt_25.unparse_tokens(tokens);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_26) = self.f.as_ref() {
                        opt_26.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

    impl PtxUnparser for TexGradGeomV2F16x2Ctype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            push_opcode(tokens, "tex");
                    push_directive(tokens, "grad");
                    match &self.geom {
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                    }
                    push_directive(tokens, "v2");
                    push_directive(tokens, "f16x2");
                    match &self.ctype {
                            Ctype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                            Ctype::F32 => {
                                    push_directive(tokens, "f32");
                            }
                    }
                    self.d.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    tokens.push(PtxToken::LBracket);
                    let &( ref group_27_0, ref group_27_1, ref group_27_2) = &self.a;
                    group_27_0.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_27_1.unparse_tokens(tokens);
                    tokens.push(PtxToken::Comma);
                    group_27_2.unparse_tokens(tokens);
                    tokens.push(PtxToken::RBracket);
            tokens.push(PtxToken::Comma);
                    self.dpdx.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
                    self.dpdy.unparse_tokens(tokens);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_28) = self.e.as_ref() {
                        opt_28.unparse_tokens(tokens);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_29) = self.f.as_ref() {
                        opt_29.unparse_tokens(tokens);
                    }
            tokens.push(PtxToken::Semicolon);
        }
    }

}

