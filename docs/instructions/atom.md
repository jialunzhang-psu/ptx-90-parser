### Description

Atomically loads the original value at location a into destination register d, performs a
reduction operation with operand b and the value in location a, and stores the result of the
specified operation at location a, overwriting the original value. Operand a specifies a
location in the specified state space. If no state space is given, perform the memory accesses using
Generic Addressing. atom with scalar type may be used only
with .global and .shared spaces and with generic addressing, where the address points to
.global or .shared space. atom with vector type may be used only with .global space
and with generic addressing where the address points to .global space.
For atom with vector type, operands d and b are brace-enclosed vector expressions, size
of which is equal to the size of vector qualifier.
If no sub-qualifier is specified with .shared state space, then ::cta is assumed by default.
The optional .sem qualifier specifies a memory synchronizing effect as described in the
Memory Consistency Model. If the .sem qualifier is absent,
.relaxed is assumed by default.
The optional .scope qualifier specifies the set of threads that can directly observe the memory
synchronizing effect of this operation, as described in the Memory Consistency Model.
If the .scope qualifier is absent, .gpu scope is
assumed by default.
For atom with vector type, the supported combinations of vector qualifier and types, and atomic
operations supported on these combinations are depicted in the following table:
Two atomic operations (atom or red) are performed atomically with respect to each other only
if each operation specifies a scope that includes the other. When this condition is not met, each
operation observes the other operation being performed as if it were split into a read followed by a
dependent write.
atom instruction on packed type or vector type, accesses adjacent scalar elements in memory. In
such cases, the atomicity is guaranteed separately for each of the individual scalar elements; the
entire atom is not guaranteed to be atomic as a single access.
For sm_6x and earlier architectures, atom operations on .shared state space do not
guarantee atomicity with respect to normal store instructions to the same address. It is the
programmer’s responsibility to guarantee correctness of programs that use shared memory atomic
instructions, e.g., by inserting barriers between normal stores and atomic operations to a common
address, or by using atom.exch to store to locations accessed by other atomic operations.
Supported addressing modes for operand a and alignment requirements are described in Addresses as Operands
The bit-size operations are .and, .or, .xor, .cas (compare-and-swap), and .exch
(exchange).
The integer operations are .add, .inc, .dec, .min, .max. The .inc and
.dec operations return a result in the range [0..b].
The floating-point operation .add operation rounds to nearest even. Current implementation of
atom.add.f32 on global memory flushes subnormal inputs and results to sign-preserving zero;
whereas atom.add.f32 on shared memory supports subnormal inputs and results and doesn’t flush
them to zero.
atom.add.f16, atom.add.f16x2, atom.add.bf16 and atom.add.bf16x2 operation requires
the .noftz qualifier; it preserves subnormal inputs and results, and does not flush them to
zero.
When the optional argument cache-policy is specified, the qualifier .level::cache_hint is
required. The 64-bit operand cache-policy specifies the cache eviction policy that may be used
during the memory access.
The qualifier .level::cache_hint is only supported for .global state space and for generic
addressing where the address points to the .global state space.
cache-policy is a hint to the cache subsystem and may not always be respected. It is treated as
a performance hint only, and does not change the memory consistency behavior of the program.

### Syntax

