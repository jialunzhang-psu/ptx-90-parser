### Description

Copies the current value of stack pointer into the destination register d. Pointer returned by
stacksave can be used in a subsequent stackrestore instruction to restore the stack
pointer. If d is modified prior to use in stackrestore instruction, it may corrupt data in
the stack.
Destination operand d has the same type as the instruction type.

### Syntax

```
stacksave.type  d;

.type = { .u32, .u64 };
```

### Semantics

```
d = stackptr;
```

### Examples

```
.reg .u32 rd;
stacksave.u32 rd;

.reg .u64 rd1;
stacksave.u64 rd1;
```

