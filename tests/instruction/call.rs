use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::*,
        instruction::call::{Call, CallArgument, CallKind, CallReturn, CallTargetList},
    },
};

#[test]
fn parses_direct_call_without_arguments() {
    assert_eq!(
        parse::<Call>("call foo;"),
        Call {
            uniform: false,
            kind: CallKind::Direct {
                callee: FunctionSymbol("foo".into())
            }
        }
    );
}

#[test]
fn parses_uniform_direct_call_with_arguments() {
    assert_eq!(
        parse::<Call>("call.uni bar, (%r1, 0x1, arg);"),
        Call {
            uniform: true,
            kind: CallKind::DirectArguments {
                callee: FunctionSymbol("bar".into()),
                arguments: vec![
                    CallArgument::Register(RegisterOperand::Single("%r1".into())),
                    CallArgument::Immediate(Immediate("0x1".into())),
                    CallArgument::Param(VariableSymbol("arg".into())),
                ],
            }
        }
    );
}

#[test]
fn parses_direct_call_with_return_and_arguments() {
    assert_eq!(
        parse::<Call>("call (%r2), foo, (%r3);"),
        Call {
            uniform: false,
            kind: CallKind::DirectReturnAndArguments {
                return_parameter: CallReturn::Register(RegisterOperand::Single("%r2".into())),
                callee: FunctionSymbol("foo".into()),
                arguments: vec![CallArgument::Register(RegisterOperand::Single(
                    "%r3".into()
                ))],
            }
        }
    );
}

#[test]
fn parses_indirect_call_with_target_table() {
    assert_eq!(
        parse::<Call>("call %rd1, (%r1, param), jmptbl;"),
        Call {
            uniform: false,
            kind: CallKind::IndirectTargetsArguments {
                pointer: RegisterOperand::Single("%rd1".into()),
                arguments: vec![
                    CallArgument::Register(RegisterOperand::Single("%r1".into())),
                    CallArgument::Param(VariableSymbol("param".into())),
                ],
                targets: CallTargetList::Table(VariableSymbol("jmptbl".into())),
            }
        }
    );
}

#[test]
fn parses_uniform_indirect_call_with_prototype_and_return() {
    assert_eq!(
        parse::<Call>("call.uni (%r0), %rd2, (%r3), Fproto;"),
        Call {
            uniform: true,
            kind: CallKind::IndirectPrototypeReturnAndArguments {
                return_parameter: CallReturn::Register(RegisterOperand::Single("%r0".into())),
                pointer: RegisterOperand::Single("%rd2".into()),
                arguments: vec![CallArgument::Register(RegisterOperand::Single(
                    "%r3".into()
                ))],
                prototype: Label("Fproto".into()),
            }
        }
    );
}

#[test]
fn rejects_call_missing_terminating_semicolon() {
    let err = parse_result::<Call>("call foo").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedEof));
}

#[test]
fn rejects_indirect_call_missing_target_operand() {
    let err = parse_result::<Call>("call %rd1, (%r1);").unwrap_err();
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
