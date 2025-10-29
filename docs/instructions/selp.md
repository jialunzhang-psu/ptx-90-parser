### Description

Conditional selection. If c is True, a is stored in d, b otherwise. Operands
d, a, and b must be of the same type. Operand c is a predicate.

### Syntax

```
selp.type d, a, b, c;

.type = { .b16, .b32, .b64,
          .u16, .u32, .u64,
          .s16, .s32, .s64,
                .f32, .f64 };
```

### Semantics

```
d = (c == 1) ? a : b;
```

### Examples

```
selp.s32  r0,r,g,p;
@q  selp.f32  f0,t,x,xp;
```

