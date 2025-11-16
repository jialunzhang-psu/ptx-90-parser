use crate::{
    lexer::PtxToken,
    parser::{PtxParseError, PtxParser, PtxTokenStream, Span},
};

/* -------------------------------------------------- */
/* -------------- Primitive Parsers ----------------- */
/* -------------------------------------------------- */

/// Parser that expects a specific string token and returns a value.
///
/// This is a primitive parser builder that creates a parser for matching string tokens.
pub fn string<T>(
    expected: String,
    value: T,
) -> impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>
where
    T: Clone + 'static,
{
    move |stream| {
        let start_pos = stream.position().index;
        stream.expect_string(&expected)?;
        let end_pos = stream.position().index;
        Ok((value.clone(), start_pos..end_pos))
    }
}

/// Macro to generate simple token parsers.
macro_rules! define_token_parser {
    ($name:ident, $token:expr, $doc:expr) => {
        #[doc = $doc]
        pub fn $name() -> impl Fn(&mut PtxTokenStream) -> Result<((), Span), PtxParseError> {
            move |stream| {
                let start_pos = stream.position().index;
                stream.expect(&$token)?;
                let end_pos = stream.position().index;
                Ok(((), start_pos..end_pos))
            }
        }
    };
}

// Generate token parsers
define_token_parser!(comma_p, PtxToken::Comma, "Parser that expects a comma token.");
define_token_parser!(semicolon_p, PtxToken::Semicolon, "Parser that expects a semicolon token.");
define_token_parser!(colon_p, PtxToken::Colon, "Parser that expects a colon token.");
define_token_parser!(plus_p, PtxToken::Plus, "Parser that expects a plus token.");
define_token_parser!(minus_p, PtxToken::Minus, "Parser that expects a minus token.");
define_token_parser!(lparen_p, PtxToken::LParen, "Parser that expects a left parenthesis token.");
define_token_parser!(rparen_p, PtxToken::RParen, "Parser that expects a right parenthesis token.");
define_token_parser!(lbracket_p, PtxToken::LBracket, "Parser that expects a left bracket token.");
define_token_parser!(rbracket_p, PtxToken::RBracket, "Parser that expects a right bracket token.");
define_token_parser!(lbrace_p, PtxToken::LBrace, "Parser that expects a left brace token.");
define_token_parser!(rbrace_p, PtxToken::RBrace, "Parser that expects a right brace token.");

/// Parser that expects an identifier token and returns (name, span).
pub fn identifier_p() -> impl Fn(&mut PtxTokenStream) -> Result<((String, Span), Span), PtxParseError> {
    |stream| {
        let start_pos = stream.position().index;
        let (name, span) = stream.expect_identifier()?;
        let end_pos = stream.position().index;
        Ok(((name, span), start_pos..end_pos))
    }
}

/// Parser that expects a directive token and returns (directive, span).
pub fn directive_p() -> impl Fn(&mut PtxTokenStream) -> Result<((String, Span), Span), PtxParseError> {
    |stream| {
        let start_pos = stream.position().index;
        let (dir, span) = stream.expect_directive()?;
        let end_pos = stream.position().index;
        Ok(((dir, span), start_pos..end_pos))
    }
}

/// Parser that expects a register token and returns (name, span).
pub fn register_p() -> impl Fn(&mut PtxTokenStream) -> Result<((String, Span), Span), PtxParseError> {
    |stream| {
        let start_pos = stream.position().index;
        let (name, span) = stream.expect_register()?;
        let end_pos = stream.position().index;
        Ok(((name, span), start_pos..end_pos))
    }
}

