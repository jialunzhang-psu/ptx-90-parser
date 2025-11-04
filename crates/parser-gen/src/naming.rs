//! Naming utilities for converting PTX identifiers to Rust naming conventions.
//!
//! This module provides functions for:
//! - Converting identifiers to snake_case (for variables and fields)
//! - Converting identifiers to PascalCase (for types and variants)
//! - Sanitizing identifiers to be valid Rust identifiers

/// Convert PascalCase to snake_case.
///
/// Examples:
/// - `CpSize` -> `cp_size`
/// - `Dsel` -> `dsel`
/// - `FooBarBaz` -> `foo_bar_baz`
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(ch);
        }
    }
    result
}

/// Convert an identifier to PascalCase.
///
/// Used for variant names, struct names, and enum names.
/// Handles special characters like '.', '-', '::', etc.
///
/// Examples:
/// - `cp-size` -> `CpSize`
/// - `.ftz` -> `Ftz`
/// - `::buffer::op` -> `BufferOp`
pub fn to_pascal_case(s: &str) -> String {
    let cleaned = s
        .trim_start_matches('.')
        .trim_start_matches("::")
        .replace("::", "_")
        .replace('.', "_")
        .replace('-', "_")
        .replace('*', "")
        .replace('+', "plus");

    let pascal = cleaned
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<String>();

    // If starts with digit, prefix with underscore
    if pascal.chars().next().map_or(false, |c| c.is_ascii_digit()) {
        format!("_{}", pascal)
    } else {
        pascal
    }
}

/// Sanitize an identifier for use as a Rust field or variable name (snake_case).
///
/// Handles special cases:
/// - Converts to lowercase
/// - Replaces special characters with underscores
/// - Prefixes digits with underscore
/// - Escapes Rust keywords
///
/// Examples:
/// - `_` -> `operand`
/// - `cp-size` -> `cp_size`
/// - `.type` -> `type_` (keyword escaping)
pub fn sanitize_field_name(s: &str) -> String {
    if s == "_" {
        return "operand".to_string();
    }

    let cleaned = s
        .trim_start_matches('.')
        .trim_start_matches("::")
        .replace("::", "_")
        .replace('.', "_")
        .replace('-', "_")
        .replace('*', "")
        .replace('+', "plus")
        .to_lowercase();

    // If the string is empty after cleaning, return a default name
    if cleaned.is_empty() {
        return "flag".to_string();
    }

    // Ensure it starts with lowercase or underscore
    let with_prefix = if cleaned.chars().next().map_or(false, |c| c.is_ascii_digit()) {
        format!("_{}", cleaned)
    } else {
        cleaned
    };

    // Escape Rust keywords
    match with_prefix.as_str() {
        "type" | "match" | "const" | "static" | "extern" | "async" | "await"
        | "fn" | "let" | "mut" | "impl" | "trait" | "struct" | "enum"
        | "mod" | "pub" | "use" | "crate" | "self" | "super" | "in"
        | "loop" | "while" | "for" | "if" | "else" | "return" | "break" | "continue" => {
            format!("{}_", with_prefix)
        }
        _ => with_prefix,
    }
}

/// Sanitize an identifier for use as a Rust variant name (PascalCase).
///
/// This is an alias for `to_pascal_case`.
pub fn sanitize_variant_name(s: &str) -> String {
    to_pascal_case(s)
}

/// Sanitize an identifier for use as a Rust struct name (PascalCase).
///
/// This is an alias for `to_pascal_case`.
pub fn sanitize_struct_name(s: &str) -> String {
    to_pascal_case(s)
}

/// Sanitize an identifier for use as a Rust enum name (PascalCase).
///
/// This is an alias for `to_pascal_case`.
pub fn sanitize_enum_name(s: &str) -> String {
    to_pascal_case(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("CpSize"), "cp_size");
        assert_eq!(to_snake_case("Dsel"), "dsel");
        assert_eq!(to_snake_case("FooBarBaz"), "foo_bar_baz");
        assert_eq!(to_snake_case("lowercase"), "lowercase");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("cp-size"), "CpSize");
        assert_eq!(to_pascal_case(".ftz"), "Ftz");
        assert_eq!(to_pascal_case("::buffer::op"), "BufferOp");
        assert_eq!(to_pascal_case("foo_bar"), "FooBar");
    }

    #[test]
    fn test_sanitize_field_name() {
        assert_eq!(sanitize_field_name("_"), "operand");
        assert_eq!(sanitize_field_name("cp-size"), "cp_size");
        assert_eq!(sanitize_field_name(".type"), "type_");
        assert_eq!(sanitize_field_name("123abc"), "_123abc");
    }

    #[test]
    fn test_sanitize_enum_name() {
        assert_eq!(sanitize_enum_name("cp-size"), "CpSize");
        assert_eq!(sanitize_enum_name(".dsel"), "Dsel");
    }
}
