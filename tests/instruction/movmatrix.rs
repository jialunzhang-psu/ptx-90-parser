use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::{Operand, RegisterOperand},
    r#type::instruction::movmatrix::{Movmatrix, Shape, Type},
};

#[test]
fn parses_movmatrix_instruction() {
    assert_eq!(
        parse::<Movmatrix>("movmatrix.sync.aligned.m8n8.trans.b16 %a0, %a1;"),
        Movmatrix {
            sync: (),
            aligned: (),
            shape: Shape::M8n8,
            trans: (),
            type_: Type::B16,
            d: Operand::Register(RegisterOperand::Single("%a0".into())),
            a: Operand::Register(RegisterOperand::Single("%a1".into())),
        }
    );
    assert_roundtrip::<Movmatrix>("movmatrix.sync.aligned.m8n8.trans.b16 %a0, %a1;");
}
