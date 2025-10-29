### Description

The tensormap.replace instruction replaces the field, specified by .field qualifier,
of the tensor-map object at the location specified by the address operand addr with a
new value. The new value is specified by the argument new_val.
Qualifier .mode specifies the mode of the tensor-map object
located at the address operand addr.
Instruction type .b1024 indicates the size of the tensor-map
object, which is 1024 bits.
Operand new_val has the type .type. When .field is specified as .global_address
or .global_stride, .type must be .b64. Otherwise, .type must be .b32.
The immediate integer operand ord specifies the ordinal of the field across the rank of the
tensor which needs to be replaced in the tensor-map object.
For field .rank, the operand new_val must be ones less than the desired tensor rank as
this field uses zero-based numbering.
When .field3 is specified, the operand new_val must be an immediate and the
Table 33 shows the mapping of the operand new_val across various fields.
If no state space is specified then Generic Addressing is used.
If the address specified by addr does not fall within the address window of .global
or .shared::cta state space then the behavior is undefined.
tensormap.replace is treated as a weak memory operation, on the entire 1024-bit opaque
tensor-map object, in the Memory Consistency Model.

### Syntax

```
tensormap.replace.mode.field1{.ss}.b1024.type  [addr], new_val;
tensormap.replace.mode.field2{.ss}.b1024.type  [addr], ord, new_val;
tensormap.replace.mode.field3{.ss}.b1024.type  [addr], new_val;

.mode    = { .tile }
.field1  = { .global_address, .rank }
.field2  = { .box_dim, .global_dim, .global_stride, .element_stride  }
.field3  = { .elemtype,  .interleave_layout, .swizzle_mode, .swizzle_atomicity, .fill_mode }
.ss      = { .global, .shared::cta }
.type    = { .b32, .b64 }
```

### Examples

```
tensormap.replace.tile.global_address.shared::cta.b1024.b64   [sMem], new_val;
```

