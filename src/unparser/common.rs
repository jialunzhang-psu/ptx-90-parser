use crate::{
    lexer::{PtxToken, tokenize},
    r#type::{
        common::{
            AddressBase, AddressOffset, AddressOperand, AttributeDirective, Axis, CodeLinkage,
            DataLinkage, DataType, FunctionSymbol, GeneralOperand, Immediate, Label, Operand,
            PredicateRegister, RegisterOperand, Sign, SpecialRegister, TexHandler2, TexHandler3,
            TexHandler3Optional, VariableSymbol, VectorOperand,
        },
        function::{DwarfDirective, DwarfDirectiveKind},
        variable::ParamStateSpace,
    },
    unparser::PtxUnparser,
};

fn push_tokenized(tokens: &mut Vec<PtxToken>, text: &str) {
    if text.trim().is_empty() {
        return;
    }
    let lexemes =
        tokenize(text).unwrap_or_else(|_| panic!("failed to tokenize literal {:?}", text));
    tokens.extend(lexemes.into_iter().map(|(token, _)| token));
}

pub(crate) fn push_directive(tokens: &mut Vec<PtxToken>, name: &str) {
    let raw = if name.starts_with('.') {
        name.to_string()
    } else {
        format!(".{}", name)
    };
    push_tokenized(tokens, &raw);
}

pub(crate) fn push_token_from_str(tokens: &mut Vec<PtxToken>, value: &str) {
    push_tokenized(tokens, value);
}

pub(crate) fn push_identifier(tokens: &mut Vec<PtxToken>, name: &str) {
    tokens.push(PtxToken::Identifier(name.to_string()));
}

pub(crate) fn push_register(tokens: &mut Vec<PtxToken>, name: &str) {
    tokens.push(PtxToken::Register(name.to_string()));
}

pub(crate) fn push_decimal<T: ToString>(tokens: &mut Vec<PtxToken>, value: T) {
    tokens.push(PtxToken::DecimalInteger(value.to_string()));
}

fn push_hex_literal(tokens: &mut Vec<PtxToken>, value: u64) {
    tokens.push(PtxToken::HexInteger(format!("0x{:x}", value)));
}

pub(crate) fn push_opcode(tokens: &mut Vec<PtxToken>, opcode: &str) {
    push_identifier(tokens, opcode);
}

fn push_register_with_axis(tokens: &mut Vec<PtxToken>, base: &str, axis: &Axis) {
    push_register(tokens, base);
    match axis {
        Axis::None { .. } => {}
        Axis::X { .. } => push_directive(tokens, "x"),
        Axis::Y { .. } => push_directive(tokens, "y"),
        Axis::Z { .. } => push_directive(tokens, "z"),
    };
}

fn numeric_token(literal: &str) -> PtxToken {
    if literal.starts_with("0f") || literal.starts_with("0F") {
        PtxToken::HexFloatSingle(literal.to_string())
    } else if literal.starts_with("0d") || literal.starts_with("0D") {
        PtxToken::HexFloatDouble(literal.to_string())
    } else if literal.starts_with("0x") || literal.starts_with("0X") {
        PtxToken::HexInteger(literal.to_string())
    } else if literal.starts_with("0b") || literal.starts_with("0B") {
        PtxToken::BinaryInteger(literal.to_string())
    } else if literal.len() > 1
        && literal.starts_with('0')
        && literal.chars().all(|c| c >= '0' && c <= '7')
    {
        PtxToken::OctalInteger(literal.to_string())
    } else if literal.contains('e') || literal.contains('E') {
        PtxToken::FloatExponent(literal.to_string())
    } else if literal.contains('.') {
        PtxToken::Float(literal.to_string())
    } else {
        PtxToken::DecimalInteger(literal.to_string())
    }
}

fn push_numeric(tokens: &mut Vec<PtxToken>, literal: &str) {
    tokens.push(numeric_token(literal));
}

fn push_dwarf_values<I>(tokens: &mut Vec<PtxToken>, iter: I)
where
    I: IntoIterator<Item = u64>,
{
    for (idx, value) in iter.into_iter().enumerate() {
        if idx > 0 {
            tokens.push(PtxToken::Comma);
        }
        push_hex_literal(tokens, value);
    }
}

