use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{RegisterOperand, VariableSymbol},
        instruction::sured::{
            ByteDataType, Clamp, Coordinate2d, Operator, Reduction, SampleDataType, Sured, Surface,
        },
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

#[test]
fn parses_byte_2d_sured_with_surface_reference() {
    assert_eq!(
        parse::<Sured>("sured.b.add.2d.u32.clamp [surf_tex, {%r4, %r5}], %r6;",),
        Sured::Byte2d(Reduction {
            operator: Operator::Add,
            data_type: ByteDataType::U32,
            clamp: Clamp::Clamp,
            surface: Surface::Reference(VariableSymbol("surf_tex".into())),
            coordinates: Coordinate2d {
                x: reg("%r4"),
                y: reg("%r5"),
            },
            source: reg("%r6"),
        }),
    );
    assert_roundtrip::<Sured>("sured.b.add.2d.u32.clamp [surf_tex, {%r4, %r5}], %r6;");
}

#[test]
fn parses_sample_1d_without_coordinate_braces() {
    assert_eq!(
        parse::<Sured>("sured.p.min.1d.b32.zero [%rd10, %r1], %r2;"),
        Sured::Sample1d(Reduction {
            operator: Operator::Min,
            data_type: SampleDataType::B32,
            clamp: Clamp::Zero,
            surface: Surface::Indirect(reg("%rd10")),
            coordinates: reg("%r1"),
            source: reg("%r2"),
        }),
    );
    assert_roundtrip::<Sured>("sured.p.min.1d.b32.zero [%rd10, %r1], %r2;");
}

#[test]
fn rejects_unknown_operator() {
    let error = parse_result::<Sured>("sured.b.xor.1d.u32.trap [surf_tex, %r1], %r2;")
        .expect_err("operator xor is invalid");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_sample_with_integer_data_type() {
    let error = parse_result::<Sured>("sured.p.add.1d.u32.trap [surf_tex, %r1], %r2;")
        .expect_err(".u32 is not a valid sample data type");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
