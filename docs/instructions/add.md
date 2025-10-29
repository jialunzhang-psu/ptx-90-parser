### Description

Performs addition and writes the resulting value into a destination register.
For .u16x2, .s16x2 instruction types, forms input vectors by half word values from source
operands. Half-word operands are then added in parallel to produce .u16x2, .s16x2 result in
destination.
Operands d, a and b have type .type. For instruction types .u16x2, .s16x2,
operands d, a and b have type .b32.

### Syntax

```
add.type       d, a, b;
add{.sat}.s32  d, a, b;     // .sat applies only to .s32

.type = { .u16, .u32, .u64,
          .s16, .s32, .s64,
          .u16x2, .s16x2 };
```

### Semantics

```
if (type == u16x2 || type == s16x2) {
    iA[0] = a[0:15];
    iA[1] = a[16:31];
    iB[0] = b[0:15];
    iB[1] = b[16:31];
    for (i = 0; i < 2; i++) {
         d[i] = iA[i] + iB[i];
    }
} else {
    d = a + b;
}
```

### Examples

```
@p  add.u32     x,y,z;
    add.sat.s32 c,c,1;
    add.u16x2   u,v,w;
```

