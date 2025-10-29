### Description

Compute 1/sqrt(a) and store the result in d.

### Syntax

```
rsqrt.approx{.ftz}.f32  d, a;
rsqrt.approx.f64        d, a;
```

### Semantics

```
d = 1/sqrt(a);
```

### Examples

```
rsqrt.approx.ftz.f32  isr, x;
rsqrt.approx.f64      ISR, X;
```

