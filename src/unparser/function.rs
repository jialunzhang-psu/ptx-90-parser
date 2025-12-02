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

fn unparse_param(tokens: &mut Vec<PtxToken>, param: &ParameterDirective, spaced: bool) {
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
            push_space(tokens, spaced);
            // Alignment must come before type in PTX syntax
            if let Some(value) = align {
                push_directive(tokens, "align");
                push_space(tokens, spaced);
                push_decimal(tokens, *value);
                push_space(tokens, spaced);
            }
            ty.unparse_tokens_mode(tokens, spaced);
            if *ptr {
                push_directive(tokens, "ptr");
            }
            if let Some(address_space) = space {
                address_space.unparse_tokens_mode(tokens, spaced);
            }
            push_space(tokens, spaced);
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
            push_space(tokens, spaced);
            ty.unparse_tokens_mode(tokens, spaced);
            push_space(tokens, spaced);
            push_register_components(tokens, &name.val);
        }
    }
}

fn unparse_param_list(tokens: &mut Vec<PtxToken>, params: &[ParameterDirective], spaced: bool) {
    for (idx, param) in params.iter().enumerate() {
        if idx > 0 {
            tokens.push(PtxToken::Comma);
            push_space(tokens, spaced);
        }
        unparse_param(tokens, param, spaced);
    }
}

