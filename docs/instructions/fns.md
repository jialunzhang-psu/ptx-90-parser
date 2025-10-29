### Description

Given a 32-bit value mask and an integer value base (between 0 and 31), find the n-th (given
by offset) set bit in mask from the base bit, and store the bit position in d. If not
found, store 0xffffffff in d.
Operand mask has a 32-bit type. Operand base has .b32, .u32 or .s32
type. Operand offset has .s32 type. Destination d has type .b32.
Operand base must be <= 31, otherwise behavior is undefined.

### Syntax

```
fns.b32 d, mask, base, offset;
```

### Semantics

```
d = 0xffffffff;
if (offset == 0) {
    if (mask[base] == 1) {
        d = base;
    }
} else {
    pos = base;
    count = |offset| - 1;
    inc = (offset > 0) ? 1 : -1;

    while ((pos >= 0) && (pos < 32)) {
        if (mask[pos] == 1) {
            if (count == 0) {
              d = pos;
              break;
           } else {
               count = count - 1;
           }
        }
        pos = pos + inc;
    }
}
```

### Examples

```
fns.b32 d, 0xaaaaaaaa, 3, 1;   // d = 3
fns.b32 d, 0xaaaaaaaa, 3, -1;  // d = 3
fns.b32 d, 0xaaaaaaaa, 2, 1;   // d = 3
fns.b32 d, 0xaaaaaaaa, 2, -1;  // d = 1
```

