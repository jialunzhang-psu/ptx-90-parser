### Description

Query an attribute of a texture or sampler. Operand a is either a .texref or .samplerref variable, or a .u64 register.
Texture attributes are queried by supplying a .texref argument to txq. In unified mode,
sampler attributes are also accessed via a .texref argument, and in independent mode sampler
attributes are accessed via a separate .samplerref argument.
txq.level
txq.level requires an additional 32bit integer argument, lod, which specifies LOD and
queries requested attribute for the specified LOD.

### Syntax

```
txq.tquery.b32         d, [a];       // texture attributes
txq.level.tlquery.b32  d, [a], lod;  // texture attributes
txq.squery.b32         d, [a];       // sampler attributes

.tquery  = { .width, .height, .depth,
             .channel_data_type, .channel_order,
             .normalized_coords, .array_size,
             .num_mipmap_levels, .num_samples};

.tlquery = { .width, .height, .depth };

.squery  = { .force_unnormalized_coords, .filter_mode,
             .addr_mode_0, addr_mode_1, addr_mode_2 };
```

### Examples

```
txq.width.b32       %r1, [tex_A];
txq.filter_mode.b32 %r1, [tex_A];   // unified mode
txq.addr_mode_0.b32 %r1, [smpl_B];  // independent mode
txq.level.width.b32 %r1, [tex_A], %r_lod;
```

