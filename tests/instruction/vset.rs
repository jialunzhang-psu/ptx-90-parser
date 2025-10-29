use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::vset::{
            Compare, DataMerge, DataType, Scalar, ScalarWithSecondary, SecondaryOperation,
            Selection, Source, Vset,
        },
    },
};

#[test]
fn parses_scalar_form_with_selectors_on_sources() {
    assert_eq!(
        parse::<Vset>("vset.u32.s32.lt %r0, %r1.b0, %r2.h1;"),
        Vset::Scalar(Scalar {
            atype: DataType::U32,
            btype: DataType::S32,
            cmp: Compare::Lt,
            d: RegisterOperand::Single("%r0".into()),
            a: Source {
                register: RegisterOperand::Single("%r1".into()),
                selection: Some(Selection::B0),
            },
            b: Source {
                register: RegisterOperand::Single("%r2".into()),
                selection: Some(Selection::H1),
            },
        })
    );
}

#[test]
fn parses_scalar_with_secondary_operation() {
    assert_eq!(
        parse::<Vset>("vset.s32.u32.ge.max %r3, %r4, %r5, %r6;"),
        Vset::ScalarWithSecondary(ScalarWithSecondary {
            atype: DataType::S32,
            btype: DataType::U32,
            cmp: Compare::Ge,
            op2: SecondaryOperation::Max,
            d: RegisterOperand::Single("%r3".into()),
            a: Source {
                register: RegisterOperand::Single("%r4".into()),
                selection: None,
            },
            b: Source {
                register: RegisterOperand::Single("%r5".into()),
                selection: None,
            },
            c: RegisterOperand::Single("%r6".into()),
        })
    );
}

#[test]
fn parses_data_merge_form() {
    assert_eq!(
        parse::<Vset>("vset.u32.u32.eq %r7.b3, %r8, %r9.h0, %r10;"),
        Vset::DataMerge(DataMerge {
            atype: DataType::U32,
            btype: DataType::U32,
            cmp: Compare::Eq,
            d: RegisterOperand::Single("%r7".into()),
            dsel: Selection::B3,
            a: Source {
                register: RegisterOperand::Single("%r8".into()),
                selection: None,
            },
            b: Source {
                register: RegisterOperand::Single("%r9".into()),
                selection: Some(Selection::H0),
            },
            c: RegisterOperand::Single("%r10".into()),
        })
    );
}

#[test]
fn rejects_unknown_comparison_modifier() {
    let err = parse_result::<Vset>("vset.u32.u32.foo %r0, %r1, %r2;")
        .expect_err("parsing should fail for unknown comparison modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_scalar_form_with_destination_selector() {
    let err = parse_result::<Vset>("vset.u32.u32.ne %r1.b0, %r2, %r3;")
        .expect_err("scalar form should not allow destination selector");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
