use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{PredicateRegister, RegisterOperand, VariableSymbol},
        instruction::tex::*,
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

fn pred(name: &str) -> PredicateRegister {
    PredicateRegister(name.into())
}

fn var(name: &str) -> VariableSymbol {
    VariableSymbol(name.into())
}

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<Tex>(source);
}

#[test]
fn parses_vector4_implicit_with_offset_and_compare() {
    assert_eq!(
        parse::<Tex>(
            "tex.2d.v4.f32.f32 {%f0, %f1, %f2, %f3}, [tex_a, {%f4, %f5}], {%r0, %r1}, %f6;",
        ),
        Tex::Vector4ImplicitSampler(Vector4ImplicitSampler {
            geometry: Geometry::TwoD,
            data_type: Vector4DataType::F32,
            coordinate_type: CoordinateType::F32,
            destination: vec4("%f0", "%f1", "%f2", "%f3"),
            predicate: None,
            texture: TextureOperand::Symbol(var("tex_a")),
            coordinates: vec2("%f4", "%f5"),
            offset: Some(Offset::Pair(vec2("%r0", "%r1"))),
            depth_compare: Some(reg("%f6")),
        })
    );
    assert_roundtrip(
        "tex.2d.v4.f32.f32 {%f0, %f1, %f2, %f3}, [tex_a, {%f4, %f5}], {%r0, %r1}, %f6;",
    );
}

#[test]
fn parses_vector4_explicit_sampler() {
    assert_eq!(
        parse::<Tex>("tex.a1d.v4.s32.s32 {%r0, %r1, %r2, %r3}|%p1, [tex_ref, smpl_x, %r4];",),
        Tex::Vector4ExplicitSampler(Vector4ExplicitSampler {
            geometry: Geometry::Array1D,
            data_type: Vector4DataType::S32,
            coordinate_type: CoordinateType::S32,
            destination: vec4("%r0", "%r1", "%r2", "%r3"),
            predicate: Some(pred("%p1")),
            texture: TextureOperand::Symbol(var("tex_ref")),
            sampler: SamplerOperand::Symbol(var("smpl_x")),
            coordinates: reg("%r4"),
            offset: None,
            depth_compare: None,
        })
    );
    assert_roundtrip("tex.a1d.v4.s32.s32 {%r0, %r1, %r2, %r3}|%p1, [tex_ref, smpl_x, %r4];");
}

#[test]
fn parses_vector4_mip_level_with_sampler_and_offset() {
    assert_eq!(
        parse::<Tex>(
            "tex.level.2d.v4.s32.f32 {%r0, %r1, %r2, %r3}, [tex_l, %r4, {%f0, %f1}], %f2, {%r5, %r6};",
        ),
        Tex::Vector4MipLevel(Vector4MipLevel {
            geometry: Geometry::TwoD,
            data_type: Vector4DataType::S32,
            coordinate_type: CoordinateType::F32,
            destination: vec4("%r0", "%r1", "%r2", "%r3"),
            predicate: None,
            texture: TextureOperand::Symbol(var("tex_l")),
            sampler: Some(SamplerOperand::Register(reg("%r4"))),
            coordinates: vec2("%f0", "%f1"),
            level_of_detail: LevelOfDetail::F32(reg("%f2")),
            offset: Some(Offset::Pair(vec2("%r5", "%r6"))),
            depth_compare: None,
        })
    );
    assert_roundtrip(
        "tex.level.2d.v4.s32.f32 {%r0, %r1, %r2, %r3}, [tex_l, %r4, {%f0, %f1}], %f2, {%r5, %r6};",
    );
}

#[test]
fn parses_vector2_mip_gradient() {
    assert_eq!(
        parse::<Tex>("tex.grad.1d.v2.f16x2.f32 {%h0, %h1}, [tex_g, %f2], %f3, %f4;",),
        Tex::Vector2F16x2MipGradient(Vector2F16x2MipGradient {
            geometry: Geometry::OneD,
            coordinate_type: CoordinateType::F32,
            destination: vec2("%h0", "%h1"),
            predicate: None,
            texture: TextureOperand::Symbol(var("tex_g")),
            sampler: None,
            coordinates: reg("%f2"),
            gradients: Gradients {
                dpdx: GradientVector::Scalar(reg("%f3")),
                dpdy: GradientVector::Scalar(reg("%f4")),
            },
            offset: None,
            depth_compare: None,
        })
    );
    assert_roundtrip("tex.grad.1d.v2.f16x2.f32 {%h0, %h1}, [tex_g, %f2], %f3, %f4;");
}

#[test]
fn rejects_unknown_geometry() {
    let error = parse_result::<Tex>("tex.xyz.v4.s32.s32 {%r0, %r1, %r2, %r3}, [tex_a, %r4];")
        .expect_err("geometry modifier should be validated");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_f16x2_modifier() {
    let error = parse_result::<Tex>("tex.1d.v2.s32.s32 {%h0, %h1}, [tex_a, %f0];")
        .expect_err("v2 form requires .f16x2 modifier");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_cube_with_integer_coordinates() {
    let error = parse_result::<Tex>(
        "tex.cube.v4.s32.s32 {%r0, %r1, %r2, %r3}, [tex_c, {%f0, %f1, %f2, %f3}];",
    )
    .expect_err("cube textures require .f32 coordinates");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
