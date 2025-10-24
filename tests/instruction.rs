use ptx_parser::{
    parse_instruction_line, AddressBase, AddressDisplacementKind, AddressSign, CallModifier,
    ConditionModifier, ModifierKind, OpcodeKind, Operand, TypeModifier,
};

#[test]
fn parses_call_instruction() {
    let instr =
        parse_instruction_line("call.uni (_), _ZN6Kokkos4Impl12device_abortEPKc, (param0);")
            .expect("call instruction should parse");
    assert!(matches!(instr.opcode.kind, OpcodeKind::Call));
    assert!(matches!(
        instr.opcode.modifiers.as_slice(),
        [ModifierKind::Call(CallModifier::Uni)]
    ));
    assert_eq!(instr.operands.len(), 3);
}

#[test]
fn parses_memory_store() {
    let instr =
        parse_instruction_line("st.param.b64 [param0+0], %rd2;").expect("store should parse");
    assert!(matches!(instr.opcode.kind, OpcodeKind::St));
    assert!(matches!(
        instr.opcode.modifiers.last(),
        Some(ModifierKind::Type(TypeModifier::B64))
    ));

    match &instr.operands[0] {
        Operand::Memory(memory) => {
            assert!(matches!(
                memory.base,
                Some(AddressBase::Symbol(ref name)) if name == "param0"
            ));
            assert_eq!(memory.displacements.len(), 1);
            let disp = &memory.displacements[0];
            assert!(matches!(disp.sign, AddressSign::Positive));
            match &disp.kind {
                AddressDisplacementKind::Immediate(value) => assert_eq!(value, "0"),
                other => panic!("expected immediate displacement, got {:?}", other),
            }
        }
        other => panic!("expected memory operand, got {:?}", other),
    }
}

#[test]
fn recognises_condition_modifier() {
    let instr = parse_instruction_line("setp.ge.s32 %p1, %r1, %r2;")
        .expect("setp instruction should parse");
    assert!(matches!(instr.opcode.kind, OpcodeKind::Setp));
    assert_eq!(instr.opcode.modifiers.len(), 2);
    assert!(matches!(
        instr.opcode.modifiers[0],
        ModifierKind::Condition(ConditionModifier::Ge)
    ));
    assert!(matches!(
        instr.opcode.modifiers[1],
        ModifierKind::Type(TypeModifier::S32)
    ));
}

#[test]
fn parses_param_stores_and_call() {
    for line in [
        "st.param.f64 [mystruct+0], dbl;",
        "st.param.s32 [mystruct+8], x;",
        "call foo, (4, mystruct);",
    ] {
        parse_instruction_line(line).expect("instruction should parse");
    }
}
