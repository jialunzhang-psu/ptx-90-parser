### Description

Four-way byte dot product which is accumulated in 32-bit result.
Operand a and b are 32-bit inputs which hold 4 byte inputs in packed form for dot product.
Operand c has type .u32 if both .atype and .btype are .u32 else operand c
has type .s32.

### Syntax

```
dp4a.atype.btype  d, a, b, c;

.atype = .btype = { .u32, .s32 };
```

### Semantics

```
d = c;

// Extract 4 bytes from a 32bit input and sign or zero extend
// based on input type.
Va = extractAndSignOrZeroExt_4(a, .atype);
Vb = extractAndSignOrZeroExt_4(b, .btype);

for (i = 0; i < 4; ++i) {
    d += Va[i] * Vb[i];
}
```

### Examples

```
dp4a.u32.u32           d0, a0, b0, c0;
dp4a.u32.s32           d1, a1, b1, c1;
```

