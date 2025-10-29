### Description

Pick four arbitrary bytes from two 32-bit registers, and reassemble them into a 32-bit destination
register.
In the generic form (no mode specified), the permute control consists of four 4-bit selection
values. The bytes in the two source registers are numbered from 0 to 7: {b, a} = {{b7, b6, b5,
b4}, {b3, b2, b1, b0}}. For each byte in the target register, a 4-bit selection value is defined.
The 3 lsbs of the selection value specify which of the 8 source bytes should be moved into the
target position. The msb defines if the byte value should be copied, or if the sign (msb of the
byte) should be replicated over all 8 bits of the target position (sign extend of the byte value);
msb=0 means copy the literal value; msb=1 means replicate the sign. Note that the sign
extension is only performed as part of generic form.
Thus, the four 4-bit values fully specify an arbitrary byte permute, as a 16b permute code.
The more specialized form of the permute control uses the two lsb’s of operand c (which is
typically an address pointer) to control the byte extraction.

### Syntax

```
prmt.b32{.mode}  d, a, b, c;

.mode = { .f4e, .b4e, .rc8, .ecl, .ecr, .rc16 };
```

### Semantics

```
tmp64 = (b<<32) | a;  // create 8 byte source

if ( ! mode ) {
   ctl[0] = (c >>  0) & 0xf;
   ctl[1] = (c >>  4) & 0xf;
   ctl[2] = (c >>  8) & 0xf;
   ctl[3] = (c >> 12) & 0xf;
} else {
   ctl[0] = ctl[1] = ctl[2] = ctl[3] = (c >>  0) & 0x3;
}

tmp[07:00] = ReadByte( mode, ctl[0], tmp64 );
tmp[15:08] = ReadByte( mode, ctl[1], tmp64 );
tmp[23:16] = ReadByte( mode, ctl[2], tmp64 );
tmp[31:24] = ReadByte( mode, ctl[3], tmp64 );
```

### Examples

```
prmt.b32      r1, r2, r3, r4;
prmt.b32.f4e  r1, r2, r3, r4;
```

