### Description

Convert between different types and sizes.
For .f16x2 and .bf16x2 instruction type, two inputs a and b of .f32 type are
converted into .f16 or .bf16 type and the converted values are packed in the destination
register d, such that the value converted from input a is stored in the upper half of d
and the value converted from input b is stored in the lower half of d
For .f16x2 instruction type, destination operand d has .f16x2 or .b32 type. For
.bf16 instruction type, operand d has .b16 type. For .bf16x2 instruction type,
operand d has .b32 type. For .tf32 instruction type, operand d has .b32 type.
When converting to .e4m3x2/.e5m2x2 data formats, the destination operand d has .b16
type. When converting two .f32 inputs to .e4m3x2/.e5m2x2, each input is converted to the
specified format, and the converted values are packed in the destination operand d such that the
value converted from input a is stored in the upper 8 bits of d and the value converted from
input b is stored in the lower 8 bits of d. When converting an .f16x2 input to
.e4m3x2/ .e5m2x2, each .f16 input from operand a is converted to the specified
format. The converted values are packed in the destination operand d such that the value
converted from the upper 16 bits of input a is stored in the upper 8 bits of d and the value
converted from the lower 16 bits of input a is stored in the lower 8 bits of d.
When converting from .e4m3x2/.e5m2x2 to .f16x2, source operand a has .b16
type. Each 8-bit input value in operand a is converted to .f16 type. The converted values
are packed in the destination operand d such that the value converted from the upper 8 bits of
a is stored in the upper 16 bits of d and the value converted from the lower 8 bits of a
is stored in the lower 16 bits of d.
When converting to .e2m1x2 data formats, the destination operand d has .b8 type.
When converting two .f32 inputs to .e2m1x2, each input is converted to the specified format,
and the converted values are packed in the destination operand d such that the value converted
from input a is stored in the upper 4 bits of d and the value converted from input b is
stored in the lower 4 bits of d.
When converting from .e2m1x2 to .f16x2, source operand a has .b8 type. Each 4-bit
input value in operand a is converted to .f16 type. The converted values are packed in the
destination operand d such that the value converted from the upper 4 bits of a is stored in
the upper 16 bits of d and the value converted from the lower 4 bits of a is stored in the
lower 16 bits of d.
When converting to .e2m1x4 data format, the destination operand d has .b16 type. When
converting four .f32 inputs to .e2m1x4, each input is converted to the specified format,
and the converted values are packed in the destination operand d such that the value converted
from inputs a, b, e, f are stored in each 4 bits starting from upper bits of d.
When converting to .e2m3x2/.e3m2x2 data formats, the destination operand d has .b16
type. When converting two .f32 inputs to .e2m3x2/.e3m2x2, each input is converted to the
specified format, and the converted values are packed in the destination operand d such that the
value converted from input a is stored in the upper 8 bits of d with 2 MSB bits padded with
zeros and the value converted from input b is stored in the lower 8 bits of d with 2 MSB bits
padded with zeros.
When converting from .e2m3x2/.e3m2x2 to .f16x2, source operand a has .b16 type.
Each 8-bit input value with 2 MSB bits 0 in operand a is converted to .f16 type. The converted
values are packed in the destination operand d such that the value converted from the upper 8 bits
of a is stored in the upper 16 bits of d and the value converted from the lower 8 bits of a
is stored in the lower 16 bits of d.
When converting to .e5m2x4/.e4m3x4/.e3m2x4/.e2m3x4 data format, the destination
operand d has .b32 type. When converting four .f32 inputs to
.e5m2x4/.e4m3x4/.e3m2x4/.e2m3x4, each input is converted to the specified format,
and the converted values are packed in the destination operand d such that the value converted
from inputs a, b, e, f are stored in each 8 bits starting from upper bits of d.
For .e3m2x4/.e2m3x4, each 8-bit output will have 2 MSB bits padded with zeros.
When converting to .ue8m0x2 data formats, the destination operand d has .b16 type. When
converting two .f32 or two packed .bf16 inputs to .ue8m0x2, each input is converted to the
specified format, and the converted values are packed in the destination operand d such that the
value converted from input a is stored in the upper 8 bits of d and the value converted from
input b is stored in the lower 8 bits of d.
When converting from .ue8m0x2 to .bf16x2, source operand a has .b16 type. Each 8-bit
input value in operand a is converted to .bf16 type. The converted values are packed in the
destination operand d such that the value converted from the upper 8 bits of a is stored in
the upper 16 bits of d and the value converted from the lower 8 bits of a is stored in the
lower 16 bits of d.
rbits is a .b32 type register operand used for providing random bits for .rs rounding mode.
When converting to .f16x2, two 16-bit values are provided from rbits where 13 LSBs from
upper 16-bits are used as random bits for operand a with 3 MSBs are 0 and 13 LSBs from lower
16-bits are used as random bits for operand b with 3 MSBs are 0.
When converting to .bf16x2, two 16-bit values are provided from rbits where upper 16-bits
are used as random bits for operand a and lower 16-bits are used as random bits for operand b.
When converting to .e4m3x4/.e5m2x4/.e2m3x4/.e3m2x4, two 16-bit values are provided
from rbits where lower 16-bits are used for operands e, f and upper 16 bits are used
for operands a, b.
When converting to .e2m1x4, two 16-bit values are provided from rbits where lower 8-bits
from both 16-bits half of rbits are used for operands e, f and upper 8-bits from both
16-bits half of rbits are used for operands a, b.
Rounding modifier is mandatory in all of the following cases:
.satfinite modifier is only supported for conversions involving the following types:

