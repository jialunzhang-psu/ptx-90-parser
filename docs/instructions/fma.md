### Description

Performs a fused multiply-add with no loss of precision in the intermediate product and addition.
For .f32x2 instruction type, forms input vectors of single precision (.f32) values from
source operands. Single precision (.f32) operands are then operated in parallel to produce
.f32x2 result in destination.
For .f32x2 instruction type, operands d, a, b and c have .b64 type.

### Syntax

```
fma.rnd{.ftz}{.sat}.f32  d, a, b, c;
fma.rnd{.ftz}.f32x2      d, a, b, c;
fma.rnd.f64              d, a, b, c;

.rnd = { .rn, .rz, .rm, .rp };
```

### Semantics

```
if (type == f32 || type == f64) {
    d = a * b + c;
} else if (type == f32x2) {
    fA[0] = a[0:31];
    fA[1] = a[32:63];
    fB[0] = b[0:31];
    fB[1] = b[32:63];
    fC[0] = c[0:31];
    fC[1] = c[32:63];
    for (i = 0; i < 2; i++) {
        d[i] = fA[i] * fB[i] + fC[i];
    }
}
```

### Examples

```
fma.rn.ftz.f32  w,x,y,z;
@p  fma.rn.f64      d,a,b,c;
    fma.rp.ftz.f32x2 p,q,r,s;
```

