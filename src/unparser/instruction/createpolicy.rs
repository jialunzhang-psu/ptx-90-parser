use crate::{lexer::PtxToken, r#type::instruction::createpolicy::*, unparser::*};

impl PtxUnparser for PrimaryPriority {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "L2");
        push_double_colon(tokens);
        let name = match self {
            PrimaryPriority::L2EvictLast => "evict_last",
            PrimaryPriority::L2EvictNormal => "evict_normal",
            PrimaryPriority::L2EvictFirst => "evict_first",
            PrimaryPriority::L2EvictUnchanged => "evict_unchanged",
        };
        push_identifier(tokens, name);
    }
}

impl PtxUnparser for SecondaryPriority {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "L2");
        push_double_colon(tokens);
        let name = match self {
            SecondaryPriority::L2EvictFirst => "evict_first",
            SecondaryPriority::L2EvictUnchanged => "evict_unchanged",
        };
        push_identifier(tokens, name);
    }
}

impl PtxUnparser for Level {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Level::L2 => push_directive(tokens, "L2"),
        }
    }
}

impl PtxUnparser for Createpolicy {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "createpolicy");
        match self {
            Createpolicy::Range {
                global,
                primary_priority,
                secondary_priority,
                destination,
                address,
                primary_size,
                total_size,
            } => {
                push_directive(tokens, "range");
                if *global {
                    push_directive(tokens, "global");
                }
                primary_priority.unparse_tokens(tokens);
                if let Some(secondary) = secondary_priority {
                    secondary.unparse_tokens(tokens);
                }
                push_directive(tokens, "b64");
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                address.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                primary_size.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                total_size.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            Createpolicy::Fractional {
                primary_priority,
                secondary_priority,
                destination,
                fraction,
            } => {
                push_directive(tokens, "fractional");
                primary_priority.unparse_tokens(tokens);
                if let Some(secondary) = secondary_priority {
                    secondary.unparse_tokens(tokens);
                }
                push_directive(tokens, "b64");
                destination.unparse_tokens(tokens);
                if let Some(value) = fraction {
                    tokens.push(PtxToken::Comma);
                    value.unparse_tokens(tokens);
                }
                tokens.push(PtxToken::Semicolon);
            }
            Createpolicy::Convert {
                level,
                destination,
                access_property,
            } => {
                push_directive(tokens, "cvt");
                level.unparse_tokens(tokens);
                push_directive(tokens, "b64");
                destination.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                access_property.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
        }
    }
}