/// Parser that expects a register token with optional vector components (.x, .y, .z, .w, etc).
///
/// Examples: %r0, %r1.x, %rd2.y
pub fn register_with_components_p() -> impl Fn(&mut PtxTokenStream) -> Result<((String, Span), Span), PtxParseError> {
    |stream| {
        let start_pos = stream.position().index;
        let (mut name, mut span) = stream.expect_register()?;

        // Parse optional vector components
        loop {
            let next = match stream.peek() {
                Ok((token, _)) => token,
                Err(_) => break,
            };

            if let PtxToken::Dot = next {
                // Peek ahead to see if this is a valid register component
                if let Some((PtxToken::Identifier(component_name), _)) =
                    stream.tokens.get(stream.index + 1)
                {
                    // Only consume valid single-character register components
                    if matches!(
                        component_name.as_str(),
                        "x" | "y" | "z" | "w" | "r" | "g" | "b" | "a"
                    ) {
                        stream.consume()?; // consume dot
                        let (component, component_span) = stream.expect_identifier()?;
                        name.push('.');
                        name.push_str(&component);
                        span.end = component_span.end;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let end_pos = stream.position().index;
        Ok(((name, span), start_pos..end_pos))
    }
}

/// Optional parser - try to parse, return None if it fails.
pub fn optional<T, P>(
    parser: P,
) -> impl Fn(&mut PtxTokenStream) -> Result<(Option<T>, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>,
{
    move |stream| {
        let start_pos = stream.position().index;
        let saved = stream.position();
        match parser(stream) {
            Ok((value, _)) => {
                let end_pos = stream.position().index;
                Ok((Some(value), start_pos..end_pos))
            }
            Err(_) => {
                stream.set_position(saved);
                Ok((None, start_pos..start_pos))
            }
        }
    }
}

/* -------------------------------------------------- */
/* --------------- Sequence Combinators ------------- */
/* -------------------------------------------------- */

// Macro to generate seqN functions
macro_rules! define_seq {
    ($name:ident, $count:expr, $($ty:ident),+; $($param:ident),+; $($var:ident),+) => {
        #[doc = concat!("Sequence ", $count, " parsers, keeping all results.")]
        pub fn $name<$($ty),+, $($param),+>(
            $($param: $param),+
        ) -> impl Fn(&mut PtxTokenStream) -> Result<(($($ty),+), Span), PtxParseError>
        where
            $($param: Fn(&mut PtxTokenStream) -> Result<($ty, Span), PtxParseError>),+
        {
            move |stream| {
                let start_pos = stream.position().index;
                $(let ($var, _) = $param(stream)?;)+
                let end_pos = stream.position().index;
                Ok((($($var),+), start_pos..end_pos))
            }
        }
    };
}

// Generate seq2 through seq20
define_seq!(seq, "two", A, B; parser_a, parser_b; a, b);
define_seq!(seq3, "three", A, B, C; parser_a, parser_b, parser_c; a, b, c);
define_seq!(seq4, "four", A, B, C, D; parser_a, parser_b, parser_c, parser_d; a, b, c, d);
define_seq!(seq5, "five", A, B, C, D, E; parser_a, parser_b, parser_c, parser_d, parser_e; a, b, c, d, e);
define_seq!(seq6, "six", A, B, C, D, E, F; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f; a, b, c, d, e, f);
define_seq!(seq7, "seven", A, B, C, D, E, F, G; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g; a, b, c, d, e, f, g);
define_seq!(seq8, "eight", A, B, C, D, E, F, G, H; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h; a, b, c, d, e, f, g, h);
define_seq!(seq9, "nine", A, B, C, D, E, F, G, H, I; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i; a, b, c, d, e, f, g, h, i);
define_seq!(seq10, "ten", A, B, C, D, E, F, G, H, I, J; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j; a, b, c, d, e, f, g, h, i, j);
define_seq!(seq11, "eleven", A, B, C, D, E, F, G, H, I, J, K; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k; a, b, c, d, e, f, g, h, i, j, k);
define_seq!(seq12, "twelve", A, B, C, D, E, F, G, H, I, J, K, L; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l; a, b, c, d, e, f, g, h, i, j, k, l);
define_seq!(seq13, "thirteen", A, B, C, D, E, F, G, H, I, J, K, L, M; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m; a, b, c, d, e, f, g, h, i, j, k, l, m);
define_seq!(seq14, "fourteen", A, B, C, D, E, F, G, H, I, J, K, L, M, N; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n; a, b, c, d, e, f, g, h, i, j, k, l, m, n);
define_seq!(seq15, "fifteen", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o);
define_seq!(seq16, "sixteen", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p);
define_seq!(seq17, "seventeen", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q);
define_seq!(seq18, "eighteen", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r);
define_seq!(seq19, "nineteen", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s);
define_seq!(seq20, "twenty", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t);
define_seq!(seq21, "twenty-one", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u);
define_seq!(seq22, "twenty-two", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v);
define_seq!(seq23, "twenty-three", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w);
define_seq!(seq24, "twenty-four", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x);
define_seq!(seq25, "twenty-five", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y);
define_seq!(seq26, "twenty-six", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z);
define_seq!(seq27, "twenty-seven", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_a1; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, a1);
define_seq!(seq28, "twenty-eight", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_a1, parser_b1; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, a1, b1);
define_seq!(seq29, "twenty-nine", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1, C1; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_a1, parser_b1, parser_c1; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, a1, b1, c1);
define_seq!(seq30, "thirty", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1, C1, D1; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_a1, parser_b1, parser_c1, parser_d1; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, a1, b1, c1, d1);
define_seq!(seq31, "thirty-one", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1, C1, D1, E1; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_a1, parser_b1, parser_c1, parser_d1, parser_e1; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, a1, b1, c1, d1, e1);
define_seq!(seq32, "thirty-two", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1, C1, D1, E1, F1; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_a1, parser_b1, parser_c1, parser_d1, parser_e1, parser_f1; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, a1, b1, c1, d1, e1, f1);
define_seq!(seq33, "thirty-three", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1, C1, D1, E1, F1, G1; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_a1, parser_b1, parser_c1, parser_d1, parser_e1, parser_f1, parser_g1; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, a1, b1, c1, d1, e1, f1, g1);
define_seq!(seq34, "thirty-four", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1, C1, D1, E1, F1, G1, H1; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_a1, parser_b1, parser_c1, parser_d1, parser_e1, parser_f1, parser_g1, parser_h1; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, a1, b1, c1, d1, e1, f1, g1, h1);
define_seq!(seq35, "thirty-five", A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1, C1, D1, E1, F1, G1, H1, I1; parser_a, parser_b, parser_c, parser_d, parser_e, parser_f, parser_g, parser_h, parser_i, parser_j, parser_k, parser_l, parser_m, parser_n, parser_o, parser_p, parser_q, parser_r, parser_s, parser_t, parser_u, parser_v, parser_w, parser_x, parser_y, parser_z, parser_a1, parser_b1, parser_c1, parser_d1, parser_e1, parser_f1, parser_g1, parser_h1, parser_i1; a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, a1, b1, c1, d1, e1, f1, g1, h1, i1);

/// Sequence two parsers, keeping only the first result.
pub fn skip_second<A, B, PA, PB>(
    parser_a: PA,
    parser_b: PB,
) -> impl Fn(&mut PtxTokenStream) -> Result<(A, Span), PtxParseError>
where
    PA: Fn(&mut PtxTokenStream) -> Result<(A, Span), PtxParseError>,
    PB: Fn(&mut PtxTokenStream) -> Result<(B, Span), PtxParseError>,
{
    move |stream| {
        let start_pos = stream.position().index;
        let (a, _) = parser_a(stream)?;
        let _ = parser_b(stream)?;
        let end_pos = stream.position().index;
        Ok((a, start_pos..end_pos))
    }
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
        let start_pos = stream.position().index;
        let _ = parser_a(stream)?;
        let (b, _) = parser_b(stream)?;
        let end_pos = stream.position().index;
        Ok((b, start_pos..end_pos))
    }
}

/// Map over a parser result, transforming the value while preserving the span.
pub fn map<A, B, P, F>(
    parser: P,
    f: F,
) -> impl Fn(&mut PtxTokenStream) -> Result<(B, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(A, Span), PtxParseError>,
    F: Fn(A) -> B,
{
    move |stream| {
        let start_pos = stream.position().index;
        let (a, _) = parser(stream)?;
        let end_pos = stream.position().index;
        Ok((f(a), start_pos..end_pos))
    }
}

/// Pure (return) - creates a parser that succeeds without consuming input.
///
/// This is the monadic return/pure operation.
pub fn pure<T>(value: T) -> impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>
where
    T: Clone + 'static,
{
    move |stream| {
        let pos = stream.position().index;
        Ok((value.clone(), pos..pos))
    }
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
pub fn dispatch<A, B, P, F>(
    parser: P,
    f: F,
) -> impl Fn(&mut PtxTokenStream) -> Result<(B, Span), PtxParseError>
where
    P: Fn(&mut PtxTokenStream) -> Result<(A, Span), PtxParseError>,
    F: Fn((A, Span)) -> Box<dyn Fn(&mut PtxTokenStream) -> Result<(B, Span), PtxParseError>>,
{
    move |stream| {
        let result = parser(stream)?;
        let next_parser = f(result);
        next_parser(stream)
    }
}

/* -------------------------------------------------- */
/* ----------------- Token Combinators -------------- */
/* -------------------------------------------------- */

/// Parser that expects a comma token.
pub fn comma() -> impl Fn(&mut PtxTokenStream) -> Result<((), Span), PtxParseError> {
    |stream| {
        let start_pos = stream.position().index;
        stream.expect(&PtxToken::Comma)?;
        let end_pos = stream.position().index;
        Ok(((), start_pos..end_pos))
    }
}

/// Parser that expects a semicolon token.
pub fn semicolon() -> impl Fn(&mut PtxTokenStream) -> Result<((), Span), PtxParseError> {
    |stream| {
        let start_pos = stream.position().index;
        stream.expect(&PtxToken::Semicolon)?;
        let end_pos = stream.position().index;
        Ok(((), start_pos..end_pos))
    }
}

/// Helper to parse using PtxParser trait and return as a combinator-compatible function
pub fn parse<T: PtxParser>() -> impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>
{
    move |stream| {
        let parser = T::parse();
        parser(stream)
    }
}

/// Parser combinator that enforces complete token consumption
/// (ensures we haven't stopped mid-token).
pub fn expect_complete() -> impl Fn(&mut PtxTokenStream) -> Result<((), Span), PtxParseError> {
    |stream| {
        let pos = stream.position().index;
        stream.expect_complete()?;
        Ok(((), pos..pos))
    }
}

/// Parser that expects a token matching a predicate.
pub fn expect_token(
    predicate: fn(&PtxToken) -> bool,
) -> impl Fn(&mut PtxTokenStream) -> Result<((), Span), PtxParseError> {
    move |stream| {
        let start_pos = stream.position().index;
        let (token, span) = stream.peek()?;
        if predicate(token) {
            stream.consume()?;
            let end_pos = stream.position().index;
            Ok(((), start_pos..end_pos))
        } else {
            Err(PtxParseError {
                kind: crate::parser::ParseErrorKind::UnexpectedToken {
                    expected: vec!["matching token".to_string()],
                    found: format!("{:?}", token),
                },
                span: span.clone(),
            })
        }
    }
}

/* -------------------------------------------------- */
/* ------------------- Choice Combinators ----------- */
/* -------------------------------------------------- */

/// Alternative combinator - try first parser, if it fails try second.
///
/// This is the fundamental choice combinator for building parsers with alternatives.
/// Uses backtracking - the stream is rewound if the first parser fails.
///
/// Example:
/// ```ignore
/// let parser = alt(
///     string(".s16".to_string(), Type::S16),
///     string(".s32".to_string(), Type::S32)
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
    move |stream| {
        let saved = stream.position();
        match parser1(stream) {
            Ok(result) => Ok(result),
            Err(_) => {
                stream.set_position(saved);
                parser2(stream)
            }
        }
    }
}

/// Choice combinator for multiple alternatives.
///
/// Tries each parser in order until one succeeds. Uses backtracking.
pub fn choice<T>(
    parsers: Vec<Box<dyn Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError>>>,
) -> impl Fn(&mut PtxTokenStream) -> Result<(T, Span), PtxParseError> {
    move |stream| {
        let mut last_error = None;

        for parser in &parsers {
            let saved = stream.position();
            match parser(stream) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    stream.set_position(saved);
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| PtxParseError {
            kind: crate::parser::ParseErrorKind::UnexpectedEof,
            span: 0..0,
        }))
    }
}

/* -------------------------------------------------- */
/* -------------------- Core Combinators ------------ */
/* -------------------------------------------------- */

/// Parser combinator that attempts to parse `T` but rewinds the stream if parsing
/// fails before returning the error.
///
/// Useful for backtracking without consuming input on failure.
pub struct Try<T>(pub T);

impl<T: PtxParser> PtxParser for Try<T> {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, crate::parser::Span), PtxParseError> {
        |stream| {
            let start_pos = stream.position().index;
            let saved = stream.position();
            let parser = T::parse();
            match parser(stream) {
                Ok((value, _)) => {
                    let end_pos = stream.position().index;
                    Ok((Try(value), start_pos..end_pos))
                }
                Err(err) => {
                    stream.set_position(saved);
                    Err(err)
                }
            }
        }
    }
}

/// Parser combinator that optionally parses `T`. On failure the stream is rewound
/// and `Ok(Maybe(None))` is returned.
///
/// This is the fundamental combinator for optional parsing.
pub struct Maybe<T>(pub Option<T>);

impl<T: PtxParser> PtxParser for Maybe<T> {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, crate::parser::Span), PtxParseError> {
        |stream| {
            let start_pos = stream.position().index;
            let saved = stream.position();
            let parser = T::parse();
            match parser(stream) {
                Ok((value, _)) => {
                    let end_pos = stream.position().index;
                    Ok((Maybe(Some(value)), start_pos..end_pos))
                }
                Err(_) => {
                    stream.set_position(saved);
                    Ok((Maybe(None), start_pos..start_pos))
                }
            }
        }
    }
}

/* -------------------------------------------------- */
/* -------------------- List Combinators ------------ */
/* -------------------------------------------------- */

/// Parser combinator that consumes one or more occurrences of `T`, separated by
/// commas, yielding the collected values.
///
/// Pattern: T (, T)*
pub struct Repeat<T>(pub Vec<T>);

impl<T: PtxParser> PtxParser for Repeat<T> {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, crate::parser::Span), PtxParseError> {
        |stream| {
            let start_pos = stream.position().index;
            let mut values = Vec::new();
            let parser = T::parse();
            let (value, _) = parser(stream)?;
            values.push(value);
            while stream
                .consume_if(|token| matches!(token, PtxToken::Comma))
                .is_some()
            {
                let parser = T::parse();
                let (value, _) = parser(stream)?;
                values.push(value);
            }
            let end_pos = stream.position().index;
            Ok((Repeat(values), start_pos..end_pos))
        }
    }
}

/// Convenience function to parse comma-separated list without wrapping.
pub fn repeat<T: PtxParser>(stream: &mut PtxTokenStream) -> Result<Vec<T>, PtxParseError> {
    let parser = Repeat::<T>::parse();
    parser(stream).map(|(Repeat(values), _)| values)
}

/// Parse zero or more occurrences of `T` (greedy).
///
/// Always succeeds, returning an empty vector if no matches.
pub fn many<T: PtxParser>(stream: &mut PtxTokenStream) -> Result<Vec<T>, PtxParseError> {
    let mut values = Vec::new();
    let parser = Maybe::<T>::parse();
    while let (Maybe(Some(value)), _) = parser(stream)? {
        values.push(value);
    }
    Ok(values)
}

/// Parse one or more occurrences of `T` (greedy).
///
/// Pattern: T+
pub fn many1<T: PtxParser>(stream: &mut PtxTokenStream) -> Result<Vec<T>, PtxParseError> {
    let mut values = Vec::new();
    let parser = T::parse();
    let (first, _) = parser(stream)?;
    values.push(first);
    let maybe_parser = Maybe::<T>::parse();
    while let (Maybe(Some(value)), _) = maybe_parser(stream)? {
        values.push(value);
    }
    Ok(values)
}

/* -------------------------------------------------- */
/* -------------------- Choice Combinators ---------- */
/* -------------------------------------------------- */

/// Parser combinator that tries parsing `A` first, then `B` if `A` fails.
///
/// The stream is rewound on `A` failure before trying `B`.
pub enum Or<A, B> {
    Left(A),
    Right(B),
}

impl<A: PtxParser, B: PtxParser> PtxParser for Or<A, B> {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, crate::parser::Span), PtxParseError> {
        |stream| {
            let start_pos = stream.position().index;
            let saved = stream.position();
            let parser_a = A::parse();
            match parser_a(stream) {
                Ok((value, _)) => {
                    let end_pos = stream.position().index;
                    Ok((Or::Left(value), start_pos..end_pos))
                }
                Err(_) => {
                    stream.set_position(saved);
                    let parser_b = B::parse();
                    let (value, _) = parser_b(stream)?;
                    let end_pos = stream.position().index;
                    Ok((Or::Right(value), start_pos..end_pos))
                }
            }
        }
    }
}

