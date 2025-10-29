use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::common::RegisterOperand,
    r#type::instruction::movmatrix::{DataType, Movmatrix, Shape},
};

#[test]
fn parses_movmatrix_instruction() {
    assert_eq!(
        parse::<Movmatrix>("movmatrix.sync.aligned.m8n8.trans.b16 %a0, %a1;"),
        Movmatrix {
            shape: Shape::M8N8,
            data_type: DataType::B16,
            destination: RegisterOperand::Single("%a0".into()),
            source: RegisterOperand::Single("%a1".into()),
        }
    );
}

#[test]
fn rejects_invalid_opcode() {
    let err = parse_result::<Movmatrix>("mov.sync.aligned.m8n8.trans.b16 %a0, %a1;")
        .expect_err("parse should fail for non-movmatrix opcode");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_trans_modifier() {
    let err = parse_result::<Movmatrix>("movmatrix.sync.aligned.m8n8.b16 %a0, %a1;")
        .expect_err("parse should fail without .trans modifier");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
}
