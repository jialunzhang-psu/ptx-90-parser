### Description

Compute the product of two 24-bit integer values held in 32-bit source registers, and add a third,
32-bit value to either the high or low 32-bits of the 48-bit result. Return either the high or low
32-bits of the 48-bit result.

### Syntax

```
mad24.mode.type  d, a, b, c;
mad24.hi.sat.s32 d, a, b, c;

.mode = { .hi, .lo };
.type = { .u32, .s32 };
```

### Semantics

```
t = a * b;
d = t<47..16> + c;   // for .hi variant
d = t<31..0> + c;    // for .lo variant
```

### Examples

```
mad24.lo.s32 d,a,b,c;   // low 32-bits of 24x24-bit signed multiply.
```

