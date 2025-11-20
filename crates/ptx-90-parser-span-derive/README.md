# ptx-90-parser-span-derive

`ptx-90-parser-span-derive` (imported as `ptx_90_parser_span_derive`) is a small proc-macro crate that powers the [`ptx-90-parser`](https://github.com/jialunzhang-psu/ptx-90-parser) project.  
It generates `Spanned` trait implementations for structs and enums whose fields include a `span` value and mirrors the helper methods that the parser expects.

```rust
use ptx_90_parser_span_derive::Spanned;

#[derive(Spanned)]
pub struct Operand {
    pub value: String,
    pub span: Span,
}
```
