# Parser Generator for PTX

## Important Specs
The following specifications are considered important in terms of their syntax. They should cover most of the syntax features.
- `cp.async`: `cp-size = { 4, 8, 16 }`
- `setp`: `d|p`
- `call`
- `vmad`
- `tcgen05.mma.sp`
- `clusterlaunchcontrol.query_cancel`
- `tcgen05.cp`
- `bar`
- `tex`
- `atom`
- `min`

## TODO

- Fix the meaning of `{ .collector::buffer::op }`. For its current meaning, see `spec_parser.rs:518`. Should we treat the first segment specially?
- Clean up `main.rs`.

# WARNING

- In `ptx_syntax/red.txt`, the syntax for reduction operations is different from the official PTX documentation in the order of modifiers to match the real generated PTX files.
