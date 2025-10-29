### Description

Write the destination register d with the rank of the CTA which contains the address specified
in operand a.
Instruction type .type indicates the type of source operand a.
When space is .shared::cluster, source a is either a shared memory variable or a register
containing a valid shared memory address. When the optional qualifier .space is not specified,
a is a register containing a generic addresses pointing to shared memory. Destination d is
always a 32-bit register which holds the rank of the CTA.

### Syntax

```
getctarank{.space}.type d, a;

// Get cta rank from source shared memory address in register a.
getctarank.shared::cluster.type d, a;

// Get cta rank from shared memory variable.
getctarank.shared::cluster.type d, var;

// Get cta rank from shared memory variable+offset.
getctarank.shared::cluster.type d, var + imm;

// Get cta rank from generic address of shared memory variable in register a.
getctarank.type d, a;

.space = { .shared::cluster }
.type  = { .u32, .u64 }
```

### Examples

```
getctarank.shared::cluster.u32 d1, addr;
getctarank.shared::cluster.u64 d2, sh + 4;
getctarank.u64                 d3, src;
```

