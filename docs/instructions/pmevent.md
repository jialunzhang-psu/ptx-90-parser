### Description

Triggers one or more of a fixed number of performance monitor events, with event index or mask
specified by immediate operand a.
pmevent (without modifier .mask) triggers a single performance monitor event indexed by
immediate operand a, in the range 0..15.
pmevent.mask triggers one or more of the performance monitor events. Each bit in the 16-bit
immediate operand a controls an event.
Programmatic performance moniter events may be combined with other hardware events using Boolean
functions to increment one of the four performance counters. The relationship between events and
counters is programmed via API calls from the host.

### Syntax

```
pmevent       a;    // trigger a single performance monitor event
pmevent.mask  a;    // trigger one or more performance monitor events
```

### Examples

```
pmevent      1;
@p  pmevent      7;
@q  pmevent.mask 0xff;
```

