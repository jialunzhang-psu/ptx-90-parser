# ptx-90-parser-construct

`ptx-90-parser-construct` (imported as `ptx_90_parser_construct`) bundles the procedural macros that shorten AST construction in [`ptx-90-parser`](https://github.com/jialunzhang-psu/ptx-90-parser).

The main helpers are:

- `c!` – construct a struct and automatically fill its `span` field.
- `ok!` / `err!` – convenience constructors for `Result`-producing parser code.
- `cclosure!` / `okmap!` – turn tuples of parser outputs into AST values while threading spans.
- `func!` – append a `span` argument to an existing closure.

```rust
use ptx_90_parser_construct::{c, ok};

fn parse_operand(symbol: Symbol, span: Span) -> Result<Operand, PtxParseError> {
    ok!(Operand::Symbol { symbol })
}
```
