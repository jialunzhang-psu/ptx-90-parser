### Description

Load read-only data into register variable d from the location specified by the source address
operand a in the global state space, where the address is guaranteed to be the same across all
threads in the warp. If no state space is given, perform the load using Generic Addressing.
Supported addressing modes for operand a and alignment requirements are described in
Addresses as Operands.

### Syntax

```
ldu{.ss}.type      d, [a];       // load from address
ldu{.ss}.vec.type  d, [a];       // vec load from address

.ss   = { .global };             // state space
.vec  = { .v2, .v4 };
.type = { .b8, .b16, .b32, .b64, .b128,
          .u8, .u16, .u32, .u64,
          .s8, .s16, .s32, .s64,
                     .f32, .f64 };
```

### Semantics

```
d = a;             // named variable a
d = *(&a+immOff)   // variable-plus-offset
d = *a;            // register
d = *(a+immOff);   // register-plus-offset
d = *(immAddr);    // immediate address
```

### Examples

```
ldu.global.f32    d,[a];
ldu.global.b32    d,[p+4];
ldu.global.v4.f32 Q,[p];
ldu.global.b128   d,[a];
```

