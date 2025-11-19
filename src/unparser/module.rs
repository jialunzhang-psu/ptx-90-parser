use super::PtxUnparser;
use crate::{
    lexer::PtxToken,
    r#type::module::*,
    unparser::{push_decimal, push_directive, push_identifier},
};

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
