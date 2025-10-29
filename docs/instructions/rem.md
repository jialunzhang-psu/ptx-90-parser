### Description

Divides a by b, store the remainder in d.

### Syntax

```
rem.type  d, a, b;

.type = { .u16, .u32, .u64,
          .s16, .s32, .s64 };
```

### Semantics

```
d = a % b;
```

### Examples

```
rem.s32  x,x,8;    // x = x%8;
```

