### Description

Write predicate register p with 1 if register a points to an opaque variable of the
specified type, and with 0 otherwise. Destination p has type .pred; the source address
operand must be of type .u64.

### Syntax

```
istypep.type   p, a;  // result is .pred

.type = { .texref, .samplerref, .surfref };
```

### Examples

```
istypep.texref istex, tptr;
istypep.samplerref issampler, sptr;
istypep.surfref issurface, surfptr;
```

