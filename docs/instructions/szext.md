### Description

Sign-extends or zero-extends an N-bit value from operand a where N is specified in operand
b. The resulting value is stored in the destination operand d.
For the .s32 instruction type, the value in a is treated as an N-bit signed value and the
most significant bit of this N-bit value is replicated up to bit 31. For the .u32 instruction
type, the value in a is treated as an N-bit unsigned number and is zero-extended to 32
bits. Operand b is an unsigned 32-bit value.
If the value of N is 0, then the result of szext is 0. If the value of N is 32 or higher, then
the result of szext depends upon the value of the .mode qualifier as follows:

### Syntax

```
szext.mode.type  d, a, b;

.mode = { .clamp, .wrap };
.type = { .u32, .s32 };
```

### Semantics

```
b1        = b & 0x1f;
too_large = (b >= 32 && .mode == .clamp) ? true : false;
mask      = too_large ? 0 : (~0) << b1;
sign_pos  = (b1 - 1) & 0x1f;

if (b1 == 0 || too_large || .type != .s32) {
    sign_bit = false;
} else {
    sign_bit = (a >> sign_pos) & 1;
}
d = (a & ~mask) | (sign_bit ? mask | 0);
```

### Examples

```
szext.clamp.s32 rd, ra, rb;
szext.wrap.u32  rd, 0xffffffff, 0; // Result is 0.
```

