### Description

cp.async is a non-blocking instruction which initiates an asynchronous copy operation of data
from the location specified by source address operand src to the location specified by
destination address operand dst. Operand src specifies a location in the global state space
and dst specifies a location in the shared state space.
Operand cp-size is an integer constant which specifies the size of data in bytes to be copied to
the destination dst. cp-size can only be 4, 8 and 16.
Instruction cp.async allows optionally specifying a 32-bit integer operand src-size. Operand
src-size represents the size of the data in bytes to be copied from src to dst and must
be less than cp-size. In such case, remaining bytes in destination dst are filled with
zeros. Specifying src-size larger than cp-size results in undefined behavior.
The optional and non-immediate predicate argument ignore-src specifies whether the data from the
source location src should be ignored completely. If the source data is ignored then zeros will
be copied to destination dst. If the argument ignore-src is not specified then it defaults
to False.
Supported alignment requirements and addressing modes for operand src and dst are described
in Addresses as Operands.
The mandatory .async qualifier indicates that the cp instruction will initiate the memory
copy operation asynchronously and control will return to the executing thread before the copy
operation is complete. The executing thread can then use
async-group based completion mechanism
or the mbarrier based completion mechanism
to wait for completion of the asynchronous copy operation.
No other synchronization mechanism guarantees the completion of the asynchronous
copy operations.
There is no ordering guarantee between two cp.async operations if they are not explicitly
synchronized using cp.async.wait_all or cp.async.wait_group or mbarrier instructions.
As described in Cache Operators, the .cg qualifier indicates
caching of data only at global level cache L2 and not at L1 whereas .ca qualifier indicates
caching of data at all levels including L1 cache. Cache operator are treated as performance hints
only.
cp.async is treated as a weak memory operation in the Memory Consistency Model.
The .level::prefetch_size qualifier is a hint to fetch additional data of the specified size
into the respective cache level.The sub-qualifier prefetch_size can be set to either of 64B,
128B, 256B thereby allowing the prefetch size to be 64 Bytes, 128 Bytes or 256 Bytes
respectively.
The qualifier .level::prefetch_size may only be used with .global state space and with
generic addressing where the address points to .global state space. If the generic address does
not fall within the address window of the global memory, then the prefetching behavior is undefined.
The .level::prefetch_size qualifier is treated as a performance hint only.
When the optional argument cache-policy is specified, the qualifier .level::cache_hint is
required. The 64-bit operand cache-policy specifies the cache eviction policy that may be used
during the memory access.
The qualifier .level::cache_hint is only supported for .global state space and for generic
addressing where the address points to the .global state space.
cache-policy is a hint to the cache subsystem and may not always be respected. It is treated as
a performance hint only, and does not change the memory consistency behavior of the program.

### Syntax

```
cp.async.ca.shared{::cta}.global{.level::cache_hint}{.level::prefetch_size}
                         [dst], [src], cp-size{, src-size}{, cache-policy} ;
cp.async.cg.shared{::cta}.global{.level::cache_hint}{.level::prefetch_size}
                         [dst], [src], 16{, src-size}{, cache-policy} ;
cp.async.ca.shared{::cta}.global{.level::cache_hint}{.level::prefetch_size}
                         [dst], [src], cp-size{, ignore-src}{, cache-policy} ;
cp.async.cg.shared{::cta}.global{.level::cache_hint}{.level::prefetch_size}
                         [dst], [src], 16{, ignore-src}{, cache-policy} ;

.level::cache_hint =     { .L2::cache_hint }
.level::prefetch_size =  { .L2::64B, .L2::128B, .L2::256B }
cp-size =                { 4, 8, 16 }
```

### Examples

```
cp.async.ca.shared.global  [shrd],    [gbl + 4], 4;
cp.async.ca.shared::cta.global  [%r0 + 8], [%r1],     8;
cp.async.cg.shared.global  [%r2],     [%r3],     16;

cp.async.cg.shared.global.L2::64B   [%r2],      [%r3],     16;
cp.async.cg.shared.global.L2::128B  [%r0 + 16], [%r1],     16;
cp.async.cg.shared.global.L2::256B  [%r2 + 32], [%r3],     16;

createpolicy.fractional.L2::evict_last.L2::evict_unchanged.b64 cache-policy, 0.25;
cp.async.ca.shared.global.L2::cache_hint [%r2], [%r1], 4, cache-policy;

cp.async.ca.shared.global                   [shrd], [gbl], 4, p;
cp.async.cg.shared.global.L2::cache_hint   [%r0], [%r2], 16, q, cache-policy;
```

