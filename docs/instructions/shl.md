### Description

Shift a left by the amount specified by unsigned 32-bit value in b.

### Syntax

```
shl.type d, a, b;

.type = { .b16, .b32, .b64 };
```

### Semantics

```
d = a << b;
```

