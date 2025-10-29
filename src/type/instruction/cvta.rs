use crate::r#type::common::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cvta {
    /// `cvta.space.size  p, a;`
    /// `cvta.space.size  p, var;`
    /// `cvta.space.size  p, var+imm;`
    ToGeneric(ToGeneric),
    /// `cvta.to.space.size  p, a;`
    ToAddressSpace(ToAddressSpace),
}

/// `cvta.space.size  p, {a | var | var+imm};`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToGeneric {
    /// `.space`
    pub space: Space,
    /// `.size`
    pub size: Size,
    /// `p`
    pub destination: RegisterOperand,
    /// `{a | var | var+imm}`
    pub source: GenericSource,
}

/// `cvta.to.space.size  p, a;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToAddressSpace {
    /// `.space`
    pub space: Space,
    /// `.size`
    pub size: Size,
    /// `p`
    pub destination: RegisterOperand,
    /// `a`
    pub source: RegisterOperand,
}

/// `.space = { .const, .global, .local, .shared{::cta, ::cluster}, .param{::entry} };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    /// `.const`
    Const,
    /// `.global`
    Global,
    /// `.local`
    Local,
    /// `.shared`
    Shared,
    /// `.shared::cta`
    SharedCta,
    /// `.shared::cluster`
    SharedCluster,
    /// `.param`
    Param,
    /// `.param::entry`
    ParamEntry,
}

/// `.size  = { .u32, .u64 };`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    /// `.u32`
    U32,
    /// `.u64`
    U64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericSource {
    /// `a`
    Register(RegisterOperand),
    /// `var`
    Variable(VariableSymbol),
    /// `var+imm`
    VariableWithImmediate {
        /// `var`
        variable: VariableSymbol,
        /// `imm`
        immediate: Immediate,
    },
}