fn unparse_section_line(
    tokens: &mut Vec<PtxToken>,
    line: &StatementSectionDirectiveLine,
    spaced: bool,
) {
    match line {
        StatementSectionDirectiveLine::B8 { values, .. } => {
            push_directive(tokens, "b8");
            for (idx, value) in values.iter().enumerate() {
                if idx > 0 {
                    tokens.push(PtxToken::Comma);
                    push_space(tokens, spaced);
                }
                push_space(tokens, spaced);
                push_signed_decimal_i64(tokens, *value as i64);
            }
            push_newline(tokens, spaced);
        }
        StatementSectionDirectiveLine::B16 { values, .. } => {
            push_directive(tokens, "b16");
            for (idx, value) in values.iter().enumerate() {
                if idx > 0 {
                    tokens.push(PtxToken::Comma);
                    push_space(tokens, spaced);
                }
                push_space(tokens, spaced);
                push_signed_decimal_i64(tokens, *value as i64);
            }
            push_newline(tokens, spaced);
        }
        StatementSectionDirectiveLine::B32Immediate { values, .. } => {
            push_directive(tokens, "b32");
            for (idx, value) in values.iter().enumerate() {
                if idx > 0 {
                    tokens.push(PtxToken::Comma);
                    push_space(tokens, spaced);
                }
                push_space(tokens, spaced);
                push_signed_decimal_i64(tokens, *value);
            }
            push_newline(tokens, spaced);
        }
        StatementSectionDirectiveLine::B64Immediate { values, .. } => {
            push_directive(tokens, "b64");
            for (idx, value) in values.iter().enumerate() {
                if idx > 0 {
                    tokens.push(PtxToken::Comma);
                    push_space(tokens, spaced);
                }
                push_space(tokens, spaced);
                push_signed_decimal_i128(tokens, *value);
            }
            push_newline(tokens, spaced);
        }
        StatementSectionDirectiveLine::B32Label { labels, .. } => {
            push_directive(tokens, "b32");
            push_space(tokens, spaced);
            push_identifier(tokens, &labels.val);
            push_newline(tokens, spaced);
        }
        StatementSectionDirectiveLine::B64Label { labels, .. } => {
            push_directive(tokens, "b64");
            push_space(tokens, spaced);
            push_identifier(tokens, &labels.val);
            push_newline(tokens, spaced);
        }
        StatementSectionDirectiveLine::B32LabelPlusImm { entries, .. } => {
            push_directive(tokens, "b32");
            let (label, offset) = entries;
            push_space(tokens, spaced);
            push_identifier(tokens, &label.val);
            if *offset >= 0 {
                tokens.push(PtxToken::Plus);
                push_decimal(tokens, *offset);
            } else {
                tokens.push(PtxToken::Minus);
                let magnitude = (*offset as i128).abs();
                push_decimal(tokens, magnitude);
            }
            push_newline(tokens, spaced);
        }
        StatementSectionDirectiveLine::B64LabelPlusImm { entries, .. } => {
            push_directive(tokens, "b64");
            let (label, offset) = entries;
            push_space(tokens, spaced);
            push_identifier(tokens, &label.val);
            if *offset >= 0 {
                tokens.push(PtxToken::Plus);
                push_decimal(tokens, *offset);
            } else {
                tokens.push(PtxToken::Minus);
                let magnitude = (*offset as i128).abs();
                push_decimal(tokens, magnitude);
            }
            push_newline(tokens, spaced);
        }
        StatementSectionDirectiveLine::B32LabelDiff { entries, .. } => {
            push_directive(tokens, "b32");
            let (left, right) = entries;
            push_space(tokens, spaced);
            push_identifier(tokens, &left.val);
            tokens.push(PtxToken::Minus);
            push_space(tokens, spaced);
            push_identifier(tokens, &right.val);
            push_newline(tokens, spaced);
        }
        StatementSectionDirectiveLine::B64LabelDiff { entries, .. } => {
            push_directive(tokens, "b64");
            let (left, right) = entries;
            push_space(tokens, spaced);
            push_identifier(tokens, &left.val);
            tokens.push(PtxToken::Minus);
            push_space(tokens, spaced);
            push_identifier(tokens, &right.val);
            push_newline(tokens, spaced);
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
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        push_directive(tokens, "reg");
        push_space(tokens, spaced);
        self.ty.unparse_tokens_mode(tokens, spaced);
        for (idx, target) in self.registers.iter().enumerate() {
            if idx > 0 {
                tokens.push(PtxToken::Comma);
                push_space(tokens, spaced);
            } else {
                push_space(tokens, spaced);
            }
            push_register_components(tokens, &target.name.val);
            if let Some(range) = target.range {
                tokens.push(PtxToken::LAngle);
                push_decimal(tokens, range);
                tokens.push(PtxToken::RAngle);
            }
        }
        tokens.push(PtxToken::Semicolon);
        push_newline(tokens, spaced);
    }
}

impl PtxUnparser for StatementDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        match self {
            StatementDirective::Reg {
                directive: register,
                ..
            } => register.unparse_tokens_mode(tokens, spaced),
            StatementDirective::Local {
                directive: variable,
                ..
            } => {
                push_directive(tokens, "local");
                push_space(tokens, spaced);
                variable.unparse_tokens_mode(tokens, spaced);
            }
            StatementDirective::Param {
                directive: variable,
                ..
            } => {
                push_directive(tokens, "param");
                push_space(tokens, spaced);
                variable.unparse_tokens_mode(tokens, spaced);
            }
            StatementDirective::Shared {
                directive: variable,
                ..
            } => {
                push_directive(tokens, "shared");
                push_space(tokens, spaced);
                variable.unparse_tokens_mode(tokens, spaced);
            }
            StatementDirective::Pragma {
                directive: pragma, ..
            } => {
                push_directive(tokens, "pragma");
                push_space(tokens, spaced);
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
                push_newline(tokens, spaced);
            }
            StatementDirective::BranchTargets { directive, .. } => {
                push_directive(tokens, "branchtargets");
                push_space(tokens, spaced);
                for (idx, label) in directive.labels.iter().enumerate() {
                    if idx > 0 {
                        tokens.push(PtxToken::Comma);
                        push_space(tokens, spaced);
                    }
                    push_token_from_str(tokens, &label.val);
                }
                tokens.push(PtxToken::Semicolon);
                push_newline(tokens, spaced);
            }
            StatementDirective::CallTargets { directive, .. } => {
                push_directive(tokens, "calltargets");
                push_space(tokens, spaced);
                for (idx, target) in directive.targets.iter().enumerate() {
                    if idx > 0 {
                        tokens.push(PtxToken::Comma);
                        push_space(tokens, spaced);
                    }
                    push_token_from_str(tokens, &target.val);
                }
                tokens.push(PtxToken::Semicolon);
                push_newline(tokens, spaced);
            }
            StatementDirective::Loc { directive: loc, .. } => {
                push_directive(tokens, "loc");
                push_space(tokens, spaced);
                push_decimal(tokens, loc.file_index);
                push_space(tokens, spaced);
                push_decimal(tokens, loc.line);
                push_space(tokens, spaced);
                push_decimal(tokens, loc.column);
                if let Some(inline) = &loc.inlined_at {
                    tokens.push(PtxToken::Comma);
                    push_space(tokens, spaced);
                    push_identifier(tokens, "inlined_at");
                    push_space(tokens, spaced);
                    push_decimal(tokens, inline.file_index);
                    push_space(tokens, spaced);
                    push_decimal(tokens, inline.line);
                    push_space(tokens, spaced);
                    push_decimal(tokens, inline.column);
                    tokens.push(PtxToken::Comma);
                    push_space(tokens, spaced);
                    push_identifier(tokens, &inline.function_name.val);
                    push_space(tokens, spaced);
                    push_identifier(tokens, &inline.label.val);
                    if let Some(offset) = inline.label_offset {
                        if offset >= 0 {
                            tokens.push(PtxToken::Plus);
                        } else {
                            tokens.push(PtxToken::Minus);
                        }
                        push_decimal(tokens, offset.abs());
                    }
                }
                push_newline(tokens, spaced);
            }
            StatementDirective::Dwarf {
                directive: dwarf, ..
            } => {
                dwarf.unparse_tokens_mode(tokens, spaced);
                push_newline(tokens, spaced);
            }
            StatementDirective::Section {
                directive: section, ..
            } => {
                section.unparse_tokens_mode(tokens, spaced);
            }
            StatementDirective::CallPrototype { directive, .. } => {
                push_directive(tokens, "callprototype");
                push_space(tokens, spaced);
                if let Some(ret) = &directive.return_param {
                    unparse_param(tokens, ret, spaced);
                } else {
                    push_identifier(tokens, "_");
                }
                tokens.push(PtxToken::LParen);
                unparse_param_list(tokens, &directive.params, spaced);
                tokens.push(PtxToken::RParen);
                if directive.noreturn {
                    push_space(tokens, spaced);
                    push_directive(tokens, "noreturn");
                }
                if let Some(value) = directive.abi_preserve {
                    push_space(tokens, spaced);
                    push_directive(tokens, "abi_preserve");
                    push_space(tokens, spaced);
                    push_decimal(tokens, value);
                }
                if let Some(value) = directive.abi_preserve_control {
                    push_space(tokens, spaced);
                    push_directive(tokens, "abi_preserve_control");
                    push_space(tokens, spaced);
                    push_decimal(tokens, value);
                }
                tokens.push(PtxToken::Semicolon);
                push_newline(tokens, spaced);
            }
        }
    }
}

