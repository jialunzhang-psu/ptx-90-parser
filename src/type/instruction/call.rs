use crate::r#type::common::{FunctionSymbol, Immediate, Label, RegisterOperand, VariableSymbol};

/// `call{.uni} ...;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    /// `.uni`
    pub uniform: bool,
    pub kind: CallKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallKind {
    /// `call{.uni} (ret-param), func, (param-list);`
    DirectReturnAndArguments {
        /// `(ret-param)`
        return_parameter: CallReturn,
        /// `func`
        callee: FunctionSymbol,
        /// `(param-list)`
        arguments: Vec<CallArgument>,
    },
    /// `call{.uni} func, (param-list);`
    DirectArguments {
        /// `func`
        callee: FunctionSymbol,
        /// `(param-list)`
        arguments: Vec<CallArgument>,
    },
    /// `call{.uni} func;`
    Direct {
        /// `func`
        callee: FunctionSymbol,
    },
    /// `call{.uni} (ret-param), fptr, (param-list), flist;`
    IndirectTargetsReturnAndArguments {
        /// `(ret-param)`
        return_parameter: CallReturn,
        /// `fptr`
        pointer: RegisterOperand,
        /// `(param-list)`
        arguments: Vec<CallArgument>,
        /// `flist`
        targets: CallTargetList,
    },
    /// `call{.uni} fptr, (param-list), flist;`
    IndirectTargetsArguments {
        /// `fptr`
        pointer: RegisterOperand,
        /// `(param-list)`
        arguments: Vec<CallArgument>,
        /// `flist`
        targets: CallTargetList,
    },
    /// `call{.uni} fptr, flist;`
    IndirectTargets {
        /// `fptr`
        pointer: RegisterOperand,
        /// `flist`
        targets: CallTargetList,
    },
    /// `call{.uni} (ret-param), fptr, (param-list), fproto;`
    IndirectPrototypeReturnAndArguments {
        /// `(ret-param)`
        return_parameter: CallReturn,
        /// `fptr`
        pointer: RegisterOperand,
        /// `(param-list)`
        arguments: Vec<CallArgument>,
        /// `fproto`
        prototype: Label,
    },
    /// `call{.uni} fptr, (param-list), fproto;`
    IndirectPrototypeArguments {
        /// `fptr`
        pointer: RegisterOperand,
        /// `(param-list)`
        arguments: Vec<CallArgument>,
        /// `fproto`
        prototype: Label,
    },
    /// `call{.uni} fptr, fproto;`
    IndirectPrototype {
        /// `fptr`
        pointer: RegisterOperand,
        /// `fproto`
        prototype: Label,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallReturn {
    /// `ret-param`
    Register(RegisterOperand),
    /// `ret-param`
    Param(VariableSymbol),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallArgument {
    /// `reg`
    Register(RegisterOperand),
    /// `imm`
    Immediate(Immediate),
    /// `param`
    Param(VariableSymbol),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallTargetList {
    /// `flist`
    Table(VariableSymbol),
    /// `flist`
    Label(Label),
}
