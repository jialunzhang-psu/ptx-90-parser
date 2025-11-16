use ptx_parser::{PtxToken, tokenize};

// ============================================================================
// Basic token tests
// ============================================================================

#[test]
fn test_basic_punctuation() {
    let source = ". , ; : ( ) [ ] { }";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0].0, PtxToken::Dot);
    assert_eq!(tokens[1].0, PtxToken::Comma);
    assert_eq!(tokens[2].0, PtxToken::Semicolon);
    assert_eq!(tokens[3].0, PtxToken::Colon);
    assert_eq!(tokens[4].0, PtxToken::LParen);
    assert_eq!(tokens[5].0, PtxToken::RParen);
    assert_eq!(tokens[6].0, PtxToken::LBracket);
    assert_eq!(tokens[7].0, PtxToken::RBracket);
    assert_eq!(tokens[8].0, PtxToken::LBrace);
    assert_eq!(tokens[9].0, PtxToken::RBrace);
}

#[test]
fn test_operators() {
    let source = "+ - * / < > = % ! | & ^ ~ @";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 14);
    assert_eq!(tokens[0].0, PtxToken::Plus);
    assert_eq!(tokens[1].0, PtxToken::Minus);
    assert_eq!(tokens[2].0, PtxToken::Star);
    assert_eq!(tokens[3].0, PtxToken::Slash);
}

#[test]
fn test_directives() {
    let source = ".version .target .address_size .func .entry";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 10); // Each directive is now 2 tokens: Dot + Identifier
    assert_eq!(tokens[0].0, PtxToken::Dot);
    assert_eq!(tokens[1].0, PtxToken::Identifier("version".to_string()));
    assert_eq!(tokens[2].0, PtxToken::Dot);
    assert_eq!(tokens[3].0, PtxToken::Identifier("target".to_string()));
    assert_eq!(tokens[4].0, PtxToken::Dot);
    assert_eq!(
        tokens[5].0,
        PtxToken::Identifier("address_size".to_string())
    );
    assert_eq!(tokens[6].0, PtxToken::Dot);
    assert_eq!(tokens[7].0, PtxToken::Identifier("func".to_string()));
    assert_eq!(tokens[8].0, PtxToken::Dot);
    assert_eq!(tokens[9].0, PtxToken::Identifier("entry".to_string()));
}

#[test]
fn test_integers() {
    let source = "42 0x2A 0b101010 052";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].0, PtxToken::DecimalInteger("42".to_string()));
    assert_eq!(tokens[1].0, PtxToken::HexInteger("0x2A".to_string()));
    assert_eq!(tokens[2].0, PtxToken::BinaryInteger("0b101010".to_string()));
    assert_eq!(tokens[3].0, PtxToken::OctalInteger("052".to_string()));
}

#[test]
fn test_floats() {
    let source = "3.14 2.5e10 1e-5 0F40490000 0D4009000000000000";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].0, PtxToken::Float("3.14".to_string()));
    assert_eq!(tokens[1].0, PtxToken::FloatExponent("2.5e10".to_string()));
    assert_eq!(tokens[2].0, PtxToken::FloatExponent("1e-5".to_string()));
    assert_eq!(
        tokens[3].0,
        PtxToken::HexFloatSingle("0F40490000".to_string())
    );
    assert_eq!(
        tokens[4].0,
        PtxToken::HexFloatDouble("0D4009000000000000".to_string())
    );
}

#[test]
fn test_registers() {
    let source = "%r0 %r1 %p0 %tid %ntid %f0";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].0, PtxToken::Register("%r0".to_string()));
    assert_eq!(tokens[1].0, PtxToken::Register("%r1".to_string()));
    assert_eq!(tokens[2].0, PtxToken::Register("%p0".to_string()));
    assert_eq!(tokens[3].0, PtxToken::Register("%tid".to_string()));
    assert_eq!(tokens[4].0, PtxToken::Register("%ntid".to_string()));
    assert_eq!(tokens[5].0, PtxToken::Register("%f0".to_string()));
}

#[test]
fn test_identifiers() {
    let source = "kernel_main my_var _temp $special";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].0, PtxToken::Identifier("kernel_main".to_string()));
    assert_eq!(tokens[1].0, PtxToken::Identifier("my_var".to_string()));
    assert_eq!(tokens[2].0, PtxToken::Identifier("_temp".to_string()));
    assert_eq!(tokens[3].0, PtxToken::Identifier("$special".to_string()));
}

