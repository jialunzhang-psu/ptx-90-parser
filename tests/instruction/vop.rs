use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::vop::{
            DataMerge, DataType, MergeDestination, Opcode, Operand as VopOperand, Scalar,
            ScalarWithSecondary, SecondaryOpcode, Selection, Vop,
        },
    },
};

#[test]
fn parses_scalar_vop_without_saturate() {
    assert_eq!(
        parse::<Vop>("vadd.u32.u32.u32 %r0, %r1.b0, %r2.h1;"),
        Vop::Scalar(Scalar {
            opcode: Opcode::Vadd,
            dtype: DataType::U32,
            atype: DataType::U32,
            btype: DataType::U32,
            saturate: false,
            d: RegisterOperand::Single("%r0".into()),
            a: VopOperand {
                register: RegisterOperand::Single("%r1".into()),
                selection: Some(Selection::B0),
            },
            b: VopOperand {
                register: RegisterOperand::Single("%r2".into()),
                selection: Some(Selection::H1),
            },
        })
    );
}

#[test]
fn parses_scalar_with_secondary_opcode_and_saturate() {
    assert_eq!(
        parse::<Vop>("vsub.s32.s32.s32.sat.max %r3, %r4, %r5, %r6;"),
        Vop::ScalarWithSecondary(ScalarWithSecondary {
            opcode: Opcode::Vsub,
            dtype: DataType::S32,
            atype: DataType::S32,
            btype: DataType::S32,
            saturate: true,
            op2: SecondaryOpcode::Max,
            d: RegisterOperand::Single("%r3".into()),
            a: VopOperand {
                register: RegisterOperand::Single("%r4".into()),
                selection: None,
            },
            b: VopOperand {
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
        parse::<Vop>("vmax.s32.s32.s32 %r7.h0, %r8, %r9.b3, %r10;"),
        Vop::DataMerge(DataMerge {
            opcode: Opcode::Vmax,
            dtype: DataType::S32,
            atype: DataType::S32,
            btype: DataType::S32,
            saturate: false,
            d: MergeDestination {
                register: RegisterOperand::Single("%r7".into()),
                selection: Selection::H0,
            },
            a: VopOperand {
                register: RegisterOperand::Single("%r8".into()),
                selection: None,
            },
            b: VopOperand {
                register: RegisterOperand::Single("%r9".into()),
                selection: Some(Selection::B3),
            },
            c: RegisterOperand::Single("%r10".into()),
        })
    );
}

#[test]
fn rejects_vop_with_invalid_opcode() {
    let err = parse_result::<Vop>("vfoo.u32.u32.u32 %r0, %r1, %r2;")
        .expect_err("parse should fail with invalid opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_secondary_form_missing_c_operand() {
    let err = parse_result::<Vop>("vadd.u32.u32.u32.add %r0, %r1, %r2;")
        .expect_err("parse should fail when c operand is missing");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_vop_with_invalid_selector() {
    let err = parse_result::<Vop>("vmin.u32.u32.u32 %r0, %r1.b4, %r2;")
        .expect_err("parse should fail with invalid selector");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
