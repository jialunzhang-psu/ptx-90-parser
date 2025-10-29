### Description

Store the minimum of a and b in d.
For .u16x2, .s16x2 instruction types, forms input vectors by half word values from source
operands. Half-word operands are then processed in parallel to produce .u16x2, .s16x2 result
in destination.
Operands d, a and b have the same type as the instruction type. For instruction types
.u16x2, .s16x2, operands d, a and b have type .b32.

### Syntax

```
min.atype         d, a, b;
min{.relu}.btype  d, a, b;

.atype = { .u16, .u32, .u64,
           .u16x2, .s16, .s64 };
.btype = { .s16x2, .s32 };
```

### Semantics

```
if (type == u16x2 || type == s16x2) {
    iA[0] = a[0:15];
    iA[1] = a[16:31];
    iB[0] = b[0:15];
    iB[1] = b[16:31];
    for (i = 0; i < 2; i++) {
         d[i] = (iA[i] < iB[i]) ? iA[i] : iB[i];
    }
} else {
    d = (a < b) ? a : b; // Integer (signed and unsigned)
}
```

### Examples

```
min.s32  r0,a,b;
@p  min.u16  h,i,j;
    min.s16x2.relu u,v,w;
```

