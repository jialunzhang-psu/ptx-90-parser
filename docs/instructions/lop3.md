### Description

Compute bitwise logical operation on inputs a, b, c and store the result in destination
d.
Optionally, .BoolOp can be specified to compute the predicate result p by performing a
Boolean operation on the destination operand d with the predicate q in the following manner:
```
p = (d != 0) BoolOp q;
```
The sink symbol ‘_’ may be used in place of the destination operand d when .BoolOp qualifier
is specified.
The logical operation is defined by a look-up table which, for 3 inputs, can be represented as an
8-bit value specified by operand immLut as described below. immLut is an integer constant
that can take values from 0 to 255, thereby allowing up to 256 distinct logical operations on inputs
a, b, c.
For a logical operation F(a, b, c) the value of immLut can be computed by applying the same
operation to three predefined constant values as follows:
```
ta = 0xF0;
tb = 0xCC;
tc = 0xAA;

immLut = F(ta, tb, tc);
```
Examples:
```
If F = (a & b & c);
immLut = 0xF0 & 0xCC & 0xAA = 0x80

If F = (a | b | c);
immLut = 0xF0 | 0xCC | 0xAA = 0xFE

If F = (a & b & ~c);
immLut = 0xF0 & 0xCC & (~0xAA) = 0x40

If F = ((a & b | c) ^ a);
immLut = (0xF0 & 0xCC | 0xAA) ^ 0xF0 = 0x1A
```
The following table illustrates computation of immLut for various logical operations:

### Syntax

```
lop3.b32 d, a, b, c, immLut;
lop3.BoolOp.b32 d|p, a, b, c, immLut, q;

.BoolOp   = { .or , .and };
```

### Semantics

```
F = GetFunctionFromTable(immLut); // returns the function corresponding to immLut value
d = F(a, b, c);
if (BoolOp specified) {
    p = (d != 0) BoolOp q;
}
```

### Examples

```
lop3.b32       d, a, b, c, 0x40;
lop3.or.b32  d|p, a, b, c, 0x3f, q;
lop3.and.b32 _|p, a, b, c, 0x3f, q;
```

