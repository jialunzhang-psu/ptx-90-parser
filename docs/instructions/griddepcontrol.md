### Description

The griddepcontrol instruction allows the dependent grids and prerequisite grids as defined by
the runtime, to control execution in the following way:
.launch_dependents modifier signals that specific dependents the runtime system designated to
react to this instruction can be scheduled as soon as all other CTAs in the grid issue the same
instruction or have completed. The dependent may launch before the completion of the current
grid. There is no guarantee that the dependent will launch before the completion of the current
grid. Repeated invocations of this instruction by threads in the current CTA will have no additional
side effects past that of the first invocation.
.wait modifier causes the executing thread to wait until all prerequisite grids in flight have
completed and all the memory operations from the prerequisite grids are performed and made visible
to the current grid.

### Syntax

```
griddepcontrol.action;

.action   = { .launch_dependents, .wait }
```

### Examples

```
griddepcontrol.launch_dependents;
griddepcontrol.wait;
```

