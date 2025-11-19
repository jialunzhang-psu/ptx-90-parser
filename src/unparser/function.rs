use crate::unparser::common::push_register;
use crate::{
    lexer::PtxToken,
    r#type::{function::*, variable::ParameterDirective},
    unparser::*,
};

fn push_register_components(tokens: &mut Vec<PtxToken>, name: &str) {
    if let Some(stripped) = name.strip_prefix('%') {
        let mut parts = stripped.split('.');
        if let Some(first) = parts.next() {
            let register_name = format!("%{first}");
            push_register(tokens, &register_name);
        }
        for part in parts {
            if part.is_empty() {
                continue;
            }
            push_directive(tokens, part);
        }
    } else {
        push_identifier(tokens, name);
    }
}

fn unparse_param(tokens: &mut Vec<PtxToken>, param: &ParameterDirective) {
    match param {
        ParameterDirective::Parameter {
            align,
            ty,
            ptr,
            space,
            name,
            array,
            ..
        } => {
            push_directive(tokens, "param");
            ty.unparse_tokens(tokens);
            if *ptr {
                push_directive(tokens, "ptr");
            }
            if let Some(address_space) = space {
                address_space.unparse_tokens(tokens);
            }
            if let Some(value) = align {
                push_directive(tokens, "align");
                push_decimal(tokens, *value);
            }
            push_identifier(tokens, &name.val);
            for extent in array {
                tokens.push(PtxToken::LBracket);
                if let Some(value) = extent {
                    push_decimal(tokens, *value);
                }
                tokens.push(PtxToken::RBracket);
            }
        }
        ParameterDirective::Register { ty, name, .. } => {
            push_directive(tokens, "reg");
            ty.unparse_tokens(tokens);
            push_register_components(tokens, &name.val);
        }
    }
}

fn unparse_param_list(tokens: &mut Vec<PtxToken>, params: &[ParameterDirective]) {
    for (idx, param) in params.iter().enumerate() {
        if idx > 0 {
            tokens.push(PtxToken::Comma);
        }
        unparse_param(tokens, param);
    }
}

fn unparse_section_line(tokens: &mut Vec<PtxToken>, line: &StatementSectionDirectiveLine) {
    match line {
        StatementSectionDirectiveLine::B8 { values, .. } => {
            push_directive(tokens, "b8");
            for (idx, value) in values.iter().enumerate() {
                if idx > 0 {
                    tokens.push(PtxToken::Comma);
                }
                push_signed_decimal_i64(tokens, *value as i64);
            }
            tokens.push(PtxToken::Semicolon);
        }
        StatementSectionDirectiveLine::B16 { values, .. } => {
            push_directive(tokens, "b16");
            for (idx, value) in values.iter().enumerate() {
                if idx > 0 {
                    tokens.push(PtxToken::Comma);
                }
                push_signed_decimal_i64(tokens, *value as i64);
            }
            tokens.push(PtxToken::Semicolon);
        }
        StatementSectionDirectiveLine::B32Immediate { values, .. } => {
            push_directive(tokens, "b32");
            for (idx, value) in values.iter().enumerate() {
                if idx > 0 {
                    tokens.push(PtxToken::Comma);
                }
                push_signed_decimal_i64(tokens, *value);
            }
            tokens.push(PtxToken::Semicolon);
        }
        StatementSectionDirectiveLine::B64Immediate { values, .. } => {
            push_directive(tokens, "b64");
            for (idx, value) in values.iter().enumerate() {
                if idx > 0 {
                    tokens.push(PtxToken::Comma);
                }
                push_signed_decimal_i128(tokens, *value);
            }
            tokens.push(PtxToken::Semicolon);
        }
        StatementSectionDirectiveLine::B32Label { labels, .. } => {
            push_directive(tokens, "b32");
            push_identifier(tokens, &labels.val);
            tokens.push(PtxToken::Semicolon);
        }
        StatementSectionDirectiveLine::B64Label { labels, .. } => {
            push_directive(tokens, "b64");
            push_identifier(tokens, &labels.val);
            tokens.push(PtxToken::Semicolon);
        }
        StatementSectionDirectiveLine::B32LabelPlusImm { entries, .. } => {
            push_directive(tokens, "b32");
            let (label, offset) = entries;
            push_identifier(tokens, &label.val);
            if *offset >= 0 {
                tokens.push(PtxToken::Plus);
                push_decimal(tokens, *offset);
            } else {
                tokens.push(PtxToken::Minus);
                let magnitude = (*offset as i128).abs();
                push_decimal(tokens, magnitude);
            }
            tokens.push(PtxToken::Semicolon);
        }
        StatementSectionDirectiveLine::B64LabelPlusImm { entries, .. } => {
            push_directive(tokens, "b64");
            let (label, offset) = entries;
            push_identifier(tokens, &label.val);
            if *offset >= 0 {
                tokens.push(PtxToken::Plus);
                push_decimal(tokens, *offset);
            } else {
                tokens.push(PtxToken::Minus);
                let magnitude = (*offset as i128).abs();
                push_decimal(tokens, magnitude);
            }
            tokens.push(PtxToken::Semicolon);
        }
        StatementSectionDirectiveLine::B32LabelDiff { entries, .. } => {
            push_directive(tokens, "b32");
            let (left, right) = entries;
            push_identifier(tokens, &left.val);
            tokens.push(PtxToken::Minus);
            push_identifier(tokens, &right.val);
            tokens.push(PtxToken::Semicolon);
        }
        StatementSectionDirectiveLine::B64LabelDiff { entries, .. } => {
            push_directive(tokens, "b64");
            let (left, right) = entries;
            push_identifier(tokens, &left.val);
            tokens.push(PtxToken::Minus);
            push_identifier(tokens, &right.val);
            tokens.push(PtxToken::Semicolon);
        }
    }
}

