### Description

elect.sync elects one predicated active leader thread from among a set of threads specified by
membermask. laneid of the elected thread is returned in the 32-bit destination operand
d. The sink symbol ‘_’ can be used for destination operand d. The predicate destination
p is set to True for the leader thread, and False for all other threads.
Operand membermask specifies a 32-bit integer indicating the set of threads from which a leader
is to be elected. The behavior is undefined if the executing thread is not in membermask.
Election of a leader thread happens deterministically, i.e. the same leader thread is elected for
the same membermask every time.
The mandatory .sync qualifier indicates that elect causes the executing thread to wait until
all threads in the membermask execute the elect instruction before resuming execution.

### Syntax

```
elect.sync d|p, membermask;
```

### Examples

```
elect.sync    %r0|%p0, 0xffffffff;
```

