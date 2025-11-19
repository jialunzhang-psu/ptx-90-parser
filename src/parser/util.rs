use crate::{
    err,
    lexer::PtxToken,
    parser::{ParseErrorKind, PtxParseError, PtxTokenStream, Span},
    span,
};

// Note: The c!, ok!, err! macros have been replaced with procedural macros
// defined in crates/cmap-macro. The old m! macro has been superseded by cclosure!.

/// Macro to simplify map with cclosure!.
///
///
/// Examples:
/// mapc!(seq_n!(identifier_p(), plus_p(), Immediate::parse()),  Operand::SymbolOffset { symbol, _, offset=offset.unwrap() })
/// expands to:
/// map(seq_n!(identifier_p(), plus_p(), Immediate::parse()), cclosure!(Operand::SymbolOffset { symbol, _, offset=offset.unwrap() }))
#[macro_export]
macro_rules! mapc {
    ($parser:expr, $ty:path { $($fields:tt)* }) => {{
        $crate::parser::util::map($parser, $crate::cclosure!($ty { $($fields)* }))
    }};
}

/// Similar as mapc!, but with try_map.
#[macro_export]
macro_rules! try_mapc {
    ($parser:expr, $ty:path { $($fields:tt)* }) => {{
        $crate::parser::util::try_map($parser, $crate::okmap!($ty { $($fields)* }))
    }};
}

/* -------------------------------------------------- */
/* -------------- Numeric Helpers ------------------- */
/* -------------------------------------------------- */

pub fn parse_unsigned_integer(
    lit: &str,
    span: Span,
    min: u128,
    max: u128,
) -> Result<u128, PtxParseError> {
    let trimmed = lit.trim_end_matches('U').trim_end_matches('u');
    let (radix, digits) = if let Some(rest) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        (16, rest)
    } else if let Some(rest) = trimmed
        .strip_prefix("0b")
        .or_else(|| trimmed.strip_prefix("0B"))
    {
        (2, rest)
    } else if trimmed.starts_with('0') && trimmed.len() > 1 {
        (8, trimmed.strip_prefix('0').unwrap_or(""))
    } else {
        (10, trimmed)
    };
    let value = u128::from_str_radix(digits, radix).map_err(|_| PtxParseError {
        kind: ParseErrorKind::InvalidLiteral(format!("invalid integer literal: {lit}")),
        span,
    })?;
    if value < min || value > max {
        return Err(PtxParseError {
            kind: ParseErrorKind::InvalidLiteral(format!("integer literal out of range: {lit}")),
            span,
        });
    }
    Ok(value)
}

pub fn parse_signed_integer(
    lit: &str,
    span: Span,
    min: i128,
    max: i128,
) -> Result<i128, PtxParseError> {
    let trimmed = lit.trim_end_matches('U').trim_end_matches('u');
    let (sign, body) = if let Some(rest) = trimmed.strip_prefix('-') {
        (-1i32, rest)
    } else if let Some(rest) = trimmed.strip_prefix('+') {
        (1i32, rest)
    } else {
        (1i32, trimmed)
    };
    let limit = if sign < 0 {
        abs_limit(min)
    } else {
        max as u128
    };
    let magnitude = parse_unsigned_integer(body, span, 0, limit)? as i128;
    let value = if sign < 0 { -magnitude } else { magnitude };
    if value < min || value > max {
        return Err(PtxParseError {
            kind: ParseErrorKind::InvalidLiteral(format!("integer literal out of range: {lit}")),
            span,
        });
    }
    Ok(value)
}

pub fn parse_u32_literal(lit: &str, span: Span) -> Result<u32, PtxParseError> {
    parse_unsigned_integer(lit, span, 0, u32::MAX as u128).map(|v| v as u32)
}

pub fn parse_u64_literal(lit: &str, span: Span) -> Result<u64, PtxParseError> {
    parse_unsigned_integer(lit, span, 0, u64::MAX as u128).map(|v| v as u64)
}

pub fn parse_index_suffix(
    digits: &str,
    max: u8,
    label: &str,
    span: Span,
) -> Result<u8, PtxParseError> {
    if digits.is_empty() {
        return err!(ParseErrorKind::InvalidLiteral(format!(
            "missing index for {label}"
        )));
    }
    let value = digits.parse::<u8>().map_err(|_| PtxParseError {
        kind: ParseErrorKind::InvalidLiteral(format!("invalid index for {label}: {digits}")),
        span,
    })?;
    if value > max {
        return err!(ParseErrorKind::InvalidLiteral(format!(
            "{label} index out of range: {value}"
        )));
    }
    Ok(value)
}

fn abs_limit(min: i128) -> u128 {
    if min == i128::MIN {
        (i128::MAX as u128) + 1
    } else {
        (-min) as u128
    }
}

/* -------------------------------------------------- */
/* -------------- Primitive Parsers ----------------- */
/* -------------------------------------------------- */

/// Parser that expects a specific string token (simple version).
pub fn string_p(
    expected: impl Into<String>,
) -> impl Fn(&mut PtxTokenStream) -> Result<((), Span), PtxParseError> {
    let expected = expected.into();
    move |stream| {
        stream.try_with_span(|stream| {
            stream.with_partial_token_mode(|stream| stream.expect_string(&expected))?;
            Ok(())
        })
    }
}

