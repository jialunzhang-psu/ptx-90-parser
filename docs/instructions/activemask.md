### Description

activemask queries predicated-on active threads from the executing warp and sets the destination
d with 32-bit integer mask where bit position in the mask corresponds to the thread’s
laneid.
Destination d is a 32-bit destination register.
An active thread will contribute 1 for its entry in the result and exited or inactive or
predicated-off thread will contribute 0 for its entry in the result.

### Syntax

```
activemask.b32 d;
```

### Examples

```
activemask.b32  %r1;
```

