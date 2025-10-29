### Description

Two-way SIMD parallel arithmetic operation with secondary operation.
Elements of each dual half-word source to the operation are selected from any of the four half-words
in the two source operands a and b using the asel and bsel modifiers.
The selected half-words are then operated on in parallel.
The results are optionally clamped to the appropriate range determined by the destination type
(signed or unsigned). Saturation cannot be used with the secondary accumulate operation.
For instructions with a secondary SIMD merge operation:
For instructions with a secondary accumulate operation:

### Syntax

```
// SIMD instruction with secondary SIMD merge operation
vop2.dtype.atype.btype{.sat}  d{.mask}, a{.asel}, b{.bsel}, c;

// SIMD instruction with secondary accumulate operation
vop2.dtype.atype.btype.add  d{.mask}, a{.asel}, b{.bsel}, c;

 vop2  = { vadd2, vsub2, vavrg2, vabsdiff2, vmin2, vmax2 };
.dtype = .atype = .btype = { .u32, .s32 };
.mask  = { .h0, .h1, .h10 };  // defaults to .h10
.asel  = .bsel  = { .hxy, where x,y are from { 0, 1, 2, 3 } };
   .asel defaults to .h10
   .bsel defaults to .h32
```

### Semantics

```
// extract pairs of half-words and sign- or zero-extend
// based on operand type
Va = extractAndSignExt_2( a, b, .asel, .atype );
Vb = extractAndSignExt_2( a, b, .bsel, .btype );
Vc = extractAndSignExt_2( c );

for (i=0; i<2; i++) {
    switch ( vop2 ) {
       case vadd2:             t[i] = Va[i] + Vb[i];
       case vsub2:             t[i] = Va[i] - Vb[i];
       case vavrg2:            if ( ( Va[i] + Vb[i] ) >= 0 ) {
                                   t[i] = ( Va[i] + Vb[i] + 1 ) >> 1;
                               } else {
                                   t[i] = ( Va[i] + Vb[i] ) >> 1;
                               }
       case vabsdiff2:         t[i] = | Va[i] - Vb[i] |;
       case vmin2:             t[i] = MIN( Va[i], Vb[i] );
       case vmax2:             t[i] = MAX( Va[i], Vb[i] );
    }
    if (.sat) {
        if ( .dtype == .s32 )  t[i] = CLAMP( t[i], S16_MAX, S16_MIN );
        else                   t[i] = CLAMP( t[i], U16_MAX, U16_MIN );
    }
}
// secondary accumulate or SIMD merge
mask = extractMaskBits( .mask );
if (.add) {
    d = c;
    for (i=0; i<2; i++) {  d += mask[i] ? t[i] : 0;  }
} else {
    d = 0;
    for (i=0; i<2; i++)  {  d |= mask[i] ? t[i] : Vc[i];  }
}
```

### Examples

```
vadd2.s32.s32.u32.sat  r1, r2, r3, r1;
vsub2.s32.s32.s32.sat  r1.h0, r2.h10, r3.h32, r1;
vmin2.s32.u32.u32.add  r1.h10, r2.h00, r3.h22, r1;
```