Atomic operation with scalar type:
```
atom{.sem}{.scope}{.space}.op{.level::cache_hint}.type d, [a], b{, cache-policy};
atom{.sem}{.scope}{.space}.op.type d, [a], b, c;

atom{.sem}{.scope}{.space}.cas.b16 d, [a], b, c;

atom{.sem}{.scope}{.space}.cas.b128 d, [a], b, c;
atom{.sem}{.scope}{.space}.exch{.level::cache_hint}.b128 d, [a], b {, cache-policy};

atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16     d, [a], b{, cache-policy};
atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.f16x2   d, [a], b{, cache-policy};

atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16    d, [a], b{, cache-policy};
atom{.sem}{.scope}{.space}.add.noftz{.level::cache_hint}.bf16x2  d, [a], b{, cache-policy};

.space =              { .global, .shared{::cta, ::cluster} };
.sem =                { .relaxed, .acquire, .release, .acq_rel };
.scope =              { .cta, .cluster, .gpu, .sys };

.op =                 { .and, .or, .xor,
                        .cas, .exch,
                        .add, .inc, .dec,
                        .min, .max };
.level::cache_hint =  { .L2::cache_hint };
.type =               { .b32, .b64, .u32, .u64, .s32, .s64, .f32, .f64 };
```
Atomic operation with vector type:
```
atom{.sem}{.scope}{.global}.add{.level::cache_hint}.vec_32_bit.f32                  d, [a], b{, cache-policy};
atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_16_bit.half_word_type  d, [a], b{, cache-policy};
atom{.sem}{.scope}{.global}.op.noftz{.level::cache_hint}.vec_32_bit.packed_type     d, [a], b{, cache-policy};

.sem =               { .relaxed, .acquire, .release, .acq_rel };
.scope =             { .cta, .cluster, .gpu, .sys };
.op =                { .add, .min, .max };
.half_word_type =    { .f16, .bf16 };
.packed_type =       { .f16x2, .bf16x2 };
.vec_16_bit =        { .v2, .v4, .v8 }
.vec_32_bit =        { .v2, .v4 };
.level::cache_hint = { .L2::cache_hint }
```

### Semantics

```
atomic {
    d = *a;
    *a = (operation == cas) ? operation(*a, b, c)
                            : operation(*a, b);
}
where
    inc(r, s)  = (r >= s) ? 0 : r+1;
    dec(r, s)  = (r==0 || r > s)  ? s : r-1;
    exch(r, s) =  s;
    cas(r,s,t) = (r == s) ? t : r;
```

### Examples

```
atom.global.add.s32  d,[a],1;
atom.shared::cta.max.u32  d,[x+4],0;
@p  atom.global.cas.b32  d,[p],my_val,my_new_val;
atom.global.sys.add.u32 d, [a], 1;
atom.global.acquire.sys.inc.u32 ans, [gbl], %r0;
atom.add.noftz.f16x2 d, [a], b;
atom.add.noftz.f16   hd, [ha], hb;
atom.global.cas.b16  hd, [ha], hb, hc;
atom.add.noftz.bf16   hd, [a], hb;
atom.add.noftz.bf16x2 bd, [b], bb;
atom.add.shared::cluster.noftz.f16   hd, [ha], hb;
atom.shared.b128.cas d, a, b, c; // 128-bit atom
atom.global.b128.exch d, a, b;   // 128-bit atom

atom.global.cluster.relaxed.add.u32 d, [a], 1;

createpolicy.fractional.L2::evict_last.b64 cache-policy, 0.25;
atom.global.add.L2::cache_hint.s32  d, [a], 1, cache-policy;

atom.global.v8.f16.max.noftz  {%hd0, %hd1, %hd2, %hd3, %hd4, %hd5, %hd6, %hd7}, [gbl],
                                              {%h0, %h1, %h2, %h3, %h4, %h5, %h6, %h7};
atom.global.v8.bf16.add.noftz  {%hd0, %hd1, %hd2, %hd3, %hd4, %hd5, %hd6, %hd7}, [gbl],
                                              {%h0, %h1, %h2, %h3, %h4, %h5, %h6, %h7};
atom.global.v2.f16.add.noftz  {%hd0, %hd1}, [gbl], {%h0, %h1};
atom.global.v2.bf16.add.noftz  {%hd0, %hd1}, [gbl], {%h0, %h1};
atom.global.v4.b16x2.min.noftz  {%hd0, %hd1, %hd2, %hd3}, [gbl], {%h0, %h1, %h2, %h3};
atom.global.v4.f32.add  {%f0, %f1, %f2, %f3}, [gbl], {%f0, %f1, %f2, %f3};
atom.global.v2.f16x2.min.noftz  {%bd0, %bd1}, [g], {%b0, %b1};
atom.global.v2.bf16x2.max.noftz  {%bd0, %bd1}, [g], {%b0, %b1};
atom.global.v2.f32.add  {%f0, %f1}, [g], {%f0, %f1};
```

