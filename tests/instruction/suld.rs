use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{RegisterOperand, VariableSymbol},
        instruction::suld::{
            Array2dCoordinates, CacheOperator, Clamp, DataType, Descriptor, Suld, Surface, Vector,
        },
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn reg_vec4(a: &str, b: &str, c: &str, d: &str) -> RegisterOperand {
    RegisterOperand::Vector4([a.into(), b.into(), c.into(), d.into()])
}

#[test]
fn parses_array2d_suld_with_cache_and_vector() {
    assert_eq!(
        parse::<Suld>(
            "suld.b.a2d.cv.v4.b32.zero {%r0, %r1, %r2, %r3}, [surf_layer, {%r4, %r5, %r6, %r7}];"
        ),
        Suld::Array2D(Descriptor {
            cache_operator: Some(CacheOperator::Cv),
            vector: Vector::V4,
            data_type: DataType::B32,
            clamp: Clamp::Zero,
            destination: reg_vec4("%r0", "%r1", "%r2", "%r3"),
            surface: Surface::Reference(VariableSymbol("surf_layer".into())),
            coordinates: Array2dCoordinates {
                index: reg("%r4"),
                x: reg("%r5"),
                y: reg("%r6"),
                z: reg("%r7"),
            },
        })
    );
}

#[test]
fn rejects_missing_clamp_modifier() {
    let error = parse_result::<Suld>("suld.b.1d.v2.b16 {%r0, %r1}, [surf_tex, %r2];")
        .expect_err("clamp modifier is required");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