### Syntax

```
cvt{.irnd}{.ftz}{.sat}.dtype.atype         d, a;  // integer rounding
cvt{.frnd}{.ftz}{.sat}.dtype.atype         d, a;  // fp rounding

cvt.frnd2{.relu}{.satfinite}.f16.f32       d, a;
cvt.frnd2{.relu}{.satfinite}.f16x2.f32     d, a, b;
cvt.rs{.relu}{.satfinite}.f16x2.f32        d, a, b, rbits;

cvt.frnd2{.relu}{.satfinite}.bf16.f32      d, a;
cvt.frnd2{.relu}{.satfinite}.bf16x2.f32    d, a, b;
cvt.rs{.relu}{.satfinite}.bf16x2.f32       d, a, b, rbits;

cvt.rna{.satfinite}.tf32.f32               d, a;
cvt.frnd2{.satfinite}{.relu}.tf32.f32      d, a;

cvt.rn.satfinite{.relu}.f8x2type.f32       d, a, b;
cvt.rn.satfinite{.relu}.f8x2type.f16x2     d, a;
cvt.rn.{.relu}.f16x2.f8x2type              d, a;
cvt.rs{.relu}.satfinite.f8x4type.f32       d, {a, b, e, f}, rbits;

cvt.rn.satfinite{.relu}.f4x2type.f32       d, a, b;
cvt.rn{.relu}.f16x2.f4x2type               d, a;
cvt.rs{.relu}.satfinite.f4x4type.f32       d, {a, b, e, f}, rbits;

cvt.rn.satfinite{.relu}.f6x2type.f32       d, a, b;
cvt.rn{.relu}.f16x2.f6x2type               d, a;
cvt.rs{.relu}.satfinite.f6x4type.f32       d, {a, b, e, f}, rbits;

cvt.frnd3{.satfinite}.ue8m0x2.f32          d, a, b;
cvt.frnd3{.satfinite}.ue8m0x2.bf16x2       d, a;
cvt.rn.bf16x2.ue8m0x2                      d, a;

.irnd   = { .rni, .rzi, .rmi, .rpi };
.frnd   = { .rn,  .rz,  .rm,  .rp  };
.frnd2  = { .rn,  .rz };
.frnd2  = { .rn,  .rz };
.frnd3  = { .rz,  .rp };
.dtype = .atype = { .u8,   .u16, .u32, .u64,
                    .s8,   .s16, .s32, .s64,
                    .bf16, .f16, .f32, .f64 };
.f8x2type = { .e4m3x2, .e5m2x2 };
.f4x2type = { .e2m1x2 };
.f6x2type = { .e2m3x2, .e3m2x2 };
.f4x4type = { .e2m1x4 };
.f8x4type = { .e4m3x4, .e5m2x4 };
.f6x4type = { .e2m3x4, .e3m2x4 };
```

### Semantics