/* -------------------------------------------------- */
/* -------------- Sequence Combinators -------------- */
/* -------------------------------------------------- */

/// Parser combinator for a pair of parsers (A, B).
pub struct Pair<A, B>(pub A, pub B);

impl<A: PtxParser, B: PtxParser> PtxParser for Pair<A, B> {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, crate::parser::Span), PtxParseError> {
        |stream| {
            let start_pos = stream.position().index;
            let parser_a = A::parse();
            let (a, _) = parser_a(stream)?;
            let parser_b = B::parse();
            let (b, _) = parser_b(stream)?;
            let end_pos = stream.position().index;
            Ok((Pair(a, b), start_pos..end_pos))
        }
    }
}

/// Parser combinator for a triple of parsers (A, B, C).
pub struct Triple<A, B, C>(pub A, pub B, pub C);

impl<A: PtxParser, B: PtxParser, C: PtxParser> PtxParser for Triple<A, B, C> {
    fn parse() -> impl Fn(&mut PtxTokenStream) -> Result<(Self, crate::parser::Span), PtxParseError> {
        |stream| {
            let start_pos = stream.position().index;
            let parser_a = A::parse();
            let (a, _) = parser_a(stream)?;
            let parser_b = B::parse();
            let (b, _) = parser_b(stream)?;
            let parser_c = C::parse();
            let (c, _) = parser_c(stream)?;
            let end_pos = stream.position().index;
            Ok((Triple(a, b, c), start_pos..end_pos))
        }
    }
}

