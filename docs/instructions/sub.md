### Description

Performs subtraction and writes the resulting value into a destination register.

### Syntax

```
sub.type       d, a, b;
sub{.sat}.s32  d, a, b;     // .sat applies only to .s32

.type = { .u16, .u32, .u64,
          .s16, .s32, .s64 };
```

### Semantics

```
d = a - b;
```

### Examples

```
sub.s32 c,a,b;
```

