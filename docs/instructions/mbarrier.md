### Description

mbarrier.init initializes the mbarrier object at the location specified by the address operand
addr with the unsigned 32-bit integer count. The value of operand count must be in the range
as specified in Contents of the mbarrier object.
Initialization of the mbarrier object involves :
The valid range of values for the operand count is [1, …, 220 - 1].
Refer Contents of the mbarrier object for the
valid range of values for the various constituents of the mbarrier.
If no state space is specified then Generic Addressing is
used. If the address specified by addr does not fall within the address window of
.shared::cta state space then the behavior is undefined.
Supported addressing modes for operand addr is as described in Addresses as Operands.
Alignment for operand addr is as described in the
Size and alignment of mbarrier object.
The behavior of performing an mbarrier.init operation on a memory location containing a
valid mbarrier object is undefined; invalidate the mbarrier object using mbarrier.inval
first, before repurposing the memory location for any other purpose, including another mbarrier object.

### Syntax

```
mbarrier.init{.shared{::cta}}.b64 [addr], count;
```

### Examples

```
.shared .b64 shMem, shMem2;
.reg    .b64 addr;
.reg    .b32 %r1;

cvta.shared.u64          addr, shMem2;
mbarrier.init.b64        [addr],   %r1;
bar.cta.sync             0;
// ... other mbarrier operations on addr

mbarrier.init.shared::cta.b64 [shMem], 12;
bar.sync                 0;
// ... other mbarrier operations on shMem
```

