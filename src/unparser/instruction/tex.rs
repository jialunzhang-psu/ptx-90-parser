//! Original PTX specification:
//!
//! tex.geom.v4.dtype.ctype  d{|p}, [a, c] {, e} {, f};
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
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tex");
                    match &self.geom {
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(p_0) = self.p.as_ref() {
                        tokens.push(PtxToken::Pipe);
                        p_0.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_1) = self.e.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_1.unparse_tokens_mode(tokens, spaced);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_2) = self.f.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_2.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for TexGeomV4DtypeCtype1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tex");
                    match &self.geom {
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(p_3) = self.p.as_ref() {
                        tokens.push(PtxToken::Pipe);
                        p_3.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_4) = self.e.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_4.unparse_tokens_mode(tokens, spaced);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_5) = self.f.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_5.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for TexGeomV2F16x2Ctype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tex");
                    match &self.geom {
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(p_6) = self.p.as_ref() {
                        tokens.push(PtxToken::Pipe);
                        p_6.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_7) = self.e.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_7.unparse_tokens_mode(tokens, spaced);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_8) = self.f.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_8.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for TexGeomV2F16x2Ctype1 {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tex");
                    match &self.geom {
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(p_9) = self.p.as_ref() {
                        tokens.push(PtxToken::Pipe);
                        p_9.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_10) = self.e.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_10.unparse_tokens_mode(tokens, spaced);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_11) = self.f.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_11.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for TexBaseGeomV4DtypeCtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tex");
                    push_directive(tokens, "base");
                    match &self.geom {
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(p_12) = self.p.as_ref() {
                        tokens.push(PtxToken::Pipe);
                        p_12.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_13) = self.e.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_13.unparse_tokens_mode(tokens, spaced);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_14) = self.f.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_14.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for TexLevelGeomV4DtypeCtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tex");
                    push_directive(tokens, "level");
                    match &self.geom {
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(p_15) = self.p.as_ref() {
                        tokens.push(PtxToken::Pipe);
                        p_15.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.lod.unparse_tokens_mode(tokens, spaced);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_16) = self.e.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_16.unparse_tokens_mode(tokens, spaced);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_17) = self.f.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_17.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for TexGradGeomV4DtypeCtype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tex");
                    push_directive(tokens, "grad");
                    match &self.geom {
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(p_18) = self.p.as_ref() {
                        tokens.push(PtxToken::Pipe);
                        p_18.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.dpdx.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.dpdy.unparse_tokens_mode(tokens, spaced);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_19) = self.e.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_19.unparse_tokens_mode(tokens, spaced);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_20) = self.f.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_20.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for TexBaseGeomV2F16x2Ctype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tex");
                    push_directive(tokens, "base");
                    match &self.geom {
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(p_21) = self.p.as_ref() {
                        tokens.push(PtxToken::Pipe);
                        p_21.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_22) = self.e.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_22.unparse_tokens_mode(tokens, spaced);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_23) = self.f.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_23.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for TexLevelGeomV2F16x2Ctype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tex");
                    push_directive(tokens, "level");
                    match &self.geom {
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(p_24) = self.p.as_ref() {
                        tokens.push(PtxToken::Pipe);
                        p_24.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.lod.unparse_tokens_mode(tokens, spaced);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_25) = self.e.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_25.unparse_tokens_mode(tokens, spaced);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_26) = self.f.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_26.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for TexGradGeomV2F16x2Ctype {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "tex");
                    push_directive(tokens, "grad");
                    match &self.geom {
                            Geom::Acube => {
                                    push_directive(tokens, "acube");
                            }
                            Geom::A2dms => {
                                    push_directive(tokens, "a2dms");
                            }
                            Geom::Cube => {
                                    push_directive(tokens, "cube");
                            }
                            Geom::_2dms => {
                                    push_directive(tokens, "2dms");
                            }
                            Geom::A1d => {
                                    push_directive(tokens, "a1d");
                            }
                            Geom::A2d => {
                                    push_directive(tokens, "a2d");
                            }
                            Geom::_1d => {
                                    push_directive(tokens, "1d");
                            }
                            Geom::_2d => {
                                    push_directive(tokens, "2d");
                            }
                            Geom::_3d => {
                                    push_directive(tokens, "3d");
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
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(p_27) = self.p.as_ref() {
                        tokens.push(PtxToken::Pipe);
                        p_27.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.dpdx.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.dpdy.unparse_tokens_mode(tokens, spaced);
            if self.e.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_28) = self.e.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_28.unparse_tokens_mode(tokens, spaced);
                    }
            if self.f.is_some() { tokens.push(PtxToken::Comma); }
                    if let Some(opt_29) = self.f.as_ref() {
                        if spaced { tokens.push(PtxToken::Space); }
                        opt_29.unparse_tokens_mode(tokens, spaced);
                    }
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

