use crate::util::{parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{PredicateRegister, RegisterOperand, VariableSymbol},
        instruction::tld4::*,
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn vec2(a: &str, b: &str) -> RegisterOperand {
    RegisterOperand::Vector2([a.into(), b.into()])
}

fn vec4(a: &str, b: &str, c: &str, d: &str) -> RegisterOperand {
    RegisterOperand::Vector4([a.into(), b.into(), c.into(), d.into()])
}

fn predicate(name: &str) -> PredicateRegister {
    PredicateRegister(name.into())
}

#[test]
fn parses_implicit_sampler_with_offset_and_compare() {
    assert_eq!(
        parse::<Tld4>(
            "tld4.r.2d.v4.dtype.f32 {%f0, %f1, %f2, %f3}|%p1, [tex_a, {%f4, %f5}], {%r0, %r1}, %f6;",
        ),
        Tld4::Implicit(ImplicitSampler {
            component: Component::R,
            geometry: Geometry::TwoD {
                coordinates: vec2("%f4", "%f5"),
                offset: Some(vec2("%r0", "%r1")),
            },
            data_type: DataType::F32,
            destination: Destination {
                vector: vec4("%f0", "%f1", "%f2", "%f3"),
                predicate: Some(predicate("%p1")),
            },
            texture: TextureOperand::Symbol(VariableSymbol("tex_a".into())),
            depth_compare: Some(reg("%f6")),
        })
    );
}

#[test]
fn parses_explicit_sampler_without_optional_operands() {
    assert_eq!(
        parse::<Tld4>(
            "tld4.b.a2d.v4.dtype.s32 {%r0, %r1, %r2, %r3}, [%rd0, %r4, {%f0, %f1, %f2, %f3}];",
        ),
        Tld4::Explicit(ExplicitSampler {
            component: Component::B,
            geometry: Geometry::Array2D {
                coordinates: vec4("%f0", "%f1", "%f2", "%f3"),
                offset: None,
            },
            data_type: DataType::S32,
            destination: Destination {
                vector: vec4("%r0", "%r1", "%r2", "%r3"),
                predicate: None,
            },
            texture: TextureOperand::Register(reg("%rd0")),
            sampler: SamplerOperand::Register(reg("%r4")),
            depth_compare: None,
        })
    );
}

#[test]
fn parses_without_dtype_modifier() {
    assert_eq!(
        parse::<Tld4>("tld4.g.2d.v4.s32 {%f0, %f1, %f2, %f3}, [tex_b, {%f4, %f5}];"),
        Tld4::Implicit(ImplicitSampler {
            component: Component::G,
            geometry: Geometry::TwoD {
                coordinates: vec2("%f4", "%f5"),
                offset: None,
            },
            data_type: DataType::S32,
            destination: Destination {
                vector: vec4("%f0", "%f1", "%f2", "%f3"),
                predicate: None,
            },
            texture: TextureOperand::Symbol(VariableSymbol("tex_b".into())),
            depth_compare: None,
        })
    );
}

#[test]
fn rejects_unknown_component_modifier() {
    let error =
        parse_result::<Tld4>("tld4.x.2d.v4.dtype.f32 {%f0, %f1, %f2, %f3}, [tex_a, {%f4, %f5}];")
            .expect_err("component modifier should be validated");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_unknown_data_type_modifier() {
    let error =
        parse_result::<Tld4>("tld4.r.2d.v4.dtype.f64 {%f0, %f1, %f2, %f3}, [tex_a, {%f4, %f5}];")
            .expect_err("only 32-bit data types are supported");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
