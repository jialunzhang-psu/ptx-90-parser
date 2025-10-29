### Description

tcgen05.alloc is a potentially blocking instruction which dynamically allocates
the specified number of columns in the Tensor Memory and writes
the address of the allocated Tensor Memory into shared memory
at the location specified by address operand dst. The tcgen05.alloc blocks if the
requested amount of Tensor Memory is not available and unblocks
as soon as the requested amount of Tensor Memory becomes
available for allocation.
Instruction tcgen05.dealloc deallocates the Tensor Memory
specified by the Tensor Memory address taddr. The operand
taddr must point to a previous Tensor Memory allocation.
All of the Tensor Memory that was allocated using tcgen05.alloc instruction in a kernel,
must be explicitly deallocated using tcgen05.dealloc before the kernel exits.
The unsigned 32-bit operand nCols specify the number of columns to be allocated or
de-allocated. The unit of allocation and de-allocation is 32 columns and all of lanes
per column. The number of columns must be a power of 2. The operand nCols must be
within the range [32, 512]. The number of columns allocated should not increase between
any two allocations in the execution order within the CTA. Operand nCols must be
power of 2.
Instruction tcgen05.relinquish_alloc_permit specifies that the CTA of the executing
thread is relinquishing the right to allocate Tensor Memory. So,
it is illegal for a CTA to perform tcgen05.alloc after any of its constituent threads
execute tcgen05.relinquish_alloc_permit.
If no state space is specified then Generic Addressing is used.
If the address specified by dst does not fall within the address window of
.shared::cta state space then the behavior is undefined.
Qualifier .cta_group specifies the number of CTAs involved in the allocation and
de-allocation operation. When .cta_group::1 is specified, one warp from the CTA must
perform the allocation and de-allocation. When .cta_group::2 is specified, one warp
from each of the peer CTAs must collectively perform the allocation and
de-allocation. Refer to the Issue Granularity section.
When .cta_group::2 is specified, the issuing warp must make sure that peer CTA is launched
and is still active.
All tcgen05 instructions within a kernel must specify the same value for the .cta_group
qualifier.
The mandatory .sync qualifier indicates that the instruction causes the executing thread
to wait until all threads in the warp execute the same instruction before resuming execution.
The mandatory .aligned qualifier indicates that all threads in the warp must execute the
same instruction. In conditionally executed code, the instruction should only be used if it
is known that all threads in the warp evaluate the condition identically, otherwise behavior
is undefined.
The behavior of the instruction is undefined if all the threads in the warp do not use the
same values of nCols, or if any thread in the warp has exited.
The store operation in tcgen05.alloc is treated as a weak memory operation in the
Memory Consistency Model.

### Syntax

```
tcgen05.alloc.cta_group.sync.aligned{.shared::cta}.b32  [dst], nCols;

tcgen05.dealloc.cta_group.sync.aligned.b32              taddr, nCols;

tcgen05.relinquish_alloc_permit.cta_group.sync.aligned;

.cta_group = { .cta_group::1, .cta_group::2 }
```

### Examples

```
// Example 1:

tcgen05.alloc.cta_group::1.sync.aligned.shared::cta.b32 [sMemAddr1], 32;
ld.shared.b32 taddr, [sMemAddr1];
// use taddr ...
// more allocations and its usages ...
tcgen05.dealloc.cta_group::1.sync.aligned.b32  taddr, 32;
// more deallocations ...
tcgen05.relinquish_alloc_permit.cta_group::1.sync.aligned;

// Example 2:

// Following instructions are performed by current warp and the warp in the peer-CTA:
tcgen05.alloc.cta_group::2.sync.aligned.shared::cta.b32 [sMemAddr2], 32;
ld.shared.b32 taddr, [sMemAddr2];
// use taddr ...
// more allocations and its usages ...
tcgen05.dealloc.cta_group::2.sync.aligned.b32  taddr, 32;
// more deallocations ...
tcgen05.relinquish_alloc_permit.cta_group::2.sync.aligned;
```

