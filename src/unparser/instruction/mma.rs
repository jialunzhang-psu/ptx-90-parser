use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::mma::*},
    unparser::*,
};

fn push_scale_vec_literal(tokens: &mut Vec<PtxToken>, literal: &str) {
    let split = literal
        .char_indices()
        .find(|(_, c)| !c.is_ascii_digit())
        .map(|(index, _)| index)
        .unwrap_or_else(|| literal.len());
    if split > 0 {
        push_decimal(tokens, &literal[..split]);
    }
    if split < literal.len() {
        tokens.push(PtxToken::Identifier(literal[split..].to_string()));
    }
}

fn push_mma_prefix(tokens: &mut Vec<PtxToken>, modifier: &str) {
    push_identifier(tokens, "mma");
    push_directive(tokens, "sync");
    push_directive(tokens, "aligned");
    push_directive(tokens, modifier);
}

fn push_operands(
    destination: &RegisterOperand,
    operand_a: &RegisterOperand,
    operand_b: &RegisterOperand,
    operand_c: &RegisterOperand,
    tokens: &mut Vec<PtxToken>,
) {
    destination.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    operand_a.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    operand_b.unparse_tokens(tokens);
    tokens.push(PtxToken::Comma);
    operand_c.unparse_tokens(tokens);
}

fn push_operands_with_block_scale(
    destination: &RegisterOperand,
    operand_a: &RegisterOperand,
    operand_b: &RegisterOperand,
    operand_c: &RegisterOperand,
    block_scale: &BlockScaleOperands,
    tokens: &mut Vec<PtxToken>,
) {
    push_operands(destination, operand_a, operand_b, operand_c, tokens);
    tokens.push(PtxToken::Comma);
    block_scale.unparse_tokens(tokens);
}