fn push_signed_decimal_i64(tokens: &mut Vec<PtxToken>, value: i64) {
    if value < 0 {
        tokens.push(PtxToken::Minus);
        push_decimal(tokens, (-value) as i128);
    } else {
        push_decimal(tokens, value);
    }
}

fn push_signed_decimal_i128(tokens: &mut Vec<PtxToken>, value: i128) {
    if value < 0 {
        tokens.push(PtxToken::Minus);
        push_decimal(tokens, -value);
    } else {
        push_decimal(tokens, value);
    }
}

impl PtxUnparser for RegisterDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "reg");
        self.ty.unparse_tokens(tokens);
        for (idx, target) in self.registers.iter().enumerate() {
            if idx > 0 {
                tokens.push(PtxToken::Comma);
            }
            push_register_components(tokens, &target.name.val);
            if let Some(range) = target.range {
                tokens.push(PtxToken::LAngle);
                push_decimal(tokens, range);
                tokens.push(PtxToken::RAngle);
            }
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for StatementDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            StatementDirective::Reg {
                directive: register,
                ..
            } => register.unparse_tokens(tokens),
            StatementDirective::Local {
                directive: variable,
                ..
            } => {
                push_directive(tokens, "local");
                variable.unparse_tokens(tokens);
            }
            StatementDirective::Param {
                directive: variable,
                ..
            } => {
                push_directive(tokens, "param");
                variable.unparse_tokens(tokens);
            }
            StatementDirective::Shared {
                directive: variable,
                ..
            } => {
                push_directive(tokens, "shared");
                variable.unparse_tokens(tokens);
            }
            StatementDirective::Pragma {
                directive: pragma, ..
            } => {
                push_directive(tokens, "pragma");
                let text = match &pragma.kind {
                    PragmaDirectiveKind::Nounroll => "nounroll".to_string(),
                    PragmaDirectiveKind::EnableSmemSpilling => "enable_smem_spilling".to_string(),
                    PragmaDirectiveKind::UsedBytesMask { mask } => {
                        format!("used_bytes_mask {}", mask)
                    }
                    PragmaDirectiveKind::Frequency { value } => {
                        format!("frequency {}", value)
                    }
                    PragmaDirectiveKind::Raw(text) => text.clone(),
                };
                tokens.push(PtxToken::StringLiteral(text));
                tokens.push(PtxToken::Semicolon);
            }
            StatementDirective::BranchTargets { directive, .. } => {
                push_directive(tokens, "branchtargets");
                for (idx, label) in directive.labels.iter().enumerate() {
                    if idx > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    push_token_from_str(tokens, &label.val);
                }
                tokens.push(PtxToken::Semicolon);
            }
            StatementDirective::CallTargets { directive, .. } => {
                push_directive(tokens, "calltargets");
                for (idx, target) in directive.targets.iter().enumerate() {
                    if idx > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    push_token_from_str(tokens, &target.val);
                }
                tokens.push(PtxToken::Semicolon);
            }
            StatementDirective::Loc { directive: loc, .. } => {
                push_directive(tokens, "loc");
                push_decimal(tokens, loc.file_index);
                push_decimal(tokens, loc.line);
                push_decimal(tokens, loc.column);
                if let Some(inline) = &loc.inlined_at {
                    tokens.push(PtxToken::Comma);
                    push_identifier(tokens, "inlined_at");
                    push_decimal(tokens, inline.file_index);
                    push_decimal(tokens, inline.line);
                    push_decimal(tokens, inline.column);
                    tokens.push(PtxToken::Comma);
                    push_identifier(tokens, &inline.function_name.val);
                    push_identifier(tokens, &inline.label.val);
                    if let Some(offset) = inline.label_offset {
                        if offset >= 0 {
                            tokens.push(PtxToken::Plus);
                            push_decimal(tokens, offset);
                        } else {
                            tokens.push(PtxToken::Minus);
                            push_decimal(tokens, offset.abs());
                        }
                    }
                }
                tokens.push(PtxToken::Semicolon);
            }
            StatementDirective::Dwarf {
                directive: dwarf, ..
            } => {
                dwarf.unparse_tokens(tokens);
            }
            StatementDirective::Section {
                directive: section, ..
            } => {
                section.unparse_tokens(tokens);
            }
            StatementDirective::CallPrototype { directive, .. } => {
                push_directive(tokens, "callprototype");
                if let Some(ret) = &directive.return_param {
                    unparse_param(tokens, ret);
                } else {
                    push_identifier(tokens, "_");
                }
                tokens.push(PtxToken::LParen);
                unparse_param_list(tokens, &directive.params);
                tokens.push(PtxToken::RParen);
                if directive.noreturn {
                    push_directive(tokens, "noreturn");
                }
                if let Some(value) = directive.abi_preserve {
                    push_directive(tokens, "abi_preserve");
                    push_decimal(tokens, value);
                }
                if let Some(value) = directive.abi_preserve_control {
                    push_directive(tokens, "abi_preserve_control");
                    push_decimal(tokens, value);
                }
                tokens.push(PtxToken::Semicolon);
            }
        }
    }
}

