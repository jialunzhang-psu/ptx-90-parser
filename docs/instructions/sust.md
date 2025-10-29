### Description

sust.{1d,2d,3d}
Store to surface memory using a surface coordinate vector. The instruction stores data from operand
c to the surface named by operand a at coordinates given by operand b. Operand a is
a .surfref variable or .u64 register. Operand b is a scalar or singleton tuple for 1d
surfaces; is a two-element vector for 2d surfaces; and is a four-element vector for 3d surfaces,
where the fourth element is ignored. Coordinate elements are of type .s32.
sust.b performs an unformatted store of binary data. The lowest dimension coordinate represents
a byte offset into the surface and is not scaled. The size of the data transfer matches the size of
source operand c.
sust.p performs a formatted store of a vector of 32-bit data values to a surface sample. The
source vector elements are interpreted left-to-right as R, G, B, and A surface
components. These elements are written to the corresponding surface sample components. Source
elements that do not occur in the surface sample are ignored. Surface sample components that do not
occur in the source vector will be written with an unpredictable value. The lowest dimension
coordinate represents a sample offset rather than a byte offset.
The source data interpretation is based on the surface sample format as follows: If the surface
format contains UNORM, SNORM, or FLOAT data, then .f32 is assumed; if the surface
format contains UINT data, then .u32 is assumed; if the surface format contains SINT
data, then .s32 is assumed. The source data is then converted from this type to the surface
sample format.
sust.b.{a1d,a2d}
Surface layer selection, followed by an unformatted store to the selected surface. The instruction
first selects a surface layer from the surface array named by operand a using the index given by
the first element of the array coordinate vector b. The instruction then stores the data in
operand c to the selected surface at coordinates given by the remaining elements of operand
b. Operand a is a .surfref variable or .u64 register. Operand b is a bit-size type
vector or tuple containing an index into the array of surfaces followed by coordinates within the
selected surface, as follows:
A surface base address is assumed to be aligned to a 16 byte boundary, and the address given by the
coordinate vector must be naturally aligned to a multiple of the access size. If an address is not
properly aligned, the resulting behavior is undefined; i.e., the access may proceed by silently
masking off low-order address bits to achieve proper rounding, or the instruction may fault.
The .clamp field specifies how to handle out-of-bounds addresses:

### Syntax

```
sust.b.{1d,2d,3d}{.cop}.vec.ctype.clamp  [a, b], c;  // unformatted
sust.p.{1d,2d,3d}.vec.b32.clamp          [a, b], c;  // formatted

sust.b.{a1d,a2d}{.cop}.vec.ctype.clamp   [a, b], c;  // unformatted

.cop   = { .wb, .cg, .cs, .wt };                     // cache operation
.vec   = { none, .v2, .v4 };
.ctype = { .b8 , .b16, .b32, .b64 };
.clamp = { .trap, .clamp, .zero };
```

### Examples

```
sust.p.1d.v4.b32.trap  [surf_B, {x}], {f1,f2,f3,f4};
sust.b.3d.v2.b64.trap  [surf_A, {x,y,z,w}], {r1,r2};
sust.b.a1d.v2.b64      [surf_C, {idx,x}], {r1,r2};
sust.b.a2d.b32         [surf_D, {idx,x,y,z}], r0;  // z ignored
```

