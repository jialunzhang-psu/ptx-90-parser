use super::PtxUnparser;
use crate::{
    lexer::{PtxToken, tokenize},
    r#type::module::*,
    unparser::{push_decimal, push_directive, push_identifier, push_token_from_str},
};

fn numeric_token_from_str(value: &str) -> Option<PtxToken> {
    if value.starts_with("0f") || value.starts_with("0F") {
        Some(PtxToken::HexFloatSingle(value.to_string()))
    } else if value.starts_with("0d") || value.starts_with("0D") {
        Some(PtxToken::HexFloatDouble(value.to_string()))
    } else if value.starts_with("0x") || value.starts_with("0X") {
        Some(PtxToken::HexInteger(value.to_string()))
    } else if value.starts_with("0b") || value.starts_with("0B") {
        Some(PtxToken::BinaryInteger(value.to_string()))
    } else if value.len() > 1
        && value.starts_with('0')
        && value.chars().all(|c| c >= '0' && c <= '7')
    {
        Some(PtxToken::OctalInteger(value.to_string()))
    } else if value.chars().all(|c| c.is_ascii_digit()) {
        Some(PtxToken::DecimalInteger(value.to_string()))
    } else if is_float_exponent(value) {
        Some(PtxToken::FloatExponent(value.to_string()))
    } else if is_decimal_float(value) {
        Some(PtxToken::Float(value.to_string()))
    } else {
        None
    }
}

fn is_decimal_float(value: &str) -> bool {
    let mut has_digits = false;
    let mut has_dot = false;
    for ch in value.chars() {
        match ch {
            '0'..='9' => has_digits = true,
            '.' if !has_dot => has_dot = true,
            _ => return false,
        }
    }
    has_digits && has_dot
}

fn is_float_exponent(value: &str) -> bool {
    let mut chars = value.chars().peekable();
    let mut has_digits = false;
    while let Some(ch) = chars.peek() {
        match ch {
            '0'..='9' | '+' | '-' => {
                has_digits |= ch.is_ascii_digit();
                chars.next();
            }
            '.' => {
                chars.next();
            }
            'e' | 'E' => {
                chars.next();
                if chars.peek().is_none() {
                    return false;
                }
                let mut exponent_digits = false;
                if let Some(sign) = chars.peek() {
                    if *sign == '+' || *sign == '-' {
                        chars.next();
                    }
                }
                while let Some(next) = chars.peek() {
                    match next {
                        '0'..='9' => {
                            exponent_digits = true;
                            chars.next();
                        }
                        _ => return false,
                    }
                }
                return has_digits && exponent_digits;
            }
            _ => return false,
        }
    }
    false
}

fn push_token_from_repr(tokens: &mut Vec<PtxToken>, value: &str) {
    let trimmed = value.trim();
    match trimmed {
        "{" => tokens.push(PtxToken::LBrace),
        "}" => tokens.push(PtxToken::RBrace),
        "[" => tokens.push(PtxToken::LBracket),
        "]" => tokens.push(PtxToken::RBracket),
        "(" => tokens.push(PtxToken::LParen),
        ")" => tokens.push(PtxToken::RParen),
        "," => tokens.push(PtxToken::Comma),
        ":" => tokens.push(PtxToken::Colon),
        ";" => tokens.push(PtxToken::Semicolon),
        "." => tokens.push(PtxToken::Dot),
        "*" => tokens.push(PtxToken::Star),
        "+" => tokens.push(PtxToken::Plus),
        "-" => tokens.push(PtxToken::Minus),
        "/" => tokens.push(PtxToken::Slash),
        "%" => tokens.push(PtxToken::Percent),
        "=" => tokens.push(PtxToken::Equals),
        "|" => tokens.push(PtxToken::Pipe),
        "&" => tokens.push(PtxToken::Ampersand),
        "^" => tokens.push(PtxToken::Caret),
        "~" => tokens.push(PtxToken::Tilde),
        "!" => tokens.push(PtxToken::Exclaim),
        "<" => tokens.push(PtxToken::LAngle),
        ">" => tokens.push(PtxToken::RAngle),
        "@" => tokens.push(PtxToken::At),
        "::" => tokens.push(PtxToken::DoubleColon),
        _ => {
            if let Some(rest) = trimmed.strip_prefix('.') {
                push_directive(tokens, rest);
            } else if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
                tokens.push(PtxToken::StringLiteral(
                    trimmed[1..trimmed.len() - 1].to_string(),
                ));
            } else if trimmed.starts_with('%') {
                tokens.push(PtxToken::Register(trimmed.to_string()));
            } else if let Some(numeric) = numeric_token_from_str(trimmed) {
                tokens.push(numeric);
            } else {
                push_token_from_str(tokens, trimmed);
            }
        }
    }
}

