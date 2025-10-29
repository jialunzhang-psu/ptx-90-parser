### Description

redux.sync will cause the executing thread to wait until all non-exited threads corresponding to
membermask have executed redux.sync with the same qualifiers and same membermask value
before resuming execution.
Operand membermask specifies a 32-bit integer which is a mask indicating threads participating
in this instruction where the bit position corresponds to thread’s laneid.
redux.sync performs a reduction operation .op of the 32 bit source register src across
all non-exited threads in the membermask. The result of the reduction operation is written to
the 32 bit destination register dst.
Reduction operation can be one of the bitwise operation in .and, .or, .xor or arithmetic
operation in .add, .min , .max.
For the .add operation result is truncated to 32 bits.
For .f32 instruction type, if the input value is 0.0 then +0.0 > -0.0.
If .abs qualifier is specified, then the absolute value of the input is considered for the
reduction operation.
If the .NaN qualifier is specified, then the result of the reduction operation is canonical NaN
if the input to the reduction operation from any participating thread is NaN.
In the absence of .NaN qualifier, only non-NaN values are considered for the reduction operation
and the result will be canonical NaN when all inputs are NaNs.
The behavior of redux.sync is undefined if the executing thread is not in the membermask.

### Syntax

```
redux.sync.op.type dst, src, membermask;
.op   = {.add, .min, .max}
.type = {.u32, .s32}

redux.sync.op.b32 dst, src, membermask;
.op   = {.and, .or, .xor}

redux.sync.op{.abs.}{.NaN}.f32 dst, src, membermask;
.op   = { .min, .max }
```

### Examples

```
.reg .b32 dst, src, init, mask;
redux.sync.add.s32 dst, src, 0xff;
redux.sync.xor.b32 dst, src, mask;

redux.sync.min.abs.NaN.f32 dst, src, mask;
```

