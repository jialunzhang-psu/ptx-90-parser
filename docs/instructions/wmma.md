### Description

Collectively load a matrix across all threads in a warp from the location indicated by address
operand p in the specified state space into destination register r.
If no state space is given, perform the memory accesses using
Generic Addressing. wmma.load operation may be used only with .global and
.shared spaces and with generic addressing, where the address points to .global or
.shared space.
The mutually exclusive qualifiers .a, .b and .c indicate whether matrix A, B or C is
being loaded respectively for the wmma computation.
The destination operand r is a brace-enclosed vector expression that can hold the fragment
returned by the load operation, as described in Matrix Fragments for WMMA.
The .shape qualifier indicates the dimensions of all the matrix arguments involved in the
intended wmma computation.
The .layout qualifier indicates whether the matrix to be loaded is stored in row-major or
column-major format.
stride is an optional 32-bit integer operand that provides an offset in terms of matrix elements
between the start of consecutive instances of the leading dimension (rows or columns). The default
value of stride is described in
Matrix Storage for WMMA and must be specified if the actual value is larger than
the default. For example, if the matrix is a sub-matrix of a larger matrix, then the value of stride
is the leading dimension of the larger matrix. Specifying a value lower than the default value
results in undefined behavior.
The required alignment for address p and stride is described in the
Matrix Storage for WMMA.
The mandatory .sync qualifier indicates that wmma.load causes the executing thread to wait
until all threads in the warp execute the same wmma.load instruction before resuming execution.
The mandatory .aligned qualifier indicates that all threads in the warp must execute the same
wmma.load instruction. In conditionally executed code, a wmma.load instruction should only
be used if it is known that all threads in the warp evaluate the condition identically, otherwise
behavior is undefined.
The behavior of wmma.load is undefined if all threads do not use the same qualifiers and the
same values of p and stride, or if any thread in the warp has exited.
wmma.load is treated as a weak memory operation in the Memory Consistency Model.

### Syntax

Floating point format .f16 loads:
```
wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride};
wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride};
wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride};

.layout = {.row, .col};
.shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
.ss     = {.global, .shared{::cta}};
.atype  = {.f16, .s8, .u8};
.btype  = {.f16, .s8, .u8};
.ctype  = {.f16, .f32, .s32};
```
Alternate floating point format .bf16 loads:
```
wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride}
wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride}
wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride}
.layout = {.row, .col};
.shape  = {.m16n16k16, .m8n32k16, .m32n8k16};
.ss     = {.global, .shared{::cta}};
.atype  = {.bf16 };
.btype  = {.bf16 };
.ctype  = {.f32 };
```
Alternate floating point format .tf32 loads:
```
wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride}
wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride}
wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride}
.layout = {.row, .col};
.shape  = {.m16n16k8 };
.ss     = {.global, .shared{::cta}};
.atype  = {.tf32 };
.btype  = {.tf32 };
.ctype  = {.f32 };
```
Double precision Floating point .f64 loads:
```
wmma.load.a.sync.aligned.layout.shape{.ss}.atype r, [p] {, stride}
wmma.load.b.sync.aligned.layout.shape{.ss}.btype r, [p] {, stride}
wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride}
.layout = {.row, .col};
.shape  = {.m8n8k4 };
.ss     = {.global, .shared{::cta}};
.atype  = {.f64 };
.btype  = {.f64 };
.ctype  = {.f64 };
```
Sub-byte loads:
```
wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride}
wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride}
wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride}
.layout = {.row, .col};
.shape  = {.m8n8k32};
.ss     = {.global, .shared{::cta}};
.atype  = {.s4, .u4};
.btype  = {.s4, .u4};
.ctype  = {.s32};
```
Single-bit loads:
```
wmma.load.a.sync.aligned.row.shape{.ss}.atype r, [p] {, stride}
wmma.load.b.sync.aligned.col.shape{.ss}.btype r, [p] {, stride}
wmma.load.c.sync.aligned.layout.shape{.ss}.ctype r, [p] {, stride}
.layout = {.row, .col};
.shape  = {.m8n8k128};
.ss     = {.global, .shared{::cta}};
.atype  = {.b1};
.btype  = {.b1};
.ctype  = {.s32};
```

### Examples

```
// Load elements from f16 row-major matrix B
.reg .b32 x<8>;

wmma.load.b.sync.aligned.m16n16k16.row.f16 {x0,x1,x2,x3,x4,x5,x,x7}, [ptr];
// Now use {x0, ..., x7} for the actual wmma.mma

// Load elements from f32 column-major matrix C and scale the values:
.reg .b32 x<8>;

wmma.load.c.sync.aligned.m16n16k16.col.f32
                 {x0,x1,x2,x3,x4,x5,x6,x7}, [ptr];

mul.f32 x0, x0, 0.1;
// repeat for all registers x<8>;
...
mul.f32 x7, x7, 0.1;
// Now use {x0, ..., x7} for the actual wmma.mma

// Load elements from integer matrix A:
.reg .b32 x<4>
// destination registers x<4> contain four packed .u8 values each
wmma.load.a.sync.aligned.m32n8k16.row.u8 {x0,x1,x2,x3}, [ptr];

// Load elements from sub-byte integer matrix A:
.reg .b32 x0;
// destination register x0 contains eight packed .s4 values
wmma.load.a.sync.aligned.m8n8k32.row.s4 {x0}, [ptr];

// Load elements from .bf16 matrix A:
.reg .b32 x<4>;
wmma.load.a.sync.aligned.m16n16k16.row.bf16
                {x0,x1,x2,x3}, [ptr];

// Load elements from .tf32 matrix A:
.reg .b32 x<4>;
wmma.load.a.sync.aligned.m16n16k8.row.tf32
                {x0,x1,x2,x3}, [ptr];

// Load elements from .f64 matrix A:
.reg .b32 x<4>;
wmma.load.a.sync.aligned.m8n8k4.row.f64
                {x0}, [ptr];
```

