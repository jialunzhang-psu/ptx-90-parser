///! - c!, ok!, err! are used to construct structs with automatic span field.
///! - cclosure! is used to create closures that map tuples to structs with automatic span field. okmap! is like cclosure! but wraps the result in Ok(...).
///! - func! adds a span parameter to closures argument list. 
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, Path, Token,
};

/// A field in the cmap macro - either `field = expr`, `field`, or `_`
enum CmapField {
    /// `field = expr` - assign expr to field
    Assign { name: Ident, expr: Expr },
    /// `field` - shorthand for `field: field`
    Shorthand { name: Ident },
    /// `_` - wildcard, ignored in pattern
    Wildcard,
}

impl Parse for CmapField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Check for wildcard first
        if input.peek(Token![_]) {
            let _: Token![_] = input.parse()?;
            return Ok(CmapField::Wildcard);
        }

        let name: Ident = input.parse()?;

        // Check if there's an `=` token
        if input.peek(Token![=]) {
            let _eq: Token![=] = input.parse()?;
            let expr: Expr = input.parse()?;
            Ok(CmapField::Assign { name, expr })
        } else {
            Ok(CmapField::Shorthand { name })
        }
    }
}

/// Input structure for the cmap macro
///
/// Syntax: `Type { field1 = expr1, field2, ... }`
struct CmapInput {
    type_path: Path,
    fields: Punctuated<CmapField, Token![,]>,
}

impl Parse for CmapInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse Type path
        let type_path: Path = input.parse()?;

        // Parse { fields } - now required
        let fields_content;
        syn::braced!(fields_content in input);
        let fields = fields_content.parse_terminated(CmapField::parse, Token![,])?;

        Ok(CmapInput {
            type_path,
            fields,
        })
    }
}

