# ptx-parser

There is already a crate named `ptx-parser` on crates.io, but it has not been
updated for 2 years and does not support PTX 9.0. This crate, `ptx-parser-90`,
parses NVIDIA PTX 9.0 assembly source into a structured abstract syntax tree. It
also ships with a small companion CLI that prints module summaries and
optionally emits a tree representation of the parsed PTX.

## Features

- Parses PTX source into rich Rust data types for downstream analysis or transformation
- Distinguishes directives, statements, and instructions in kernel bodies
- Provides a CLI (`ptx-parser-bin`) that reports module statistics and outputs a tree view

## Library quick start

Add the crate to your project:

```shell
cargo add ptx-parser
```

Parse a PTX module from source text:

```rust
use ptx_parser::parse;

fn main() -> Result<(), ptx_parser::PtxParseError> {
    let module = parse(
        r#"
        .version 7.8
        .target sm_90
        .entry add(.param .u64 a) { ret; }
        "#,
    )?;
    println!("directives: {}", module.directives.len());
    Ok(())
}
```

Refer to the items exported from `ptx_parser::type` for the full AST shape.

## CLI usage

```
cargo run --bin ptx-parser-bin -- --input ./examples/module.ptx
```

Options:

- `--input` (`-i`): path to the PTX file to parse (required)
- `--output` (`-o`): optional path for writing the textual tree representation

Example:

```
ptx-parser-bin --input kernel.ptx --output kernel.tree
```

## License

This project is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details.