/// Macro to generate simple token parsers.
macro_rules! define_token_parser {
    ($name:ident, $token:expr, $doc:expr) => {
        #[doc = $doc]
        pub fn $name() -> impl Fn(&mut PtxTokenStream) -> Result<((), Span), PtxParseError> {
            move |stream| {
                stream.try_with_span(|stream| {
                    stream.expect(&$token)?;
                    Ok(())
                })
            }
        }
    };
}

// Generate token parsers - these return parser functions (Layer 1 primitives)
define_token_parser!(
    comma_p,
    PtxToken::Comma,
    "Parser that expects a comma token."
);
define_token_parser!(
    semicolon_p,
    PtxToken::Semicolon,
    "Parser that expects a semicolon token."
);
define_token_parser!(
    colon_p,
    PtxToken::Colon,
    "Parser that expects a colon token."
);
define_token_parser!(plus_p, PtxToken::Plus, "Parser that expects a plus token.");
define_token_parser!(
    minus_p,
    PtxToken::Minus,
    "Parser that expects a minus token."
);
define_token_parser!(
    lparen_p,
    PtxToken::LParen,
    "Parser that expects a left parenthesis token."
);
define_token_parser!(
    rparen_p,
    PtxToken::RParen,
    "Parser that expects a right parenthesis token."
);
define_token_parser!(
    lbracket_p,
    PtxToken::LBracket,
    "Parser that expects a left bracket token."
);
define_token_parser!(
    rbracket_p,
    PtxToken::RBracket,
    "Parser that expects a right bracket token."
);
define_token_parser!(
    lbrace_p,
    PtxToken::LBrace,
    "Parser that expects a left brace token."
);
define_token_parser!(
    rbrace_p,
    PtxToken::RBrace,
    "Parser that expects a right brace token."
);
define_token_parser!(
    langle_p,
    PtxToken::LAngle,
    "Parser that expects a left angle bracket token."
);
define_token_parser!(
    rangle_p,
    PtxToken::RAngle,
    "Parser that expects a right angle bracket token."
);
define_token_parser!(
    equals_p,
    PtxToken::Equals,
    "Parser that expects an equals token."
);
define_token_parser!(at_p, PtxToken::At, "Parser that expects an @ token.");
define_token_parser!(
    exclamation_p,
    PtxToken::Exclaim,
    "Parser that expects an exclamation mark token."
);
define_token_parser!(pipe_p, PtxToken::Pipe, "Parser that expects a pipe token.");

/// Parser that expects an integer literal token and returns the string value.
pub fn integer_p() -> impl Fn(&mut PtxTokenStream) -> Result<(String, Span), PtxParseError> {
    move |stream| {
        stream.try_with_span(|stream| {
            let (token, token_span) = stream.consume()?;

            match token {
                PtxToken::DecimalInteger(s)
                | PtxToken::HexInteger(s)
                | PtxToken::BinaryInteger(s)
                | PtxToken::OctalInteger(s) => Ok(s.clone()),
                _ => Err(crate::unexpected_token!(
                    *token_span,
                    &["integer literal".to_string()],
                    format!("{:?}", token)
                )),
            }
        })
    }
}

/// Parser that expects an identifier token and returns (name, span).
pub fn identifier_p() -> impl Fn(&mut PtxTokenStream) -> Result<(String, Span), PtxParseError> {
    move |stream| {
        stream.try_with_span(|stream| {
            let (name, _) = stream.expect_identifier()?;
            Ok(name)
        })
    }
}

/// Parser that expects a directive token and returns (directive, span).
pub fn directive_p() -> impl Fn(&mut PtxTokenStream) -> Result<(String, Span), PtxParseError> {
    move |stream| {
        stream.try_with_span(|stream| {
            let (dir, _) = stream.expect_directive()?;
            Ok(dir)
        })
    }
}

/// Expect a specific directive name (e.g., `.align`).
pub fn directive_exact_p(
    expected: impl Into<String>,
) -> impl Fn(&mut PtxTokenStream) -> Result<((), Span), PtxParseError> {
    let expected = expected.into();
    move |stream| {
        let (dir, span) = directive_p()(stream)?;
        if dir == expected {
            Ok(((), span))
        } else {
            Err(crate::unexpected_value!(
                span,
                &[&format!(".{expected}")],
                format!(".{dir}")
            ))
        }
    }
}

/// Parser that expects a register token and returns (name, span).
pub fn register_p() -> impl Fn(&mut PtxTokenStream) -> Result<(String, Span), PtxParseError> {
    move |stream| {
        stream.try_with_span(|stream| {
            let (mut name, _) = stream.expect_register()?;
            loop {
                let has_dot = matches!(stream.peek(), Ok((PtxToken::Dot, _)));
                if !has_dot {
                    break;
                }

                // Peek ahead to ensure this is a valid single-character component (e.g., .x, .y)
                let is_component = match stream.peek_n(1) {
                    Ok((PtxToken::Identifier(component_name), _)) => matches!(
                        component_name.as_str(),
                        "x" | "y" | "z" | "w" | "r" | "g" | "b" | "a"
                    ),
                    _ => false,
                };

                if !is_component {
                    break;
                }

                // Consume '.' and the component identifier
                let _ = stream.consume()?;
                let (suffix, _) = stream.expect_identifier()?;
                name.push('.');
                name.push_str(&suffix);
            }
            Ok(name)
        })
    }
}

