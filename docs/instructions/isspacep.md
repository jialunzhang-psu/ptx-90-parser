### Description

Write predicate register p with 1 if generic address a falls within the specified state
space window and with 0 otherwise. Destination p has type .pred; the source address
operand must be of type .u32 or .u64.
isspacep.param{::entry} returns 1 if the generic address falls within the window of
Kernel Function Parameters, otherwise returns 0. If .param
is specified without any sub-qualifiers then it defaults to .param::entry.
isspacep.global returns 1 for Kernel Function Parameters
as .param window is contained within the .global
window.
If no sub-qualifier is specified with .shared state space, then ::cta is assumed by default.

### Syntax

```
isspacep.space  p, a;    // result is .pred

.space = { const, .global, .local, .shared{::cta, ::cluster}, .param{::entry} };
```

### Examples

```
isspacep.const           iscnst, cptr;
isspacep.global          isglbl, gptr;
isspacep.local           islcl,  lptr;
isspacep.shared          isshrd, sptr;
isspacep.param::entry    isparam, pptr;
isspacep.shared::cta     isshrdcta, sptr;
isspacep.shared::cluster ishrdany sptr;
```

