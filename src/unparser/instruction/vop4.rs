//! Original PTX specification:
//!
//! // SIMD instruction with secondary SIMD merge operation
//! vop4.dtype.atype.btype{.sat}  d{.mask}, a{.asel}, b{.bsel}, c;
//! // SIMD instruction with secondary accumulate operation
//! vop4.dtype.atype.btype.add  d{.mask}, a{.asel}, b{.bsel}, c;
//! vop4  = { vadd4, vsub4, vavrg4, vabsdiff4, vmin4, vmax4 };
//! .dtype = .atype = .btype = { .u32, .s32 };
//! .mask  = { .b0,
//! .b1, .b10,
//! .b2, .b20, .b21, .b210,
//! .b3, .b30, .b31, .b310, .b32, .b320, .b321, .b3210 };
//! // defaults to .b3210
//! .asel = .bsel = { .b.n.n.n.n };
//! .n = { 0, 1, 2, 3, 4, 5, 6, 7};
//! // .asel defaults to .b3210
//! // .bsel defaults to .b7654

#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::{PtxUnparser, common::*};

pub mod section_0 {
    use super::*;
    use crate::r#type::instruction::vop4::section_0::*;

    impl PtxUnparser for Vadd4DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vadd4");
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
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_1) = self.asel.as_ref() {
                            match asel_1 {
                                    Asel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_2) = self.bsel.as_ref() {
                            match bsel_2 {
                                    Bsel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
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

    impl PtxUnparser for Vsub4DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vsub4");
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
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_4) = self.asel.as_ref() {
                            match asel_4 {
                                    Asel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_5) = self.bsel.as_ref() {
                            match bsel_5 {
                                    Bsel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
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

    impl PtxUnparser for Vavrg4DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vavrg4");
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
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_7) = self.asel.as_ref() {
                            match asel_7 {
                                    Asel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_8) = self.bsel.as_ref() {
                            match bsel_8 {
                                    Bsel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
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

    impl PtxUnparser for Vabsdiff4DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vabsdiff4");
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
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_10) = self.asel.as_ref() {
                            match asel_10 {
                                    Asel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_11) = self.bsel.as_ref() {
                            match bsel_11 {
                                    Bsel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
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

    impl PtxUnparser for Vmin4DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vmin4");
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
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_13) = self.asel.as_ref() {
                            match asel_13 {
                                    Asel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_14) = self.bsel.as_ref() {
                            match bsel_14 {
                                    Bsel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
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

    impl PtxUnparser for Vmax4DtypeAtypeBtypeSat {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vmax4");
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
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_16) = self.asel.as_ref() {
                            match asel_16 {
                                    Asel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_17) = self.bsel.as_ref() {
                            match bsel_17 {
                                    Bsel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
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

    impl PtxUnparser for Vadd4DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vadd4");
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
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_19) = self.asel.as_ref() {
                            match asel_19 {
                                    Asel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_20) = self.bsel.as_ref() {
                            match bsel_20 {
                                    Bsel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
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

    impl PtxUnparser for Vsub4DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vsub4");
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
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_22) = self.asel.as_ref() {
                            match asel_22 {
                                    Asel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_23) = self.bsel.as_ref() {
                            match bsel_23 {
                                    Bsel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
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

    impl PtxUnparser for Vavrg4DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vavrg4");
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
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_25) = self.asel.as_ref() {
                            match asel_25 {
                                    Asel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_26) = self.bsel.as_ref() {
                            match bsel_26 {
                                    Bsel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
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

    impl PtxUnparser for Vabsdiff4DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vabsdiff4");
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
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_28) = self.asel.as_ref() {
                            match asel_28 {
                                    Asel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_29) = self.bsel.as_ref() {
                            match bsel_29 {
                                    Bsel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
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

    impl PtxUnparser for Vmin4DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vmin4");
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
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_31) = self.asel.as_ref() {
                            match asel_31 {
                                    Asel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_32) = self.bsel.as_ref() {
                            match bsel_32 {
                                    Bsel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
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

    impl PtxUnparser for Vmax4DtypeAtypeBtypeAdd {
        fn unparse_tokens(&self, tokens: &mut ::std::vec::Vec<PtxToken>) {
            self.unparse_tokens_mode(tokens, false);
        }
        fn unparse_tokens_mode(&self, tokens: &mut ::std::vec::Vec<PtxToken>, spaced: bool) {
            push_opcode(tokens, "vmax4");
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
                                    Mask::B3210 => {
                                            push_directive(tokens, "b3210");
                                    }
                                    Mask::B210 => {
                                            push_directive(tokens, "b210");
                                    }
                                    Mask::B310 => {
                                            push_directive(tokens, "b310");
                                    }
                                    Mask::B320 => {
                                            push_directive(tokens, "b320");
                                    }
                                    Mask::B321 => {
                                            push_directive(tokens, "b321");
                                    }
                                    Mask::B10 => {
                                            push_directive(tokens, "b10");
                                    }
                                    Mask::B20 => {
                                            push_directive(tokens, "b20");
                                    }
                                    Mask::B21 => {
                                            push_directive(tokens, "b21");
                                    }
                                    Mask::B30 => {
                                            push_directive(tokens, "b30");
                                    }
                                    Mask::B31 => {
                                            push_directive(tokens, "b31");
                                    }
                                    Mask::B32 => {
                                            push_directive(tokens, "b32");
                                    }
                                    Mask::B0 => {
                                            push_directive(tokens, "b0");
                                    }
                                    Mask::B1 => {
                                            push_directive(tokens, "b1");
                                    }
                                    Mask::B2 => {
                                            push_directive(tokens, "b2");
                                    }
                                    Mask::B3 => {
                                            push_directive(tokens, "b3");
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.a.unparse_tokens_mode(tokens, spaced);
                    if let Some(asel_34) = self.asel.as_ref() {
                            match asel_34 {
                                    Asel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
                                    }
                            }
                    }
            tokens.push(PtxToken::Comma);
                    if spaced { tokens.push(PtxToken::Space); }
                    self.b.unparse_tokens_mode(tokens, spaced);
                    if let Some(bsel_35) = self.bsel.as_ref() {
                            match bsel_35 {
                                    Bsel::BNNNN(_, n1, n2, n3, n4) => {
                                            let mut combined = String::new();
                                            combined.push_str(format!("{:?}", n1).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n2).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n3).trim_start_matches('_'));
                                            combined.push_str(format!("{:?}", n4).trim_start_matches('_'));
                                            tokens.push(PtxToken::Dot);
                                            tokens.push(PtxToken::Identifier(format!("{}{}", "b", combined).into()));
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

