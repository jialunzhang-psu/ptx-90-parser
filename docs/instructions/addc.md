### Description

Performs integer addition with carry-in and optionally writes the carry-out value into the condition
code register.

### Syntax

```
addc{.cc}.type  d, a, b;

.type = { .u32, .s32, .u64, .s64 };
```

### Semantics

```
d = a + b + CC.CF;
```
if .cc specified, carry-out written to CC.CF

### Examples

```
@p  add.cc.u32   x1,y1,z1;   // extended-precision addition of
@p  addc.cc.u32  x2,y2,z2;   // two 128-bit values
@p  addc.cc.u32  x3,y3,z3;
@p  addc.u32     x4,y4,z4;
```

