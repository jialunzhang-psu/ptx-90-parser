use crate::{lexer::PtxToken, r#type::instruction::clusterlaunchcontrol, unparser::*};

impl PtxUnparser for clusterlaunchcontrol::Space {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            clusterlaunchcontrol::Space::SharedCta => {
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                push_identifier(tokens, "cta");
            }
        }
    }
}

impl PtxUnparser for clusterlaunchcontrol::CompletionMechanism {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            clusterlaunchcontrol::CompletionMechanism::MbarrierCompleteTxBytes => {
                push_directive(tokens, "mbarrier");
                push_double_colon(tokens);
                push_identifier(tokens, "complete_tx");
                push_double_colon(tokens);
                push_identifier(tokens, "bytes");
            }
        }
    }
}

impl PtxUnparser for clusterlaunchcontrol::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            clusterlaunchcontrol::DataType::B128 => push_directive(tokens, "b128"),
        }
    }
}

impl PtxUnparser for clusterlaunchcontrol::Multicast {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            clusterlaunchcontrol::Multicast::ClusterAll => {
                push_directive(tokens, "multicast");
                push_double_colon(tokens);
                push_identifier(tokens, "cluster");
                push_double_colon(tokens);
                push_identifier(tokens, "all");
            }
        }
    }
}

impl PtxUnparser for clusterlaunchcontrol::TryCancel {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, "clusterlaunchcontrol");
        push_directive(tokens, "try_cancel");
        push_directive(tokens, "async");

        if let Some(space) = self.space {
            space.unparse_tokens(tokens);
        }

        self.completion_mechanism.unparse_tokens(tokens);

        if let Some(multicast) = self.multicast {
            multicast.unparse_tokens(tokens);
        }

        self.data_type.unparse_tokens(tokens);
        self.address.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.mbarrier.unparse_tokens(tokens);
        tokens.push(PtxToken::Semicolon);
    }
}
