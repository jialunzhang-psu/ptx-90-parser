### Description

Performs a reduction of the source predicate across all active threads in a warp. The destination
predicate value is the same across all threads in the warp.
The reduction modes are:
In the ballot form, vote.ballot.b32 simply copies the predicate from each thread in a warp
into the corresponding bit position of destination register d, where the bit position
corresponds to the thread’s lane id.
An inactive thread in warp will contribute a 0 for its entry when participating in
vote.ballot.b32.

### Syntax

```
vote.mode.pred  d, {!}a;
vote.ballot.b32 d, {!}a;  // 'ballot' form, returns bitmask

.mode = { .all, .any, .uni };
```

### Examples

```
vote.all.pred    p,q;
vote.uni.pred    p,q;
vote.ballot.b32  r1,p;  // get 'ballot' across warp
```

