### Description

Continue execution at the target. Conditional branches are specified by using a guard predicate. The
branch target must be a label.
bra.uni is guaranteed to be non-divergent, i.e. all active threads in a warp that are currently
executing this instruction have identical values for the guard predicate and branch target.

### Syntax

```
@p   bra{.uni}  tgt;           // tgt is a label
     bra{.uni}  tgt;           // unconditional branch
```

### Semantics

```
if (p) {
    pc = tgt;
}
```

### Examples

```
bra.uni  L_exit;    // uniform unconditional jump
@q  bra      L23;   // conditional branch
```

