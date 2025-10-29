### Description

setmaxnreg provides a hint to the system to update the maximum number of per-thread registers
owned by the executing warp to the value specified by the imm-reg-count operand.
Qualifier .dec is used to release extra registers such that the absolute per-thread maximum
register count is reduced from its current value to imm-reg-count. Qualifier .inc is used to
request additional registers such that the absolute per-thread maximum register count is increased
from its current value to imm-reg-count.
A pool of available registers is maintained per-CTA. Register adjustments requested by the
setmaxnreg instructions are handled by supplying extra registers from this pool to the
requesting warp or by releasing extra registers from the requesting warp to this pool, depending
upon the value of the .action qualifier.
The setmaxnreg.inc instruction blocks the execution until enough registers are available in the
CTA’s register pool. After the instruction setmaxnreg.inc obtains new registers from the CTA
pool, the initial contents of the new registers are undefined. The new registers must be initialized
before they are used.
The same setmaxnreg instruction must be executed by all warps in a
warpgroup. After executing a
setmaxnreg instruction, all warps in the warpgroup must synchronize explicitly before
executing subsequent setmaxnreg instructions. If a setmaxnreg instruction is not executed by all
warps in the warpgroup, then the behavior is undefined.
Operand imm-reg-count is an integer constant. The value of imm-reg-count must be in the
range 24 to 256 (both inclusive) and must be a multiple of 8.
Changes to the register file of the warp always happen at the tail-end of the register file.
The setmaxnreg instruction requires that the kernel has been launched with a valid value of
maximum number of per-thread registers specified via the appropriate compilation via the appropriate
compile-time option or the appropriate performance tuning directive. Otherwise, the setmaxnreg
instruction may have no effect.
When qualifier .dec is specified, the maximum number of per-thread registers owned by the warp
prior to the execution of setmaxnreg instruction should be greater than or equal to the
imm-reg-count. Otherwise, the behaviour is undefined.
When qualifier .inc is specified, the maximum number of per-thread registers owned by the warp
prior to the execution of setmaxnreg instruction should be less than or equal to the
imm-reg-count. Otherwise, the behaviour is undefined.
The mandatory .sync qualifier indicates that setmaxnreg instruction causes the executing
thread to wait until all threads in the warp execute the same setmaxnreg instruction before
resuming execution.
The mandatory .aligned qualifier indicates that all threads in the warpgroup must execute the
same setmaxnreg instruction. In conditionally executed code, setmaxnreg instruction should
only be used if it is known that all threads in warpgroup evaluate the condition identically,
otherwise the behavior is undefined.

### Syntax

```
setmaxnreg.action.sync.aligned.u32 imm-reg-count;

.action = { .inc, .dec };
```

### Examples

```
setmaxnreg.dec.sync.aligned.u32 64;
setmaxnreg.inc.sync.aligned.u32 192;
```

