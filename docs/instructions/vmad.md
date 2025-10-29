### Description

Calculate (a*b) + c, with optional operand negates, plus one mode, and scaling.
The source operands support optional negation with some restrictions. Although PTX syntax allows
separate negation of the a and b operands, internally this is represented as negation of the
product (a*b). That is, (a*b) is negated if and only if exactly one of a or b is
negated. PTX allows negation of either (a*b) or c.
The plus one mode (.po) computes (a*b) + c + 1, which is used in computing averages. Source
operands may not be negated in .po mode.
The intermediate result of (a*b) is unsigned if atype and btype are unsigned and the product
(a*b) is not negated; otherwise, the intermediate result is signed. Input c has the same
sign as the intermediate result.
The final result is unsigned if the intermediate result is unsigned and c is not negated.
Depending on the sign of the a and b operands, and the operand negates, the following
combinations of operands are supported for VMAD:
```
(u32 * u32) + u32  // intermediate unsigned; final unsigned
-(u32 * u32) + s32  // intermediate   signed; final   signed
 (u32 * u32) - u32  // intermediate unsigned; final   signed
 (u32 * s32) + s32  // intermediate   signed; final   signed
-(u32 * s32) + s32  // intermediate   signed; final   signed
 (u32 * s32) - s32  // intermediate   signed; final   signed
 (s32 * u32) + s32  // intermediate   signed; final   signed
-(s32 * u32) + s32  // intermediate   signed; final   signed
 (s32 * u32) - s32  // intermediate   signed; final   signed
 (s32 * s32) + s32  // intermediate   signed; final   signed
-(s32 * s32) + s32  // intermediate   signed; final   signed
 (s32 * s32) - s32  // intermediate   signed; final   signed
```
The intermediate result is optionally scaled via right-shift; this result is sign-extended if the
final result is signed, and zero-extended otherwise.
The final result is optionally saturated to the appropriate 32-bit range based on the type (signed
or unsigned) of the final result.

### Syntax

```
// 32-bit scalar operation
vmad.dtype.atype.btype{.sat}{.scale}     d, {-}a{.asel}, {-}b{.bsel},
                                         {-}c;
vmad.dtype.atype.btype.po{.sat}{.scale}  d, a{.asel}, b{.bsel}, c;

.dtype = .atype = .btype = { .u32, .s32 };
.asel  = .bsel  = { .b0, .b1, .b2, .b3, .h0, .h1 };
.scale = { .shr7, .shr15 };
```

### Semantics

```
// extract byte/half-word/word and sign- or zero-extend
// based on source operand type
ta = partSelectSignExtend( a, atype, asel );
tb = partSelectSignExtend( b, btype, bsel );
signedFinal = isSigned(atype) || isSigned(btype) ||
                                 (a.negate ^ b.negate) || c.negate;
tmp[127:0] = ta * tb;

lsb = 0;
if ( .po )                  {              lsb = 1; } else
if ( a.negate ^ b.negate )  { tmp = ~tmp;  lsb = 1; } else
if ( c.negate )             { c   = ~c;    lsb = 1; }

c128[127:0] = (signedFinal) sext32( c ) : zext ( c );
tmp = tmp + c128 + lsb;
switch( scale ) {
   case .shr7:   result = (tmp >>  7) & 0xffffffffffffffff;
   case .shr15:  result = (tmp >> 15) & 0xffffffffffffffff;
}
if ( .sat ) {
     if (signedFinal) result = CLAMP(result, S32_MAX, S32_MIN);
     else             result = CLAMP(result, U32_MAX, U32_MIN);
}
```

### Examples

```
vmad.s32.s32.u32.sat    r0, r1, r2, -r3;
vmad.u32.u32.u32.shr15  r0, r1.h0, r2.h0, r3;
```

