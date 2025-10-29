use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{RegisterOperand, VariableSymbol},
        instruction::sust::{
            Array1dCoordinates, Block, CacheOperator, Clamp, ComponentType, Coordinate2d,
            Coordinate3d, Formatted, FormattedComponentType, Surface, Sust, Vector,
        },
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn reg_vec2(a: &str, b: &str) -> RegisterOperand {
    RegisterOperand::Vector2([a.into(), b.into()])
}

fn reg_vec4(a: &str, b: &str, c: &str, d: &str) -> RegisterOperand {
    RegisterOperand::Vector4([a.into(), b.into(), c.into(), d.into()])
}

fn surf(name: &str) -> Surface {
    Surface::Reference(VariableSymbol(name.into()))
}

#[test]
fn parses_block_3d_sust_with_cache_and_vector() {
    assert_eq!(
        parse::<Sust>(
            "sust.b.3d.wb.v4.b32.zero [surf_A, {%r1, %r2, %r3, %r4}], {%r5, %r6, %r7, %r8};"
        ),
        Sust::Block3d(Block {
            cache_operator: Some(CacheOperator::Wb),
            vector: Vector::V4,
            component_type: ComponentType::B32,
            clamp: Clamp::Zero,
            surface: surf("surf_A"),
            coordinates: Coordinate3d {
                x: reg("%r1"),
                y: reg("%r2"),
                z: reg("%r3"),
                w: reg("%r4"),
            },
            value: reg_vec4("%r5", "%r6", "%r7", "%r8"),
        })
    );
}

#[test]
fn parses_formatted_2d_sust() {
    assert_eq!(
        parse::<Sust>("sust.p.2d.v2.b32.clamp [surf_fmt, {%r10, %r11}], {%r12, %r13};"),
        Sust::Formatted2d(Formatted {
            vector: Vector::V2,
            component_type: FormattedComponentType::B32,
            clamp: Clamp::Clamp,
            surface: surf("surf_fmt"),
            coordinates: Coordinate2d {
                x: reg("%r10"),
                y: reg("%r11"),
            },
            value: reg_vec2("%r12", "%r13"),
        })
    );
}

#[test]
fn parses_block_array1d_without_optional_modifiers() {
    assert_eq!(
        parse::<Sust>("sust.b.a1d.b64.trap [surf_arr, {%r0, %r1}], %r2;"),
        Sust::BlockArray1d(Block {
            cache_operator: None,
            vector: Vector::None,
            component_type: ComponentType::B64,
            clamp: Clamp::Trap,
            surface: surf("surf_arr"),
            coordinates: Array1dCoordinates {
                index: reg("%r0"),
                x: reg("%r1"),
            },
            value: reg("%r2"),
        })
    );
}

#[test]
fn rejects_missing_clamp_modifier() {
    let error = parse_result::<Sust>("sust.b.1d.b32 [surf_B, %r1], %r2;")
        .expect_err("clamp modifier is mandatory for sust");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_formatted_component_type() {
    let error = parse_result::<Sust>(
        "sust.p.3d.v4.b16.trap [surf_C, {%r0, %r1, %r2, %r3}], {%r4, %r5, %r6, %r7};",
    )
    .expect_err("formatted sust only supports .b32 component type");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
