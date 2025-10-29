### Description

testp tests common properties of floating-point numbers and returns a predicate value of 1
if True and 0 if False.
As a special case, positive and negative zero are considered normal numbers.

### Syntax

```
testp.op.type  p, a;  // result is .pred

.op   = { .finite, .infinite,
          .number, .notanumber,
          .normal, .subnormal };
.type = { .f32, .f64 };
```

### Examples

```
testp.notanumber.f32  isnan, f0;
testp.infinite.f64    p, X;
```

