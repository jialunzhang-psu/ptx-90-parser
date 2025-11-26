use super::tree_formatter::truncate_with_ellipsis;
use crate::Span;

#[test]
fn test_truncate_short() {
    assert_eq!(truncate_with_ellipsis("hello", 40), "hello");
}

#[test]
fn test_truncate_exact() {
    let s = "a".repeat(40);
    assert_eq!(truncate_with_ellipsis(&s, 40), s);
}

#[test]
fn test_truncate_long() {
    let s = "a".repeat(20) + &"b".repeat(30);
    let result = truncate_with_ellipsis(&s, 40);
    assert!(result.contains("..."));
    assert!(result.len() <= 43); // 20 + 3 + 20
    assert!(result.starts_with("aaaaaaaa"));
    assert!(result.ends_with("bbbbbbbb"));
}

fn extract_span_text(span: Span, source: &str) -> String {
    if span.start <= span.end && span.end <= source.len() {
        source[span.start..span.end].to_string()
    } else {
        format!("<invalid span: {}..{}>", span.start, span.end)
    }
}

#[test]
fn test_extract_span_valid() {
    let source = "hello world";
    let span = Span { start: 0, end: 5 };
    assert_eq!(extract_span_text(span, source), "hello");
}

#[test]
fn test_extract_span_invalid() {
    let source = "hello";
    let span = Span { start: 0, end: 100 };
    assert!(extract_span_text(span, source).contains("invalid"));
}
