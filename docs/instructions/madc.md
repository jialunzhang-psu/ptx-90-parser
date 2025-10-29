### Description

Multiplies two values, extracts either the high or low part of the result, and adds a third value
along with carry-in. Writes the result to the destination register and optionally writes the
carry-out from the addition into the condition code register.

### Syntax

```
madc{.hi,.lo}{.cc}.type  d, a, b, c;

.type = { .u32, .s32, .u64, .s64 };
```

### Semantics

```
t = a * b;
d = t<63..32> + c + CC.CF;     // for .hi variant
d = t<31..0> + c + CC.CF;      // for .lo variant
```
if .cc specified, carry-out from addition is written to CC.CF

### Examples

```
// extended-precision multiply:  [r3,r2,r1,r0] = [r5,r4] * [r7,r6]
mul.lo.u32     r0,r4,r6;      // r0=(r4*r6).[31:0], no carry-out
mul.hi.u32     r1,r4,r6;      // r1=(r4*r6).[63:32], no carry-out
mad.lo.cc.u32  r1,r5,r6,r1;   // r1+=(r5*r6).[31:0], may carry-out
madc.hi.u32    r2,r5,r6,0;    // r2 =(r5*r6).[63:32]+carry-in,
                              // no carry-out
mad.lo.cc.u32   r1,r4,r7,r1;  // r1+=(r4*r7).[31:0], may carry-out
madc.hi.cc.u32  r2,r4,r7,r2;  // r2+=(r4*r7).[63:32]+carry-in,
                              // may carry-out
addc.u32        r3,0,0;       // r3 = carry-in, no carry-out
mad.lo.cc.u32   r2,r5,r7,r2;  // r2+=(r5*r7).[31:0], may carry-out
madc.hi.u32     r3,r5,r7,r3;  // r3+=(r5*r7).[63:32]+carry-in
```

