### Description

Compute the product of two 24-bit integer values held in 32-bit source registers, and return either
the high or low 32-bits of the 48-bit result.

### Syntax

```
mul24.mode.type  d, a, b;

.mode = { .hi, .lo };
.type = { .u32, .s32 };
```

### Semantics

```
t = a * b;
d = t<47..16>;    // for .hi variant
d = t<31..0>;     // for .lo variant
```

### Examples

```
mul24.lo.s32 d,a,b;   // low 32-bits of 24x24-bit signed multiply.
```

