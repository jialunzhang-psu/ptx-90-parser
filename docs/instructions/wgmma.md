### Description

Instruction wgmma.mma_async issues a MxNxK matrix multiply and accumulate operation, D =
A*B+D, where the A matrix is MxK, the B matrix is KxN, and the D matrix is MxN.
The operation of the form D = A*B is issued when the input predicate argument scale-d is
false.
wgmma.fence instruction must be used to fence the register accesses of wgmma.mma_async
instruction from their prior accesses. Otherwise, the behavior is undefined.
wgmma.commit_group and wgmma.wait_group operations must be used to wait for the completion
of the asynchronous matrix multiply and accumulate operations before the results are accessed.
Register operand d represents the accumulator matrix as well as the destination matrix,
distributed across the participating threads. Register operand a represents the multiplicand
matrix A in register distributed across the participating threads. The 64-bit register operands
a-desc and b-desc are the matrix descriptors which represent the multiplicand matrices A and
B in shared memory respectively. The contents of a matrix descriptor must be same across all the warps
in the warpgroup. The format of the matrix descriptor is described in
Matrix Descriptor Format.
Matrices A and B are stored in row-major and column-major format respectively. For certain floating
point variants, the input matrices A and B can be transposed by specifying the value 1 for the
immediate integer arguments imm-trans-a and imm-trans-b respectively. A value of 0 can be
used to avoid the transpose operation. The valid values of imm-trans-a and imm-trans-b are 0
and 1. The transpose operation is only supported for the wgmma.mma_async variants with .f16/
.bf16 types on matrices accessed from shared memory using matrix descriptors.
For the floating point variants of the wgmma.mma_async operation, each element of the input
matrices A and B can be negated by specifying the value -1 for operands imm-scale-a and
imm-scale-b respectively. A value of 1 can be used to avoid the negate operation. The valid
values of imm-scale-a and imm-scale-b are -1 and 1.
The qualifiers .dtype, .atype and .btype indicate the data type of the elements in
matrices D, A and B respectively. .atype and .btype must be the same for all floating point
wgmma.mma_async variants except for the FP8 floating point variants. The sizes of individual
data elements of matrices A and B in alternate floating point variants of the wgmma.mma_async
operation are as follows:
Precision and rounding:
The mandatory .sync qualifier indicates that wgmma.mma_async instruction causes the
executing thread to wait until all threads in the warp execute the same wgmma.mma_async
instruction before resuming execution.
The mandatory .aligned qualifier indicates that all threads in the warpgroup must execute the
same wgmma.mma_async instruction. In conditionally executed code, a wgmma.mma_async
instruction should only be used if it is known that all threads in the warpgroup evaluate the
condition identically, otherwise behavior is undefined.

### Syntax

