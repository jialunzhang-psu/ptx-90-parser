use crate::util::{parse, parse_result};
use ptx_parser::parser::ParseErrorKind;
use ptx_parser::r#type::common::*;
use ptx_parser::r#type::instruction::createpolicy::*;

#[test]
fn parses_createpolicy_range_with_secondary() {
    assert_eq!(
        parse::<Createpolicy>(
            "createpolicy.range.global.level::.L2::evict_last.level::.L2::evict_unchanged.b64 %rd0, [%rd1], %r2, 128;"
        ),
        Createpolicy::Range {
            global: true,
            primary_priority: PrimaryPriority::L2EvictLast,
            secondary_priority: Some(SecondaryPriority::L2EvictUnchanged),
            destination: RegisterOperand::Single("%rd0".into()),
            address: AddressOperand::Offset(
                AddressBase::Register(RegisterOperand::Single("%rd1".into())),
                None,
            ),
            primary_size: Operand::Register(RegisterOperand::Single("%r2".into())),
            total_size: Operand::Immediate(Immediate("128".into())),
        }
    );
}

#[test]
fn parses_createpolicy_fractional_with_fraction() {
    assert_eq!(
        parse::<Createpolicy>(
            "createpolicy.fractional.level::.L2::evict_normal.level::.L2::evict_first.b64 %rd3, 0.5;"
        ),
        Createpolicy::Fractional {
            primary_priority: PrimaryPriority::L2EvictNormal,
            secondary_priority: Some(SecondaryPriority::L2EvictFirst),
            destination: RegisterOperand::Single("%rd3".into()),
            fraction: Some(Immediate("0.5".into())),
        }
    );
}

#[test]
fn parses_createpolicy_fractional_without_fraction() {
    assert_eq!(
        parse::<Createpolicy>("createpolicy.fractional.level::.L2::evict_first.b64 %rd4;"),
        Createpolicy::Fractional {
            primary_priority: PrimaryPriority::L2EvictFirst,
            secondary_priority: None,
            destination: RegisterOperand::Single("%rd4".into()),
            fraction: None,
        }
    );
}

#[test]
fn parses_createpolicy_convert() {
    assert_eq!(
        parse::<Createpolicy>("createpolicy.cvt.L2.b64 %rd5, %rd6;"),
        Createpolicy::Convert {
            level: Level::L2,
            destination: RegisterOperand::Single("%rd5".into()),
            access_property: RegisterOperand::Single("%rd6".into()),
        }
    );
}

#[test]
fn rejects_invalid_primary_level() {
    let err = parse_result::<Createpolicy>(
        "createpolicy.range.level::.L1::evict_last.b64 %rd0, [%rd1], %r2, 128;",
    )
    .expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_convert_size() {
    let err = parse_result::<Createpolicy>("createpolicy.cvt.L2.b32 %rd0, %rd1;")
        .expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
