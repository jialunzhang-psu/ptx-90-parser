### Description

Store the value of operand b in the location specified by the destination address
operand a in specified state space. If no state space is given, perform the store using
Generic Addressing. Stores to const memory are illegal.
If no sub-qualifier is specified with .shared state space, then ::cta is assumed by default.
Supported addressing modes for operand a and alignment requirements are described in
Addresses as Operands.
If .param is specified without any sub-qualifiers then it defaults to .param::func.
Instruction st.param{::func} used for passing arguments to device function cannot be predicated.
See Parameter State Space and Function Declarations and Definitions
for descriptions of the proper use
of st.param.
The qualifiers .relaxed and .release indicate memory synchronization as described in the
Memory Consistency Model. The .scope qualifier
indicates the set of threads with which an st.relaxed or st.release instruction can directly
synchronize1. The .weak qualifier indicates a memory instruction with no synchronization.
The effects of this instruction become visible to other threads only when synchronization is established
by other means.
The semantic details of .mmio qualifier are described in the Memory Consistency Model.
Only .sys thread scope is valid for st.mmio operation. The
qualifiers .mmio and .relaxed must be specified together.
The semantic details of .volatile qualifier are described in the
Memory Consistency Model.
The .weak, .volatile, .relaxed and .release qualifiers are mutually exclusive. When
none of these is specified, the .weak qualifier is assumed by default.
The qualifiers .volatile, .relaxed and .release may be used only with .global and
.shared spaces and with generic addressing, where the address points to .global or
.shared space. Cache operations are not permitted with these qualifiers. The qualifier .mmio
may be used only with .global space and with generic addressing, where the address points to
.global space.
The .v8 (.vec) qualifier is supported if:
The .v4 (.vec) qualifier with type .b64 or .s64 or .u64 or .f64 is supported if:
Qualifiers .level1::eviction_priority and .level2::eviction_priority specify the eviction policy
for L1 and L2 cache respectively which may be applied during memory access.
Qualifier .level2::eviction_priority is supported if:
Optionally, sink symbol ‘_’ can be used in vector expression b when:
which indicates that no data is being written at the corresponding destination address.
When the optional argument cache-policy is specified, the qualifier .level::cache_hint is
required. The 64-bit operand cache-policy specifies the cache eviction policy that may be used
during the memory access.
The qualifier .level::cache_hint is only supported for .global state space and for generic
addressing where the address points to the .global state space.
cache-policy is a hint to the cache subsystem and may not always be respected. It is treated as
a performance hint only, and does not change the memory consistency behavior of the program.
1 This synchronization is further extended to other threads through the transitive nature of
causality order, as described in the memory consistency model.

### Syntax

```
st{.weak}{.ss}{.cop}{.level::cache_hint}{.vec}.type   [a], b{, cache-policy};
st{.weak}{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type
                                                      [a], b{, cache-policy};
st.volatile{.ss}{.vec}.type                           [a], b;
st.relaxed.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type
                                                      [a], b{, cache-policy};
st.release.scope{.ss}{.level1::eviction_priority}{.level2::eviction_priority}{.level::cache_hint}{.vec}.type
                                                      [a], b{, cache-policy};
st.mmio.relaxed.sys{.global}.type         [a], b;

.ss =                       { .global, .local, .param{::func}, .shared{::cta, ::cluster} };
.level1::eviction_priority = { .L1::evict_normal, .L1::evict_unchanged,
                               .L1::evict_first, .L1::evict_last, .L1::no_allocate };
.level2::eviction_priority = { .L2::evict_normal, .L2::evict_first, .L2::evict_last };
.level::cache_hint =        { .L2::cache_hint };
.cop =                      { .wb, .cg, .cs, .wt };
.sem =                      { .relaxed, .release };
.scope =                    { .cta, .cluster, .gpu, .sys };
.vec =                      { .v2, .v4, .v8 };
.type =                     { .b8, .b16, .b32, .b64, .b128,
                              .u8, .u16, .u32, .u64,
                              .s8, .s16, .s32, .s64,
                              .f32, .f64 };
```

### Semantics

```
d = a;                // named variable d
*(&a+immOffset) = b;            // variable-plus-offset
*a = b;               // register
*(a+immOffset) = b;   // register-plus-offset
*(immAddr) = b;       // immediate address
```

### Examples

```
st.global.f32    [a],b;
st.local.b32     [q+4],a;
st.global.v4.s32 [p],Q;
st.local.b32     [q+-8],a; // negative offset
st.local.s32     [100],r7; // immediate address

cvt.f16.f32      %r,%r;    // %r is 32-bit register
st.b16           [fs],%r;  // store lower
st.global.relaxed.sys.u32 [gbl], %r0;
st.shared.release.cta.u32 [sh], %r1;
st.global.relaxed.cluster.u32 [gbl], %r2;
st.shared::cta.release.cta.u32 [sh + 4], %r1;
st.shared::cluster.u32 [sh + 8], %r1;
st.global.mmio.relaxed.sys.u32 [gbl], %r1;

st.global.L1::no_allocate.f32 [p], a;

createpolicy.fractional.L2::evict_last.b64 cache-policy, 0.25;
st.global.L2::cache_hint.b32  [a], b, cache-policy;

st.param::func.b64 [param1], %rp1;

st.global.b128  [a], b;  // 128-bit store

// 256-bit store
st.global.L2::evict_last.v8.f32 [addr], { %reg0, _, %reg2, %reg3, %reg4, %reg5, %reg6, %reg7};
```

