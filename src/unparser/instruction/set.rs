use crate::{lexer::PtxToken, r#type::instruction::set::*, unparser::*};

impl PtxUnparser for CompareOp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            CompareOp::Eq => "eq",
            CompareOp::Ne => "ne",
            CompareOp::Lt => "lt",
            CompareOp::Le => "le",
            CompareOp::Gt => "gt",
            CompareOp::Ge => "ge",
            CompareOp::Lo => "lo",
            CompareOp::Ls => "ls",
            CompareOp::Hi => "hi",
            CompareOp::Hs => "hs",
            CompareOp::Equ => "equ",
            CompareOp::Neu => "neu",
            CompareOp::Ltu => "ltu",
            CompareOp::Leu => "leu",
            CompareOp::Gtu => "gtu",
            CompareOp::Geu => "geu",
            CompareOp::Num => "num",
            CompareOp::Nan => "nan",
        };
        tokens.push(PtxToken::Directive(name.to_string()));
    }
}

impl PtxUnparser for BoolOp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            BoolOp::And => "and",
            BoolOp::Or => "or",
            BoolOp::Xor => "xor",
        };
        tokens.push(PtxToken::Directive(name.to_string()));
    }
}

impl PtxUnparser for DestinationType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            DestinationType::U32 => "u32",
            DestinationType::S32 => "s32",
            DestinationType::F32 => "f32",
        };
        tokens.push(PtxToken::Directive(name.to_string()));
    }
}

impl PtxUnparser for SourceType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            SourceType::B16 => "b16",
            SourceType::B32 => "b32",
            SourceType::B64 => "b64",
            SourceType::U16 => "u16",
            SourceType::U32 => "u32",
            SourceType::U64 => "u64",
            SourceType::S16 => "s16",
            SourceType::S32 => "s32",
            SourceType::S64 => "s64",
            SourceType::F32 => "f32",
            SourceType::F64 => "f64",
        };
        tokens.push(PtxToken::Directive(name.to_string()));
    }
}

impl PtxUnparser for Predicate {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        if self.negated {
            tokens.push(PtxToken::Exclaim);
        }
        self.register.unparse_tokens(tokens);
    }
}

impl PtxUnparser for Compare {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.compare_op.unparse_tokens(tokens);
        push_flush_to_zero(tokens, self.flush_to_zero);
        self.destination_type.unparse_tokens(tokens);
        self.source_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for CompareBool {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.compare_op.unparse_tokens(tokens);
        self.bool_op.unparse_tokens(tokens);
        push_flush_to_zero(tokens, self.flush_to_zero);
        self.destination_type.unparse_tokens(tokens);
        self.source_type.unparse_tokens(tokens);
        self.destination.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.a.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.b.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.predicate.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for Set {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("set".to_string()));
        match self {
            Set::Compare(compare) => compare.unparse_tokens(tokens),
            Set::CompareBool(compare_bool) => compare_bool.unparse_tokens(tokens),
        }
    }
}
