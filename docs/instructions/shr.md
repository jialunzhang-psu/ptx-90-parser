### Description

Shift a right by the amount specified by unsigned 32-bit value in b. Signed shifts fill with
the sign bit, unsigned and untyped shifts fill with 0.

### Syntax

```
shr.type d, a, b;

.type = { .b16, .b32, .b64,
          .u16, .u32, .u64,
          .s16, .s32, .s64 };
```

### Semantics

```
d = a >> b;
```

