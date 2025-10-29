### Description

Load register variable d from the location specified by the source address operand a in
specified state space. If no state space is given, perform the load using Generic Addressing.
If no sub-qualifier is specified with .shared state space, then ::cta is assumed by default.
Supported addressing modes for operand a and alignment requirements are described in
Addresses as Operands
If no sub-qualifier is specified with .param state space, then:
For ld.param::entry instruction, operand a must be a kernel parameter address, otherwise behavior
is undefined. For ld.param::func instruction, operand a must be a device function parameter address,
otherwise behavior is undefined.
Instruction ld.param{::func} used for reading value returned from device function call cannot be
predicated. See Parameter State Space and
Function Declarations and Definitions for descriptions
of the proper use of ld.param.
The .relaxed and .acquire qualifiers indicate memory synchronization as described in the
Memory Consistency Model. The .scope qualifier
indicates the set of threads with which an ld.relaxed or ld.acquire instruction can directly
synchronize1. The .weak qualifier indicates a memory instruction with no synchronization.
The effects of this instruction become visible to other threads only when synchronization is established
by other means.
The semantic details of .mmio qualifier are described in the Memory Consistency Model.
Only .sys thread scope is valid for ld.mmio operation. The
qualifiers .mmio and .relaxed must be specified together.
The semantic details of .volatile qualifier are described in the Memory Consistency Model.
The .weak, .volatile, .relaxed and .acquire qualifiers are mutually exclusive. When
none of these is specified, the .weak qualifier is assumed by default.
The qualifiers .volatile, .relaxed and .acquire may be used only with .global and
.shared spaces and with generic addressing, where the address points to .global or
.shared space. Cache operations are not permitted with these qualifiers. The qualifier .mmio
may be used only with .global space and with generic addressing, where the address points to
.global space.
The optional qualifier .unified must be specified on operand a if a is the address of a
variable declared with .unified attribute as described in Variable and Function Attribute Directive: .attribute.
The .v8 (.vec) qualifier is supported if:
The .v4 (.vec) qualifier with type .b64 or .s64 or .u64 or .f64 is supported if:
Qualifiers .level1::eviction_priority and .level2::eviction_priority specify the eviction policy
for L1 and L2 cache respectively which may be applied during memory access.
Qualifier .level2::eviction_priority is supported if:
Optionally, sink symbol ‘_’ can be used in vector expression d when:
which indicates that data from corresponding memory location is not read.
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
The qualifiers .unified and .level::cache_hint are only supported for .global state
space and for generic addressing where the address points to the .global state space.
cache-policy is a hint to the cache subsystem and may not always be respected. It is treated as
a performance hint only, and does not change the memory consistency behavior of the program.
1 This synchronization is further extended to other threads through the transitive nature of
causality order, as described in the memory consistency model.

### Syntax

```
ld{.weak}{.ss}{.cop}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{.unified}{, cache-policy};

ld{.weak}{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{.unified}{, cache-policy};

ld.volatile{.ss}{.level::prefetch_size}{.vec}.type  d, [a];

ld.relaxed.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{, cache-policy};

ld.acquire.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{, cache-policy};

ld.mmio.relaxed.sys{.global}.type  d, [a];

.ss =                       { .const, .global, .local, .param{::entry, ::func}, .shared{::cta, ::cluster} };
.cop =                      { .ca, .cg, .cs, .lu, .cv };
.level1::eviction_priority = { .L1::evict_normal, .L1::evict_unchanged,
                               .L1::evict_first, .L1::evict_last, .L1::no_allocate };
.level2::eviction_priority = {.L2::evict_normal, .L2::evict_first, .L2::evict_last};
.level::cache_hint =        { .L2::cache_hint };
.level::prefetch_size =     { .L2::64B, .L2::128B, .L2::256B }
.scope =                    { .cta, .cluster, .gpu, .sys };
.vec =                      { .v2, .v4, .v8 };
.type =                     { .b8, .b16, .b32, .b64, .b128,
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
ld.global.f32    d,[a];
ld.shared.v4.b32 Q,[p];
ld.const.s32     d,[p+4];
ld.local.b32     x,[p+-8]; // negative offset
ld.local.b64     x,[240];  // immediate address

ld.global.b16    %r,[fs];  // load .f16 data into 32-bit reg
cvt.f32.f16      %r,%r;    // up-convert f16 data to f32

ld.global.b32    %r0, [fs];     // load .f16x2 data in 32-bit reg
ld.global.b32    %r1, [fs + 4]; // load .f16x2 data in 32-bit reg
add.rn.f16x2     %d0, %r0, %r1; // addition of f16x2 data
ld.global.relaxed.gpu.u32 %r0, [gbl];
ld.shared.acquire.gpu.u32 %r1, [sh];
ld.global.relaxed.cluster.u32 %r2, [gbl];
ld.shared::cta.acquire.gpu.u32 %r2, [sh + 4];
ld.shared::cluster.u32 %r3, [sh + 8];
ld.global.mmio.relaxed.sys.u32 %r3, [gbl];

ld.global.f32    d,[ugbl].unified;
ld.b32           %r0, [%r1].unified;

ld.global.L1::evict_last.u32  d, [p];

ld.global.L2::64B.b32   %r0, [gbl]; // Prefetch 64B to L2
ld.L2::128B.f64         %r1, [gbl]; // Prefetch 128B to L2
ld.global.L2::256B.f64  %r2, [gbl]; // Prefetch 256B to L2

createpolicy.fractional.L2::evict_last.L2::evict_unchanged.b64 cache-policy, 1;
ld.global.L2::cache_hint.b64  x, [p], cache-policy;
ld.param::entry.b32 %rp1, [kparam1];

ld.global.b128   %r0, [gbl];   // 128-bit load

// 256-bit load
ld.global.L2::evict_last.v8.f32 { %reg0, _, %reg2, %reg3, %reg4, %reg5, %reg6, %reg7}, [addr];
ld.global.L2::evict_last.L1::evict_last.v4.u64 { %reg0, %reg1, %reg2, %reg3}, [addr];
```

