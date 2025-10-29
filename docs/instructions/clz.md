### Description

Count the number of leading zeros in a starting with the most-significant bit and place the
result in 32-bit destination register d. Operand a has the instruction type, and destination
d has type .u32. For .b32 type, the number of leading zeros is between 0 and 32,
inclusively. For .b64 type, the number of leading zeros is between 0 and 64, inclusively.

### Syntax

```
clz.type  d, a;

.type = { .b32, .b64 };
```

### Semantics

```
.u32  d = 0;
if (.type == .b32)   { max = 32; mask = 0x80000000; }
else                 { max = 64; mask = 0x8000000000000000; }

while (d < max && (a&mask == 0) ) {
    d++;
    a = a << 1;
}
```

### Examples

```
clz.b32  d, a;
clz.b64  cnt, X;  // cnt is .u32
```

