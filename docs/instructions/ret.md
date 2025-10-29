### Description

Return execution to caller’s environment. A divergent return suspends threads until all threads are
ready to return to the caller. This allows multiple divergent ret instructions.
A ret is assumed to be divergent unless the .uni suffix is present, indicating that the
return is guaranteed to be non-divergent.
Any values returned from a function should be moved into the return parameter variables prior to
executing the ret instruction.
A return instruction executed in a top-level entry routine will terminate thread execution.

### Syntax

```
ret{.uni};
```

### Examples

```
ret;
@p  ret;
```

