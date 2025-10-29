### Description

Divides a by b, stores result in d.

### Syntax

```
div.type  d, a, b;

.type = { .u16, .u32, .u64,
          .s16, .s32, .s64 };
```

### Semantics

```
d = a / b;
```

### Examples

```
div.s32  b,n,i;
```