#[test]
fn test_string_literals() {
    let source = r#""hello world" "escaped \"quote\"" """#;
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 3);
    assert_eq!(
        tokens[0].0,
        PtxToken::StringLiteral("hello world".to_string())
    );
    assert_eq!(
        tokens[1].0,
        PtxToken::StringLiteral(r#"escaped \"quote\""#.to_string())
    );
    assert_eq!(tokens[2].0, PtxToken::StringLiteral("".to_string()));
}

#[test]
fn test_comments() {
    let source = r#"
        // This is a line comment
        .version 7.0
        /* This is a 
           block comment */
        .target sm_80
    "#;
    let tokens = tokenize(source).unwrap();

    // Should only have identifiers and numbers, comments are skipped
    // Directives are now Dot + Identifier
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "version"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "target"))
    );
}

#[test]
fn test_newlines() {
    let source = "mov.u32 %r0, 1;\nmov.u32 %r1, 2;";
    let tokens = tokenize(source).unwrap();

    assert_eq!(
        tokens
            .iter()
            .filter(|(t, _)| matches!(t, PtxToken::Semicolon))
            .count(),
        2
    );
}

#[test]
fn test_label() {
    let source = "BB_1:\nmov.u32 %r0, 1;";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].0, PtxToken::Identifier("BB_1".to_string()));
    assert_eq!(tokens[1].0, PtxToken::Colon);
}

// ============================================================================
// Simple instruction tests
// ============================================================================

#[test]
fn test_ptx_instruction() {
    let source = "mov.u32 %r0, 42;";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 7); // mov + . + u32 + %r0 + , + 42 + ;
    assert_eq!(tokens[0].0, PtxToken::Identifier("mov".to_string()));
    assert_eq!(tokens[1].0, PtxToken::Dot);
    assert_eq!(tokens[2].0, PtxToken::Identifier("u32".to_string()));
    assert_eq!(tokens[3].0, PtxToken::Register("%r0".to_string()));
    assert_eq!(tokens[4].0, PtxToken::Comma);
    assert_eq!(tokens[5].0, PtxToken::DecimalInteger("42".to_string()));
    assert_eq!(tokens[6].0, PtxToken::Semicolon);
}

#[test]
fn test_ptx_function_declaration() {
    let source = ".visible .entry kernel_name(.param .u64 ptr)";
    let tokens = tokenize(source).unwrap();

    // Directives are now Dot + Identifier
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "visible"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "entry"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "kernel_name"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "param"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "u64"))
    );
}

#[test]
fn test_predicated_instruction() {
    let source = "@%p0 add.s32 %r1, %r2, %r3;";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].0, PtxToken::At);
    assert_eq!(tokens[1].0, PtxToken::Register("%p0".to_string()));
    assert_eq!(tokens[2].0, PtxToken::Identifier("add".to_string()));
}

#[test]
fn test_memory_access() {
    let source = "ld.global.u32 %r0, [%r1+4];";
    let tokens = tokenize(source).unwrap();

    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "ld"))
    );
    assert!(tokens.iter().any(|(t, _)| matches!(t, PtxToken::LBracket)));
    assert!(tokens.iter().any(|(t, _)| matches!(t, PtxToken::RBracket)));
    assert!(tokens.iter().any(|(t, _)| matches!(t, PtxToken::Plus)));
}

#[test]
fn test_vector_register() {
    let source = "{%r0, %r1, %r2, %r3}";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].0, PtxToken::LBrace);
    assert_eq!(tokens[1].0, PtxToken::Register("%r0".to_string()));
    assert_eq!(tokens[2].0, PtxToken::Comma);
    assert_eq!(tokens[3].0, PtxToken::Register("%r1".to_string()));
}

#[test]
fn test_complex_ptx_snippet() {
    let source = r#"
.version 7.0
.target sm_80
.address_size 64

.visible .entry vector_add(
    .param .u64 a,
    .param .u64 b,
    .param .u64 c
)
{
    .reg .u32 %r<4>;
    .reg .u64 %rd<4>;
    
    ld.param.u64 %rd0, [a];
    mov.u32 %r0, %tid.x;
    
    @%p0 add.f32 %f0, %f1, %f2;
}
"#;

    let result = tokenize(source);
    assert!(result.is_ok());
    let tokens = result.unwrap();

    // Verify we have the key tokens (directives are now Dot + Identifier)
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "version"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "target"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "entry"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "reg"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "ld"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Register(s) if s == "%rd0"))
    );
}

