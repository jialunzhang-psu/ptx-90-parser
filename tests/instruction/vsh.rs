use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::vsh::{
            DataMerge, DataType, MergeDestination, Mode, Opcode, Scalar, ScalarWithSecondary,
            SecondaryOp, Selection, Source, Vsh,
        },
    },
};

#[test]
fn parses_scalar_vsh_without_saturate() {
    assert_eq!(
        parse::<Vsh>("vshl.u32.s32.u32.clamp %r0, %r1.b0, %r2.h1;"),
        Vsh::Scalar(Scalar {
            opcode: Opcode::Vshl,
            dtype: DataType::U32,
            atype: DataType::S32,
            saturate: false,
            mode: Mode::Clamp,
            destination: RegisterOperand::Single("%r0".into()),
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
fn parses_scalar_with_secondary_and_saturate() {
    assert_eq!(
        parse::<Vsh>("vshr.s32.s32.u32.sat.wrap.add %r3, %r4, %r5, %r6;"),
        Vsh::ScalarWithSecondary(ScalarWithSecondary {
            opcode: Opcode::Vshr,
            dtype: DataType::S32,
            atype: DataType::S32,
            saturate: true,
            mode: Mode::Wrap,
            secondary: SecondaryOp::Add,
            destination: RegisterOperand::Single("%r3".into()),
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
fn parses_data_merge_vsh() {
    assert_eq!(
        parse::<Vsh>("vshl.s32.u32.u32.wrap %r7.h0, %r8, %r9.b3, %r10;"),
        Vsh::DataMerge(DataMerge {
            opcode: Opcode::Vshl,
            dtype: DataType::S32,
            atype: DataType::U32,
            saturate: false,
            mode: Mode::Wrap,
            destination: MergeDestination {
                register: RegisterOperand::Single("%r7".into()),
                selection: Selection::H0,
            },
            a: Source {
                register: RegisterOperand::Single("%r8".into()),
                selection: None,
            },
            b: Source {
                register: RegisterOperand::Single("%r9".into()),
                selection: Some(Selection::B3),
            },
            c: RegisterOperand::Single("%r10".into()),
        })
    );
}

#[test]
fn rejects_vsh_with_invalid_opcode() {
    let err = parse_result::<Vsh>("vfoo.u32.u32.u32.clamp %r0, %r1, %r2;")
        .expect_err("parsing should fail with invalid opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_scalar_missing_c_operand_in_secondary_form() {
    let err = parse_result::<Vsh>("vshr.u32.u32.u32.wrap.add %r0, %r1, %r2;")
        .expect_err("parsing should fail when c operand is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_data_merge_without_destination_selector() {
    let err = parse_result::<Vsh>("vshl.u32.u32.u32.wrap %r0, %r1, %r2, %r3;")
        .expect_err("parsing should fail when destination selector is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
