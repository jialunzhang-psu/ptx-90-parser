### Description

Performs integer subtraction with borrow-in and optionally writes the borrow-out value into the
condition code register.

### Syntax

```
subc{.cc}.type  d, a, b;

.type = { .u32, .s32, .u64, .s64 };
```

### Semantics

```
d = a  - (b + CC.CF);
```
if .cc specified, borrow-out written to CC.CF

### Examples

```
@p  sub.cc.u32   x1,y1,z1;   // extended-precision subtraction
@p  subc.cc.u32  x2,y2,z2;   // of two 128-bit values
@p  subc.cc.u32  x3,y3,z3;
@p  subc.u32     x4,y4,z4;
```

