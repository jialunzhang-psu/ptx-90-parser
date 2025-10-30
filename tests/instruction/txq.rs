use crate::util::*;
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{RegisterOperand, VariableSymbol},
        instruction::txq::{Operand, SamplerQuery, TextureLevelQuery, TextureQuery, Txq},
    },
};

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn var(name: &str) -> VariableSymbol {
    VariableSymbol(name.into())
}

#[test]
fn parses_texture_query_with_symbol_operand() {
    assert_eq!(
        parse::<Txq>("txq.width.b32 %r1, [tex_A];"),
        Txq::Texture {
            query: TextureQuery::Width,
            destination: reg("%r1"),
            address: Operand::Texture(var("tex_A")),
        }
    );
    assert_roundtrip::<Txq>("txq.width.b32 %r1, [tex_A];");
}

#[test]
fn parses_texture_query_with_register_operand() {
    assert_eq!(
        parse::<Txq>("txq.num_mipmap_levels.b32 %r2, [%rd3];"),
        Txq::Texture {
            query: TextureQuery::NumMipmapLevels,
            destination: reg("%r2"),
            address: Operand::Register(reg("%rd3")),
        }
    );
    assert_roundtrip::<Txq>("txq.num_mipmap_levels.b32 %r2, [%rd3];");
}

#[test]
fn parses_texture_level_query() {
    assert_eq!(
        parse::<Txq>("txq.level.height.b32 %r4, [tex_B], %r5;"),
        Txq::TextureLevel {
            query: TextureLevelQuery::Height,
            destination: reg("%r4"),
            address: Operand::Texture(var("tex_B")),
            lod: reg("%r5"),
        }
    );
    assert_roundtrip::<Txq>("txq.level.height.b32 %r4, [tex_B], %r5;");
}

#[test]
fn parses_sampler_query() {
    assert_eq!(
        parse::<Txq>("txq.filter_mode.b32 %r6, [smpl_C];"),
        Txq::Sampler {
            query: SamplerQuery::FilterMode,
            destination: reg("%r6"),
            address: Operand::Sampler(var("smpl_C")),
        }
    );
    assert_roundtrip::<Txq>("txq.filter_mode.b32 %r6, [smpl_C];");
}

#[test]
fn rejects_unknown_query_modifier() {
    let error =
        parse_result::<Txq>("txq.unknown.b32 %r1, [tex_A];").expect_err("modifier must be valid");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_invalid_data_type() {
    let error =
        parse_result::<Txq>("txq.width.b64 %r1, [tex_A];").expect_err("only .b32 is supported");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}

#[test]
fn rejects_missing_level_lod_operand() {
    let error = parse_result::<Txq>("txq.level.width.b32 %r1, [tex_A];")
        .expect_err("lod operand is required");
    assert!(matches!(error.kind, ParseErrorKind::UnexpectedToken { .. }));
}
