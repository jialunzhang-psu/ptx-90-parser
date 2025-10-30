use crate::{
    lexer::PtxToken,
    r#type::instruction::setp::{
        BoolOp, Compare, CompareBool, CompareOp, DataType, Destination, Predicate, PredicateTarget,
        Setp,
    },
    unparser::*,
};

fn push_compare_op(tokens: &mut Vec<PtxToken>, name: &str) {
    tokens.push(PtxToken::Directive(name.to_string()));
}

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
        push_compare_op(tokens, name);
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

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            DataType::B16 => "b16",
            DataType::B32 => "b32",
            DataType::B64 => "b64",
            DataType::U16 => "u16",
            DataType::U32 => "u32",
            DataType::U64 => "u64",
            DataType::S16 => "s16",
            DataType::S32 => "s32",
            DataType::S64 => "s64",
            DataType::F32 => "f32",
            DataType::F64 => "f64",
        };
        tokens.push(PtxToken::Directive(name.to_string()));
    }
}

impl PtxUnparser for PredicateTarget {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            PredicateTarget::Register(register) => register.unparse_tokens(tokens),
            PredicateTarget::Sink => tokens.push(PtxToken::Identifier("_".to_string())),
        }
    }
}

impl PtxUnparser for Destination {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.predicate.unparse_tokens(tokens);
        if let Some(complement) = &self.complement {
            tokens.push(PtxToken::Pipe);
            complement.unparse_tokens(tokens);
        }
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
        self.data_type.unparse_tokens(tokens);
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
        self.data_type.unparse_tokens(tokens);
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

impl PtxUnparser for Setp {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::Identifier("setp".to_string()));
        match self {
            Setp::Compare(compare) => compare.unparse_tokens(tokens),
            Setp::CompareBool(compare_bool) => compare_bool.unparse_tokens(tokens),
        }
    }
}
