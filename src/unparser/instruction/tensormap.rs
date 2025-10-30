use crate::{
    lexer::PtxToken,
    r#type::{common::*, instruction::tensormap},
    unparser::*,
};

impl PtxUnparser for tensormap::Mode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            tensormap::Mode::Tile => push_directive(tokens, "tile"),
        }
    }
}

impl PtxUnparser for tensormap::StateSpace {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            tensormap::StateSpace::Global => push_directive(tokens, "global"),
            tensormap::StateSpace::SharedCta => {
                push_directive(tokens, "shared");
                push_double_colon(tokens);
                push_identifier(tokens, "cta");
            }
        }
    }
}

impl PtxUnparser for tensormap::ObjectSize {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            tensormap::ObjectSize::B1024 => push_directive(tokens, "b1024"),
        }
    }
}

impl PtxUnparser for tensormap::DataType {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            tensormap::DataType::B32 => push_directive(tokens, "b32"),
            tensormap::DataType::B64 => push_directive(tokens, "b64"),
        }
    }
}

fn element_type_to_immediate(value: tensormap::ElementType) -> u32 {
    match value {
        tensormap::ElementType::U8 => 0,
        tensormap::ElementType::U16 => 1,
        tensormap::ElementType::U32 => 2,
        tensormap::ElementType::S32 => 3,
        tensormap::ElementType::U64 => 4,
        tensormap::ElementType::S64 => 5,
        tensormap::ElementType::F16 => 6,
        tensormap::ElementType::F32 => 7,
        tensormap::ElementType::F64 => 8,
        tensormap::ElementType::Bf16 => 9,
        tensormap::ElementType::F32Ftz => 10,
        tensormap::ElementType::Tf32 => 11,
        tensormap::ElementType::Tf32Ftz => 12,
        tensormap::ElementType::B4x16 => 13,
        tensormap::ElementType::B4x16P64 => 14,
        tensormap::ElementType::B6x16P32 => 15,
    }
}

fn interleave_layout_to_immediate(value: tensormap::InterleaveLayout) -> u32 {
    match value {
        tensormap::InterleaveLayout::None => 0,
        tensormap::InterleaveLayout::Interleave16B => 1,
        tensormap::InterleaveLayout::Interleave32B => 2,
    }
}

fn swizzle_mode_to_immediate(value: tensormap::SwizzleMode) -> u32 {
    match value {
        tensormap::SwizzleMode::None => 0,
        tensormap::SwizzleMode::Swizzle32B => 1,
        tensormap::SwizzleMode::Swizzle64B => 2,
        tensormap::SwizzleMode::Swizzle128B => 3,
        tensormap::SwizzleMode::Swizzle96B => 4,
    }
}

fn swizzle_atomicity_to_immediate(value: tensormap::SwizzleAtomicity) -> u32 {
    match value {
        tensormap::SwizzleAtomicity::Bytes16 => 0,
        tensormap::SwizzleAtomicity::Bytes32 => 1,
        tensormap::SwizzleAtomicity::Bytes32With8ByteFlip => 2,
        tensormap::SwizzleAtomicity::Bytes64 => 3,
    }
}

fn fill_mode_to_immediate(value: tensormap::FillMode) -> u32 {
    match value {
        tensormap::FillMode::Zero => 0,
        tensormap::FillMode::OobNan => 1,
    }
}

fn push_field_prefix(tokens: &mut Vec<PtxToken>, field: &str) {
    push_directive(tokens, field);
}

fn unparse_state_space(tokens: &mut Vec<PtxToken>, state_space: &Option<tensormap::StateSpace>) {
    if let Some(space) = state_space {
        space.unparse_tokens(tokens);
    }
}

fn unparse_common_prefix(
    tokens: &mut Vec<PtxToken>,
    mode: &tensormap::Mode,
    field_directive: &str,
    state_space: &Option<tensormap::StateSpace>,
    object_size: &tensormap::ObjectSize,
    data_type: &tensormap::DataType,
) {
    push_identifier(tokens, "tensormap");
    push_directive(tokens, "replace");
    mode.unparse_tokens(tokens);
    push_field_prefix(tokens, field_directive);
    unparse_state_space(tokens, state_space);
    object_size.unparse_tokens(tokens);
    data_type.unparse_tokens(tokens);
}

impl PtxUnparser for tensormap::TensormapOpcode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            tensormap::TensormapOpcode::Field1(field) => {
                let (field_directive, register): (&str, &RegisterOperand) = match &field.field {
                    tensormap::Field1Field::GlobalAddress(reg) => ("global_address", reg),
                    tensormap::Field1Field::Rank(reg) => ("rank", reg),
                };
                unparse_common_prefix(
                    tokens,
                    &field.mode,
                    field_directive,
                    &field.state_space,
                    &field.object_size,
                    &field.data_type,
                );
                field.address.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                register.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            tensormap::TensormapOpcode::Field2(field) => {
                let (field_directive, register): (&str, &RegisterOperand) = match &field.field {
                    tensormap::Field2Field::BoxDim(reg) => ("box_dim", reg),
                    tensormap::Field2Field::GlobalDim(reg) => ("global_dim", reg),
                    tensormap::Field2Field::GlobalStride(reg) => ("global_stride", reg),
                    tensormap::Field2Field::ElementStride(reg) => ("element_stride", reg),
                };
                unparse_common_prefix(
                    tokens,
                    &field.mode,
                    field_directive,
                    &field.state_space,
                    &field.object_size,
                    &field.data_type,
                );
                field.address.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_decimal(tokens, field.ordinal);
                tokens.push(PtxToken::Comma);
                register.unparse_tokens(tokens);
                tokens.push(PtxToken::Semicolon);
            }
            tensormap::TensormapOpcode::Field3(field) => {
                let (field_directive, immediate) = match field.field {
                    tensormap::Field3Field::ElemType(value) => {
                        ("elemtype", element_type_to_immediate(value))
                    }
                    tensormap::Field3Field::InterleaveLayout(value) => {
                        ("interleave_layout", interleave_layout_to_immediate(value))
                    }
                    tensormap::Field3Field::SwizzleMode(value) => {
                        ("swizzle_mode", swizzle_mode_to_immediate(value))
                    }
                    tensormap::Field3Field::SwizzleAtomicity(value) => {
                        ("swizzle_atomicity", swizzle_atomicity_to_immediate(value))
                    }
                    tensormap::Field3Field::FillMode(value) => {
                        ("fill_mode", fill_mode_to_immediate(value))
                    }
                };
                unparse_common_prefix(
                    tokens,
                    &field.mode,
                    field_directive,
                    &field.state_space,
                    &field.object_size,
                    &field.data_type,
                );
                field.address.unparse_tokens(tokens);
                tokens.push(PtxToken::Comma);
                push_decimal(tokens, immediate);
                tokens.push(PtxToken::Semicolon);
            }
        }
    }
}