impl PtxUnparser for SectionDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        push_directive(tokens, "section");
        push_space(tokens, spaced);
        push_token_from_str(tokens, &self.name);
        push_space(tokens, spaced);
        tokens.push(PtxToken::LBrace);
        push_newline(tokens, spaced);
        for entry in &self.entries {
            match entry {
                SectionEntry::Label { label, .. } => {
                    push_identifier(tokens, &label.val);
                    tokens.push(PtxToken::Colon);
                    push_newline(tokens, spaced);
                }
                SectionEntry::Directive(line) => {
                    unparse_section_line(tokens, line, spaced);
                }
            }
        }
        tokens.push(PtxToken::RBrace);
        push_newline(tokens, spaced);
    }
}

impl PtxUnparser for FunctionStatement {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        match self {
            FunctionStatement::Label { label, .. } => {
                push_identifier(tokens, &label.val);
                tokens.push(PtxToken::Colon);
                push_newline(tokens, spaced);
            }
            FunctionStatement::Instruction { instruction, .. } => {
                instruction.unparse_tokens_mode(tokens, spaced)
            }
            FunctionStatement::Directive { directive, .. } => {
                directive.unparse_tokens_mode(tokens, spaced)
            }
            FunctionStatement::Block {
                statements: block, ..
            } => {
                tokens.push(PtxToken::LBrace);
                for statement in block {
                    statement.unparse_tokens_mode(tokens, spaced);
                }
                tokens.push(PtxToken::RBrace);
                push_newline(tokens, spaced);
            }
        }
    }
}

impl PtxUnparser for FunctionBody {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        tokens.push(PtxToken::LBrace);
        push_newline(tokens, spaced);
        for statement in &self.statements {
            statement.unparse_tokens_mode(tokens, spaced);
        }
        tokens.push(PtxToken::RBrace);
        push_newline(tokens, spaced);
    }
}

impl PtxUnparser for FunctionDim {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        match self {
            FunctionDim::X { x, .. } => {
                push_decimal(tokens, *x);
            }
            FunctionDim::XY { x, y, .. } => {
                push_decimal(tokens, *x);
                tokens.push(PtxToken::Comma);
                push_space(tokens, spaced);
                push_decimal(tokens, *y);
            }
            FunctionDim::XYZ { x, y, z, .. } => {
                push_decimal(tokens, *x);
                tokens.push(PtxToken::Comma);
                push_space(tokens, spaced);
                push_decimal(tokens, *y);
                tokens.push(PtxToken::Comma);
                push_space(tokens, spaced);
                push_decimal(tokens, *z);
            }
        }
    }
}

