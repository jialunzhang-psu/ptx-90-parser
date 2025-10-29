### Description

The alloca instruction dynamically allocates memory on the stack frame of the current function
and updates the stack pointer accordingly. The returned pointer ptr points to local memory and
can be used in the address operand of ld.local and st.local instructions.
If sufficient memory is unavailable for allocation on the stack, then execution of alloca may
result in stack overflow. In such cases, attempting to access the allocated memory with ptr will
result in undefined program behavior.
The memory allocated by alloca is deallocated in the following ways:
size is an unsigned value which specifies the amount of memory in number of bytes to be
allocated on stack. size = 0 may not lead to a valid memory allocation.
Both ptr and size have the same type as the instruction type.
immAlign is a 32-bit value which specifies the alignment requirement in number of bytes for the
memory allocated by alloca. It is an integer constant, must be a power of 2 and must not exceed
2^23. immAlign is an optional argument with default value being 8 which is the minimum
guaranteed alignment.

### Syntax

```
alloca.type  ptr, size{, immAlign};

.type = { .u32, .u64 };
```

### Semantics

```
alloca.type ptr, size, immAlign:

a = max(immAlign, frame_align); // frame_align is the minimum guaranteed alignment

// Allocate size bytes of stack memory with alignment a and update the stack pointer.
// Since the stack grows down, the updated stack pointer contains a lower address.
stackptr = alloc_stack_mem(size, a);

// Return the new value of stack pointer as ptr. Since ptr is the lowest address of the memory
// allocated by alloca, the memory can be accessed using ptr up to (ptr + size of allocated memory).
stacksave ptr;
```

### Examples

```
.reg .u32 ra, stackptr, ptr, size;

stacksave.u32 stackptr;     // Save the current stack pointer
alloca ptr, size, 8;        // Allocate stack memory
st.local.u32 [ptr], ra;     // Use the allocated stack memory
stackrestore.u32 stackptr;  // Deallocate memory by restoring the stack pointer
```

