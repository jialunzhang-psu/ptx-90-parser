### Description

Compute the bit-wise exclusive-or operation for the bits in a and b.

### Syntax

```
xor.type d, a, b;

.type = { .pred, .b16, .b32, .b64 };
```

### Semantics

```
d = a ^ b;
```

### Examples

```
xor.b32  d,q,r;
xor.b16  d,x,0x0001;
```

