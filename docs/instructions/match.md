### Description

match.sync will cause executing thread to wait until all non-exited threads from membermask
have executed match.sync with the same qualifiers and same membermask value before resuming
execution.
Operand membermask specifies a 32-bit integer which is a mask indicating threads participating
in this instruction where the bit position corresponds to thread’s laneid.
match.sync performs broadcast and compare of operand a across all non-exited threads in
membermask and sets destination d and optional predicate p based on mode.
Operand a has instruction type and d has .b32 type.
Destination d is a 32-bit mask where bit position in mask corresponds to thread’s laneid.
The matching operation modes are:
The behavior of match.sync is undefined if the executing thread is not in the membermask.

### Syntax

```
match.any.sync.type  d, a, membermask;
match.all.sync.type  d[|p], a, membermask;

.type = { .b32, .b64 };
```

### Examples

```
match.any.sync.b32    d, a, 0xffffffff;
match.all.sync.b64    d|p, a, mask;
```

