use crate::util::{assert_roundtrip as assert_roundtrip_generic, parse, parse_result};
use ptx_parser::{
    parser::ParseErrorKind,
    r#type::{
        common::{AddressBase, AddressOperand, RegisterOperand},
        instruction::tensormap::*,
    },
};

fn assert_roundtrip(source: &str) {
    assert_roundtrip_generic::<TensormapOpcode>(source);
}

fn reg(name: &str) -> RegisterOperand {
    RegisterOperand::Single(name.into())
}

fn address_from_register(name: &str) -> AddressOperand {
    AddressOperand::Offset(AddressBase::Register(reg(name)), None)
}

#[test]
fn parses_field1_global_address_with_global_space() {
    let source = "tensormap.replace.tile.global_address.global.b1024.b64 [%rd0], %rd1;";
    assert_roundtrip(source);
    assert_eq!(
        parse::<TensormapOpcode>(source),
        TensormapOpcode::Field1(Field1 {
            mode: Mode::Tile,
            state_space: Some(StateSpace::Global),
            object_size: ObjectSize::B1024,
            data_type: DataType::B64,
            address: address_from_register("%rd0"),
            field: Field1Field::GlobalAddress(reg("%rd1")),
        })
    );
}

#[test]
fn parses_field2_global_stride_with_shared_space() {
    let source = "tensormap.replace.tile.global_stride.shared::cta.b1024.b64 [%rd2], 3, %rd4;";
    assert_roundtrip(source);
    assert_eq!(
        parse::<TensormapOpcode>(source),
        TensormapOpcode::Field2(Field2 {
            mode: Mode::Tile,
            state_space: Some(StateSpace::SharedCta),
            object_size: ObjectSize::B1024,
            data_type: DataType::B64,
            address: address_from_register("%rd2"),
            ordinal: 3,
            field: Field2Field::GlobalStride(reg("%rd4")),
        })
    );
}

#[test]
fn parses_field3_interleave_layout() {
    let source = "tensormap.replace.tile.interleave_layout.b1024.b32 [%rd5], 2;";
    assert_roundtrip(source);
    assert_eq!(
        parse::<TensormapOpcode>(source),
        TensormapOpcode::Field3(Field3 {
            mode: Mode::Tile,
            state_space: None,
            object_size: ObjectSize::B1024,
            data_type: DataType::B32,
            address: address_from_register("%rd5"),
            field: Field3Field::InterleaveLayout(InterleaveLayout::Interleave32B),
        })
    );
}

#[test]
fn rejects_field1_with_incorrect_data_type() {
    let err = parse_result::<TensormapOpcode>(
        "tensormap.replace.tile.global_address.b1024.b32 [%rd0], %rd1;",
    )
    .expect_err("global_address must use .b64 data type");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("tensormap.replace.tile.global_address.global.b1024.b64 [%rd0], %rd1;");
}

#[test]
fn rejects_field3_with_invalid_immediate() {
    let err =
        parse_result::<TensormapOpcode>("tensormap.replace.tile.fill_mode.b1024.b32 [%rd0], 2;")
            .expect_err("fill_mode immediate must be 0 or 1");
    assert!(matches!(err.kind, ParseErrorKind::UnexpectedToken { .. }));
    assert_roundtrip("tensormap.replace.tile.fill_mode.b1024.b32 [%rd0], 1;");
}
