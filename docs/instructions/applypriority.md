### Description

The applypriority instruction applies the cache eviction priority specified by the
.level::eviction_priority qualifier to the address range [a..a+size) in the specified cache
level.
If no state space is specified then Generic Addressing is
used. If the specified address does not fall within the address window of .global state space
then the behavior is undefined.
The operand size is an integer constant that specifies the amount of data, in bytes, in the
specified cache level on which the priority is to be applied. The only supported value for the
size operand is 128.
Supported addressing modes for operand a are described in Addresses as Operands.
a must be aligned to 128 bytes.

### Syntax

```
applypriority{.global}.level::eviction_priority  [a], size;

.level::eviction_priority = { .L2::evict_normal };
```

### Examples

```
applypriority.global.L2::evict_normal [ptr], 128;
```

