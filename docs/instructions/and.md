### Description

Compute the bit-wise and operation for the bits in a and b.

### Syntax

```
and.type d, a, b;

.type = { .pred, .b16, .b32, .b64 };
```

### Semantics

```
d = a & b;
```

### Examples

```
and.b32  x,q,r;
and.b32  sign,fpvalue,0x80000000;
```