/* -------------------------------------------------- */
/* ------------- Separated List Combinators --------- */
/* -------------------------------------------------- */

/// Parse comma-separated list allowing trailing comma.
/// Returns empty vector on zero matches.
pub fn sep_by<T: PtxParser>(stream: &mut PtxTokenStream) -> Result<Vec<T>, PtxParseError> {
    let mut values = Vec::new();
    let maybe_parser = Maybe::<T>::parse();
    loop {
        let (Maybe(value), _) = maybe_parser(stream)?;
        match value {
            Some(v) => {
                values.push(v);
                if stream
                    .consume_if(|token| matches!(token, PtxToken::Comma))
                    .is_none()
                {
                    break;
                }
            }
            None => break,
        }
    }
    Ok(values)
}

/// Parse comma-separated list requiring at least one element.
pub fn sep_by1<T: PtxParser>(stream: &mut PtxTokenStream) -> Result<Vec<T>, PtxParseError> {
    let parser = T::parse();
    let (first, _) = parser(stream)?;
    let mut values = vec![first];
    while stream
        .consume_if(|token| matches!(token, PtxToken::Comma))
        .is_some()
    {
        let parser = T::parse();
        let (value, _) = parser(stream)?;
        values.push(value);
    }
    Ok(values)
}

/* -------------------------------------------------- */
/* -------------------- Utility Helpers ------------- */
/* -------------------------------------------------- */

