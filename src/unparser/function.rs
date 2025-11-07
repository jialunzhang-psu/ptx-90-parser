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

fn unparse_register_directive(tokens: &mut Vec<PtxToken>, directive: &RegisterDirective) {
    push_directive(tokens, "reg");
    if let Some(ty) = &directive.ty {
        push_directive(tokens, ty);
    }
    push_register_components(tokens, &directive.name);
    if let Some(range) = directive.range {
        tokens.push(PtxToken::LAngle);
        push_decimal(tokens, range);
        tokens.push(PtxToken::RAngle);
    }
    tokens.push(PtxToken::Semicolon);
}

fn unparse_entry_directive(tokens: &mut Vec<PtxToken>, directive: &FunctionEntryDirective) {
    match directive {
        FunctionEntryDirective::Reg(register) => unparse_register_directive(tokens, register),
        FunctionEntryDirective::Local(variable) => variable.unparse_tokens(tokens),
        FunctionEntryDirective::Param(variable) => variable.unparse_tokens(tokens),
        FunctionEntryDirective::Shared(variable) => variable.unparse_tokens(tokens),
        FunctionEntryDirective::Pragma(_) => {
            panic!("unimplemented: unparsing .pragma function entry directives");
        }
        FunctionEntryDirective::Loc(_) => {
            panic!("unimplemented: unparsing .loc function entry directives");
        }
        FunctionEntryDirective::Dwarf(_) => {
            panic!("unimplemented: unparsing dwarf function entry directives");
        }
    }
}

fn unparse_extern_call_setup(tokens: &mut Vec<PtxToken>, setup: &ExternCallSetup) {
    match setup {
        ExternCallSetup::Param(variable) => variable.unparse_tokens(tokens),
        ExternCallSetup::Store(instruction) => instruction.unparse_tokens(tokens),
    }
}

fn unparse_extern_call_block(tokens: &mut Vec<PtxToken>, block: &ExternCallBlock) {
    tokens.push(PtxToken::LBrace);
    for directive in &block.declarations {
        unparse_entry_directive(tokens, directive);
    }
    for entry in &block.setup {
        unparse_extern_call_setup(tokens, entry);
    }
    block.call.unparse_tokens(tokens);
    for instruction in &block.post_call {
        instruction.unparse_tokens(tokens);
    }
    tokens.push(PtxToken::RBrace);
}

fn unparse_function_statement(tokens: &mut Vec<PtxToken>, statement: &FunctionStatement) {
    match statement {
        FunctionStatement::Instruction(instruction) => instruction.unparse_tokens(tokens),
        FunctionStatement::ExternCallBlock(block) => unparse_extern_call_block(tokens, block),
        FunctionStatement::Directive(_) => {
            panic!("unimplemented: unparsing function statement directives");
        }
    }
}

fn unparse_function_dim(tokens: &mut Vec<PtxToken>, dim: &FunctionDim3) {
    push_decimal(tokens, dim.x);
    if let Some(y) = dim.y {
        tokens.push(PtxToken::Comma);
        push_decimal(tokens, y);
    }
    if let Some(z) = dim.z {
        tokens.push(PtxToken::Comma);
        push_decimal(tokens, z);
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

fn unparse_function_header_directive(
    tokens: &mut Vec<PtxToken>,
    directive: &FunctionHeaderDirective,
) {
    match directive {
        FunctionHeaderDirective::Linkage(linkage) => linkage.unparse_tokens(tokens),
        FunctionHeaderDirective::NoReturn => push_directive(tokens, "noreturn"),
        FunctionHeaderDirective::AbiPreserve(value) => {
            push_directive(tokens, "abipreserve");
            push_decimal(tokens, *value);
        }
        FunctionHeaderDirective::AbiPreserveControl(value) => {
            push_directive(tokens, "abipreserve_control");
            push_decimal(tokens, *value);
        }
        FunctionHeaderDirective::MaxClusterRank(value) => {
            push_directive(tokens, "maxclusterrank");
            push_decimal(tokens, *value);
        }
        FunctionHeaderDirective::BlocksAreClusters => push_directive(tokens, "blocksareclusters"),
        FunctionHeaderDirective::ExplicitCluster(dim) => {
            push_directive(tokens, "explicitcluster");
            unparse_function_dim(tokens, dim);
        }
        FunctionHeaderDirective::ReqNctaPerCluster(dim) => {
            push_directive(tokens, "reqnctapercluster");
            unparse_function_dim(tokens, dim);
        }
        FunctionHeaderDirective::MaxNReg(value) => {
            push_directive(tokens, "maxnreg");
            push_decimal(tokens, *value);
        }
        FunctionHeaderDirective::MaxNTid(dim) => {
            push_directive(tokens, "maxntid");
            unparse_function_dim(tokens, dim);
        }
        FunctionHeaderDirective::MinNCtaPerSm(value) => {
            push_directive(tokens, "minnctapersm");
            push_decimal(tokens, *value);
        }
        FunctionHeaderDirective::ReqNTid(dim) => {
            push_directive(tokens, "reqntid");
            unparse_function_dim(tokens, dim);
        }
        FunctionHeaderDirective::MaxNCtaPerSm(value) => {
            push_directive(tokens, "maxnctapersm");
            push_decimal(tokens, *value);
        }
        FunctionHeaderDirective::Pragma(arguments) => {
            push_directive(tokens, "pragma");
            for argument in arguments {
                tokens.push(PtxToken::Identifier(argument.clone()));
            }
        }
    }
}

fn unparse_function_headers(tokens: &mut Vec<PtxToken>, directives: &[FunctionHeaderDirective]) {
    for directive in directives {
        unparse_function_header_directive(tokens, directive);
    }
}

fn unparse_function_body(tokens: &mut Vec<PtxToken>, body: &FunctionBody, prefer_braces: bool) {
    if body.entry_directives.is_empty() && body.statements.is_empty() {
        if prefer_braces {
            tokens.push(PtxToken::LBrace);
            tokens.push(PtxToken::RBrace);
        } else {
            tokens.push(PtxToken::Semicolon);
        }
        return;
    }

    tokens.push(PtxToken::LBrace);
    for directive in &body.entry_directives {
        unparse_entry_directive(tokens, directive);
    }
    for statement in &body.statements {
        unparse_function_statement(tokens, statement);
    }
    tokens.push(PtxToken::RBrace);
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
        unparse_function_headers(tokens, &self.directives);
        push_directive(tokens, "entry");
        push_identifier(tokens, &self.name);
        tokens.push(PtxToken::LParen);
        unparse_param_list(tokens, &self.params);
        tokens.push(PtxToken::RParen);
        unparse_function_body(tokens, &self.body, true);
    }
}

impl PtxUnparser for FuncFunction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        unparse_function_headers(tokens, &self.directives);
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
        unparse_function_body(tokens, &self.body, false);
    }
}

impl PtxUnparser for FunctionKernelDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            FunctionKernelDirective::Entry(entry) => entry.unparse_tokens(tokens),
            FunctionKernelDirective::Func(func) => func.unparse_tokens(tokens),
            FunctionKernelDirective::Alias(alias) => alias.unparse_tokens(tokens),
        }
    }
}
