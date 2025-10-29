### Description

Index into a list of possible destination labels, and continue execution from the chosen
label. Conditional branches are specified by using a guard predicate.
brx.idx.uni guarantees that the branch is non-divergent, i.e. all active threads in a warp that
are currently executing this instruction have identical values for the guard predicate and the
index argument.
The index operand is a .u32 register. The tlist operand must be the label of a
.branchtargets directive. It is accessed as a zero-based sequence using index. Behaviour is
undefined if the value of index is greater than or equal to the length of tlist.
The .branchtargets directive must be defined in the local function scope before it is used. It
must refer to labels within the current function.

### Syntax

```
@p    brx.idx{.uni} index, tlist;
      brx.idx{.uni} index, tlist;
```

### Semantics

```
if (p) {
    if (index < length(tlist)) {
      pc = tlist[index];
    } else {
      pc = undefined;
    }
}
```

### Examples

```
.function foo () {
    .reg .u32 %r0;
    ...
    L1:
    ...
    L2:
    ...
    L3:
    ...
    ts: .branchtargets L1, L2, L3;
    @p brx.idx %r0, ts;
    ...
}
```

