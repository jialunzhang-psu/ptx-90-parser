use crate::{
    lexer::PtxToken,
    r#type::{function::DwarfDirective, module::*},
    unparser::*,
};

use crate::lexer::tokenize;
fn numeric_token_from_str(value: &str) -> Option<PtxToken> {
    if value.starts_with("0f")
        || value.starts_with("0F")
        || value.starts_with("0d")
        || value.starts_with("0D")
    {
        Some(PtxToken::HexFloat(value.to_string()))
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
                push_identifier(tokens, trimmed);
            }
        }
    }
}

fn push_entries(tokens: &mut Vec<PtxToken>, entries: &[String]) {
    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            tokens.push(PtxToken::Comma);
        }
        if let Some(rest) = entry.strip_prefix('.') {
            push_directive(tokens, rest);
        } else if let Some(numeric) = numeric_token_from_str(entry) {
            tokens.push(numeric);
        } else {
            push_identifier(tokens, entry);
        }
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
        push_decimal(tokens, self.size);
    }
}

impl PtxUnparser for ModuleInfoDirectiveKind {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ModuleInfoDirectiveKind::Version(version) => version.unparse_tokens(tokens),
            ModuleInfoDirectiveKind::Target(target) => target.unparse_tokens(tokens),
            ModuleInfoDirectiveKind::AddressSize(size) => size.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for FileDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "file");
        push_decimal(tokens, self.index);
        tokens.push(PtxToken::StringLiteral(self.path.clone()));
    }
}

impl PtxUnparser for SectionDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "section");
        if let Some(rest) = self.name.strip_prefix('.') {
            push_directive(tokens, rest);
        } else {
            push_identifier(tokens, &self.name);
        }
        for attribute in &self.attributes {
            push_token_from_repr(tokens, attribute);
        }
    }
}

impl PtxUnparser for DwarfDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_raw_tokens(tokens, &self.raw);
    }
}

impl PtxUnparser for ModuleDebugDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ModuleDebugDirective::File(file) => file.unparse_tokens(tokens),
            ModuleDebugDirective::Section(section) => section.unparse_tokens(tokens),
            ModuleDebugDirective::Dwarf(dwarf) => dwarf.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for LinkingDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.kind.unparse_tokens(tokens);
        push_raw_tokens(tokens, &self.prototype);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for ModuleDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ModuleDirective::ModuleVariable(variable) => variable.unparse_tokens(tokens),
            ModuleDirective::FunctionKernel(function) => function.unparse_tokens(tokens),
            ModuleDirective::ModuleInfo(info) => info.unparse_tokens(tokens),
            ModuleDirective::Debug(debug) => debug.unparse_tokens(tokens),
            ModuleDirective::Linking(linking) => linking.unparse_tokens(tokens),
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