Half precision floating point type:
```
wgmma.mma_async.sync.aligned.shape.dtype.f16.f16  d, a-desc, b-desc, scale-d, imm-scale-a, imm-scale-b, imm-trans-a, imm-trans-b;

wgmma.mma_async.sync.aligned.shape.dtype.f16.f16  d, a, b-desc, scale-d, imm-scale-a, imm-scale-b, imm-trans-b;

.shape   = {.m64n8k16, .m64n16k16, .m64n24k16, .m64n32k16,
            .m64n40k16, .m64n48k16, .m64n56k16, .m64n64k16,
            .m64n72k16, .m64n80k16, .m64n88k16, .m64n96k16,
            .m64n104k16, .m64n112k16, .m64n120k16, .m64n128k16,
            .m64n136k16, .m64n144k16, .m64n152k16, .m64n160k16,
            .m64n168k16, .m64n176k16, .m64n184k16, .m64n192k16,
            .m64n200k16, .m64n208k16, .m64n216k16, .m64n224k16,
            .m64n232k16, .m64n240k16, .m64n248k16, .m64n256k16};
.dtype   = {.f16, .f32};
```
Alternate floating point type :
```
.bf16 floating point type:

wgmma.mma_async.sync.aligned.shape.dtype.bf16.bf16  d, a-desc, b-desc, scale-d, imm-scale-a, imm-scale-b, imm-trans-a, imm-trans-b;

wgmma.mma_async.sync.aligned.shape.dtype.bf16.bf16  d, a, b-desc, scale-d, imm-scale-a, imm-scale-b, imm-trans-b;

.shape   = {.m64n8k16, .m64n16k16, .m64n24k16, .m64n32k16,
            .m64n40k16, .m64n48k16, .m64n56k16, .m64n64k16,
            .m64n72k16, .m64n80k16, .m64n88k16, .m64n96k16,
            .m64n104k16, .m64n112k16, .m64n120k16, .m64n128k16,
            .m64n136k16, .m64n144k16, .m64n152k16, .m64n160k16,
            .m64n168k16, .m64n176k16, .m64n184k16, .m64n192k16,
            .m64n200k16, .m64n208k16, .m64n216k16, .m64n224k16,
            .m64n232k16, .m64n240k16, .m64n248k16, .m64n256k16};
.dtype  = {.f32};

.tf32 floating point type:

wgmma.mma_async.sync.aligned.shape.dtype.tf32.tf32  d, a-desc, b-desc, scale-d, imm-scale-a, imm-scale-b;

wgmma.mma_async.sync.aligned.shape.dtype.tf32.tf32  d, a, b-desc, scale-d, imm-scale-a, imm-scale-b;

.shape   = {.m64n8k8, .m64n16k8, .m64n24k8, .m64n32k8,
            .m64n40k8, .m64n48k8, .m64n56k8, .m64n64k8,
            .m64n72k8, .m64n80k8, .m64n88k8, .m64n96k8,
            .m64n104k8, .m64n112k8, .m64n120k8, .m64n128k8,
            .m64n136k8, .m64n144k8, .m64n152k8, .m64n160k8,
            .m64n168k8, .m64n176k8, .m64n184k8, .m64n192k8,
            .m64n200k8, .m64n208k8, .m64n216k8, .m64n224k8,
            .m64n232k8, .m64n240k8, .m64n248k8, .m64n256k8};
.dtype  = {.f32};

FP8 floating point type

wgmma.mma_async.sync.aligned.shape.dtype.atype.btype  d, a-desc, b-desc, scale-d, imm-scale-a, imm-scale-b;

wgmma.mma_async.sync.aligned.shape.dtype.atype.btype  d, a, b-desc, scale-d, imm-scale-a, imm-scale-b;

.shape   = {.m64n8k32, .m64n16k32, .m64n24k32, .m64n32k32,
            .m64n40k32, .m64n48k32, .m64n56k32, .m64n64k32,
            .m64n72k32, .m64n80k32, .m64n88k32, .m64n96k32,
            .m64n104k32, .m64n112k32, .m64n120k32, .m64n128k32,
            .m64n136k32, .m64n144k32, .m64n152k32, .m64n160k32,
            .m64n168k32, .m64n176k32, .m64n184k32, .m64n192k32,
            .m64n200k32, .m64n208k32, .m64n216k32, .m64n224k32,
            .m64n232k32, .m64n240k32, .m64n248k32, .m64n256k32};
.atype  = {.e4m3, .e5m2};
.btype  = {.e4m3, .e5m2};
.dtype  = {.f16, .f32};
```
Integer type:
```
wgmma.mma_async.sync.aligned.shape{.satfinite}.s32.atype.btype  d, a-desc, b-desc, scale-d;

wgmma.mma_async.sync.aligned.shape{.satfinite}.s32.atype.btype  d, a, b-desc, scale-d;

.shape   = {.m64n8k32, .m64n16k32, .m64n24k32, .m64n32k32,
            .m64n48k32, .m64n64k32, .m64n80k32, .m64n96k32,
            .m64n112k32, .m64n128k32, .m64n144k32, .m64n160k32,
            .m64n176k32, .m64n192k32, .m64n208k32, .m64n224k32};
.atype  = {.s8, .u8};
.btype  = {.s8, .u8};
```
Single bit:
```
wgmma.mma_async.sync.aligned.shape.s32.b1.b1.op.popc  d, a-desc, b-desc, scale-d;

wgmma.mma_async.sync.aligned.shape.s32.b1.b1.op.popc  d, a, b-desc, scale-d;

.shape   = {.m64n8k256, .m64n16k256, .m64n24k256, .m64n32k256,
            .m64n48k256, .m64n64k256, .m64n80k256, .m64n96k256,
            .m64n112k256, .m64n128k256, .m64n144k256, .m64n160k256,
            .m64n176k256, .m64n192k256, .m64n208k256, .m64n224k256,
            .m64n240k256, .m64n256k256};
.op  = {.and};
```

