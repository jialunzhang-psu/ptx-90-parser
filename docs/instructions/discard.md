### Description

Semantically, this behaves like a weak write of an unstable indeterminate value:
reads of memory locations with unstable indeterminate values may return different
bit patterns each time until the memory is overwritten.
This operation hints to the implementation that data in the specified cache .level
can be destructively discarded without writing it back to memory.
The operand size is an integer constant that specifies the length in bytes of the
address range [a, a + size) to write unstable indeterminate values into.
The only supported value for the size operand is 128.
If no state space is specified then Generic Addressing is used.
If the specified address does not fall within the address window of .global state space
then the behavior is undefined.
Supported addressing modes for address operand a are described in Addresses as Operands.
a must be aligned to 128 bytes.

### Syntax

```
discard{.global}.level  [a], size;

.level = { .L2 };
```

### Examples

```
discard.global.L2 [ptr], 128;
ld.weak.u32 r0, [ptr];
ld.weak.u32 r1, [ptr];
// The values in r0 and r1 may differ!
```