impl PtxUnparser for DwarfDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_directive(tokens, "dwarf");
        match &self.kind {
            DwarfDirectiveKind::ByteValues(values) => {
                push_directive(tokens, "byte");
                push_dwarf_values(tokens, values.iter().map(|v| u64::from(*v)));
            }
            DwarfDirectiveKind::FourByteValues(values) => {
                push_directive(tokens, "4byte");
                push_dwarf_values(tokens, values.iter().map(|v| u64::from(*v)));
            }
            DwarfDirectiveKind::QuadValues(values) => {
                push_directive(tokens, "quad");
                push_dwarf_values(tokens, values.iter().copied());
            }
            DwarfDirectiveKind::FourByteLabel(label) => {
                push_directive(tokens, "4byte");
                push_identifier(tokens, &label.val);
            }
            DwarfDirectiveKind::QuadLabel(label) => {
                push_directive(tokens, "quad");
                push_identifier(tokens, &label.val);
            }
        }
        tokens.push(PtxToken::Semicolon);
    }
}

impl PtxUnparser for CodeLinkage {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            CodeLinkage::Visible { .. } => push_directive(tokens, "visible"),
            CodeLinkage::Extern { .. } => push_directive(tokens, "extern"),
            CodeLinkage::Weak { .. } => push_directive(tokens, "weak"),
        }
    }
}

impl PtxUnparser for DataLinkage {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            DataLinkage::Visible { .. } => push_directive(tokens, "visible"),
            DataLinkage::Extern { .. } => push_directive(tokens, "extern"),
            DataLinkage::Weak { .. } => push_directive(tokens, "weak"),
            DataLinkage::Common { .. } => push_directive(tokens, "common"),
        }
    }
}

impl PtxUnparser for ParamStateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            ParamStateSpace::Const { .. } => push_directive(tokens, "const"),
            ParamStateSpace::Global { .. } => push_directive(tokens, "global"),
            ParamStateSpace::Local { .. } => push_directive(tokens, "local"),
            ParamStateSpace::Shared { .. } => push_directive(tokens, "shared"),
        }
    }
}

impl PtxUnparser for AttributeDirective {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            AttributeDirective::Unified { uuid1, uuid2, .. } => {
                push_directive(tokens, "unified");
                tokens.push(PtxToken::LParen);
                let first = uuid1.to_string();
                push_numeric(tokens, &first);
                tokens.push(PtxToken::Comma);
                let second = uuid2.to_string();
                push_numeric(tokens, &second);
                tokens.push(PtxToken::RParen);
            }
            AttributeDirective::Managed { .. } => push_directive(tokens, "managed"),
        }
    }
}

impl PtxUnparser for DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let directive = match self {
            DataType::U8 { .. } => "u8",
            DataType::U16 { .. } => "u16",
            DataType::U32 { .. } => "u32",
            DataType::U64 { .. } => "u64",
            DataType::S8 { .. } => "s8",
            DataType::S16 { .. } => "s16",
            DataType::S32 { .. } => "s32",
            DataType::S64 { .. } => "s64",
            DataType::F16 { .. } => "f16",
            DataType::F16x2 { .. } => "f16x2",
            DataType::F32 { .. } => "f32",
            DataType::F64 { .. } => "f64",
            DataType::B8 { .. } => "b8",
            DataType::B16 { .. } => "b16",
            DataType::B32 { .. } => "b32",
            DataType::B64 { .. } => "b64",
            DataType::B128 { .. } => "b128",
            DataType::Pred { .. } => "pred",
            // Texture types (merged from TexType)
            DataType::TexRef { .. } => "texref",
            DataType::SamplerRef { .. } => "samplerref",
            DataType::SurfRef { .. } => "surfref",
        };
        push_directive(tokens, directive);
    }
}

impl PtxUnparser for Sign {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Sign::Negative { .. } => tokens.push(PtxToken::Minus),
            Sign::Positive { .. } => tokens.push(PtxToken::Plus),
        }
    }
}

impl PtxUnparser for Immediate {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let literal = self.value.as_str();
        if let Some(rest) = literal.strip_prefix('-') {
            tokens.push(PtxToken::Minus);
            push_numeric(tokens, rest);
        } else if let Some(rest) = literal.strip_prefix('+') {
            tokens.push(PtxToken::Plus);
            push_numeric(tokens, rest);
        } else {
            push_numeric(tokens, literal);
        }
    }
}

impl PtxUnparser for RegisterOperand {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let mut repr = self.name.clone();
        if let Some(component) = &self.component {
            repr.push('.');
            repr.push_str(component);
        }
        push_register(tokens, &repr);
    }
}

