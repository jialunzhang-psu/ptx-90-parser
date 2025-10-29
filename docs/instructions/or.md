### Description

Compute the bit-wise or operation for the bits in a and b.

### Syntax

```
or.type d, a, b;

.type = { .pred, .b16, .b32, .b64 };
```

### Semantics

```
d = a | b;
```

### Examples

```
or.b32  mask mask,0x00010001
or.pred  p,q,r;
```

