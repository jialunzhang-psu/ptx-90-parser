### Description

Invert the bits in a.

### Syntax

```
not.type d, a;

.type = { .pred, .b16, .b32, .b64 };
```

### Semantics

```
d = ~a;
```

### Examples

```
not.b32  mask,mask;
not.pred  p,q;
```