/// Helper to check if the stream has a specific directive coming next.
pub fn peek_directive(stream: &PtxTokenStream, _directive: &str) -> bool {
    // Directives are represented as Dot + Identifier
    // We can't peek ahead 2 tokens without mutating, so
    // this function is a best-effort check
    // Just check if we have a Dot token for now
    matches!(stream.peek(), Ok((PtxToken::Dot, _)))
}

/// Helper to check if the stream has a specific token coming next.
pub fn peek_token(stream: &PtxTokenStream, predicate: impl FnOnce(&PtxToken) -> bool) -> bool {
    stream
        .peek()
        .map(|(tok, _)| predicate(tok))
        .unwrap_or(false)
}

/// Combinator that parses a token satisfying a predicate.
/// Returns the token and its span.
pub fn satisfy<F>(
    predicate: F,
    expected_msg: &'static str,
) -> impl Fn(&mut PtxTokenStream) -> Result<((PtxToken, Span), Span), PtxParseError>
where
    F: Fn(&PtxToken) -> bool + Clone + 'static,
{
    move |stream| {
        let start_pos = stream.position().index;
        let (token, span) = stream.peek().map_err(|_| PtxParseError {
            kind: crate::parser::ParseErrorKind::UnexpectedEof,
            span: start_pos..start_pos,
        })?;

        if predicate(&token) {
            stream.consume()?;
            let end_pos = stream.position().index;
            Ok(((token.clone(), span.clone()), start_pos..end_pos))
        } else {
            Err(PtxParseError {
                kind: crate::parser::ParseErrorKind::UnexpectedToken {
                    expected: vec![expected_msg.to_string()],
                    found: format!("{:?}", token),
                },
                span: span.clone(),
            })
        }
    }
}

