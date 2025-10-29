### Description

Conditional selection. If c >= 0, a is stored in d, otherwise b is stored in
d. Operands d, a, and b are treated as a bitsize type of the same width as the first
instruction type; operand c must match the second instruction type (.s32 or .f32). The
selected input is copied to the output without modification.

### Syntax

```
slct.dtype.s32        d, a, b, c;
slct{.ftz}.dtype.f32  d, a, b, c;

.dtype = { .b16, .b32, .b64,
           .u16, .u32, .u64,
           .s16, .s32, .s64,
                 .f32, .f64 };
```

### Semantics

```
d = (c >= 0) ? a : b;
```

### Examples

```
slct.u32.s32  x, y, z, val;
slct.ftz.u64.f32  A, B, C, fval;
```

