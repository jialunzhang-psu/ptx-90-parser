//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vop2.dtype.atype.btype{.sat}  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vop2.dtype.atype.btype.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! vop2  = { vadd2, vsub2, vavrg2, vabsdiff2, vmin2, vmax2 };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .mask  = { .h0, .h1, .h10 };  // defaults to .h10
//! .asel  = .bsel  = { .h00, .h01, .h02, .h03, .h10, .h11, .h12, .h13, .h20, .h21, .h22, .h23, .h30, .h31, .h32, .h33 }; 
//! // .asel defaults to .h10
//! // .bsel defaults to .h32

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vop2::section_0::*;

    impl PtxUnparser for Vadd2DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vadd2");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(mask_0) = self.mask.as_ref() {
                            match mask_0 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_1) = self.asel.as_ref() {
                            match asel_1 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_2) = self.bsel.as_ref() {
                            match bsel_2 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for Vsub2DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vsub2");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(mask_3) = self.mask.as_ref() {
                            match mask_3 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_4) = self.asel.as_ref() {
                            match asel_4 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_5) = self.bsel.as_ref() {
                            match bsel_5 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for Vavrg2DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vavrg2");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(mask_6) = self.mask.as_ref() {
                            match mask_6 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_7) = self.asel.as_ref() {
                            match asel_7 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_8) = self.bsel.as_ref() {
                            match bsel_8 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for Vabsdiff2DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vabsdiff2");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(mask_9) = self.mask.as_ref() {
                            match mask_9 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_10) = self.asel.as_ref() {
                            match asel_10 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_11) = self.bsel.as_ref() {
                            match bsel_11 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for Vmin2DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vmin2");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(mask_12) = self.mask.as_ref() {
                            match mask_12 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_13) = self.asel.as_ref() {
                            match asel_13 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_14) = self.bsel.as_ref() {
                            match bsel_14 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for Vmax2DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vmax2");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    if self.sat {
                            push_directive(tokens, "sat");
                    }
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(mask_15) = self.mask.as_ref() {
                            match mask_15 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_16) = self.asel.as_ref() {
                            match asel_16 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_17) = self.bsel.as_ref() {
                            match bsel_17 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for Vadd2DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vadd2");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "add");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(mask_18) = self.mask.as_ref() {
                            match mask_18 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_19) = self.asel.as_ref() {
                            match asel_19 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_20) = self.bsel.as_ref() {
                            match bsel_20 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for Vsub2DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vsub2");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "add");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(mask_21) = self.mask.as_ref() {
                            match mask_21 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_22) = self.asel.as_ref() {
                            match asel_22 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_23) = self.bsel.as_ref() {
                            match bsel_23 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for Vavrg2DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vavrg2");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "add");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(mask_24) = self.mask.as_ref() {
                            match mask_24 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_25) = self.asel.as_ref() {
                            match asel_25 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_26) = self.bsel.as_ref() {
                            match bsel_26 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for Vabsdiff2DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vabsdiff2");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "add");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(mask_27) = self.mask.as_ref() {
                            match mask_27 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_28) = self.asel.as_ref() {
                            match asel_28 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_29) = self.bsel.as_ref() {
                            match bsel_29 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for Vmin2DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vmin2");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "add");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(mask_30) = self.mask.as_ref() {
                            match mask_30 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_31) = self.asel.as_ref() {
                            match asel_31 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_32) = self.bsel.as_ref() {
                            match bsel_32 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

    impl PtxUnparser for Vmax2DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vmax2");
                    match &self.dtype {
                            Dtype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Dtype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.atype {
                            Atype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Atype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    match &self.btype {
                            Btype::U32 => {
                                    push_directive(tokens, "u32");
                            }
                            Btype::S32 => {
                                    push_directive(tokens, "s32");
                            }
                    }
                    push_directive(tokens, "add");
                    if spaced { tokens.push(PtxToken::Space); }
                    self.d.unparse_tokens_mode(tokens, spaced);
                    if let Some(mask_33) = self.mask.as_ref() {
                            match mask_33 {
                                    Mask::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Mask::H0 => {
                                            push_directive(tokens, "h0");
                                    }
                                    Mask::H1 => {
                                            push_directive(tokens, "h1");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_34) = self.asel.as_ref() {
                            match asel_34 {
                                    Asel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Asel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Asel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Asel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Asel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Asel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Asel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Asel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Asel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Asel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Asel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Asel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Asel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Asel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Asel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Asel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_35) = self.bsel.as_ref() {
                            match bsel_35 {
                                    Bsel::H00 => {
                                            push_directive(tokens, "h00");
                                    }
                                    Bsel::H01 => {
                                            push_directive(tokens, "h01");
                                    }
                                    Bsel::H02 => {
                                            push_directive(tokens, "h02");
                                    }
                                    Bsel::H03 => {
                                            push_directive(tokens, "h03");
                                    }
                                    Bsel::H10 => {
                                            push_directive(tokens, "h10");
                                    }
                                    Bsel::H11 => {
                                            push_directive(tokens, "h11");
                                    }
                                    Bsel::H12 => {
                                            push_directive(tokens, "h12");
                                    }
                                    Bsel::H13 => {
                                            push_directive(tokens, "h13");
                                    }
                                    Bsel::H20 => {
                                            push_directive(tokens, "h20");
                                    }
                                    Bsel::H21 => {
                                            push_directive(tokens, "h21");
                                    }
                                    Bsel::H22 => {
                                            push_directive(tokens, "h22");
                                    }
                                    Bsel::H23 => {
                                            push_directive(tokens, "h23");
                                    }
                                    Bsel::H30 => {
                                            push_directive(tokens, "h30");
                                    }
                                    Bsel::H31 => {
                                            push_directive(tokens, "h31");
                                    }
                                    Bsel::H32 => {
                                            push_directive(tokens, "h32");
                                    }
                                    Bsel::H33 => {
                                            push_directive(tokens, "h33");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.c.unparse_tokens_mode(tokens, spaced);
            tokens.push(PtxToken::Semicolon);
            if spaced { tokens.push(PtxToken::Newline); }
        }
    }

}

