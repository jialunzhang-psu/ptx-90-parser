### Description

The createpolicy instruction creates a cache eviction policy for the specified cache level in an
opaque 64-bit register specified by the destination operand cache-policy. The cache eviction
policy specifies how cache eviction priorities are applied to global memory addresses used in memory
operations with .level::cache_hint qualifier.
There are two types of cache eviction policies:
The access property created using the CUDA APIs can be converted into cache eviction policy by the
instruction createpolicy.cvt. The source operand access-property is a 64-bit opaque
register. Refer to CUDA programming guide for more details.

### Syntax

```
// Range-based policy
createpolicy.range{.global}.level::primary_priority{.level::secondary_priority}.b64
                                   cache-policy, [a], primary-size, total-size;

// Fraction-based policy
createpolicy.fractional.level::primary_priority{.level::secondary_priority}.b64
                                   cache-policy{, fraction};

// Converting the access property from CUDA APIs
createpolicy.cvt.L2.b64            cache-policy, access-property;

.level::primary_priority =   { .L2::evict_last, .L2::evict_normal,
                               .L2::evict_first, .L2::evict_unchanged };
.level::secondary_priority = { .L2::evict_first, .L2::evict_unchanged };
```

### Examples

```
createpolicy.fractional.L2::evict_last.b64                      policy, 1.0;
createpolicy.fractional.L2::evict_last.L2::evict_unchanged.b64  policy, 0.5;

createpolicy.range.L2::evict_last.L2::evict_first.b64
                                            policy, [ptr], 0x100000, 0x200000;

// access-prop is created by CUDA APIs.
createpolicy.cvt.L2.b64 policy, access-prop;
```

