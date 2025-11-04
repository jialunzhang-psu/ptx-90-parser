use crate::util::*;
use ptx_parser::r#type::common::{Operand, RegisterOperand};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::instruction::slct::{Dtype, Slct1, Slct2},
};

#[test]
fn parses_slct_s32_variant() {
    assert_eq!(
        parse::<Slct1>("slct.u32.s32 %r0, %r1, %r2, %r3;"),
        Slct1 {
            dtype: Dtype::U32,
            s32: (),
            d: Operand::Register(RegisterOperand::Single("%r0".into())),
            a: Operand::Register(RegisterOperand::Single("%r1".into())),
            b: Operand::Register(RegisterOperand::Single("%r2".into())),
            c: Operand::Register(RegisterOperand::Single("%r3".into())),
        }
    );
    assert_roundtrip::<Slct1>("slct.u32.s32 %r0, %r1, %r2, %r3;");
}

#[test]
fn parses_slct_f32_variant_with_ftz() {
    assert_eq!(
        parse::<Slct2>("slct.ftz.u64.f32 %rd0, %rd1, %rd2, %f3;"),
        Slct2 {
            ftz: true,
            dtype: Dtype::U64,
            f32: (),
            d: Operand::Register(RegisterOperand::Single("%rd0".into())),
            a: Operand::Register(RegisterOperand::Single("%rd1".into())),
            b: Operand::Register(RegisterOperand::Single("%rd2".into())),
            c: Operand::Register(RegisterOperand::Single("%f3".into())),
        }
    );
    assert_roundtrip::<Slct2>("slct.ftz.u64.f32 %rd0, %rd1, %rd2, %f3;");
}
