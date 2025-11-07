#![allow(dead_code)]
use ptx_parser::{
    lexer::{tokenize, PtxToken},
    parser::{PtxParseError, PtxParser, PtxTokenStream},
    unparser::PtxUnparser,
};

pub fn parse_result<T: PtxParser>(source: &str) -> Result<T, PtxParseError> {
    let tokens = tokenize(source).expect("tokenization should succeed");
    let mut stream = PtxTokenStream::new(&tokens);
    T::parse(&mut stream)
}

pub fn parse<T: PtxParser>(source: &str) -> T {
    parse_result(source).expect("parse should succeed")
}

pub fn tokenize_only(source: &str) -> Vec<ptx_parser::lexer::PtxToken> {
    let tokens = tokenize(source).expect("tokenization should succeed");
    tokens.into_iter().map(|(token, _)| token).collect()
}

/// Compare two token lists semantically, allowing for equivalent representations.
/// For example, Register("%ctaid.x") should match Register("%ctaid") + Dot + Identifier("x")
pub fn tokens_equivalent(left: &[PtxToken], right: &[PtxToken]) -> bool {
    let mut left_idx = 0;
    let mut right_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        let left_token = &left[left_idx];
        let right_token = &right[right_idx];

        // Direct match - most common case
        if left_token == right_token {
            left_idx += 1;
            right_idx += 1;
            continue;
        }

        // Try to match Register with dot notation
        // Case 1: Left has Register("%ctaid.x"), Right has Register("%ctaid") + Dot + Identifier("x")
        if let PtxToken::Register(left_reg) = left_token {
            if let Some(dot_pos) = left_reg.find('.') {
                // Left register has a dot component
                if let PtxToken::Register(right_reg) = right_token {
                    let (left_base, left_component_with_dot) = left_reg.split_at(dot_pos);
                    let left_component = &left_component_with_dot[1..]; // Skip the '.'
                    
                    // Check if right has base + dot + component
                    if right_reg == left_base
                        && right_idx + 2 < right.len()
                        && right[right_idx + 1] == PtxToken::Dot
                    {
                        if let PtxToken::Identifier(right_comp) = &right[right_idx + 2] {
                            if right_comp == left_component {
                                left_idx += 1;
                                right_idx += 3; // Skip base + dot + component
                                continue;
                            }
                        }
                    }
                }
            }
        }

        // Case 2: Right has Register("%ctaid.x"), Left has Register("%ctaid") + Dot + Identifier("x")
        if let PtxToken::Register(right_reg) = right_token {
            if let Some(dot_pos) = right_reg.find('.') {
                // Right register has a dot component
                if let PtxToken::Register(left_reg) = left_token {
                    let (right_base, right_component_with_dot) = right_reg.split_at(dot_pos);
                    let right_component = &right_component_with_dot[1..]; // Skip the '.'
                    
                    // Check if left has base + dot + component
                    if left_reg == right_base
                        && left_idx + 2 < left.len()
                        && left[left_idx + 1] == PtxToken::Dot
                    {
                        if let PtxToken::Identifier(left_comp) = &left[left_idx + 2] {
                            if left_comp == right_component {
                                left_idx += 3; // Skip base + dot + component
                                right_idx += 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }

        // No match found
        return false;
    }

    // Both should be exhausted
    left_idx == left.len() && right_idx == right.len()
}

pub fn assert_roundtrip<T>(source: &str)
where
    T: PtxParser + PtxUnparser,
{
    let original_tokens = tokenize_only(source);
    let parsed = parse::<T>(source);
    let unparsed_tokens = parsed.to_tokens();
    assert!(
        tokens_equivalent(&unparsed_tokens, &original_tokens),
        "Roundtrip failed for: {}\nUnparsed: {:?}\nOriginal: {:?}",
        source,
        unparsed_tokens,
        original_tokens
    );
}

