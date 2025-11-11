use crate::unparser::common::push_register;
use crate::{
    lexer::PtxToken,
    r#type::{function::*, variable::VariableDirective},
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

fn unparse_param(tokens: &mut Vec<PtxToken>, param: &VariableDirective) {
    let mut param_tokens = param.to_tokens();
    if matches!(param_tokens.last(), Some(PtxToken::Semicolon)) {
        param_tokens.pop();
    }
    tokens.extend(param_tokens);
}

fn unparse_param_list(tokens: &mut Vec<PtxToken>, params: &[VariableDirective]) {
    for (idx, param) in params.iter().enumerate() {
        if idx > 0 {
            tokens.push(PtxToken::Comma);
        }
        unparse_param(tokens, param);
    }
}

impl PtxUnparser for RegisterDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "reg");
        if let Some(ty) = &self.ty {
            push_directive(tokens, ty);
        }
        push_register_components(tokens, &self.name);
        if let Some(range) = self.range {
            tokens.push(PtxToken::LAngle);
            push_decimal(tokens, range);
            tokens.push(PtxToken::RAngle);
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for StatementDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            StatementDirective::Reg { directive: register, .. } => register.unparse_tokens(tokens),
            StatementDirective::Local { directive: variable, .. }
            | StatementDirective::Param { directive: variable, .. }
            | StatementDirective::Shared { directive: variable, .. } => variable.unparse_tokens(tokens),
            StatementDirective::Pragma { directive: pragma, .. } => {
                push_directive(tokens, "pragma");
                for (idx, argument) in pragma.arguments.iter().enumerate() {
                    if idx > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    push_token_from_str(tokens, argument);
                }
                tokens.push(PtxToken::Semicolon);
            }
            StatementDirective::Loc { directive: loc, .. } => {
                push_directive(tokens, "loc");
                push_decimal(tokens, loc.file_index);
                push_decimal(tokens, loc.line);
                push_decimal(tokens, loc.column);
                for option in &loc.options {
                    push_token_from_str(tokens, option);
                }
                tokens.push(PtxToken::Semicolon);
            }
            StatementDirective::Dwarf { directive: dwarf, .. } => {
                push_directive(tokens, "dwarf");
                push_identifier(tokens, &dwarf.keyword);
                for argument in &dwarf.arguments {
                    tokens.push(PtxToken::Comma);
                    push_token_from_str(tokens, argument);
                }
                tokens.push(PtxToken::Semicolon);
            }
            StatementDirective::Section { directive: section, .. } => {
                push_directive(tokens, "section");
                push_token_from_str(tokens, &section.name);
                for argument in &section.arguments {
                    tokens.push(PtxToken::Comma);
                    push_token_from_str(tokens, argument);
                }
                tokens.push(PtxToken::Semicolon);
            }
        }
    }
}

impl PtxUnparser for FunctionStatement {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            FunctionStatement::Label { name, .. } => {
                push_identifier(tokens, name);
                tokens.push(PtxToken::Colon);
            }
            FunctionStatement::Instruction { instruction, .. } => instruction.unparse_tokens(tokens),
            FunctionStatement::Directive { directive, .. } => directive.unparse_tokens(tokens),
            FunctionStatement::Block { statements: block, .. } => {
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

impl PtxUnparser for FunctionDim3 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_decimal(tokens, self.x);
        if let Some(y) = self.y {
            tokens.push(PtxToken::Comma);
            push_decimal(tokens, y);
        }
        if let Some(z) = self.z {
            tokens.push(PtxToken::Comma);
            push_decimal(tokens, z);
        }
    }
}

impl PtxUnparser for FunctionHeaderDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            FunctionHeaderDirective::Linkage { linkage, .. } => linkage.unparse_tokens(tokens),
            FunctionHeaderDirective::NoReturn { .. } => push_directive(tokens, "noreturn"),
            FunctionHeaderDirective::AbiPreserve { value, .. } => {
                push_directive(tokens, "abipreserve");
                push_decimal(tokens, *value);
            }
            FunctionHeaderDirective::AbiPreserveControl { value, .. } => {
                push_directive(tokens, "abipreserve_control");
                push_decimal(tokens, *value);
            }
            FunctionHeaderDirective::MaxClusterRank { value, .. } => {
                push_directive(tokens, "maxclusterrank");
                push_decimal(tokens, *value);
            }
            FunctionHeaderDirective::BlocksAreClusters { .. } => {
                push_directive(tokens, "blocksareclusters")
            }
            FunctionHeaderDirective::ExplicitCluster { dim, .. } => {
                push_directive(tokens, "explicitcluster");
                dim.unparse_tokens(tokens);
            }
            FunctionHeaderDirective::ReqNctaPerCluster { dim, .. } => {
                push_directive(tokens, "reqnctapercluster");
                dim.unparse_tokens(tokens);
            }
            FunctionHeaderDirective::MaxNReg { value, .. } => {
                push_directive(tokens, "maxnreg");
                push_decimal(tokens, *value);
            }
            FunctionHeaderDirective::MaxNTid { dim, .. } => {
                push_directive(tokens, "maxntid");
                dim.unparse_tokens(tokens);
            }
            FunctionHeaderDirective::MinNCtaPerSm { value, .. } => {
                push_directive(tokens, "minnctapersm");
                push_decimal(tokens, *value);
            }
            FunctionHeaderDirective::ReqNTid { dim, .. } => {
                push_directive(tokens, "reqntid");
                dim.unparse_tokens(tokens);
            }
            FunctionHeaderDirective::MaxNCtaPerSm { value, .. } => {
                push_directive(tokens, "maxnctapersm");
                push_decimal(tokens, *value);
            }
            FunctionHeaderDirective::Pragma { args: arguments, .. } => {
                push_directive(tokens, "pragma");
                for argument in arguments {
                    tokens.push(PtxToken::Identifier(argument.clone()));
                }
            }
        }
    }
}

impl PtxUnparser for FunctionAlias {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "alias");
        push_identifier(tokens, &self.alias);
        tokens.push(PtxToken::Comma);
        push_identifier(tokens, &self.target);
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for EntryFunction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        for directive in &self.directives {
            directive.unparse_tokens(tokens);
        }
        push_directive(tokens, "entry");
        push_identifier(tokens, &self.name);
        tokens.push(PtxToken::LParen);
        unparse_param_list(tokens, &self.params);
        tokens.push(PtxToken::RParen);
        if self.body.statements.is_empty() {
            tokens.push(PtxToken::LBrace);
            tokens.push(PtxToken::RBrace);
        } else {
            self.body.unparse_tokens(tokens);
        }
    }
}

impl PtxUnparser for FuncFunction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        for directive in &self.directives {
            directive.unparse_tokens(tokens);
        }
        push_directive(tokens, "func");
        if let Some(ret) = &self.return_param {
            tokens.push(PtxToken::LParen);
            unparse_param(tokens, ret);
            tokens.push(PtxToken::RParen);
        }
        push_identifier(tokens, &self.name);
        tokens.push(PtxToken::LParen);
        unparse_param_list(tokens, &self.params);
        tokens.push(PtxToken::RParen);
        if self.body.statements.is_empty() {
            tokens.push(PtxToken::Semicolon);
        } else {
            self.body.unparse_tokens(tokens);
        }
    }
}

impl PtxUnparser for FunctionKernelDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            FunctionKernelDirective::Entry { function: entry, .. } => entry.unparse_tokens(tokens),
            FunctionKernelDirective::Func { function: func, .. } => func.unparse_tokens(tokens),
            FunctionKernelDirective::Alias { alias, .. } => alias.unparse_tokens(tokens),
        }
    }
}
