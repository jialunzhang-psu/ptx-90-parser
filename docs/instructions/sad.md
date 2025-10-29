### Description

Adds the absolute value of a-b to c and writes the resulting value into d.

### Syntax

```
sad.type  d, a, b, c;

.type = { .u16, .u32, .u64,
          .s16, .s32, .s64 };
```

### Semantics

```
d = c + ((a<b) ? b-a : a-b);
```

### Examples

```
sad.s32  d,a,b,c;
sad.u32  d,a,b,d;  // running sum
```