/// Constructor mapping macro - eliminates field repetition
///
/// Syntax:
/// ```rust
/// cclosure!(Type { field1, field2=expr })
/// ```
///
/// Example:
/// ```rust
/// cclosure!(Operand::SymbolOffset { symbol, _, offset=offset.unwrap() })
/// // Generates: |(symbol, _, offset), span| c!(Operand::SymbolOffset { symbol, _, offset=offset.unwrap() })
/// ```
#[proc_macro]
pub fn cclosure(input: TokenStream) -> TokenStream {
    let CmapInput {
        type_path,
        fields,
    } = parse_macro_input!(input as CmapInput);

    // Extract pattern elements from field names (in order)
    let mut pattern_elements = Vec::new();
    for field in &fields {
        match field {
            CmapField::Assign { name, .. } => {
                pattern_elements.push(quote! { #name });
            }
            CmapField::Shorthand { name } => {
                pattern_elements.push(quote! { #name });
            }
            CmapField::Wildcard => {
                pattern_elements.push(quote! { _ });
            }
        }
    }

    // Build the tuple pattern from the field names
    let pattern = if pattern_elements.len() == 1 {
        let elem = &pattern_elements[0];
        quote! { #elem }
    } else {
        quote! { (#(#pattern_elements),*) }
    };

    // Generate field assignments (skip wildcards)
    let field_assignments = fields.iter().filter_map(|field| match field {
        CmapField::Assign { name, expr } => {
            Some(quote! { #name: #expr })
        }
        CmapField::Shorthand { name } => {
            Some(quote! { #name: #name })
        }
        CmapField::Wildcard => None,
    });

    // Generate the closure
    let expanded = quote! {
        move |#pattern, span| {
            #type_path {
                #(#field_assignments,)*
                span
            }
        }
    };

    TokenStream::from(expanded)
}

/// Constructor macro - builds a struct with automatic span field
///
/// Syntax:
/// ```rust
/// c!(Type { field1 = expr1, field2, ... })
/// ```
///
/// Example:
/// ```rust
/// c!(VariableDirective { name=something, foo })
/// // Expands to: VariableDirective { name: something, foo: foo, span: span }
/// ```
#[proc_macro]
pub fn c(input: TokenStream) -> TokenStream {
    // Parse: [span_expr =>] Type { fields }
    let input_parsed = parse_macro_input!(input with parse_c_input);

    let (span_expr, type_path, fields) = input_parsed;

    // Process fields similar to cmap
    let field_assignments = fields.iter().filter_map(|field| match field {
        CmapField::Assign { name, expr } => {
            Some(quote! { #name: #expr })
        }
        CmapField::Shorthand { name } => {
            Some(quote! { #name: #name })
        }
        CmapField::Wildcard => None,
    });

    // Generate the struct construction
    let expanded = quote! {
        #type_path {
            #(#field_assignments,)*
            span: #span_expr
        }
    };

    TokenStream::from(expanded)
}

fn parse_c_input(input: ParseStream) -> syn::Result<(Expr, Path, Punctuated<CmapField, Token![,]>)> {
    // Fork the input to try parsing with span expression first
    let fork = input.fork();

    // Try to parse: Expr => Path { fields }
    let (span_expr, type_path) = if let Ok(_expr) = fork.parse::<Expr>() {
        if fork.peek(Token![=>]) {
            // Successfully parsed expr followed by =>, so advance the main input
            let expr: Expr = input.parse()?;
            let _arrow: Token![=>] = input.parse()?;
            let type_path: Path = input.parse()?;
            (expr, type_path)
        } else {
            // No =>, so the first thing must be the type path
            let type_path: Path = input.parse()?;
            (syn::parse_quote!(span), type_path)
        }
    } else {
        // Couldn't parse as expr, try as path
        let type_path: Path = input.parse()?;
        (syn::parse_quote!(span), type_path)
    };

    // Parse { fields } if present
    let fields = if input.peek(syn::token::Brace) {
        let fields_content;
        syn::braced!(fields_content in input);
        fields_content.parse_terminated(CmapField::parse, Token![,])?
    } else {
        Punctuated::new()
    };

    Ok((span_expr, type_path, fields))
}

/// Ok wrapper macro - wraps result in Ok(...) with automatic span field
///
/// Syntax:
/// ```rust
/// ok!(Type { field1 = expr1, field2, ... })
/// ```
///
/// Example:
/// ```rust
/// ok!(TexHandler2 { operands, name=foo.to_string() })
/// // Expands to: Ok(TexHandler2 { operands: operands, name: foo.to_string(), span: span })
/// ```
#[proc_macro]
pub fn ok(input: TokenStream) -> TokenStream {
    // Reuse c! macro logic but wrap in Ok(value)
    // Used inside try_map closures where try_with_span will add the outer tuple
    let c_result = c(input);
    let c_tokens: proc_macro2::TokenStream = c_result.into();

    let expanded = quote! {
        Ok(#c_tokens)
    };

    TokenStream::from(expanded)
}

/// Error constructor macro - builds a PtxParseError with automatic span field
///
/// Syntax:
/// ```rust
/// err!(ErrorKind)
/// ```
///
/// Example:
/// ```rust
/// err!(ParseErrorKind::InvalidLiteral("message".into()))
/// // Expands to: Err(PtxParseError { kind: ParseErrorKind::InvalidLiteral("message".into()), span: span })
/// ```
#[proc_macro]
pub fn err(input: TokenStream) -> TokenStream {
    // Parse: [span_expr =>] error_kind
    let input_parsed = parse_macro_input!(input with parse_err_input);

    let (span_expr, error_kind) = input_parsed;

    // Generate the error construction
    // Note: We use crate::parser::PtxParseError which will resolve in the calling code's context
    let expanded = quote! {
        Err(crate::parser::PtxParseError {
            kind: #error_kind,
            span: #span_expr
        })
    };

    TokenStream::from(expanded)
}

fn parse_err_input(input: ParseStream) -> syn::Result<(Expr, Expr)> {
    // Try to parse span expression followed by =>
    let span_expr = if input.peek2(Token![=>]) {
        let expr: Expr = input.parse()?;
        let _arrow: Token![=>] = input.parse()?;
        expr
    } else {
        // If no span => provided, use `span` identifier
        syn::parse_quote!(span)
    };

    // Parse error kind expression
    let error_kind: Expr = input.parse()?;

    Ok((span_expr, error_kind))
}

/// Constructor mapping macro with Ok wrapper - like cclosure! but wraps result with Ok
///
/// Syntax:
/// ```rust
/// okmap!(Type { field1, field2=expr })
/// ```
///
/// Example:
/// ```rust
/// okmap!(Operand::SymbolOffset { symbol, _, offset=offset.unwrap() })
/// // Generates: |(symbol, _, offset), span| Ok(Operand::SymbolOffset { symbol, offset: offset.unwrap(), span })
/// ```
#[proc_macro]
pub fn okmap(input: TokenStream) -> TokenStream {
    let CmapInput {
        type_path,
        fields,
    } = parse_macro_input!(input as CmapInput);

    // Extract pattern elements from field names (in order)
    let mut pattern_elements = Vec::new();
    for field in &fields {
        match field {
            CmapField::Assign { name, .. } => {
                pattern_elements.push(quote! { #name });
            }
            CmapField::Shorthand { name } => {
                pattern_elements.push(quote! { #name });
            }
            CmapField::Wildcard => {
                pattern_elements.push(quote! { _ });
            }
        }
    }

    // Build the tuple pattern from the field names
    let pattern = if pattern_elements.len() == 1 {
        let elem = &pattern_elements[0];
        quote! { #elem }
    } else {
        quote! { (#(#pattern_elements),*) }
    };

    // Generate field assignments (skip wildcards)
    let field_assignments = fields.iter().filter_map(|field| match field {
        CmapField::Assign { name, expr } => {
            Some(quote! { #name: #expr })
        }
        CmapField::Shorthand { name } => {
            Some(quote! { #name: #name })
        }
        CmapField::Wildcard => None,
    });

    // Generate the closure wrapped in Ok
    let expanded = quote! {
        move |#pattern, span| {
            Ok(#type_path {
                #(#field_assignments,)*
                span
            })
        }
    };

    TokenStream::from(expanded)
}

/// Function macro - adds span parameter to closures
///
/// Syntax:
/// ```rust
/// func!(|param1, param2| body)
/// ```
///
/// Example:
/// ```rust
/// func!(|x, y| x + y)
/// // Expands to: |x, y, span| x + y
/// ```
#[proc_macro]
pub fn func(input: TokenStream) -> TokenStream {
    use syn::{ExprClosure, Pat};

    let closure = parse_macro_input!(input as ExprClosure);

    // Extract the existing parameters
    let mut params = closure.inputs.clone();

    // Add span parameter
    let span_param: Pat = syn::parse_quote!(span);
    params.push(span_param);

    // Get the body
    let body = closure.body;

    // Generate the expanded closure
    let expanded = quote! {
        |#params| #body
    };

    TokenStream::from(expanded)
}