// ============================================================================
// Complex program tests
// ============================================================================

#[test]
fn test_complete_ptx_program() {
    let source = r#"
.version 7.0
.target sm_80
.address_size 64

.visible .entry vector_add(
    .param .u64 param_a,
    .param .u64 param_b,
    .param .u64 param_c,
    .param .u32 param_n
)
{
    .reg .pred %p<2>;
    .reg .u32 %r<8>;
    .reg .u64 %rd<8>;
    .reg .f32 %f<4>;

    // Get thread ID
    mov.u32 %r0, %tid.x;
    mov.u32 %r1, %ntid.x;
    mov.u32 %r2, %ctaid.x;
    
    // Calculate global thread ID
    mad.lo.u32 %r3, %r2, %r1, %r0;
    
    // Load parameters
    ld.param.u64 %rd0, [param_a];
    ld.param.u64 %rd1, [param_b];
    ld.param.u64 %rd2, [param_c];
    ld.param.u32 %r4, [param_n];
    
    // Bounds check
    setp.ge.u32 %p0, %r3, %r4;
    @%p0 bra DONE;
    
    // Calculate addresses
    mul.wide.u32 %rd3, %r3, 4;
    add.u64 %rd4, %rd0, %rd3;
    add.u64 %rd5, %rd1, %rd3;
    add.u64 %rd6, %rd2, %rd3;
    
    // Load values
    ld.global.f32 %f0, [%rd4];
    ld.global.f32 %f1, [%rd5];
    
    // Perform addition
    add.f32 %f2, %f0, %f1;
    
    // Store result
    st.global.f32 [%rd6], %f2;

DONE:
    ret;
}
"#;

    let result = tokenize(source);
    assert!(result.is_ok(), "Failed to tokenize PTX program");

    let tokens = result.unwrap();

    // Verify key elements are present (directives are now Dot + Identifier)
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "version"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "target"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "visible"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "entry"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "vector_add"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Register(s) if s == "%tid"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "ret"))
    );
}

#[test]
fn test_all_number_formats() {
    let source = r#"
        42              // decimal
        0x2A            // hexadecimal
        0b101010        // binary
        052             // octal
        3.14            // float
        3.14159e0       // float with exponent
        1.5e-10         // scientific notation
        0F40490000      // hex float (32-bit)
        0D4009000000000000  // hex double (64-bit)
    "#;

    let result = tokenize(source);
    assert!(result.is_ok());

    let tokens = result.unwrap();

    // Check that we have the right token types
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::DecimalInteger(s) if s == "42"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::HexInteger(s) if s == "0x2A"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::BinaryInteger(s) if s == "0b101010"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::OctalInteger(s) if s == "052"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Float(s) if s == "3.14"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::HexFloatSingle(s) if s == "0F40490000"))
    );
}

#[test]
fn test_special_registers() {
    let source = r#"
        %tid.x %tid.y %tid.z
        %ntid.x %ntid.y %ntid.z
        %ctaid.x %ctaid.y %ctaid.z
        %nctaid.x %nctaid.y %nctaid.z
        %laneid %warpid
        %clock %clock64
        %pm0 %pm1 %envreg0
    "#;

    let result = tokenize(source);
    assert!(result.is_ok());

    let tokens = result.unwrap();

    // Verify special registers are recognized
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Register(s) if s == "%tid"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Register(s) if s == "%ntid"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Register(s) if s == "%laneid"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Register(s) if s == "%clock64"))
    );
}

#[test]
fn test_predicated_instructions() {
    let source = r#"
        @%p0 mov.u32 %r0, 1;
        @!%p1 add.s32 %r1, %r2, %r3;
        setp.eq.u32 %p2, %r0, 0;
    "#;

    let result = tokenize(source);
    assert!(result.is_ok());

    let tokens = result.unwrap();

    // Check for @ symbol and predicate registers
    assert!(tokens.iter().any(|(t, _)| matches!(t, PtxToken::At)));
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Register(s) if s == "%p0"))
    );
    assert!(tokens.iter().any(|(t, _)| matches!(t, PtxToken::Exclaim)));
}

