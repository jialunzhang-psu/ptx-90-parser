### Description

Extract bit field from a and place the zero or sign-extended result in d. Source b gives
the bit field starting bit position, and source c gives the bit field length in bits.
Operands a and d have the same type as the instruction type. Operands b and c are
type .u32, but are restricted to the 8-bit value range 0..255.
The sign bit of the extracted field is defined as:
If the bit field length is zero, the result is zero.
The destination d is padded with the sign bit of the extracted field. If the start position is
beyond the msb of the input, the destination d is filled with the replicated sign bit of the
extracted field.

### Syntax

```
bfe.type  d, a, b, c;

.type = { .u32, .u64,
          .s32, .s64 };
```

### Semantics

```
msb = (.type==.u32 || .type==.s32) ? 31 : 63;
pos = b & 0xff;  // pos restricted to 0..255 range
len = c & 0xff;  // len restricted to 0..255 range

if (.type==.u32 || .type==.u64 || len==0)
    sbit = 0;
else
    sbit = a[min(pos+len-1,msb)];

d = 0;
for (i=0; i<=msb; i++) {
    d[i] = (i<len && pos+i<=msb) ? a[pos+i] : sbit;
}
```

### Examples

```
bfe.b32  d,a,start,len;
```

