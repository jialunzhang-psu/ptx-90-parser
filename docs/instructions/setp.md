### Description

Compares two values and combines the result with another predicate value by applying a Boolean
operator. This result is written to the first destination operand. A related value computed using
the complement of the compare result is written to the second destination operand.
Applies to all numeric types. Operands a and b have type .type; operands p, q,
and c have type .pred. The sink symbol ‘_’ may be used in place of any one of the
destination operands.

### Syntax

```
setp.CmpOp{.ftz}.type         p[|q], a, b;
setp.CmpOp.BoolOp{.ftz}.type  p[|q], a, b, {!}c;

.CmpOp  = { eq, ne, lt, le, gt, ge, lo, ls, hi, hs,
            equ, neu, ltu, leu, gtu, geu, num, nan };
.BoolOp = { and, or, xor };
.type   = { .b16, .b32, .b64,
            .u16, .u32, .u64,
            .s16, .s32, .s64,
                  .f32, .f64 };
```

### Semantics

```
t = (a CmpOp b) ? 1 : 0;
p = BoolOp(t, c);
q = BoolOp(!t, c);
```

### Examples

```
setp.lt.and.s32  p|q,a,b,r;
@q  setp.eq.u32      p,i,n;
```

