use crate::util::*;
use ptx_parser::parser::ParseErrorKind;
use ptx_parser::r#type::common::*;
use ptx_parser::r#type::instruction::createpolicy::Createpolicy3;

#[test]
fn parses_createpolicy_convert() {
    let source = "createpolicy.cvt.L2.b64 %rd5, %rd6;";
    assert_eq!(
        parse::<Createpolicy3>(source),
        Createpolicy3 {
            cvt: (),
            l2: (),
            b64: (),
            cache_policy: Operand::Register(RegisterOperand::Single("%rd5".into())),
            access_property: Operand::Register(RegisterOperand::Single("%rd6".into())),
        }
    );
    assert_roundtrip::<Createpolicy3>(source);
}

#[test]
fn parses_createpolicy_convert_different_regs() {
    let source = "createpolicy.cvt.L2.b64 %rd0, %rd1;";
    assert_eq!(
        parse::<Createpolicy3>(source),
        Createpolicy3 {
            cvt: (),
            l2: (),
            b64: (),
            cache_policy: Operand::Register(RegisterOperand::Single("%rd0".into())),
            access_property: Operand::Register(RegisterOperand::Single("%rd1".into())),
        }
    );
    assert_roundtrip::<Createpolicy3>(source);
}
