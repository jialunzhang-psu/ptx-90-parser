use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, VariableSymbol},
        instruction::clusterlaunchcontrol::{
            CompletionMechanism, DataType, Multicast, Space, TryCancel,
        },
    },
};

#[test]
fn parses_try_cancel_without_optional_modifiers() {
    assert_roundtrip::<TryCancel>(
        "clusterlaunchcontrol.try_cancel.async.mbarrier::complete_tx::bytes.b128 [addr], [mbar];",
    );
    assert_eq!(
        parse::<TryCancel>(
            "clusterlaunchcontrol.try_cancel.async.mbarrier::complete_tx::bytes.b128 [addr], [mbar];",
        ),
        TryCancel {
            space: None,
            completion_mechanism: CompletionMechanism::MbarrierCompleteTxBytes,
            multicast: None,
            data_type: DataType::B128,
            address: AddressOperand::Offset(
                AddressBase::Variable(VariableSymbol("addr".into())),
                None
            ),
            mbarrier: AddressOperand::Offset(
                AddressBase::Variable(VariableSymbol("mbar".into())),
                None
            ),
        }
    );
}

#[test]
fn parses_try_cancel_with_space_and_multicast() {
    assert_roundtrip::<TryCancel>(
        "clusterlaunchcontrol.try_cancel.async.shared::cta.mbarrier::complete_tx::bytes.multicast::cluster::all.b128 [addr], [mbar];",
    );
    assert_eq!(
        parse::<TryCancel>(
            "clusterlaunchcontrol.try_cancel.async.shared::cta.mbarrier::complete_tx::bytes.multicast::cluster::all.b128 [addr], [mbar];"
        ),
        TryCancel {
            space: Some(Space::SharedCta),
            completion_mechanism: CompletionMechanism::MbarrierCompleteTxBytes,
            multicast: Some(Multicast::ClusterAll),
            data_type: DataType::B128,
            address: AddressOperand::Offset(
                AddressBase::Variable(VariableSymbol("addr".into())),
                None
            ),
            mbarrier: AddressOperand::Offset(
                AddressBase::Variable(VariableSymbol("mbar".into())),
                None
            ),
        }
    );
}

#[test]
fn rejects_missing_async_modifier() {
    let err = parse_result::<TryCancel>(
        "clusterlaunchcontrol.try_cancel.mbarrier::complete_tx::bytes.b128 [addr], [mbar];",
    )
    .expect_err("parse should fail when .async is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_completion_mechanism() {
    let err = parse_result::<TryCancel>(
        "clusterlaunchcontrol.try_cancel.async.shared::cta.invalid::complete_tx::bytes.b128 [addr], [mbar];",
    )
    .expect_err("parse should fail for unsupported completion mechanism");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_semicolon() {
    let err = parse_result::<TryCancel>(
        "clusterlaunchcontrol.try_cancel.async.mbarrier::complete_tx::bytes.b128 [addr], [mbar]",
    )
    .expect_err("parse should fail without trailing semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
