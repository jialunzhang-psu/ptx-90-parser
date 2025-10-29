### Description

Suspends the thread for a sleep duration approximately close to the delay t, specified in
nanoseconds. t may be a register or an immediate value.
The sleep duration is approximated, but guaranteed to be in the interval [0, 2*t]. The maximum
sleep duration is 1 millisecond. The implementation may reduce the sleep duration for individual
threads within a warp such that all sleeping threads in the warp wake up together.

### Syntax

```
nanosleep.u32 t;
```

### Examples

```
.reg .b32 r;
.reg .pred p;

nanosleep.u32 r;
nanosleep.u32 42;
@p nanosleep.u32 r;
```

