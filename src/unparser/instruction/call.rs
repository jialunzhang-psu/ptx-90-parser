use crate::{lexer::PtxToken, r#type::instruction::call::*, unparser::*};

fn push_optional_uniform(tokens: &mut Vec<PtxToken>, uniform: bool) {
    if uniform {
        tokens.push(PtxToken::Directive("uni".to_string()));
    }
}

fn push_argument_list(tokens: &mut Vec<PtxToken>, arguments: &[CallArgument]) {
    tokens.push(PtxToken::LParen);
    for (index, argument) in arguments.iter().enumerate() {
        if index > 0 {
            tokens.push(PtxToken::Comma);
        }
        argument.unparse_tokens(tokens);
    }
    tokens.push(PtxToken::RParen);
}

impl PtxUnparser for CallReturn {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            CallReturn::Register(register) => register.unparse_tokens(tokens),
            CallReturn::Param(symbol) => symbol.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for CallArgument {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            CallArgument::Register(register) => register.unparse_tokens(tokens),
            CallArgument::Immediate(immediate) => immediate.unparse_tokens(tokens),
            CallArgument::Param(symbol) => symbol.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for CallTargetList {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            CallTargetList::Table(symbol) => symbol.unparse_tokens(tokens),
            CallTargetList::Label(label) => label.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for CallKind {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            CallKind::DirectReturnAndArguments {
                return_parameter,
                callee,
                arguments,
            } => {
                tokens.push(PtxToken::LParen);
                return_parameter.unparse_tokens(tokens);
                tokens.push(PtxToken::RParen);
                tokens.push(PtxToken::Comma);
                callee.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_argument_list(tokens, arguments);
            }
            CallKind::DirectArguments { callee, arguments } => {
                callee.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_argument_list(tokens, arguments);
            }
            CallKind::Direct { callee } => {
                callee.unparse_tokens(tokens);
            }
            CallKind::IndirectTargetsReturnAndArguments {
                return_parameter,
                pointer,
                arguments,
                targets,
            } => {
                tokens.push(PtxToken::LParen);
                return_parameter.unparse_tokens(tokens);
                tokens.push(PtxToken::RParen);
                tokens.push(PtxToken::Comma);
                pointer.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_argument_list(tokens, arguments);
                tokens.push(PtxToken::Comma);
                targets.unparse_tokens(tokens);
            }
            CallKind::IndirectTargetsArguments {
                pointer,
                arguments,
                targets,
            } => {
                pointer.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_argument_list(tokens, arguments);
                tokens.push(PtxToken::Comma);
                targets.unparse_tokens(tokens);
            }
            CallKind::IndirectTargets { pointer, targets } => {
                pointer.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                targets.unparse_tokens(tokens);
            }
            CallKind::IndirectPrototypeReturnAndArguments {
                return_parameter,
                pointer,
                arguments,
                prototype,
            } => {
                tokens.push(PtxToken::LParen);
                return_parameter.unparse_tokens(tokens);
                tokens.push(PtxToken::RParen);
                tokens.push(PtxToken::Comma);
                pointer.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_argument_list(tokens, arguments);
                tokens.push(PtxToken::Comma);
                prototype.unparse_tokens(tokens);
            }
            CallKind::IndirectPrototypeArguments {
                pointer,
                arguments,
                prototype,
            } => {
                pointer.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_argument_list(tokens, arguments);
                tokens.push(PtxToken::Comma);
                prototype.unparse_tokens(tokens);
            }
            CallKind::IndirectPrototype { pointer, prototype } => {
                pointer.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                prototype.unparse_tokens(tokens);
            }
        }
    }
}

impl PtxUnparser for Call {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_opcode(tokens, "call");
        push_optional_uniform(tokens, self.uniform);
        self.kind.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
