### Description

Ends execution of a thread.
As threads exit, barriers waiting on all threads are checked to see if the exiting threads are the
only threads that have not yet made it to a barrier{.cta} for all threads in the CTA or to a
barrier.cluster for all threads in the cluster. If the exiting threads are holding up the
barrier, the barrier is released.

### Syntax

```
exit;
```

### Examples

```
exit;
@p  exit;
```

