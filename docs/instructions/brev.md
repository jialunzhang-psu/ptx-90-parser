### Description

Perform bitwise reversal of input.

### Syntax

```
brev.type  d, a;

.type = { .b32, .b64 };
```

### Semantics

```
msb = (.type==.b32) ? 31 : 63;

for (i=0; i<=msb; i++) {
    d[i] = a[msb-i];
}
```

### Examples

```
brev.b32  d, a;
```