impl PtxUnparser for crate::r#type::instruction::mma::Layout {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::Row => "row",
            Self::Col => "col",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for crate::r#type::instruction::mma::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Self::F16 => "f16",
            Self::F32 => "f32",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for AlternateMatrixType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            AlternateMatrixType::Bf16 => "bf16",
            AlternateMatrixType::Tf32 => "tf32",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for F8Type {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            F8Type::E4M3 => "e4m3",
            F8Type::E5M2 => "e5m2",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for F8F6F4Type {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            F8F6F4Type::E4M3 => "e4m3",
            F8F6F4Type::E5M2 => "e5m2",
            F8F6F4Type::E3M2 => "e3m2",
            F8F6F4Type::E2M3 => "e2m3",
            F8F6F4Type::E2M1 => "e2m1",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for F8Shape {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            F8Shape::M16N8K16 => "m16n8k16",
            F8Shape::M16N8K32 => "m16n8k32",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for F64Shape {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            F64Shape::M8N8K4 => "m8n8k4",
            F64Shape::M16N8K4 => "m16n8k4",
            F64Shape::M16N8K8 => "m16n8k8",
            F64Shape::M16N8K16 => "m16n8k16",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Integer8Shape {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Integer8Shape::M8N8K16 => "m8n8k16",
            Integer8Shape::M16N8K16 => "m16n8k16",
            Integer8Shape::M16N8K32 => "m16n8k32",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Integer4Shape {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Integer4Shape::M8N8K32 => "m8n8k32",
            Integer4Shape::M16N8K32 => "m16n8k32",
            Integer4Shape::M16N8K64 => "m16n8k64",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Integer8Type {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Integer8Type::U8 => "u8",
            Integer8Type::S8 => "s8",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Integer4Type {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Integer4Type::U4 => "u4",
            Integer4Type::S4 => "s4",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for SingleBitShape {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            SingleBitShape::M8N8K128 => "m8n8k128",
            SingleBitShape::M16N8K128 => "m16n8k128",
            SingleBitShape::M16N8K256 => "m16n8k256",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for BitOp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            BitOp::Xor => "xor",
            BitOp::And => "and",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for BlockScaleOperands {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.scale_a_data.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        tokens.push(PtxToken::LBrace);
        push_decimal(tokens, self.scale_a_byte_id);
        tokens.push(PtxToken::Comma);
        push_decimal(tokens, self.scale_a_thread_id);
        tokens.push(PtxToken::RBrace);
        tokens.push(PtxToken::Comma);
        self.scale_b_data.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        tokens.push(PtxToken::LBrace);
        push_decimal(tokens, self.scale_b_byte_id);
        tokens.push(PtxToken::Comma);
        push_decimal(tokens, self.scale_b_thread_id);
        tokens.push(PtxToken::RBrace);
    }
}

impl PtxUnparser for MxF4ScaleVecSize {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "scale_vec");
        push_double_colon(tokens);
        match self {
            MxF4ScaleVecSize::TwoX => push_scale_vec_literal(tokens, "2X"),
        }
    }
}

impl PtxUnparser for MxF4NvF4ScaleVecSize {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "scale_vec");
        push_double_colon(tokens);
        match self {
            MxF4NvF4ScaleVecSize::TwoX => push_scale_vec_literal(tokens, "2X"),
            MxF4NvF4ScaleVecSize::FourX => push_scale_vec_literal(tokens, "4X"),
        }
    }
}

impl PtxUnparser for MxF8F6F4ScaleVecSize {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "scale_vec");
        push_double_colon(tokens);
        match self {
            MxF8F6F4ScaleVecSize::OneX => push_scale_vec_literal(tokens, "1X"),
        }
    }
}

impl PtxUnparser for MxF4ScaleDataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "stype");
        let directive = match self {
            MxF4ScaleDataType::UE8M0 => "ue8m0",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for MxF4NvF4ScaleDataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "stype");
        let directive = match self {
            MxF4NvF4ScaleDataType::UE8M0 => "ue8m0",
            MxF4NvF4ScaleDataType::UE4M3 => "ue4m3",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for MxF8F6F4ScaleDataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "stype");
        let directive = match self {
            MxF8F6F4ScaleDataType::UE8M0 => "ue8m0",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for SyncAlignedM8N8K4 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "m8n8k4");
        push_directive(tokens, "alayout");
        self.a_layout.unparse_tokens(tokens);
        push_directive(tokens, "blayout");
        self.b_layout.unparse_tokens(tokens);
        push_directive(tokens, "dtype");
        self.d_type.unparse_tokens(tokens);
        push_directive(tokens, "f16");
        push_directive(tokens, "f16");
        push_directive(tokens, "ctype");
        self.c_type.unparse_tokens(tokens);
        push_operands(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedM16N8K8 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "m16n8k8");
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        push_directive(tokens, "dtype");
        self.d_type.unparse_tokens(tokens);
        push_directive(tokens, "f16");
        push_directive(tokens, "f16");
        push_directive(tokens, "ctype");
        self.c_type.unparse_tokens(tokens);
        push_operands(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedM16N8K16 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "m16n8k16");
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        push_directive(tokens, "dtype");
        self.d_type.unparse_tokens(tokens);
        push_directive(tokens, "f16");
        push_directive(tokens, "f16");
        push_directive(tokens, "ctype");
        self.c_type.unparse_tokens(tokens);
        push_operands(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedM16N8K4Tf32 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "m16n8k4");
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        push_directive(tokens, "f32");
        push_directive(tokens, "tf32");
        push_directive(tokens, "tf32");
        push_directive(tokens, "f32");
        push_operands(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedM16N8K8Mixed {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "m16n8k8");
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        push_directive(tokens, "f32");
        push_directive(tokens, "atype");
        self.a_type.unparse_tokens(tokens);
        push_directive(tokens, "btype");
        self.b_type.unparse_tokens(tokens);
        push_directive(tokens, "f32");
        push_operands(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedM16N8K16Bf16 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "m16n8k16");
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        push_directive(tokens, "f32");
        push_directive(tokens, "bf16");
        push_directive(tokens, "bf16");
        push_directive(tokens, "f32");
        push_operands(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedF8 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "shape");
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        push_directive(tokens, "dtype");
        self.d_type.unparse_tokens(tokens);
        push_directive(tokens, "f8type");
        self.a_type.unparse_tokens(tokens);
        push_directive(tokens, "f8type");
        self.b_type.unparse_tokens(tokens);
        push_directive(tokens, "ctype");
        self.c_type.unparse_tokens(tokens);
        push_operands(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedM16N8K32F8F6F4 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "m16n8k32");
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        push_directive(tokens, "kind");
        push_directive(tokens, "f8f6f4");
        push_directive(tokens, "dtype");
        self.d_type.unparse_tokens(tokens);
        push_directive(tokens, "f8f6f4type");
        self.a_type.unparse_tokens(tokens);
        push_directive(tokens, "f8f6f4type");
        self.b_type.unparse_tokens(tokens);
        push_directive(tokens, "ctype");
        self.c_type.unparse_tokens(tokens);
        push_operands(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedBlockScaleM16N8K64MxF4 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "m16n8k64");
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        push_directive(tokens, "kind");
        push_directive(tokens, "mxf4");
        push_directive(tokens, "block_scale");
        if let Some(scale_vec_size) = &self.scale_vec_size {
            scale_vec_size.unparse_tokens(tokens);
        }
        push_directive(tokens, "f32");
        push_directive(tokens, "e2m1");
        push_directive(tokens, "e2m1");
        push_directive(tokens, "f32");
        self.stype.unparse_tokens(tokens);
        push_operands_with_block_scale(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            &self.block_scale,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedBlockScaleM16N8K64MxF4NvF4 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "m16n8k64");
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        push_directive(tokens, "kind");
        push_directive(tokens, "mxf4nvf4");
        push_directive(tokens, "block_scale");
        self.scale_vec_size.unparse_tokens(tokens);
        push_directive(tokens, "f32");
        push_directive(tokens, "e2m1");
        push_directive(tokens, "e2m1");
        push_directive(tokens, "f32");
        self.stype.unparse_tokens(tokens);
        push_operands_with_block_scale(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            &self.block_scale,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedBlockScaleM16N8K32MxF8F6F4 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "m16n8k32");
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        push_directive(tokens, "kind");
        push_directive(tokens, "mxf8f6f4");
        push_directive(tokens, "block_scale");
        if let Some(scale_vec_size) = &self.scale_vec_size {
            scale_vec_size.unparse_tokens(tokens);
        }
        push_directive(tokens, "f32");
        push_directive(tokens, "f8f6f4type");
        self.a_type.unparse_tokens(tokens);
        push_directive(tokens, "f8f6f4type");
        self.b_type.unparse_tokens(tokens);
        push_directive(tokens, "f32");
        self.stype.unparse_tokens(tokens);
        push_operands_with_block_scale(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            &self.block_scale,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedF64 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "shape");
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        push_directive(tokens, "f64");
        push_directive(tokens, "f64");
        push_directive(tokens, "f64");
        push_directive(tokens, "f64");
        push_operands(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedInteger8Bit {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "shape");
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        if self.satfinite {
            push_directive(tokens, "satfinite");
        }
        push_directive(tokens, "s32");
        push_directive(tokens, "atype");
        self.a_type.unparse_tokens(tokens);
        push_directive(tokens, "btype");
        self.b_type.unparse_tokens(tokens);
        push_directive(tokens, "s32");
        push_operands(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedInteger4Bit {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "shape");
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        if self.satfinite {
            push_directive(tokens, "satfinite");
        }
        push_directive(tokens, "s32");
        push_directive(tokens, "atype");
        self.a_type.unparse_tokens(tokens);
        push_directive(tokens, "btype");
        self.b_type.unparse_tokens(tokens);
        push_directive(tokens, "s32");
        push_operands(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for SyncAlignedSingleBit {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_mma_prefix(tokens, "shape");
        self.shape.unparse_tokens(tokens);
        push_directive(tokens, "row");
        push_directive(tokens, "col");
        push_directive(tokens, "s32");
        push_directive(tokens, "b1");
        push_directive(tokens, "b1");
        push_directive(tokens, "s32");
        push_directive(tokens, "bitOp");
        self.bit_op.unparse_tokens(tokens);
        push_directive(tokens, "popc");
        push_operands(
            &self.destination,
            &self.operand_a,
            &self.operand_b,
            &self.operand_c,
            tokens,
        );
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for MmaInstruction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            MmaInstruction::SyncAlignedM8N8K4(inst) => inst.unparse_tokens(tokens),
            MmaInstruction::SyncAlignedM16N8K8(inst) => inst.unparse_tokens(tokens),
            MmaInstruction::SyncAlignedM16N8K16(inst) => inst.unparse_tokens(tokens),
            MmaInstruction::SyncAlignedM16N8K4Tf32(inst) => inst.unparse_tokens(tokens),
            MmaInstruction::SyncAlignedM16N8K8Mixed(inst) => inst.unparse_tokens(tokens),
            MmaInstruction::SyncAlignedM16N8K16Bf16(inst) => inst.unparse_tokens(tokens),
            MmaInstruction::SyncAlignedF8(inst) => inst.unparse_tokens(tokens),
            MmaInstruction::SyncAlignedM16N8K32F8F6F4(inst) => inst.unparse_tokens(tokens),
            MmaInstruction::SyncAlignedBlockScaleM16N8K64MxF4(inst) => inst.unparse_tokens(tokens),
            MmaInstruction::SyncAlignedBlockScaleM16N8K64MxF4NvF4(inst) => {
                inst.unparse_tokens(tokens)
            }
            MmaInstruction::SyncAlignedBlockScaleM16N8K32MxF8F6F4(inst) => {
                inst.unparse_tokens(tokens)
            }
            MmaInstruction::SyncAlignedF64(inst) => inst.unparse_tokens(tokens),
            MmaInstruction::SyncAlignedInteger8Bit(inst) => inst.unparse_tokens(tokens),
            MmaInstruction::SyncAlignedInteger4Bit(inst) => inst.unparse_tokens(tokens),
            MmaInstruction::SyncAlignedSingleBit(inst) => inst.unparse_tokens(tokens),
        }
    }
}
