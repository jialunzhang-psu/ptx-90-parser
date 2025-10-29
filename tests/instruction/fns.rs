use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::RegisterOperand,
        instruction::fns::{Base, Fns, Mask},
    },
};

#[test]
fn parses_fns_instruction() {
    assert_eq!(
        parse::<Fns>("fns.b32 %r0, .b32 %r1, .s32 %r2, %r3;"),
        Fns {
            destination: RegisterOperand::Single("%r0".into()),
            mask: Mask::B32(RegisterOperand::Single("%r1".into())),
            base: Base::S32(RegisterOperand::Single("%r2".into())),
            offset: RegisterOperand::Single("%r3".into()),
        }
    );
}

#[test]
fn parses_fns_with_u32_operands() {
    assert_eq!(
        parse::<Fns>("fns.b32 %r4, .u32 %r5, .u32 %r6, %r7;"),
        Fns {
            destination: RegisterOperand::Single("%r4".into()),
            mask: Mask::U32(RegisterOperand::Single("%r5".into())),
            base: Base::U32(RegisterOperand::Single("%r6".into())),
            offset: RegisterOperand::Single("%r7".into()),
        }
    );
}

#[test]
fn rejects_non_fns_opcode() {
    let err = parse_result::<Fns>("foo.b32 %r0, .b32 %r1, .b32 %r2, %r3;")
        .expect_err("parsing should fail for unsupported opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_mask_type() {
    let err = parse_result::<Fns>("fns.b32 %r0, .f32 %r1, .b32 %r2, %r3;")
        .expect_err("parsing should fail for unsupported mask type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
