use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::cvt::*},
    unparser::*,
};

fn push_register_vector(tokens: &mut Vec<PtxToken>, registers: &[&RegisterOperand]) {
    tokens.push(PtxToken::LBrace);
    for (index, register) in registers.iter().enumerate() {
        if index > 0 {
            tokens.push(PtxToken::Comma);
        }
        (*register).unparse_tokens(tokens);
    }
    tokens.push(PtxToken::RBrace);
}

impl PtxUnparser for IntegerRounding {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            IntegerRounding::Rni => "rni",
            IntegerRounding::Rzi => "rzi",
            IntegerRounding::Rmi => "rmi",
            IntegerRounding::Rpi => "rpi",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for FloatRounding {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            FloatRounding::Rn => "rn",
            FloatRounding::Rz => "rz",
            FloatRounding::Rm => "rm",
            FloatRounding::Rp => "rp",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Rounding {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Rounding::Integer(rounding) => rounding.unparse_tokens(tokens),
            Rounding::Float(rounding) => rounding.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for ScalarType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            ScalarType::U8 => "u8",
            ScalarType::U16 => "u16",
            ScalarType::U32 => "u32",
            ScalarType::U64 => "u64",
            ScalarType::S8 => "s8",
            ScalarType::S16 => "s16",
            ScalarType::S32 => "s32",
            ScalarType::S64 => "s64",
            ScalarType::Bf16 => "bf16",
            ScalarType::F16 => "f16",
            ScalarType::F32 => "f32",
            ScalarType::F64 => "f64",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Basic {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        if let Some(rounding) = &self.rounding {
            rounding.unparse_tokens(tokens);
        }
        if self.flush_to_zero {
            push_directive(tokens, "ftz");
        }
        if self.saturate {
            push_directive(tokens, "sat");
        }
        self.destination_type.unparse_tokens(tokens);
        self.source_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Frnd2Rounding {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Frnd2Rounding::Rn => "rn",
            Frnd2Rounding::Rz => "rz",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Frnd2Kind {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Frnd2Kind::F16FromF32 => {
                push_directive(tokens, "f16");
                push_directive(tokens, "f32");
            }
            Frnd2Kind::F16x2FromF32 => {
                push_directive(tokens, "f16x2");
                push_directive(tokens, "f32");
            }
            Frnd2Kind::Bf16FromF32 => {
                push_directive(tokens, "bf16");
                push_directive(tokens, "f32");
            }
            Frnd2Kind::Bf16x2FromF32 => {
                push_directive(tokens, "bf16x2");
                push_directive(tokens, "f32");
            }
            Frnd2Kind::Tf32FromF32 => {
                push_directive(tokens, "tf32");
                push_directive(tokens, "f32");
            }
        }
    }
}

impl PtxUnparser for Frnd2 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "frnd2");
        self.rounding.unparse_tokens(tokens);
        if self.relu {
            push_directive(tokens, "relu");
        }
        if self.satfinite {
            push_directive(tokens, "satfinite");
        }
        self.kind.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a.unparse_tokens(tokens);
        if let Some(b) = &self.b {
            tokens.push(PtxToken::Comma);
            b.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for F8x4Type {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            F8x4Type::E4m3x4 => "e4m3x4",
            F8x4Type::E5m2x4 => "e5m2x4",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for F4x4Type {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            F4x4Type::E2m1x4 => "e2m1x4",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for F6x4Type {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            F6x4Type::E2m3x4 => "e2m3x4",
            F6x4Type::E3m2x4 => "e3m2x4",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Rs {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "rs");
        if self.relu {
            push_directive(tokens, "relu");
        }
        if self.satfinite {
            push_directive(tokens, "satfinite");
        }
        match &self.kind {
            RsKind::F16x2FromF32 { a, b } => {
                push_directive(tokens, "f16x2");
                push_directive(tokens, "f32");
                self.destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                a.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                b.unparse_tokens(tokens);
            }
            RsKind::Bf16x2FromF32 { a, b } => {
                push_directive(tokens, "bf16x2");
                push_directive(tokens, "f32");
                self.destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                a.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                b.unparse_tokens(tokens);
            }
            RsKind::F8x4FromF32 {
                data_type,
                a,
                b,
                e,
                f,
            } => {
                push_directive(tokens, "f8x4type");
                data_type.unparse_tokens(tokens);
                push_directive(tokens, "f32");
                self.destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_register_vector(tokens, &[a, b, e, f]);
            }
            RsKind::F4x4FromF32 {
                data_type,
                a,
                b,
                e,
                f,
            } => {
                push_directive(tokens, "f4x4type");
                data_type.unparse_tokens(tokens);
                push_directive(tokens, "f32");
                self.destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_register_vector(tokens, &[a, b, e, f]);
            }
            RsKind::F6x4FromF32 {
                data_type,
                a,
                b,
                e,
                f,
            } => {
                push_directive(tokens, "f6x4type");
                data_type.unparse_tokens(tokens);
                push_directive(tokens, "f32");
                self.destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_register_vector(tokens, &[a, b, e, f]);
            }
        }
        tokens.push(PtxToken::Comma);
        self.rbits.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Rna {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "rna");
        if self.satfinite {
            push_directive(tokens, "satfinite");
        }
        push_directive(tokens, "tf32");
        push_directive(tokens, "f32");
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.source.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for F8x2Type {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            F8x2Type::E4m3x2 => "e4m3x2",
            F8x2Type::E5m2x2 => "e5m2x2",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for F4x2Type {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            F4x2Type::E2m1x2 => "e2m1x2",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for F6x2Type {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            F6x2Type::E2m3x2 => "e2m3x2",
            F6x2Type::E3m2x2 => "e3m2x2",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Rn {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "rn");
        if self.satfinite {
            push_directive(tokens, "satfinite");
        }
        if self.relu {
            push_directive(tokens, "relu");
        }
        match &self.kind {
            RnKind::F8x2FromF32 {
                data_type,
                destination,
                a,
                b,
            } => {
                data_type.unparse_tokens(tokens);
                push_directive(tokens, "f32");
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                a.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                b.unparse_tokens(tokens);
            }
            RnKind::F8x2FromF16x2 {
                data_type,
                destination,
                a,
            } => {
                data_type.unparse_tokens(tokens);
                push_directive(tokens, "f16x2");
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                a.unparse_tokens(tokens);
            }
            RnKind::F16x2FromF8x2 {
                data_type,
                destination,
                a,
            } => {
                push_directive(tokens, "f16x2");
                data_type.unparse_tokens(tokens);
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                a.unparse_tokens(tokens);
            }
            RnKind::F4x2FromF32 {
                data_type,
                destination,
                a,
                b,
            } => {
                data_type.unparse_tokens(tokens);
                push_directive(tokens, "f32");
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                a.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                b.unparse_tokens(tokens);
            }
            RnKind::F16x2FromF4x2 {
                data_type,
                destination,
                a,
            } => {
                push_directive(tokens, "f16x2");
                data_type.unparse_tokens(tokens);
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                a.unparse_tokens(tokens);
            }
            RnKind::F6x2FromF32 {
                data_type,
                destination,
                a,
                b,
            } => {
                data_type.unparse_tokens(tokens);
                push_directive(tokens, "f32");
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                a.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                b.unparse_tokens(tokens);
            }
            RnKind::F16x2FromF6x2 {
                data_type,
                destination,
                a,
            } => {
                push_directive(tokens, "f16x2");
                data_type.unparse_tokens(tokens);
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                a.unparse_tokens(tokens);
            }
            RnKind::Bf16x2FromUe8m0x2 { destination, a } => {
                push_directive(tokens, "bf16x2");
                push_directive(tokens, "ue8m0x2");
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                a.unparse_tokens(tokens);
            }
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Frnd3Rounding {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            Frnd3Rounding::Rz => "rz",
            Frnd3Rounding::Rp => "rp",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Frnd3Kind {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Frnd3Kind::Ue8m0x2FromF32 => {
                push_directive(tokens, "ue8m0x2");
                push_directive(tokens, "f32");
            }
            Frnd3Kind::Ue8m0x2FromBf16x2 => {
                push_directive(tokens, "ue8m0x2");
                push_directive(tokens, "bf16x2");
            }
        }
    }
}

impl PtxUnparser for Frnd3 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "frnd3");
        self.rounding.unparse_tokens(tokens);
        if self.satfinite {
            push_directive(tokens, "satfinite");
        }
        self.kind.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a.unparse_tokens(tokens);
        if let Some(b) = &self.b {
            tokens.push(PtxToken::Comma);
            b.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Cvt {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "cvt");
        match self {
            Cvt::Basic(basic) => basic.unparse_tokens(tokens),
            Cvt::Frnd2(frnd2) => frnd2.unparse_tokens(tokens),
            Cvt::Rs(rs) => rs.unparse_tokens(tokens),
            Cvt::Rna(rna) => rna.unparse_tokens(tokens),
            Cvt::Rn(rn) => rn.unparse_tokens(tokens),
            Cvt::Frnd3(frnd3) => frnd3.unparse_tokens(tokens),
        }
    }
}
