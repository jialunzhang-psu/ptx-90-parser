### Description

Count the number of one bits in a and place the resulting population count in 32-bit
destination register d. Operand a has the instruction type and destination d has type
.u32.

### Syntax

```
popc.type  d, a;

.type = { .b32, .b64 };
```

### Semantics

```
.u32  d = 0;
while (a != 0) {
   if (a & 0x1)  d++;
   a = a >> 1;
}
```

### Examples

```
popc.b32  d, a;
popc.b64  cnt, X;  // cnt is .u32
```

