### Description

Shift the 64-bit value formed by concatenating operands a and b left or right by the amount
specified by the unsigned 32-bit value in c. Operand b holds bits 63:32 and operand a
holds bits 31:0 of the 64-bit source value. The source is shifted left or right by the clamped
or wrapped value in c. For shf.l, the most-significant 32-bits of the result are written
into d; for shf.r, the least-significant 32-bits of the result are written into d.

### Syntax

```
shf.l.mode.b32  d, a, b, c;  // left shift
shf.r.mode.b32  d, a, b, c;  // right shift

.mode = { .clamp, .wrap };
```

### Semantics

```
u32  n = (.mode == .clamp) ? min(c, 32) : c & 0x1f;
switch (shf.dir) {  // shift concatenation of [b, a]
    case shf.l:     // extract 32 msbs
           u32  d = (b << n)      | (a >> (32-n));
    case shf.r:     // extract 32 lsbs
           u32  d = (b << (32-n)) | (a >> n);
}
```