```
if (/* inst type is .f16x2 or .bf16x2 */) {
    d[31:16] = convert(a);
    d[15:0]  = convert(b);
} else if (/* inst destination type is .e5m2x2 or .e4m3x2 or .ue8m0x2 */) {
    d[15:8] = convert(a);
    d[7:0]  = convert(b);
} else if (/* inst destination type is .e2m1x2 */) {
    d[7:4] = convert(a);
    d[3:0] = convert(b);
} else if (/* inst destination type is .e2m3x2 or .e3m2x2 */) {
    d[15:14] = 0;
    d[13:8] = convert(a);
    d[7:6] = 0;
    d[5:0] = convert(b);
} else if (/* inst destination type is .e2m1x4 */) {
    d[15:12] = convert(a);
    d[11:8] = convert(b);
    d[7:4] = convert(e);
    d[3:0] = convert(f);
} else if (/* inst destination type is .e4m3x4 or .e5m2x4 */) {
    d[31:24] = convert(a);
    d[23:16] = convert(b);
    d[15:8] = convert(e);
    d[7:0] = convert(f);
} else if (/* inst destination type is .e2m3x4 or .e3m2x4 */) {
    d[31:30] = 0;
    d[29:24] = convert(a);
    d[23:22] = 0;
    d[21:16] = convert(b);
    d[15:14] = 0;
    d[13:8] = convert(e);
    d[7:6] = 0;
    d[5:0] = convert(f);
} else {
    d = convert(a);
}
```
// Random bits rbits semantics for .rs rounding:

### Examples

```
cvt.f32.s32 f,i;
cvt.s32.f64 j,r;     // float-to-int saturates by default
cvt.rni.f32.f32 x,y; // round to nearest int, result is fp
cvt.f32.f32 x,y;     // note .ftz behavior for sm_1x targets
cvt.rn.relu.f16.f32      b, f;        // result is saturated with .relu saturation mode
cvt.rz.f16x2.f32         b1, f, f1;   // convert two fp32 values to packed fp16 outputs
cvt.rn.relu.satfinite.f16x2.f32    b1, f, f1;   // convert two fp32 values to packed fp16 outputs with .relu saturation on each output
cvt.rn.bf16.f32          b, f;        // convert fp32 to bf16
cvt.rz.relu.satfinite.bf16.f3 2    b, f;        // convert fp32 to bf16 with .relu and .satfinite saturation
cvt.rz.satfinite.bf16x2.f32        b1, f, f1;   // convert two fp32 values to packed bf16 outputs
cvt.rn.relu.bf16x2.f32   b1, f, f1;   // convert two fp32 values to packed bf16 outputs with .relu saturation on each output
cvt.rna.satfinite.tf32.f32         b1, f;       // convert fp32 to tf32 format
cvt.rn.relu.tf32.f32     d, a;        // convert fp32 to tf32 format
cvt.f64.bf16.rp          f, b;        // convert bf16 to f64 format
cvt.bf16.f16.rz          b, f         // convert f16 to bf16 format
cvt.bf16.u64.rz          b, u         // convert u64 to bf16 format
cvt.s8.bf16.rpi          s, b         // convert bf16 to s8 format
cvt.bf16.bf16.rpi        b1, b2       // convert bf16 to corresponding int represented in bf16 format
cvt.rn.satfinite.e4m3x2.f32 d, a, b;  // convert a, b to .e4m3 and pack as .e4m3x2 output
cvt.rn.relu.satfinite.e5m2x2.f16x2 d, a; // unpack a and convert the values to .e5m2 outputs with .relu
                                         // saturation on each output and pack as .e5m2x2
cvt.rn.f16x2.e4m3x2 d, a;             // unpack a, convert two .e4m3 values to packed f16x2 output
cvt.rn.satfinite.tf32.f32 d, a;       // convert fp32 to tf32 format
cvt.rn.relu.f16x2.e2m1x2 d, a;        // unpack a, convert two .e2m1 values to packed f16x2 output
cvt.rn.satfinite.e2m3x2.f32 d, a, b;  // convert a, b to .e2m3 and pack as .e2m3x2 output
cvt.rn.relu.f16x2.e3m2x2 d, a;        // unpack a, convert two .e3m2 values to packed f16x2 output

cvt.rs.f16x2.f32    d, a, b, rbits;  // convert 2 fp32 values to packed fp16 with applying .rs rounding
cvt.rs.satfinite.e2m1x4.f32  d, {a, b, e, f}, rbits; // convert 4 fp32 values to packed 4 e2m1 values with applying .rs rounding
```