/// Parser that expects a string literal token and returns its string contents.
pub fn string_literal_p() -> impl Fn(&mut PtxTokenStream) -> Result<(String, Span), PtxParseError> {
    move |stream| {
        stream.try_with_span(|stream| {
            let (token, token_span) = stream.consume()?;
            match token {
                PtxToken::StringLiteral(value) => Ok(value.clone()),
                _ => Err(crate::unexpected_token!(
                    *token_span,
                    &["string literal".to_string()],
                    format!("{:?}", token)
                )),
            }
        })
    }
}

/// Parser that expects any literal token (integer or float) and returns its string form.
pub fn literal_p() -> impl Fn(&mut PtxTokenStream) -> Result<(String, Span), PtxParseError> {
    move |stream| {
        stream.try_with_span(|stream| {
            let (token, token_span) = stream.consume()?;
            match token {
                PtxToken::DecimalInteger(s)
                | PtxToken::HexInteger(s)
                | PtxToken::BinaryInteger(s)
                | PtxToken::OctalInteger(s)
                | PtxToken::Float(s)
                | PtxToken::FloatExponent(s)
                | PtxToken::HexFloatSingle(s)
                | PtxToken::HexFloatDouble(s) => Ok(s.clone()),
                _ => Err(crate::unexpected_token!(
                    *token_span,
                    &["literal".to_string()],
                    format!("{:?}", token)
                )),
            }
        })
    }
}

/* -------------------------------------------------- */
/* --------------- Sequence Combinators ------------- */
/* -------------------------------------------------- */

// Macro to generate seqN functions
macro_rules! define_seq {
    ($name:ident, $count:expr, $($ty:ident),+; $($param:ident),+; $($var:ident),+) => {
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        #[doc = concat!("Sequence ", $count, " parsers, keeping all results.")]
        pub fn $name<$($ty),+, $($param),+>(
            $($param: $param),+
        ) -> impl Fn(&mut PtxTokenStream) -> Result<(($($ty),+), Span), PtxParseError>
        where
            $($param: Fn(&mut PtxTokenStream) -> Result<($ty, Span), PtxParseError>),+
        {
            move |stream| stream.try_with_span(|stream| {
                $(let ($var, _) = $param(stream)?;)+
                Ok(($($var),+))
            })
        }
    };
}

