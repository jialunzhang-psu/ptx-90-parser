### Description

Perform a MxNxK matrix multiply and accumulate operation, D = A*B+C, where the A matrix is
MxK, the B matrix is KxN, and the C and D matrices are MxN.
Qualifier .block_scale specifies that the matrices A and B are scaled with scale_A and
scale_B matrices respectively before performing the matrix multiply and accumulate operation
as specified in the section Block Scaling. The data type
corresponding to each of the element within scale_A and Scale_B matrices is specified
by .stype. Qualifier .scale_vec_size specifies the number of columns of scale_A matrix
and number of rows in the matrix scale_B.
The valid combinations of .kind, .stype and .scale_vec_size are described in
Table 36. For mma with .kind::mxf4 when the
qualifier .scale_vec_size is not specified, then it defaults to 2X. In contrast, when
.kind is specified as .kind::mxf8f6f4 then the qualifier .scale_vec_size defaults
to 1X. However, for .kind::mxf4nvf4, it is mandatory to provide valid .scale_vec_size.
A warp executing mma.sync.m8n8k4 instruction computes 4 matrix multiply and accumulate
operations. Rest of the mma.sync operations compute a single matrix mutliply and accumulate
operation per warp.
For single-bit mma.sync, multiplication is replaced by a sequence of logical operations;
specifically, mma.xor.popc and mma.and.popc computes the XOR, AND respectively of a k-bit
row of A with a k-bit column of B, then counts the number of set bits in the result (popc). This
result is added to the corresponding element of C and written into D.
Operands a and b represent two multiplicand matrices A and B, while c and d
represent the accumulator and destination matrices, distributed across the threads in warp.
When .block_scale qualifier is specified, operand scale-a-data, scale-b-data represents
the scale matrix metadata corresponding to scale_A and scale_B matrices respectively. The
tuple {byte-id-a, thread-id-a} and {byte-id-b, thread-id-b} represent selectors for matrices
scale_A and scale_B respectively from their corresponding metadata arguments scale-a-data,
scale-b-data. The operands scale-a-data, scale-b-data are of type .b32. The operands
byte-id-a, thread-id-a, byte-id-b, thread-id-b are unsigned 16-bit integer values.
For more details on selector arguments refer Block Scaling section.
The registers in each thread hold a fragment of matrix as described in
Matrix multiply-accumulate operation using mma instruction.
The qualifiers .dtype, .atype, .btype and .ctype indicate the data-type of the
elements in the matrices D, A, B and C respectively. The qualifier .stype indicate the data-type
of the elements in the matrices scale_A and scale_B. Specific shapes have type restrictions :
The qualifiers .alayout and .blayout indicate the row-major or column-major layouts of
matrices A and B respectively.
When .kind is either of .kind::mxf8f6f4 or .kind::f8f6f4, the individual 4-bit and the
6-bit floating point type elements must be packed in an 8-bit container. The matrix element of type
.e2m1 resides in central 4 bits of the 8-bit container with padding in the upper 2 bits and
lower 2 bits of the container. When the matrix element is of type .e3m2 or .e2m3, the
matrix element resides in the lower 6 bits of the 8-bit container with padding in the upper 2 bits
of the container. In contrast, note that when using mma with .kind::mxf4 or
.kind::mxf4nvf4, no explicit padding is necessary even though matrix elements are of type .e2m1.
The mandatory .sync qualifier indicates that mma instruction causes the executing thread to
wait until all threads in the warp execute the same mma instruction before resuming execution.
The mandatory .aligned qualifier indicates that all threads in the warp must execute the same
mma instruction. In conditionally executed code, a mma instruction should only be used if it
is known that all threads in the warp evaluate the condition identically, otherwise behavior is
undefined.
The behavior of mma instruction is undefined if all threads in the same warp do not use the same
qualifiers, or if any thread in the warp has exited.

### Syntax

Half precision floating point type:
```
mma.sync.aligned.m8n8k4.alayout.blayout.dtype.f16.f16.ctype  d, a, b, c;
mma.sync.aligned.m16n8k8.row.col.dtype.f16.f16.ctype  d, a, b, c;
mma.sync.aligned.m16n8k16.row.col.dtype.f16.f16.ctype d, a, b, c;

.alayout = {.row, .col};
.blayout = {.row, .col};
.ctype   = {.f16, .f32};
.dtype   = {.f16, .f32};
```
Alternate floating point type:
```
mma.sync.aligned.m16n8k4.row.col.f32.tf32.tf32.f32        d, a, b, c;
mma.sync.aligned.m16n8k8.row.col.f32.atype.btype.f32      d, a, b, c;
mma.sync.aligned.m16n8k16.row.col.f32.bf16.bf16.f32       d, a, b, c;
mma.sync.aligned.shape.row.col.dtype.f8type.f8type.ctype  d, a, b, c;
mma.sync.aligned.m16n8k32.row.col.kind.dtype.f8f6f4type.f8f6f4type.ctype d, a, b, c;

.atype      = {.bf16, .tf32};
.btype      = {.bf16, .tf32};
.f8type     = {.e4m3, .e5m2};
.f8f6f4type = {.e4m3, .e5m2, .e3m2, .e2m3, .e2m1};
.ctype      = {.f16, .f32};
.dtype      = {.f16, .f32};
.shape      = {.m16n8k16, .m16n8k32};
.kind       = {.kind::f8f6f4};
```
Alternate floating point type with block scaling:
```
mma.sync.aligned.m16n8k64.row.col.kind.block_scale{.scale_vec_size}.f32.e2m1.e2m1.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};

.kind           = {.kind::mxf4};
.scale_vec_size = {.scale_vec::2X};
.stype          = {.ue8m0};

mma.sync.aligned.m16n8k64.row.col.kind.block_scale.scale_vec_size.f32.e2m1.e2m1.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};

.kind           = {.kind::mxf4nvf4};
.scale_vec_size = {.scale_vec::2X, .scale_vec::4X};
.stype          = {.ue8m0, .ue4m3};

mma.sync.aligned.m16n8k32.row.col.kind.block_scale{.scale_vec_size}.f32.f8f6f4type.f8f6f4type.f32.stype d, a, b, c, scale-a-data, {byte-id-a, thread-id-a}, scale-b-data, {byte-id-b, thread-id-b};

.kind           = {.kind::mxf8f6f4};
.scale_vec_size = {.scale_vec::1X};
.f8f6f4type     = {.e4m3, .e5m2, .e3m2, .e2m3, .e2m1};
.stype          = {.ue8m0};
```
Double precision floating point type:
```
mma.sync.aligned.shape.row.col.f64.f64.f64.f64 d, a, b, c;

.shape   = {.m8n84, .m16n8k4, .m16n8k8, .m16n8k16};
```
Integer type:
```
mma.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c;

.shape   = {.m8n8k16, .m16n8k16, .m16n8k32}
.atype   = {.u8, .s8};
.btype   = {.u8, .s8};

mma.sync.aligned.shape.row.col{.satfinite}.s32.atype.btype.s32 d, a, b, c;

.shape   = {.m8n8k32, .m16n8k32, .m16n8k64}
.atype   = {.u4, .s4};
.btype   = {.u4, .s4};
```
Single bit:
```
mma.sync.aligned.shape.row.col.s32.b1.b1.s32.bitOp.popc d, a, b, c;

.bitOp = {.xor, .and}
.shape = {.m8n8k128, .m16n8k128, .m16n8k256}
```