fn push_entries(tokens: &mut Vec<PtxToken>, entries: &[TargetString]) {
    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            tokens.push(PtxToken::Comma);
        }
        let name = match entry {
            TargetString::Sm120a { .. } => "sm_120a",
            TargetString::Sm120f { .. } => "sm_120f",
            TargetString::Sm120 { .. } => "sm_120",
            TargetString::Sm121a { .. } => "sm_121a",
            TargetString::Sm121f { .. } => "sm_121f",
            TargetString::Sm121 { .. } => "sm_121",
            TargetString::Sm110a { .. } => "sm_110a",
            TargetString::Sm110f { .. } => "sm_110f",
            TargetString::Sm110 { .. } => "sm_110",
            TargetString::Sm100a { .. } => "sm_100a",
            TargetString::Sm100f { .. } => "sm_100f",
            TargetString::Sm100 { .. } => "sm_100",
            TargetString::Sm101a { .. } => "sm_101a",
            TargetString::Sm101f { .. } => "sm_101f",
            TargetString::Sm101 { .. } => "sm_101",
            TargetString::Sm103a { .. } => "sm_103a",
            TargetString::Sm103f { .. } => "sm_103f",
            TargetString::Sm103 { .. } => "sm_103",
            TargetString::Sm90a { .. } => "sm_90a",
            TargetString::Sm90 { .. } => "sm_90",
            TargetString::Sm80 { .. } => "sm_80",
            TargetString::Sm86 { .. } => "sm_86",
            TargetString::Sm87 { .. } => "sm_87",
            TargetString::Sm88 { .. } => "sm_88",
            TargetString::Sm89 { .. } => "sm_89",
            TargetString::Sm70 { .. } => "sm_70",
            TargetString::Sm72 { .. } => "sm_72",
            TargetString::Sm75 { .. } => "sm_75",
            TargetString::Sm60 { .. } => "sm_60",
            TargetString::Sm61 { .. } => "sm_61",
            TargetString::Sm62 { .. } => "sm_62",
            TargetString::Sm50 { .. } => "sm_50",
            TargetString::Sm52 { .. } => "sm_52",
            TargetString::Sm53 { .. } => "sm_53",
            TargetString::Sm30 { .. } => "sm_30",
            TargetString::Sm32 { .. } => "sm_32",
            TargetString::Sm35 { .. } => "sm_35",
            TargetString::Sm37 { .. } => "sm_37",
            TargetString::Sm20 { .. } => "sm_20",
            TargetString::Sm10 { .. } => "sm_10",
            TargetString::Sm11 { .. } => "sm_11",
            TargetString::Sm12 { .. } => "sm_12",
            TargetString::Sm13 { .. } => "sm_13",
            TargetString::TexmodeUnified { .. } => "texmode_unified",
            TargetString::TexmodeIndependent { .. } => "texmode_independent",
            TargetString::Debug { .. } => "debug",
            TargetString::MapF64ToF32 { .. } => "map_f64_to_f32",
        };
        push_identifier(tokens, name);
    }
}

fn push_raw_tokens(tokens: &mut Vec<PtxToken>, raw: &str) {
    if raw.trim().is_empty() {
        return;
    }
    let lexemes = tokenize(raw).expect("tokenizing stored PTX snippet");
    tokens.extend(lexemes.into_iter().map(|(token, _)| token));
}

impl PtxUnparser for VersionDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "version");
        let value = format!("{}.{:}", self.major, self.minor);
        tokens.push(PtxToken::Float(value));
    }
}

impl PtxUnparser for TargetDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "target");
        push_entries(tokens, &self.entries);
    }
}

impl PtxUnparser for AddressSizeDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "address_size");
        match &self.size {
            AddressSize::Size32 { .. } => push_decimal(tokens, 32),
            AddressSize::Size64 { .. } => push_decimal(tokens, 64),
        }
    }
}

impl PtxUnparser for ModuleInfoDirectiveKind {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ModuleInfoDirectiveKind::Version { directive, .. } => directive.unparse_tokens(tokens),
            ModuleInfoDirectiveKind::Target { directive, .. } => directive.unparse_tokens(tokens),
            ModuleInfoDirectiveKind::AddressSize { directive, .. } => {
                directive.unparse_tokens(tokens)
            }
        }
    }
}

impl PtxUnparser for FileDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "file");
        push_decimal(tokens, self.index);
        tokens.push(PtxToken::StringLiteral(self.path.clone()));
        if let Some(timestamp) = self.timestamp {
            tokens.push(PtxToken::Comma);
            push_decimal(tokens, timestamp);
            if let Some(size) = self.file_size {
                tokens.push(PtxToken::Comma);
                push_decimal(tokens, size);
            }
        }
    }
}

impl PtxUnparser for ModuleDebugDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ModuleDebugDirective::File { directive, .. } => directive.unparse_tokens(tokens),
            ModuleDebugDirective::Section { directive, .. } => directive.unparse_tokens(tokens),
            ModuleDebugDirective::Dwarf { directive, .. } => directive.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for ModuleDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ModuleDirective::ModuleVariable {
                linkage, directive, ..
            } => {
                if let Some(link) = linkage {
                    link.unparse_tokens(tokens);
                }
                directive.unparse_tokens(tokens)
            }
            ModuleDirective::EntryFunction {
                linkage, directive, ..
            } => {
                if let Some(link) = linkage {
                    link.unparse_tokens(tokens);
                }
                directive.unparse_tokens(tokens)
            }
            ModuleDirective::FuncFunction {
                linkage, directive, ..
            } => {
                if let Some(link) = linkage {
                    link.unparse_tokens(tokens);
                }
                directive.unparse_tokens(tokens)
            }
            ModuleDirective::AliasFunction { directive, .. } => directive.unparse_tokens(tokens),
            ModuleDirective::ModuleInfo { directive, .. } => directive.unparse_tokens(tokens),
            ModuleDirective::Debug { directive, .. } => directive.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for Module {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        for directive in &self.directives {
            directive.unparse_tokens(tokens);
        }
    }
}