impl PtxUnparser for PredicateRegister {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_register(tokens, &self.name);
    }
}

impl PtxUnparser for Label {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, &self.val);
    }
}

impl PtxUnparser for SpecialRegister {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        let name = match self {
            SpecialRegister::AggrSmemSize { .. } => "%aggr_smem_size".to_string(),
            SpecialRegister::DynamicSmemSize { .. } => "%dynamic_smem_size".to_string(),
            SpecialRegister::LanemaskGt { .. } => "%lanemask_gt".to_string(),
            SpecialRegister::ReservedSmemOffsetBegin { .. } => {
                "%reserved_smem_offset_begin".to_string()
            }
            SpecialRegister::Clock { .. } => "%clock".to_string(),
            SpecialRegister::Envreg { index, .. } => format!("%envreg{}", index),
            SpecialRegister::LanemaskLe { .. } => "%lanemask_le".to_string(),
            SpecialRegister::ReservedSmemOffsetCap { .. } => {
                "%reserved_smem_offset_cap".to_string()
            }
            SpecialRegister::Clock64 { .. } => "%clock64".to_string(),
            SpecialRegister::Globaltimer { .. } => "%globaltimer".to_string(),
            SpecialRegister::LanemaskLt { .. } => "%lanemask_lt".to_string(),
            SpecialRegister::ReservedSmemOffsetEnd { .. } => {
                "%reserved_smem_offset_end".to_string()
            }
            SpecialRegister::ClusterCtaid { axis, .. } => {
                push_register_with_axis(tokens, "%cluster_ctaid", axis);
                return;
            }
            SpecialRegister::GlobaltimerHi { .. } => "%globaltimer_hi".to_string(),
            SpecialRegister::Nclusterid { .. } => "%nclusterid".to_string(),
            SpecialRegister::Smid { .. } => "%smid".to_string(),
            SpecialRegister::ClusterCtarank { axis, .. } => {
                push_register_with_axis(tokens, "%cluster_ctarank", axis);
                return;
            }
            SpecialRegister::GlobaltimerLo { .. } => "%globaltimer_lo".to_string(),
            SpecialRegister::Nctaid { axis, .. } => {
                push_register_with_axis(tokens, "%nctaid", axis);
                return;
            }
            SpecialRegister::Tid { axis, .. } => {
                push_register_with_axis(tokens, "%tid", axis);
                return;
            }
            SpecialRegister::ClusterNctaid { axis, .. } => {
                push_register_with_axis(tokens, "%cluster_nctaid", axis);
                return;
            }
            SpecialRegister::Gridid { .. } => "%gridid".to_string(),
            SpecialRegister::Nsmid { .. } => "%nsmid".to_string(),
            SpecialRegister::TotalSmemSize { .. } => "%total_smem_size".to_string(),
            SpecialRegister::ClusterNctarank { axis, .. } => {
                push_register_with_axis(tokens, "%cluster_nctarank", axis);
                return;
            }
            SpecialRegister::IsExplicitCluster { .. } => "%is_explicit_cluster".to_string(),
            SpecialRegister::Ntid { axis, .. } => {
                push_register_with_axis(tokens, "%ntid", axis);
                return;
            }
            SpecialRegister::Warpid { .. } => "%warpid".to_string(),
            SpecialRegister::Clusterid { .. } => "%clusterid".to_string(),
            SpecialRegister::Laneid { .. } => "%laneid".to_string(),
            SpecialRegister::Nwarpid { .. } => "%nwarpid".to_string(),
            SpecialRegister::WARPSZ { .. } => "%WARPSZ".to_string(),
            SpecialRegister::Ctaid { axis, .. } => {
                push_register_with_axis(tokens, "%ctaid", axis);
                return;
            }
            SpecialRegister::LanemaskEq { .. } => "%lanemask_eq".to_string(),
            SpecialRegister::Pm { index, .. } => format!("%pm{}", index),
            SpecialRegister::Pm64 { index, .. } => format!("%pm{}_64", index),
            SpecialRegister::CurrentGraphExec { .. } => "%current_graph_exec".to_string(),
            SpecialRegister::LanemaskGe { .. } => "%lanemask_ge".to_string(),
            SpecialRegister::ReservedSmemOffset { index, .. } => {
                format!("%reserved_smem_offset_{}", index)
            }
        };
        push_register(tokens, &name);
    }
}

