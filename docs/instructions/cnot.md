### Description

Compute the logical negation using C/C++ semantics.

### Syntax

```
cnot.type d, a;

.type = { .b16, .b32, .b64 };
```

### Semantics

```
d = (a==0) ? 1 : 0;
```

### Examples

```
cnot.b32 d,a;
```