impl PtxUnparser for SectionDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "section");
        push_token_from_str(tokens, &self.name);
        tokens.push(PtxToken::LBrace);
        for entry in &self.entries {
            match entry {
                SectionEntry::Label { label, .. } => {
                    push_identifier(tokens, &label.val);
                    tokens.push(PtxToken::Colon);
                }
                SectionEntry::Directive(line) => {
                    unparse_section_line(tokens, line);
                }
            }
        }
        tokens.push(PtxToken::RBrace);
    }
}

impl PtxUnparser for FunctionStatement {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            FunctionStatement::Label { label, .. } => {
                push_identifier(tokens, &label.val);
                tokens.push(PtxToken::Colon);
            }
            FunctionStatement::Instruction { instruction, .. } => {
                instruction.unparse_tokens(tokens)
            }
            FunctionStatement::Directive { directive, .. } => directive.unparse_tokens(tokens),
            FunctionStatement::Block {
                statements: block, ..
            } => {
                tokens.push(PtxToken::LBrace);
                for statement in block {
                    statement.unparse_tokens(tokens);
                }
                tokens.push(PtxToken::RBrace);
            }
        }
    }
}

impl PtxUnparser for FunctionBody {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::LBrace);
        for statement in &self.statements {
            statement.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::RBrace);
    }
}

impl PtxUnparser for FunctionDim {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            FunctionDim::X { x, .. } => {
                push_decimal(tokens, *x);
            }
            FunctionDim::XY { x, y, .. } => {
                push_decimal(tokens, *x);
                tokens.push(PtxToken::Comma);
                push_decimal(tokens, *y);
            }
            FunctionDim::XYZ { x, y, z, .. } => {
                push_decimal(tokens, *x);
                tokens.push(PtxToken::Comma);
                push_decimal(tokens, *y);
                tokens.push(PtxToken::Comma);
                push_decimal(tokens, *z);
            }
        }
    }
}