// Generate seq2 through seq64
define_seq!(seq, "2", A, B; parser_a, parser_b; a, b);
define_seq!(seq3, "3", A, B, C; parser_a, parser_b, parser_c; a, b, c);
define_seq!(seq4, "4", A, B, C, D; parser_a, parser_b, parser_c, parser_d; a, b, c, d);
define_seq!(seq5, "5", A, B, C, D, E; parser_a, parser_b, parser_c, parser_d, parser_e; a, b, c, d, e);
define_seq!(seq6, "6", A, B, C, D, E, F; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f; a, b, c, d, e, f);
define_seq!(seq7, "7", A, B, C, D, E, F, G; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g; a, b, c, d, e, f, g);
define_seq!(seq8, "8", A, B, C, D, E, F, G, H; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h; a, b, c, d, e, f, g, h);
define_seq!(seq9, "9", A, B, C, D, E, F, G, H, I; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i; a, b, c, d, e, f, g, h, i);
define_seq!(seq10, "10", A, B, C, D, E, F, G, H, I, J; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j; a, b, c, d, e, f, g, h, i, j);
define_seq!(seq11, "11", A, B, C, D, E, F, G, H, I, J, K; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k; a, b, c, d, e, f, g, h, i, j, k);
define_seq!(seq12, "12", A, B, C, D, E, F, G, H, I, J, K, L; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l; a, b, c, d, e, f, g, h, i, j, k, l);
define_seq!(seq13, "13", A, B, C, D, E, F, G, H, I, J, K, L, M; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m; a, b, c, d, e, f, g, h, i, j, k, l, m);
define_seq!(seq14, "14", A, B, C, D, E, F, G, H, I, J, K, L, M, N; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n; a, b, c, d, e, f, g, h, i, j, k, l, m, n);
define_seq!(seq15, "15", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o);
define_seq!(seq16, "16", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p);
define_seq!(seq17, "17", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q);
define_seq!(seq18, "18", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r);
define_seq!(seq19, "19", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s);
define_seq!(seq20, "20", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t);
define_seq!(seq21, "21", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u);
define_seq!(seq22, "22", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v);
define_seq!(seq23, "23", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w);
define_seq!(seq24, "24", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x);
define_seq!(seq25, "25", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y);
define_seq!(seq26, "26", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z);
define_seq!(seq27, "27", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa);
define_seq!(seq28, "28", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab);
define_seq!(seq29, "29", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac);
define_seq!(seq30, "30", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad);
define_seq!(seq31, "31", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae);
define_seq!(seq32, "32", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af);
define_seq!(seq33, "33", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag);
define_seq!(seq34, "34", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah);
define_seq!(seq35, "35", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai);
define_seq!(seq36, "36", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj);
define_seq!(seq37, "37", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak);
define_seq!(seq38, "38", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al);
define_seq!(seq39, "39", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am);
define_seq!(seq40, "40", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an);
define_seq!(seq41, "41", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao);
define_seq!(seq42, "42", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap);
define_seq!(seq43, "43", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq);
define_seq!(seq44, "44", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar);
define_seq!(seq45, "45", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as);
define_seq!(seq46, "46", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at);
define_seq!(seq47, "47", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au);
define_seq!(seq48, "48", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av);
define_seq!(seq49, "49", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw);
define_seq!(seq50, "50", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax);
define_seq!(seq51, "51", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay);
define_seq!(seq52, "52", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az);
define_seq!(seq53, "53", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ, BA; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az, parser_ba; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az, ba);
define_seq!(seq54, "54", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ, BA, BB; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az, parser_ba, parser_bb; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az, ba, bb);
define_seq!(seq55, "55", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ, BA, BB, BC; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az, parser_ba, parser_bb, parser_bc; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az, ba, bb, bc);
define_seq!(seq56, "56", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ, BA, BB, BC, BD; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az, parser_ba, parser_bb, parser_bc, parser_bd; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az, ba, bb, bc, bd);
define_seq!(seq57, "57", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ, BA, BB, BC, BD, BE; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az, parser_ba, parser_bb, parser_bc, parser_bd, parser_be; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az, ba, bb, bc, bd, be);
define_seq!(seq58, "58", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ, BA, BB, BC, BD, BE, BF; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az, parser_ba, parser_bb, parser_bc, parser_bd, parser_be, parser_bf; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az, ba, bb, bc, bd, be, bf);
define_seq!(seq59, "59", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ, BA, BB, BC, BD, BE, BF, BG; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az, parser_ba, parser_bb, parser_bc, parser_bd, parser_be, parser_bf, parser_bg; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az, ba, bb, bc, bd, be, bf, bg);
define_seq!(seq60, "60", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ, BA, BB, BC, BD, BE, BF, BG, BH; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az, parser_ba, parser_bb, parser_bc, parser_bd, parser_be, parser_bf, parser_bg, parser_bh; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az, ba, bb, bc, bd, be, bf, bg, bh);
define_seq!(seq61, "61", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ, BA, BB, BC, BD, BE, BF, BG, BH, BI; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az, parser_ba, parser_bb, parser_bc, parser_bd, parser_be, parser_bf, parser_bg, parser_bh, parser_bi; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az, ba, bb, bc, bd, be, bf, bg, bh, bi);
define_seq!(seq62, "62", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ, BA, BB, BC, BD, BE, BF, BG, BH, BI, BJ; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az, parser_ba, parser_bb, parser_bc, parser_bd, parser_be, parser_bf, parser_bg, parser_bh, parser_bi, parser_bj; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az, ba, bb, bc, bd, be, bf, bg, bh, bi, bj);
define_seq!(seq63, "63", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ, BA, BB, BC, BD, BE, BF, BG, BH, BI, BJ, BK; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az, parser_ba, parser_bb, parser_bc, parser_bd, parser_be, parser_bf, parser_bg, parser_bh, parser_bi, parser_bj, parser_bk; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az, ba, bb, bc, bd, be, bf, bg, bh, bi, bj, bk);
define_seq!(seq64, "64", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH, AI, AJ, AK, AL, AM, AN, AO, AP, AQ, AR, AS, AT, AU, AV, AW, AX, AY, AZ, BA, BB, BC, BD, BE, BF, BG, BH, BI, BJ, BK, BL; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_aa, parser_ab, parser_ac, parser_ad, parser_ae, parser_af, parser_ag, parser_ah, parser_ai, parser_aj, parser_ak, parser_al, parser_am, parser_an, parser_ao, parser_ap, parser_aq, parser_ar, parser_as, parser_at, parser_au, parser_av, parser_aw, parser_ax, parser_ay, parser_az, parser_ba, parser_bb, parser_bc, parser_bd, parser_be, parser_bf, parser_bg, parser_bh, parser_bi, parser_bj, parser_bk, parser_bl; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al, am, an, ao, ap, aq, ar, r#as, at, au, av, aw, ax, ay, az, ba, bb, bc, bd, be, bf, bg, bh, bi, bj, bk, bl);

/// Compose multiple parsers at once by selecting the appropriate `seqN` combinator.
#[macro_export]
macro_rules! seq_n {
    () => {
        compile_error!("seq_n! requires at least two parsers");
    };
    ($parser_a:expr $(,)?) => {
        compile_error!("seq_n! requires at least two parsers");
    };
    ($parser_0:expr, $parser_1:expr $(,)?) => {
        $crate::parser::util::seq($parser_0, $parser_1)
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr $(,)?) => {
        $crate::parser::util::seq3($parser_0, $parser_1, $parser_2)
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr $(,)?) => {
        $crate::parser::util::seq4($parser_0, $parser_1, $parser_2, $parser_3)
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr $(,)?) => {
        $crate::parser::util::seq5($parser_0, $parser_1, $parser_2, $parser_3, $parser_4)
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr $(,)?) => {
        $crate::parser::util::seq6(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr $(,)?) => {
        $crate::parser::util::seq7(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr $(,)?) => {
        $crate::parser::util::seq8(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr $(,)?) => {
        $crate::parser::util::seq9(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr $(,)?) => {
        $crate::parser::util::seq10(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr $(,)?) => {
        $crate::parser::util::seq11(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr $(,)?) => {
        $crate::parser::util::seq12(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr $(,)?) => {
        $crate::parser::util::seq13(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr $(,)?) => {
        $crate::parser::util::seq14(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr $(,)?) => {
        $crate::parser::util::seq15(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr $(,)?) => {
        $crate::parser::util::seq16(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr $(,)?) => {
        $crate::parser::util::seq17(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr $(,)?) => {
        $crate::parser::util::seq18(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr $(,)?) => {
        $crate::parser::util::seq19(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr $(,)?) => {
        $crate::parser::util::seq20(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr $(,)?) => {
        $crate::parser::util::seq21(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr $(,)?) => {
        $crate::parser::util::seq22(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr $(,)?) => {
        $crate::parser::util::seq23(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr $(,)?) => {
        $crate::parser::util::seq24(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr $(,)?) => {
        $crate::parser::util::seq25(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr $(,)?) => {
        $crate::parser::util::seq26(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr $(,)?) => {
        $crate::parser::util::seq27(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr $(,)?) => {
        $crate::parser::util::seq28(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr $(,)?) => {
        $crate::parser::util::seq29(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr $(,)?) => {
        $crate::parser::util::seq30(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr $(,)?) => {
        $crate::parser::util::seq31(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr $(,)?) => {
        $crate::parser::util::seq32(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr $(,)?) => {
        $crate::parser::util::seq33(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr $(,)?) => {
        $crate::parser::util::seq34(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr $(,)?) => {
        $crate::parser::util::seq35(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr $(,)?) => {
        $crate::parser::util::seq36(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr $(,)?) => {
        $crate::parser::util::seq37(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr $(,)?) => {
        $crate::parser::util::seq38(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr $(,)?) => {
        $crate::parser::util::seq39(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr $(,)?) => {
        $crate::parser::util::seq40(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr $(,)?) => {
        $crate::parser::util::seq41(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr $(,)?) => {
        $crate::parser::util::seq42(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr $(,)?) => {
        $crate::parser::util::seq43(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr $(,)?) => {
        $crate::parser::util::seq44(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr $(,)?) => {
        $crate::parser::util::seq45(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr $(,)?) => {
        $crate::parser::util::seq46(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr $(,)?) => {
        $crate::parser::util::seq47(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr $(,)?) => {
        $crate::parser::util::seq48(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr $(,)?) => {
        $crate::parser::util::seq49(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr $(,)?) => {
        $crate::parser::util::seq50(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr $(,)?) => {
        $crate::parser::util::seq51(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr $(,)?) => {
        $crate::parser::util::seq52(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr, $parser_52:expr $(,)?) => {
        $crate::parser::util::seq53(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51, $parser_52,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr, $parser_52:expr, $parser_53:expr $(,)?) => {
        $crate::parser::util::seq54(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51, $parser_52, $parser_53,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr, $parser_52:expr, $parser_53:expr, $parser_54:expr $(,)?) => {
        $crate::parser::util::seq55(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51, $parser_52, $parser_53, $parser_54,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr, $parser_52:expr, $parser_53:expr, $parser_54:expr, $parser_55:expr $(,)?) => {
        $crate::parser::util::seq56(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51, $parser_52, $parser_53, $parser_54, $parser_55,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr, $parser_52:expr, $parser_53:expr, $parser_54:expr, $parser_55:expr, $parser_56:expr $(,)?) => {
        $crate::parser::util::seq57(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51, $parser_52, $parser_53, $parser_54, $parser_55, $parser_56,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr, $parser_52:expr, $parser_53:expr, $parser_54:expr, $parser_55:expr, $parser_56:expr, $parser_57:expr $(,)?) => {
        $crate::parser::util::seq58(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51, $parser_52, $parser_53, $parser_54, $parser_55, $parser_56,
            $parser_57,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr, $parser_52:expr, $parser_53:expr, $parser_54:expr, $parser_55:expr, $parser_56:expr, $parser_57:expr, $parser_58:expr $(,)?) => {
        $crate::parser::util::seq59(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51, $parser_52, $parser_53, $parser_54, $parser_55, $parser_56,
            $parser_57, $parser_58,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr, $parser_52:expr, $parser_53:expr, $parser_54:expr, $parser_55:expr, $parser_56:expr, $parser_57:expr, $parser_58:expr, $parser_59:expr $(,)?) => {
        $crate::parser::util::seq60(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51, $parser_52, $parser_53, $parser_54, $parser_55, $parser_56,
            $parser_57, $parser_58, $parser_59,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr, $parser_52:expr, $parser_53:expr, $parser_54:expr, $parser_55:expr, $parser_56:expr, $parser_57:expr, $parser_58:expr, $parser_59:expr, $parser_60:expr $(,)?) => {
        $crate::parser::util::seq61(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51, $parser_52, $parser_53, $parser_54, $parser_55, $parser_56,
            $parser_57, $parser_58, $parser_59, $parser_60,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr, $parser_52:expr, $parser_53:expr, $parser_54:expr, $parser_55:expr, $parser_56:expr, $parser_57:expr, $parser_58:expr, $parser_59:expr, $parser_60:expr, $parser_61:expr $(,)?) => {
        $crate::parser::util::seq62(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51, $parser_52, $parser_53, $parser_54, $parser_55, $parser_56,
            $parser_57, $parser_58, $parser_59, $parser_60, $parser_61,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr, $parser_52:expr, $parser_53:expr, $parser_54:expr, $parser_55:expr, $parser_56:expr, $parser_57:expr, $parser_58:expr, $parser_59:expr, $parser_60:expr, $parser_61:expr, $parser_62:expr $(,)?) => {
        $crate::parser::util::seq63(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51, $parser_52, $parser_53, $parser_54, $parser_55, $parser_56,
            $parser_57, $parser_58, $parser_59, $parser_60, $parser_61, $parser_62,
        )
    };
    ($parser_0:expr, $parser_1:expr, $parser_2:expr, $parser_3:expr, $parser_4:expr, $parser_5:expr, $parser_6:expr, $parser_7:expr, $parser_8:expr, $parser_9:expr, $parser_10:expr, $parser_11:expr, $parser_12:expr, $parser_13:expr, $parser_14:expr, $parser_15:expr, $parser_16:expr, $parser_17:expr, $parser_18:expr, $parser_19:expr, $parser_20:expr, $parser_21:expr, $parser_22:expr, $parser_23:expr, $parser_24:expr, $parser_25:expr, $parser_26:expr, $parser_27:expr, $parser_28:expr, $parser_29:expr, $parser_30:expr, $parser_31:expr, $parser_32:expr, $parser_33:expr, $parser_34:expr, $parser_35:expr, $parser_36:expr, $parser_37:expr, $parser_38:expr, $parser_39:expr, $parser_40:expr, $parser_41:expr, $parser_42:expr, $parser_43:expr, $parser_44:expr, $parser_45:expr, $parser_46:expr, $parser_47:expr, $parser_48:expr, $parser_49:expr, $parser_50:expr, $parser_51:expr, $parser_52:expr, $parser_53:expr, $parser_54:expr, $parser_55:expr, $parser_56:expr, $parser_57:expr, $parser_58:expr, $parser_59:expr, $parser_60:expr, $parser_61:expr, $parser_62:expr, $parser_63:expr $(,)?) => {
        $crate::parser::util::seq64(
            $parser_0, $parser_1, $parser_2, $parser_3, $parser_4, $parser_5, $parser_6, $parser_7,
            $parser_8, $parser_9, $parser_10, $parser_11, $parser_12, $parser_13, $parser_14,
            $parser_15, $parser_16, $parser_17, $parser_18, $parser_19, $parser_20, $parser_21,
            $parser_22, $parser_23, $parser_24, $parser_25, $parser_26, $parser_27, $parser_28,
            $parser_29, $parser_30, $parser_31, $parser_32, $parser_33, $parser_34, $parser_35,
            $parser_36, $parser_37, $parser_38, $parser_39, $parser_40, $parser_41, $parser_42,
            $parser_43, $parser_44, $parser_45, $parser_46, $parser_47, $parser_48, $parser_49,
            $parser_50, $parser_51, $parser_52, $parser_53, $parser_54, $parser_55, $parser_56,
            $parser_57, $parser_58, $parser_59, $parser_60, $parser_61, $parser_62, $parser_63,
        )
    };
    ($($parser:expr),+ $(,)?) => {
        compile_error!("seq_n! currently supports up to 64 parsers");
    };
}

/// Sequence two parsers, keeping only the second result.
pub fn skip_first<A, B, PA, PB>(
    parser_a: PA,
    parser_b: PB,
) -> impl Fn(&mut PtxTokenStream) -> Result<(B, Span), PtxParseError>
where
    PA: Fn(&mut PtxTokenStream) -> Result<(A, Span), PtxParseError>,
    PB: Fn(&mut PtxTokenStream) -> Result<(B, Span), PtxParseError>,
{
    move |stream| {
        stream.try_with_span(|stream| {
            let _ = parser_a(stream)?;
            let (b, _) = parser_b(stream)?;
            Ok(b)
        })
    }
}

/// Sequence a parser and consume a trailing semicolon.
pub fn skip_semicolon<T, P>(
    parser: P,
) -> impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
{
    move |stream| {
        let (value, span) = parser(stream)?;
        let _ = semicolon_p()(stream)?;
        Ok((value, span))
    }
}

/// Sequence two parsers, keeping only the second result.
#[allow(dead_code)]
pub fn skip_second<A, B, PA, PB>(
    parser_a: PA,
    parser_b: PB,
) -> impl Fn(&mut PtxTokenStream) -> Result<(A, Span), PtxParseError>
where
    PA: Fn(&mut PtxTokenStream) -> Result<(A, Span), PtxParseError>,
    PB: Fn(&mut PtxTokenStream) -> Result<(B, Span), PtxParseError>,
{
    move |stream| {
        stream.try_with_span(|stream| {
            let (a, _) = parser_a(stream)?;
            let (_, _) = parser_b(stream)?;
            Ok(a)
        })
    }
}

/* -------------------------------------------------- */
/* ----------------- Mapping Combinators ------------ */
/* -------------------------------------------------- */

/// Pure (return) - creates a parser that succeeds without consuming input.
///
/// This is the monadic return/pure operation.
#[allow(dead_code)]
pub fn pure<T>(value: T) -> impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>
where
    T: Clone + 'static,
{
    move |stream| stream.try_with_span(|_| Ok(value.clone()))
}

/// Dispatch combinator - parse a value, then choose a parser based on the result.
///
/// This combinator first runs `parser` to extract a value and span, then calls the
/// dispatch function `f` with that (value, span) tuple to determine which parser to run next.
///
/// This is useful for parsers that need to peek at a token (like a directive or identifier)
/// and then dispatch to different parsers based on the value.
///
/// Example:
/// ```ignore
/// dispatch(
///     directive_p(),
///     |(dir, span)| match dir.as_str() {
///         "visible" => Box::new(pure(CodeLinkage::Visible { span })),
///         "extern" => Box::new(pure(CodeLinkage::Extern { span })),
///         _ => Box::new(|_| Err(error))
///     }
/// )
/// ```
#[allow(dead_code)]
pub fn dispatch<A, B, P, F, Next>(
    parser: P,
    f: F,
) -> impl Fn(&mut PtxTokenStream) -> Result<(B, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(A, Span), PtxParseError>,
    F: Fn(A, Span) -> Next,
    Next: Fn(&mut PtxTokenStream) -> Result<(B, Span), PtxParseError>,
{
    move |stream| {
        stream.try_with_span(|stream| {
            let (value, span) = parser(stream)?;
            let next_parser = f(value, span);
            let (result, _) = next_parser(stream)?;
            Ok(result)
        })
    }
}

/// Map over a parser result and assign the span to the mapped value.
pub fn map<A, B, P, F>(
    parser: P,
    f: F,
) -> impl Fn(&mut PtxTokenStream) -> Result<(B, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(A, Span), PtxParseError>,
    F: Fn(A, Span) -> B,
{
    move |stream| {
        let (value, span) = parser(stream)?;
        Ok((f(value, span), span))
    }
}

/// Map over a parser result with a fallible transformation while assigning spans.
pub fn try_map<A, B, P, F>(
    parser: P,
    f: F,
) -> impl Fn(&mut PtxTokenStream) -> Result<(B, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(A, Span), PtxParseError>,
    F: Fn(A, Span) -> Result<B, PtxParseError>,
{
    move |stream| {
        stream.try_with_span(|stream| {
            let (value, span) = parser(stream)?;
            match f(value, span) {
                Ok(node) => Ok(node),
                Err(mut err) => {
                    err.span = span;
                    Err(err)
                }
            }
        })
    }
}

/* -------------------------------------------------- */
/* ------------------- Choice Combinators ----------- */
/* -------------------------------------------------- */

/// Chain together multiple alternative parsers using the binary `alt` combinator.
#[macro_export]
macro_rules! alt {
    () => {
        compile_error!("alt! requires at least one parser")
    };
    ($parser:expr $(,)?) => {
        $parser
    };
    ($parser_a:expr, $parser_b:expr $(,)?) => {
        $crate::parser::util::alt($parser_a, $parser_b)
    };
    ($parser_a:expr, $parser_b:expr, $($rest:expr),+ $(,)?) => {
        $crate::parser::util::alt($parser_a, $crate::alt!($parser_b, $($rest),+))
    };
}

/// Alternative combinator - try first parser, if it fails try second.
///
/// This is the fundamental choice combinator for building parsers with alternatives.
/// Uses backtracking - the stream is rewound if the first parser fails.
///
/// Example:
/// ```ignore
/// let parser = alt(
///     string_value_p(".s16".to_string(), Type::S16),
///     string_value_p(".s32".to_string(), Type::S32)
/// );
/// ```
pub fn alt<T, P1, P2>(
    parser1: P1,
    parser2: P2,
) -> impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>
where
    P1: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
    P2: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
{
    move |stream| match stream.try_with_span(|stream| parser1(stream)) {
        Ok((result, _)) => Ok(result),
        Err(_) => parser2(stream),
    }
}

/// Choice combinator for multiple alternatives.
///
/// Tries each parser in order until one succeeds. Uses backtracking.
#[allow(dead_code)]
pub fn choice<T>(
    parsers: Vec<impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>>,
) -> impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError> {
    move |stream| {
        let mut last_error = None;
        for parser in &parsers {
            match stream.try_with_span(|stream| parser(stream)) {
                Ok((result, _)) => return Ok(result),
                Err(err) => {
                    last_error = Some(err);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| PtxParseError {
            kind: ParseErrorKind::UnexpectedEof,
            span: span!(0..0),
        }))
    }
}

/// Attempt combinator - run parser and rewind on failure.
#[allow(dead_code)]
pub fn attempt<T, P>(parser: P) -> impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
{
    move |stream| match stream.try_with_span(|stream| parser(stream)) {
        Ok((result, _)) => Ok(result),
        Err(err) => Err(err),
    }
}

/// Optional parser - try to parse, return None if it fails.
pub fn optional<T, P>(
    parser: P,
) -> impl Fn(&mut PtxTokenStream) -> Result<(Option<T>, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
{
    move |stream| match stream.try_with_span(|stream| parser(stream)) {
        Ok(((value, _), span)) => Ok((Some(value), span)),
        Err(_) => Ok((None, stream.current_span())),
    }
}

/* -------------------------------------------------- */
/* -------------------- List Combinators ------------ */
/* -------------------------------------------------- */

/// Parse zero or more occurrences (greedy).
///
/// Always succeeds, returning an empty vector if no matches.
pub fn many<T, P>(
    parser: P,
) -> impl Fn(&mut PtxTokenStream) -> Result<(Vec<T>, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
{
    move |stream| {
        stream.try_with_span(|stream| {
            let mut values = Vec::new();

            loop {
                match stream.try_with_span(|stream| parser(stream)) {
                    Ok(((value, _), _)) => values.push(value),
                    Err(_) => break,
                }
            }

            Ok(values)
        })
    }
}

/// Parse one or more occurrences (greedy).
///
/// Pattern: T+
#[allow(dead_code)]
pub fn many1<T, P>(
    parser: P,
) -> impl Fn(&mut PtxTokenStream) -> Result<(Vec<T>, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
{
    move |stream| {
        stream.try_with_span(|stream| {
            let (first, _) = parser(stream)?;
            let mut values = vec![first];

            loop {
                match stream.try_with_span(|stream| parser(stream)) {
                    Ok(((value, _), _)) => values.push(value),
                    Err(_) => break,
                }
            }

            Ok(values)
        })
    }
}

/* -------------------------------------------------- */
/* ------------- Separated List Combinators --------- */
/* -------------------------------------------------- */

/// Parse separator-separated list allowing trailing separator.
/// Returns empty vector on zero matches.
pub fn sep_by<T, S, P, PS>(
    parser: P,
    separator: PS,
) -> impl Fn(&mut PtxTokenStream) -> Result<(Vec<T>, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
    PS: Fn(&mut PtxTokenStream) -> Result<(S, Span), PtxParseError>,
{
    move |stream| {
        stream.try_with_span(|stream| {
            let mut values = Vec::new();

            match stream.try_with_span(|stream| parser(stream)) {
                Ok(((value, _), _)) => values.push(value),
                Err(_) => return Ok(values),
            }

            loop {
                match stream.try_with_span(|stream| separator(stream)) {
                    Ok((_, _)) => match stream.try_with_span(|stream| parser(stream)) {
                        Ok(((value, _), _)) => values.push(value),
                        Err(_) => break,
                    },
                    Err(_) => break,
                }
            }

            Ok(values)
        })
    }
}

/// Parse separator-separated list requiring at least one element.
pub fn sep_by1<T, S, P, PS>(
    parser: P,
    separator: PS,
) -> impl Fn(&mut PtxTokenStream) -> Result<(Vec<T>, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
    PS: Fn(&mut PtxTokenStream) -> Result<(S, Span), PtxParseError>,
{
    move |stream| {
        stream.try_with_span(|stream| {
            let (first, _) = parser(stream)?;
            let mut values = vec![first];

            loop {
                let mut separator_failed = false;
                match stream.try_with_span(|stream| {
                    separator(stream).map_err(|err| {
                        separator_failed = true;
                        err
                    })?;
                    parser(stream)
                }) {
                    Ok(((value, _), _)) => values.push(value),
                    Err(err) => {
                        if separator_failed {
                            break;
                        } else {
                            return Err(err);
                        }
                    }
                }
            }

            Ok(values)
        })
    }
}

/* -------------------------------------------------- */
/* -------------------- Look Ahead ------------------ */
/* -------------------------------------------------- */

/// Peek - runs a parser without consuming input.
///
/// Saves the stream position, runs the parser, then restores the position.
#[allow(dead_code)]
pub fn peek<T, P>(parser: P) -> impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
{
    move |stream| {
        let saved_pos = stream.position();
        let result = parser(stream);
        stream.set_position(saved_pos);
        result
    }
}

/// Combinator that checks if a predicate matches without consuming.
#[allow(dead_code)]
pub fn check<F>(predicate: F) -> impl Fn(&mut PtxTokenStream) -> Result<bool, PtxParseError>
where
    F: Fn(&PtxToken) -> bool,
{
    move |stream| {
        Ok(stream
            .peek()
            .map(|(token, _)| predicate(&token))
            .unwrap_or(false))
    }
}

/// Combinator that peeks at the next directive (Dot + Identifier pattern).
/// Returns Some((directive_name, span)) if a directive is found, None otherwise.
#[allow(dead_code)]
pub fn peek_directive() -> impl Fn(&mut PtxTokenStream) -> Result<Option<String>, PtxParseError> {
    |stream| {
        let directive = match (stream.peek(), stream.peek_n(1)) {
            (Ok((PtxToken::Dot, _dot_span)), Ok((PtxToken::Identifier(name), _id_span))) => {
                Some(name.clone())
            }
            _ => None,
        };
        Ok(directive)
    }
}

/* -------------------------------------------------- */
/* -------------------- Utility Helpers ------------- */
/* -------------------------------------------------- */

/// Combinator that parses a token satisfying a predicate.
/// Returns the token and its span.
#[allow(dead_code)]
pub fn satisfy<F>(
    predicate: F,
    expected_msg: &'static str,
) -> impl Fn(&mut PtxTokenStream) -> Result<((PtxToken, Span), Span), PtxParseError>
where
    F: Fn(&PtxToken) -> bool + Clone + 'static,
{
    move |stream| {
        let eof_span = stream.current_span();
        stream.try_with_span(|stream| {
            let (token, span) = stream.peek().map_err(|_| PtxParseError {
                kind: ParseErrorKind::UnexpectedEof,
                span: eof_span,
            })?;

            if predicate(&token) {
                stream.consume()?;
                Ok((token.clone(), *span))
            } else {
                Err(crate::unexpected_token!(
                    *span,
                    &[expected_msg.to_string()],
                    format!("{:?}", token)
                ))
            }
        })
    }
}

/// Parse elements enclosed by delimiters.
///
/// Example: between(lparen_p(), rparen_p(), identifier_p())
pub fn between<O, C, T, PO, PC, PT>(
    open: PO,
    close: PC,
    parser: PT,
) -> impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>
where
    PO: Fn(&mut PtxTokenStream) -> Result<(O, Span), PtxParseError>,
    PC: Fn(&mut PtxTokenStream) -> Result<(C, Span), PtxParseError>,
    PT: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
{
    move |stream| {
        stream.try_with_span(|stream| {
            let _ = open(stream)?;
            let (result, _) = parser(stream)?;
            let _ = close(stream)?;
            Ok(result)
        })
    }
}
