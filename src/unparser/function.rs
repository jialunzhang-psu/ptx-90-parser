use crate::unparser::common::push_register;
use crate::{
    lexer::PtxToken,
    r#type::{
        common::*,
        function::*,
        variable::{VariableDirective, VariableModifier},
    },
    unparser::*,
};

fn push_array_extents(tokens: &mut Vec<PtxToken>, extents: &[Option<u64>]) {
    for extent in extents {
        tokens.push(PtxToken::LBracket);
        if let Some(value) = extent {
            tokens.push(PtxToken::DecimalInteger(value.to_string()));
        }
        tokens.push(PtxToken::RBracket);
    }
}

fn unparse_variable_modifiers(tokens: &mut Vec<PtxToken>, modifiers: &[VariableModifier]) {
    for modifier in modifiers {
        modifier.unparse_tokens(tokens);
    }
}

fn unparse_variable_attributes(tokens: &mut Vec<PtxToken>, attributes: &[AttributeDirective]) {
    for attribute in attributes {
        attribute.unparse_tokens(tokens);
    }
}

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
        FunctionStatement::Label(name) => {
            push_identifier(tokens, name);
            tokens.push(PtxToken::Colon);
        }
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
    unparse_variable_attributes(tokens, &param.attributes);
    if let Some(space) = &param.address_space {
        space.unparse_tokens(tokens);
    }
    unparse_variable_modifiers(tokens, &param.modifiers);
    if let Some(ty) = &param.ty {
        ty.unparse_tokens(tokens);
    }
    push_identifier(tokens, &param.name);
    push_array_extents(tokens, &param.array);
}

fn unparse_param_list(tokens: &mut Vec<PtxToken>, params: &[VariableDirective]) {
    for (index, param) in params.iter().enumerate() {
        if index > 0 {
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
            push_directive(tokens, "abspreserve");
            tokens.push(PtxToken::DecimalInteger(value.to_string()));
        }
        FunctionHeaderDirective::AbiPreserveControl(value) => {
            push_directive(tokens, "abspreserve_control");
            tokens.push(PtxToken::DecimalInteger(value.to_string()));
        }
        FunctionHeaderDirective::MaxClusterRank(value) => {
            push_directive(tokens, "maxclusterrank");
            tokens.push(PtxToken::DecimalInteger(value.to_string()));
        }
        FunctionHeaderDirective::BlocksAreClusters => push_directive(tokens, "blocksareclusters"),
        FunctionHeaderDirective::ExplicitCluster(dim)
        | FunctionHeaderDirective::ReqNctaPerCluster(dim) => {
            let name = match directive {
                FunctionHeaderDirective::ExplicitCluster(_) => "explicitcluster",
                FunctionHeaderDirective::ReqNctaPerCluster(_) => "reqnctapercluster",
                _ => unreachable!(),
            };
            push_directive(tokens, name);
            unparse_function_dim(tokens, dim);
        }
        FunctionHeaderDirective::MaxNReg(value) => {
            push_directive(tokens, "maxnreg");
            tokens.push(PtxToken::DecimalInteger(value.to_string()));
        }
        FunctionHeaderDirective::MaxNTid(dim) | FunctionHeaderDirective::ReqNTid(dim) => {
            let name = match directive {
                FunctionHeaderDirective::MaxNTid(_) => "maxntid",
                FunctionHeaderDirective::ReqNTid(_) => "reqntid",
                _ => unreachable!(),
            };
            push_directive(tokens, name);
            unparse_function_dim(tokens, dim);
        }
        FunctionHeaderDirective::MinNCtaPerSm(value)
        | FunctionHeaderDirective::MaxNCtaPerSm(value) => {
            let name = match directive {
                FunctionHeaderDirective::MinNCtaPerSm(_) => "minnctapersm",
                FunctionHeaderDirective::MaxNCtaPerSm(_) => "maxnctapersm",
                _ => unreachable!(),
            };
            push_directive(tokens, name);
            tokens.push(PtxToken::DecimalInteger(value.to_string()));
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
