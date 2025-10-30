use std::fmt::Debug;

use crate::{lexer::PtxToken, r#type::instruction::wgmma::*, unparser::*};

fn push_lowercase_directive_from_debug<T: Debug>(tokens: &mut Vec<PtxToken>, value: &T) {
    let mut literal = format!("{value:?}");
    literal.make_ascii_lowercase();
    tokens.push(PtxToken::Directive(literal));
}

fn push_wgmma_prefix(tokens: &mut Vec<PtxToken>) {
    push_identifier(tokens, "wgmma");
    push_directive(tokens, "mma_async");
    push_directive(tokens, "sync");
    push_directive(tokens, "aligned");
    push_directive(tokens, "shape");
}

impl PtxUnparser for ShapeK16 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_lowercase_directive_from_debug(tokens, self);
    }
}

impl PtxUnparser for ShapeK8 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_lowercase_directive_from_debug(tokens, self);
    }
}

impl PtxUnparser for ShapeK32 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_lowercase_directive_from_debug(tokens, self);
    }
}

impl PtxUnparser for IntegerShape {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_lowercase_directive_from_debug(tokens, self);
    }
}

impl PtxUnparser for BitShape {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_lowercase_directive_from_debug(tokens, self);
    }
}

impl PtxUnparser for HalfAccumulatorType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_lowercase_directive_from_debug(tokens, self);
    }
}

impl PtxUnparser for Bf16AccumulatorType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_lowercase_directive_from_debug(tokens, self);
    }
}

impl PtxUnparser for Tf32AccumulatorType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_lowercase_directive_from_debug(tokens, self);
    }
}

impl PtxUnparser for Fp8AccumulatorType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_lowercase_directive_from_debug(tokens, self);
    }
}

impl PtxUnparser for Fp8InputType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_lowercase_directive_from_debug(tokens, self);
    }
}

impl PtxUnparser for IntegerInputType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_lowercase_directive_from_debug(tokens, self);
    }
}

impl PtxUnparser for BitOperation {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_lowercase_directive_from_debug(tokens, self);
    }
}

impl PtxUnparser for ScaleImmediate {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ScaleImmediate::MinusOne => {
                tokens.push(PtxToken::Minus);
                tokens.push(PtxToken::DecimalInteger("1".to_string()));
            }
            ScaleImmediate::PlusOne => {
                tokens.push(PtxToken::DecimalInteger("1".to_string()));
            }
        }
    }
}

impl PtxUnparser for TransposeImmediate {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let literal = match self {
            TransposeImmediate::Identity => "0",
            TransposeImmediate::Transpose => "1",
        };
        tokens.push(PtxToken::DecimalInteger(literal.to_string()));
    }
}

impl PtxUnparser for F16Descriptor {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wgmma_prefix(tokens);
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "dtype");
        self.dtype.unparse_tokens(tokens);
        push_directive(tokens, "f16");

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.scale_d.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_b.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_trans_a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_trans_b.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for F16Register {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wgmma_prefix(tokens);
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "dtype");
        self.dtype.unparse_tokens(tokens);
        push_directive(tokens, "f16");

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a_register.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.scale_d.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_b.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_trans_b.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Bf16Descriptor {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wgmma_prefix(tokens);
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "dtype");
        self.dtype.unparse_tokens(tokens);
        push_directive(tokens, "bf16");

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.scale_d.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_b.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_trans_a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_trans_b.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Bf16Register {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wgmma_prefix(tokens);
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "dtype");
        self.dtype.unparse_tokens(tokens);
        push_directive(tokens, "bf16");

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a_register.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.scale_d.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_b.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_trans_b.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Tf32Descriptor {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wgmma_prefix(tokens);
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "dtype");
        self.dtype.unparse_tokens(tokens);
        push_directive(tokens, "tf32");

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.scale_d.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_b.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Tf32Register {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wgmma_prefix(tokens);
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "dtype");
        self.dtype.unparse_tokens(tokens);
        push_directive(tokens, "tf32");

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a_register.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.scale_d.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_b.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Fp8Descriptor {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wgmma_prefix(tokens);
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "dtype");
        self.dtype.unparse_tokens(tokens);
        push_directive(tokens, "atype");
        self.atype.unparse_tokens(tokens);
        push_directive(tokens, "btype");
        self.btype.unparse_tokens(tokens);

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.scale_d.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_b.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Fp8Register {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wgmma_prefix(tokens);
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "dtype");
        self.dtype.unparse_tokens(tokens);
        push_directive(tokens, "atype");
        self.atype.unparse_tokens(tokens);
        push_directive(tokens, "btype");
        self.btype.unparse_tokens(tokens);

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a_register.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.scale_d.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.imm_scale_b.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for IntegerDescriptor {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wgmma_prefix(tokens);
        self.shape.unparse_tokens(tokens);
        if self.satfinite {
            push_directive(tokens, "satfinite");
        }
        push_directive(tokens, "s32");
        push_directive(tokens, "atype");
        self.atype.unparse_tokens(tokens);
        push_directive(tokens, "btype");
        self.btype.unparse_tokens(tokens);

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.scale_d.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for IntegerRegister {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wgmma_prefix(tokens);
        self.shape.unparse_tokens(tokens);
        if self.satfinite {
            push_directive(tokens, "satfinite");
        }
        push_directive(tokens, "s32");
        push_directive(tokens, "atype");
        self.atype.unparse_tokens(tokens);
        push_directive(tokens, "btype");
        self.btype.unparse_tokens(tokens);

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a_register.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.scale_d.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SingleBitDescriptor {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wgmma_prefix(tokens);
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "s32");
        push_directive(tokens, "b1");
        push_directive(tokens, "b1");
        push_directive(tokens, "op");
        self.operation.unparse_tokens(tokens);

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.scale_d.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SingleBitRegister {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_wgmma_prefix(tokens);
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "s32");
        push_directive(tokens, "b1");
        push_directive(tokens, "b1");
        push_directive(tokens, "op");
        self.operation.unparse_tokens(tokens);

        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a_register.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b_descriptor.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.scale_d.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Wgmma {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Wgmma::F16Descriptor(value) => value.unparse_tokens(tokens),
            Wgmma::F16Register(value) => value.unparse_tokens(tokens),
            Wgmma::Bf16Descriptor(value) => value.unparse_tokens(tokens),
            Wgmma::Bf16Register(value) => value.unparse_tokens(tokens),
            Wgmma::Tf32Descriptor(value) => value.unparse_tokens(tokens),
            Wgmma::Tf32Register(value) => value.unparse_tokens(tokens),
            Wgmma::Fp8Descriptor(value) => value.unparse_tokens(tokens),
            Wgmma::Fp8Register(value) => value.unparse_tokens(tokens),
            Wgmma::IntegerDescriptor(value) => value.unparse_tokens(tokens),
            Wgmma::IntegerRegister(value) => value.unparse_tokens(tokens),
            Wgmma::SingleBitDescriptor(value) => value.unparse_tokens(tokens),
            Wgmma::SingleBitRegister(value) => value.unparse_tokens(tokens),
        }
    }
}