impl PtxUnparser for EntryFunctionHeaderDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            EntryFunctionHeaderDirective::MaxNReg { value, .. } => {
                push_directive(tokens, "maxnreg");
                push_decimal(tokens, *value);
            }
            EntryFunctionHeaderDirective::MaxNTid { dim, .. } => {
                push_directive(tokens, "maxntid");
                dim.unparse_tokens(tokens);
            }
            EntryFunctionHeaderDirective::ReqNTid { dim, .. } => {
                push_directive(tokens, "reqntid");
                dim.unparse_tokens(tokens);
            }
            EntryFunctionHeaderDirective::MinNCtaPerSm { value, .. } => {
                push_directive(tokens, "minnctapersm");
                push_decimal(tokens, *value);
            }
            EntryFunctionHeaderDirective::MaxNCtaPerSm { value, .. } => {
                push_directive(tokens, "maxnctapersm");
                push_decimal(tokens, *value);
            }
            EntryFunctionHeaderDirective::Pragma {
                args: arguments, ..
            } => {
                push_directive(tokens, "pragma");
                for argument in arguments {
                    tokens.push(PtxToken::StringLiteral(argument.clone()));
                }
            }
            EntryFunctionHeaderDirective::ReqNctaPerCluster { dim, .. } => {
                push_directive(tokens, "reqnctapercluster");
                dim.unparse_tokens(tokens);
            }
            EntryFunctionHeaderDirective::ExplicitCluster { .. } => {
                push_directive(tokens, "explicitcluster");
            }
            EntryFunctionHeaderDirective::MaxClusterRank { value, .. } => {
                push_directive(tokens, "maxclusterrank");
                push_decimal(tokens, *value);
            }
            EntryFunctionHeaderDirective::BlocksAreClusters { .. } => {
                push_directive(tokens, "blocksareclusters")
            }
        }
    }
}

impl PtxUnparser for FuncFunctionHeaderDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            FuncFunctionHeaderDirective::NoReturn { .. } => push_directive(tokens, "noreturn"),
            FuncFunctionHeaderDirective::Pragma {
                args: arguments, ..
            } => {
                push_directive(tokens, "pragma");
                for argument in arguments {
                    tokens.push(PtxToken::StringLiteral(argument.clone()));
                }
            }
            FuncFunctionHeaderDirective::AbiPreserve { value, .. } => {
                push_directive(tokens, "abi_preserve");
                push_decimal(tokens, *value);
            }
            FuncFunctionHeaderDirective::AbiPreserveControl { value, .. } => {
                push_directive(tokens, "abi_preserve_control");
                push_decimal(tokens, *value);
            }
        }
    }
}

impl PtxUnparser for AliasFunctionDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "alias");
        push_identifier(tokens, &self.alias.val);
        tokens.push(PtxToken::Comma);
        push_identifier(tokens, &self.target.val);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for EntryFunctionDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        for directive in &self.directives {
            directive.unparse_tokens(tokens);
        }
        push_directive(tokens, "entry");
        push_identifier(tokens, &self.name.val);
        tokens.push(PtxToken::LParen);
        unparse_param_list(tokens, &self.params);
        tokens.push(PtxToken::RParen);
        match &self.body {
            Some(body) => body.unparse_tokens(tokens),
            None => tokens.push(PtxToken::Semicolon),
        }
    }
}

impl PtxUnparser for FuncFunctionDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        for attribute in &self.attributes {
            attribute.unparse_tokens(tokens);
        }
        for directive in &self.directives {
            directive.unparse_tokens(tokens);
        }
        push_directive(tokens, "func");
        if let Some(ret) = &self.return_param {
            tokens.push(PtxToken::LParen);
            unparse_param(tokens, ret);
            tokens.push(PtxToken::RParen);
        }
        push_identifier(tokens, &self.name.val);
        tokens.push(PtxToken::LParen);
        unparse_param_list(tokens, &self.params);
        tokens.push(PtxToken::RParen);
        match &self.body {
            Some(body) => body.unparse_tokens(tokens),
            None => tokens.push(PtxToken::Semicolon),
        }
    }
}