#[test]
fn test_complex_addressing() {
    let source = r#"
        ld.global.u32 %r0, [%r1];
        ld.global.u32 %r0, [%r1+4];
        ld.global.u32 %r0, [%r1+%r2];
        ld.global.u32 %r0, [array+4];
        st.shared.u32 [%r1], %r0;
    "#;

    let result = tokenize(source);
    assert!(result.is_ok());
}

#[test]
fn test_vector_operations() {
    let source = r#"
        {%r0, %r1, %r2, %r3}
        mov.v2.u32 {%r0, %r1}, {%r2, %r3};
        ld.v4.u32 {%r0, %r1, %r2, %r3}, [%r4];
    "#;

    let result = tokenize(source);
    assert!(result.is_ok());

    let tokens = result.unwrap();

    // Verify vector syntax elements
    assert!(tokens.iter().any(|(t, _)| matches!(t, PtxToken::LBrace)));
    assert!(tokens.iter().any(|(t, _)| matches!(t, PtxToken::RBrace)));
    assert!(
        tokens
            .iter()
            .filter(|(t, _)| matches!(t, PtxToken::Comma))
            .count()
            >= 3
    );
}

#[test]
fn test_function_calls() {
    let source = r#"
        .func (.param .u32 ret_val) my_function (
            .param .u32 arg1,
            .param .u32 arg2
        );
        
        call (ret_val), my_function, (param1, param2);
    "#;

    let result = tokenize(source);
    assert!(result.is_ok());
}

#[test]
fn test_labels_and_branches() {
    let source = r#"
        BB_1:
            setp.eq.u32 %p0, %r0, 0;
            @%p0 bra BB_2;
            add.u32 %r0, %r0, 1;
            bra BB_1;
        BB_2:
            ret;
    "#;

    let result = tokenize(source);
    assert!(result.is_ok());

    let tokens = result.unwrap();

    // Check for labels and branch instructions
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "BB_1"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "BB_2"))
    );
    assert!(
        tokens
            .iter()
            .any(|(t, _)| matches!(t, PtxToken::Identifier(s) if s == "bra"))
    );
}

#[test]
fn test_texture_operations() {
    let source = r#"
        .global .texref my_texture;
        tex.2d.v4.f32.f32 {%f0, %f1, %f2, %f3}, [my_texture, {%f4, %f5}];
    "#;

    let result = tokenize(source);
    assert!(result.is_ok());
}

#[test]
fn test_atomic_operations() {
    let source = r#"
        atom.global.add.u32 %r0, [%r1], %r2;
        atom.shared.cas.b32 %r0, [%r1], %r2, %r3;
    "#;

    let result = tokenize(source);
    assert!(result.is_ok());
}

#[test]
fn test_barrier_sync() {
    let source = r#"
        bar.sync 0;
        bar.arrive 0, %r0;
        barrier.sync 0;
    "#;

    let result = tokenize(source);
    assert!(result.is_ok());
}

#[test]
fn test_all_directives() {
    let source = r#"
        .version 7.0
        .target sm_80
        .address_size 64
        .visible .entry .func
        .reg .pred .param .local .global .const .shared
        .u8 .u16 .u32 .u64
        .s8 .s16 .s32 .s64
        .f16 .f32 .f64
        .b8 .b16 .b32 .b64
        .weak .extern .common
        .align .file .loc .section
    "#;

    let result = tokenize(source);
    assert!(result.is_ok());

    let tokens = result.unwrap();

    // Just verify we got a bunch of identifiers (directives are now Dot + Identifier)
    let identifier_count = tokens
        .iter()
        .filter(|(t, _)| matches!(t, PtxToken::Identifier(_)))
        .count();
    assert!(identifier_count > 20);
}

#[test]
fn test_error_on_invalid_input() {
    // This should fail to tokenize completely
    let source = "###INVALID###";
    let result = tokenize(source);
    assert!(result.is_err(), "Should fail on invalid token");
}

#[test]
fn test_mixed_content() {
    let source = r#"
        // Header comment
        .version 7.0
        
        /* Multi-line
           comment */
        .target sm_80
        
        .visible .entry my_kernel()
        {
            mov.u32 %r0, 42; // Inline comment
        }
    "#;

    let result = tokenize(source);
    assert!(result.is_ok());
}
