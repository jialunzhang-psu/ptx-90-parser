use crate::util::*;
use ptx_parser::parser::ParseErrorKind;
use ptx_parser::r#type::common::*;
use ptx_parser::r#type::instruction::createpolicy::*;

#[test]
fn parses_createpolicy_range_with_secondary() {
    let source =
        "createpolicy.range.global.L2::evict_last.L2::evict_unchanged.b64 %rd0, [%rd1], %r2, 128;";
    assert_eq!(
        parse::<Createpolicy>(source),
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
    assert_roundtrip::<Createpolicy>(source);
}

#[test]
fn parses_createpolicy_fractional_with_fraction() {
    let source = "createpolicy.fractional.L2::evict_normal.L2::evict_first.b64 %rd3, 0.5;";
    assert_eq!(
        parse::<Createpolicy>(source),
        Createpolicy::Fractional {
            primary_priority: PrimaryPriority::L2EvictNormal,
            secondary_priority: Some(SecondaryPriority::L2EvictFirst),
            destination: RegisterOperand::Single("%rd3".into()),
            fraction: Some(Immediate("0.5".into())),
        }
    );
    assert_roundtrip::<Createpolicy>(source);
}

#[test]
fn parses_createpolicy_fractional_without_fraction() {
    let source = "createpolicy.fractional.L2::evict_first.b64 %rd4;";
    assert_eq!(
        parse::<Createpolicy>(source),
        Createpolicy::Fractional {
            primary_priority: PrimaryPriority::L2EvictFirst,
            secondary_priority: None,
            destination: RegisterOperand::Single("%rd4".into()),
            fraction: None,
        }
    );
    assert_roundtrip::<Createpolicy>(source);
}

#[test]
fn parses_createpolicy_convert() {
    let source = "createpolicy.cvt.L2.b64 %rd5, %rd6;";
    assert_eq!(
        parse::<Createpolicy>(source),
        Createpolicy::Convert {
            level: Level::L2,
            destination: RegisterOperand::Single("%rd5".into()),
            access_property: RegisterOperand::Single("%rd6".into()),
        }
    );
    assert_roundtrip::<Createpolicy>(source);
}

#[test]
fn rejects_invalid_primary_level() {
    let err = parse_result::<Createpolicy>(
        "createpolicy.range.L1::evict_last.b64 %rd0, [%rd1], %r2, 128;",
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

#[test]
fn rejects_range_with_level_prefix() {
    let err = parse_result::<Createpolicy>(
        "createpolicy.range.global.level::.L2::evict_last.b64 %rd0, [%rd1], %r2, 128;",
    )
    .expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_fractional_with_level_prefix() {
    let err = parse_result::<Createpolicy>(
        "createpolicy.fractional.level::.L2::evict_last.b64 %rd3, 0.25;",
    )
    .expect_err("parse should fail");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
