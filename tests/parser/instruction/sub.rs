use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::sub::{DataType, Sub},
    },
};

#[test]
fn parses_sub_with_unsigned_type() {
    assert_eq!(
        parse::<Sub>("sub.u32 %r0, %r1, %r2;"),
        Sub {
            data_type: DataType::U32,
            destination: RegisterOperand::Single("%r0".into()),
            a: RegisterOperand::Single("%r1".into()),
            b: RegisterOperand::Single("%r2".into()),
        }
    );
}

#[test]
fn parses_sub_with_saturate() {
    assert_eq!(
        parse::<Sub>("sub.sat.s32 %r3, %r4, %r5;"),
        Sub {
            data_type: DataType::S32 { saturate: true },
            destination: RegisterOperand::Single("%r3".into()),
            a: RegisterOperand::Single("%r4".into()),
            b: RegisterOperand::Single("%r5".into()),
        }
    );
}

#[test]
fn rejects_sub_with_saturate_on_unsigned_type() {
    let err = parse_result::<Sub>("sub.sat.u16 %r0, %r1, %r2;")
        .expect_err("parse should fail when .sat is used with unsigned type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_sub_with_invalid_type() {
    let err = parse_result::<Sub>("sub.f32 %f0, %f1, %f2;")
        .expect_err("parse should fail for unsupported data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_sub_with_immediate_operand() {
    let err = parse_result::<Sub>("sub.u32 %r0, %r1, 1;")
        .expect_err("parse should fail when third operand is immediate");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_sub_missing_semicolon() {
    let err = parse_result::<Sub>("sub.s32 %r0, %r1, %r2")
        .expect_err("parse should fail without trailing semicolon");
    assert!(matches!(
        err.kind,
        ParseErrorKind::UnexpectedToken { .. } | ParseErrorKind::UnexpectedEof
    ));
}