impl PtxUnparser for Operand {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            Operand::Register {
                operand: register, ..
            } => register.unparse_tokens(tokens),
            Operand::Immediate {
                operand: immediate, ..
            } => immediate.unparse_tokens(tokens),
            Operand::Symbol { name: symbol, .. } => push_identifier(tokens, symbol),
            Operand::SymbolOffset { symbol, offset, .. } => {
                push_identifier(tokens, symbol);
                tokens.push(PtxToken::Plus);
                offset.unparse_tokens(tokens);
            }
        }
    }
}

impl PtxUnparser for VectorOperand {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::LBrace);
        match self {
            VectorOperand::Vector1 { operand: item, .. } => item.unparse_tokens(tokens),
            VectorOperand::Vector2 {
                operands: items, ..
            } => {
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    item.unparse_tokens(tokens);
                }
            }
            VectorOperand::Vector3 {
                operands: items, ..
            } => {
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    item.unparse_tokens(tokens);
                }
            }
            VectorOperand::Vector4 {
                operands: items, ..
            } => {
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    item.unparse_tokens(tokens);
                }
            }
            VectorOperand::Vector8 {
                operands: items, ..
            } => {
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        tokens.push(PtxToken::Comma);
                    }
                    item.unparse_tokens(tokens);
                }
            }
        }
        tokens.push(PtxToken::RBrace);
    }
}

impl PtxUnparser for GeneralOperand {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            GeneralOperand::Vec {
                operand: vector, ..
            } => vector.unparse_tokens(tokens),
            GeneralOperand::Single { operand, .. } => operand.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for TexHandler2 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::LBracket);
        for (idx, item) in self.operands.iter().enumerate() {
            if idx > 0 {
                tokens.push(PtxToken::Comma);
            }
            item.unparse_tokens(tokens);
        }
        tokens.push(PtxToken::RBracket);
    }
}

impl PtxUnparser for TexHandler3 {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::LBracket);
        self.handle.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.sampler.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        self.coords.unparse_tokens(tokens);
        tokens.push(PtxToken::RBracket);
    }
}

impl PtxUnparser for TexHandler3Optional {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        tokens.push(PtxToken::LBracket);
        self.handle.unparse_tokens(tokens);
        tokens.push(PtxToken::Comma);
        if let Some(sampler) = &self.sampler {
            sampler.unparse_tokens(tokens);
            tokens.push(PtxToken::Comma);
        }
        self.coords.unparse_tokens(tokens);
        tokens.push(PtxToken::RBracket);
    }
}

impl PtxUnparser for AddressBase {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            AddressBase::Register {
                operand: register, ..
            } => register.unparse_tokens(tokens),
            AddressBase::Variable { symbol, .. } => symbol.unparse_tokens(tokens),
        }
    }
}

impl PtxUnparser for AddressOffset {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            AddressOffset::Register {
                operand: register, ..
            } => {
                tokens.push(PtxToken::Plus);
                register.unparse_tokens(tokens);
            }
            AddressOffset::Immediate {
                sign,
                value: immediate,
                ..
            } => {
                sign.unparse_tokens(tokens);
                immediate.unparse_tokens(tokens);
            }
        }
    }
}

impl PtxUnparser for AddressOperand {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            AddressOperand::Array { base, index, .. } => {
                base.unparse_tokens(tokens);
                tokens.push(PtxToken::LBracket);
                index.unparse_tokens(tokens);
                tokens.push(PtxToken::RBracket);
            }
            AddressOperand::ImmediateAddress { addr, .. } => {
                tokens.push(PtxToken::LBracket);
                addr.unparse_tokens(tokens);
                tokens.push(PtxToken::RBracket);
            }
            AddressOperand::Offset { base, offset, .. } => {
                tokens.push(PtxToken::LBracket);
                base.unparse_tokens(tokens);
                if let Some(offset) = offset {
                    offset.unparse_tokens(tokens);
                }
                tokens.push(PtxToken::RBracket);
            }
        }
    }
}

impl PtxUnparser for FunctionSymbol {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, &self.val);
    }
}

impl PtxUnparser for VariableSymbol {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        push_identifier(tokens, &self.val);
    }
}

impl PtxUnparser for crate::r#type::common::Instruction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        // Emit predicate if present
        if let Some(predicate) = &self.predicate {
            tokens.push(PtxToken::At);
            if predicate.negated {
                tokens.push(PtxToken::Exclaim);
            }
            predicate.operand.unparse_tokens(tokens);
        }

        // Emit the instruction
        self.inst.unparse_tokens(tokens);
    }
}