impl PtxUnparser for EntryFunctionHeaderDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        match self {
            EntryFunctionHeaderDirective::MaxNReg { value, .. } => {
                push_directive(tokens, "maxnreg");
                push_space(tokens, spaced);
                push_decimal(tokens, *value);
            }
            EntryFunctionHeaderDirective::MaxNTid { dim, .. } => {
                push_directive(tokens, "maxntid");
                push_space(tokens, spaced);
                dim.unparse_tokens_mode(tokens, spaced);
            }
            EntryFunctionHeaderDirective::ReqNTid { dim, .. } => {
                push_directive(tokens, "reqntid");
                push_space(tokens, spaced);
                dim.unparse_tokens_mode(tokens, spaced);
            }
            EntryFunctionHeaderDirective::MinNCtaPerSm { value, .. } => {
                push_directive(tokens, "minnctapersm");
                push_space(tokens, spaced);
                push_decimal(tokens, *value);
            }
            EntryFunctionHeaderDirective::MaxNCtaPerSm { value, .. } => {
                push_directive(tokens, "maxnctapersm");
                push_space(tokens, spaced);
                push_decimal(tokens, *value);
            }
            EntryFunctionHeaderDirective::Pragma {
                args: arguments, ..
            } => {
                push_directive(tokens, "pragma");
                push_space(tokens, spaced);
                for argument in arguments {
                    tokens.push(PtxToken::StringLiteral(argument.clone()));
                    push_space(tokens, spaced);
                }
                if spaced {
                    if let Some(last) = tokens.last() {
                        if matches!(last, PtxToken::Space) {
                            tokens.pop();
                        }
                    }
                }
            }
            EntryFunctionHeaderDirective::ReqNctaPerCluster { dim, .. } => {
                push_directive(tokens, "reqnctapercluster");
                push_space(tokens, spaced);
                dim.unparse_tokens_mode(tokens, spaced);
            }
            EntryFunctionHeaderDirective::ExplicitCluster { .. } => {
                push_directive(tokens, "explicitcluster");
            }
            EntryFunctionHeaderDirective::MaxClusterRank { value, .. } => {
                push_directive(tokens, "maxclusterrank");
                push_space(tokens, spaced);
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
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        match self {
            FuncFunctionHeaderDirective::NoReturn { .. } => push_directive(tokens, "noreturn"),
            FuncFunctionHeaderDirective::Pragma {
                args: arguments, ..
            } => {
                push_directive(tokens, "pragma");
                push_space(tokens, spaced);
                for argument in arguments {
                    tokens.push(PtxToken::StringLiteral(argument.clone()));
                    push_space(tokens, spaced);
                }
                if spaced {
                    if let Some(PtxToken::Space) = tokens.last() {
                        tokens.pop();
                    }
                }
            }
            FuncFunctionHeaderDirective::AbiPreserve { value, .. } => {
                push_directive(tokens, "abi_preserve");
                push_space(tokens, spaced);
                push_decimal(tokens, *value);
            }
            FuncFunctionHeaderDirective::AbiPreserveControl { value, .. } => {
                push_directive(tokens, "abi_preserve_control");
                push_space(tokens, spaced);
                push_decimal(tokens, *value);
            }
        }
    }
}

impl PtxUnparser for AliasFunctionDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        push_directive(tokens, "alias");
        push_space(tokens, spaced);
        push_identifier(tokens, &self.alias.val);
        tokens.push(PtxToken::Comma);
        push_space(tokens, spaced);
        push_identifier(tokens, &self.target.val);
        tokens.push(PtxToken::Semicolon);
        push_newline(tokens, spaced);
    }
}

impl PtxUnparser for EntryFunctionDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        for directive in &self.directives {
            directive.unparse_tokens_mode(tokens, spaced);
            if spaced {
                push_space(tokens, spaced);
            }
        }
        push_directive(tokens, "entry");
        push_space(tokens, spaced);
        push_identifier(tokens, &self.name.val);
        tokens.push(PtxToken::LParen);
        unparse_param_list(tokens, &self.params, spaced);
        tokens.push(PtxToken::RParen);
        match &self.body {
            Some(body) => body.unparse_tokens_mode(tokens, spaced),
            None => {
                tokens.push(PtxToken::Semicolon);
                push_newline(tokens, spaced);
            }
        }
    }
}

impl PtxUnparser for FuncFunctionDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }

    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        for attribute in &self.attributes {
            attribute.unparse_tokens_mode(tokens, spaced);
            if spaced {
                push_space(tokens, spaced);
            }
        }
        for directive in &self.directives {
            directive.unparse_tokens_mode(tokens, spaced);
            if spaced {
                push_space(tokens, spaced);
            }
        }
        push_directive(tokens, "func");
        if let Some(ret) = &self.return_param {
            push_space(tokens, spaced);
            tokens.push(PtxToken::LParen);
            unparse_param(tokens, ret, spaced);
            tokens.push(PtxToken::RParen);
        }
        push_space(tokens, spaced);
        push_identifier(tokens, &self.name.val);
        tokens.push(PtxToken::LParen);
        unparse_param_list(tokens, &self.params, spaced);
        tokens.push(PtxToken::RParen);
        // Emit pre-body declarations (.reg, .local, .shared, .param) before the body
        for decl in &self.pre_body_declarations {
            push_newline(tokens, spaced);
            decl.unparse_tokens_mode(tokens, spaced);
        }
        match &self.body {
            Some(body) => body.unparse_tokens_mode(tokens, spaced),
            None => {
                tokens.push(PtxToken::Semicolon);
                push_newline(tokens, spaced);
            }
        }
    }
}