/// Combinator that peeks at the next token without consuming it.
pub fn peek_p() -> impl Fn(&mut PtxTokenStream) -> Result<((PtxToken, Span), Span), PtxParseError> {
    |stream| {
        let pos = stream.position().index;
        let (token, span) = stream.peek().map_err(|_| PtxParseError {
            kind: crate::parser::ParseErrorKind::UnexpectedEof,
            span: pos..pos,
        })?;
        Ok(((token.clone(), span.clone()), pos..pos))
    }
}

/// Combinator that checks if a predicate matches without consuming.
pub fn check_p<F>(predicate: F) -> impl Fn(&mut PtxTokenStream) -> Result<(bool, Span), PtxParseError>
where
    F: Fn(&PtxToken) -> bool + Clone + 'static,
{
    move |stream| {
        let pos = stream.position().index;
        let result = stream.peek()
            .map(|(token, _)| predicate(&token))
            .unwrap_or(false);
        Ok((result, pos..pos))
    }
}

/// Combinator that peeks at the next directive (Dot + Identifier pattern).
/// Returns Some((directive_name, span)) if a directive is found, None otherwise.
pub fn peek_directive_p() -> impl Fn(&mut PtxTokenStream) -> Result<(Option<(String, Span)>, Span), PtxParseError> {
    |stream| {
        let pos = stream.position().index;
        let result = crate::parser::peek_directive(stream)?;
        Ok((result, pos..pos))
    }
}
