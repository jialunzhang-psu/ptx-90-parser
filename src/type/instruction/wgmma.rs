use crate::r#type::common::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Wgmma {
    /// `wgmma.mma_async.sync.aligned.shape.dtype.f16.f16  d, a-desc, b-desc, scale-d, imm-scale-a, imm-scale-b, imm-trans-a, imm-trans-b;`
    F16Descriptor(F16Descriptor),
    /// `wgmma.mma_async.sync.aligned.shape.dtype.f16.f16  d, a, b-desc, scale-d, imm-scale-a, imm-scale-b, imm-trans-b;`
    F16Register(F16Register),
    /// `wgmma.mma_async.sync.aligned.shape.dtype.bf16.bf16  d, a-desc, b-desc, scale-d, imm-scale-a, imm-scale-b, imm-trans-a, imm-trans-b;`
    Bf16Descriptor(Bf16Descriptor),
    /// `wgmma.mma_async.sync.aligned.shape.dtype.bf16.bf16  d, a, b-desc, scale-d, imm-scale-a, imm-scale-b, imm-trans-b;`
    Bf16Register(Bf16Register),
    /// `wgmma.mma_async.sync.aligned.shape.dtype.tf32.tf32  d, a-desc, b-desc, scale-d, imm-scale-a, imm-scale-b;`
    Tf32Descriptor(Tf32Descriptor),
    /// `wgmma.mma_async.sync.aligned.shape.dtype.tf32.tf32  d, a, b-desc, scale-d, imm-scale-a, imm-scale-b;`
    Tf32Register(Tf32Register),
    /// `wgmma.mma_async.sync.aligned.shape.dtype.atype.btype  d, a-desc, b-desc, scale-d, imm-scale-a, imm-scale-b;`
    Fp8Descriptor(Fp8Descriptor),
    /// `wgmma.mma_async.sync.aligned.shape.dtype.atype.btype  d, a, b-desc, scale-d, imm-scale-a, imm-scale-b;`
    Fp8Register(Fp8Register),
    /// `wgmma.mma_async.sync.aligned.shape{.satfinite}.s32.atype.btype  d, a-desc, b-desc, scale-d;`
    IntegerDescriptor(IntegerDescriptor),
    /// `wgmma.mma_async.sync.aligned.shape{.satfinite}.s32.atype.btype  d, a, b-desc, scale-d;`
    IntegerRegister(IntegerRegister),
    /// `wgmma.mma_async.sync.aligned.shape.s32.b1.b1.op.popc  d, a-desc, b-desc, scale-d;`
    SingleBitDescriptor(SingleBitDescriptor),
    /// `wgmma.mma_async.sync.aligned.shape.s32.b1.b1.op.popc  d, a, b-desc, scale-d;`
    SingleBitRegister(SingleBitRegister),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct F16Descriptor {
    /// `.shape = {.m64n8k16, ...}`
    pub shape: ShapeK16,
    /// `.dtype = {.f16, .f32}`
    pub dtype: HalfAccumulatorType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a-desc`
    pub a_descriptor: RegisterOperand,
    /// `b-desc`
    pub b_descriptor: RegisterOperand,
    /// `scale-d`
    pub scale_d: PredicateRegister,
    /// `imm-scale-a`
    pub imm_scale_a: ScaleImmediate,
    /// `imm-scale-b`
    pub imm_scale_b: ScaleImmediate,
    /// `imm-trans-a`
    pub imm_trans_a: TransposeImmediate,
    /// `imm-trans-b`
    pub imm_trans_b: TransposeImmediate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct F16Register {
    /// `.shape = {.m64n8k16, ...}`
    pub shape: ShapeK16,
    /// `.dtype = {.f16, .f32}`
    pub dtype: HalfAccumulatorType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a_register: RegisterOperand,
    /// `b-desc`
    pub b_descriptor: RegisterOperand,
    /// `scale-d`
    pub scale_d: PredicateRegister,
    /// `imm-scale-a`
    pub imm_scale_a: ScaleImmediate,
    /// `imm-scale-b`
    pub imm_scale_b: ScaleImmediate,
    /// `imm-trans-b`
    pub imm_trans_b: TransposeImmediate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bf16Descriptor {
    /// `.shape = {.m64n8k16, ...}`
    pub shape: ShapeK16,
    /// `.dtype = {.f32}`
    pub dtype: Bf16AccumulatorType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a-desc`
    pub a_descriptor: RegisterOperand,
    /// `b-desc`
    pub b_descriptor: RegisterOperand,
    /// `scale-d`
    pub scale_d: PredicateRegister,
    /// `imm-scale-a`
    pub imm_scale_a: ScaleImmediate,
    /// `imm-scale-b`
    pub imm_scale_b: ScaleImmediate,
    /// `imm-trans-a`
    pub imm_trans_a: TransposeImmediate,
    /// `imm-trans-b`
    pub imm_trans_b: TransposeImmediate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bf16Register {
    /// `.shape = {.m64n8k16, ...}`
    pub shape: ShapeK16,
    /// `.dtype = {.f32}`
    pub dtype: Bf16AccumulatorType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a_register: RegisterOperand,
    /// `b-desc`
    pub b_descriptor: RegisterOperand,
    /// `scale-d`
    pub scale_d: PredicateRegister,
    /// `imm-scale-a`
    pub imm_scale_a: ScaleImmediate,
    /// `imm-scale-b`
    pub imm_scale_b: ScaleImmediate,
    /// `imm-trans-b`
    pub imm_trans_b: TransposeImmediate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tf32Descriptor {
    /// `.shape = {.m64n8k8, ...}`
    pub shape: ShapeK8,
    /// `.dtype = {.f32}`
    pub dtype: Tf32AccumulatorType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a-desc`
    pub a_descriptor: RegisterOperand,
    /// `b-desc`
    pub b_descriptor: RegisterOperand,
    /// `scale-d`
    pub scale_d: PredicateRegister,
    /// `imm-scale-a`
    pub imm_scale_a: ScaleImmediate,
    /// `imm-scale-b`
    pub imm_scale_b: ScaleImmediate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tf32Register {
    /// `.shape = {.m64n8k8, ...}`
    pub shape: ShapeK8,
    /// `.dtype = {.f32}`
    pub dtype: Tf32AccumulatorType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a_register: RegisterOperand,
    /// `b-desc`
    pub b_descriptor: RegisterOperand,
    /// `scale-d`
    pub scale_d: PredicateRegister,
    /// `imm-scale-a`
    pub imm_scale_a: ScaleImmediate,
    /// `imm-scale-b`
    pub imm_scale_b: ScaleImmediate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fp8Descriptor {
    /// `.shape = {.m64n8k32, ...}`
    pub shape: ShapeK32,
    /// `.dtype = {.f16, .f32}`
    pub dtype: Fp8AccumulatorType,
    /// `.atype = {.e4m3, .e5m2}`
    pub atype: Fp8InputType,
    /// `.btype = {.e4m3, .e5m2}`
    pub btype: Fp8InputType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a-desc`
    pub a_descriptor: RegisterOperand,
    /// `b-desc`
    pub b_descriptor: RegisterOperand,
    /// `scale-d`
    pub scale_d: PredicateRegister,
    /// `imm-scale-a`
    pub imm_scale_a: ScaleImmediate,
    /// `imm-scale-b`
    pub imm_scale_b: ScaleImmediate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fp8Register {
    /// `.shape = {.m64n8k32, ...}`
    pub shape: ShapeK32,
    /// `.dtype = {.f16, .f32}`
    pub dtype: Fp8AccumulatorType,
    /// `.atype = {.e4m3, .e5m2}`
    pub atype: Fp8InputType,
    /// `.btype = {.e4m3, .e5m2}`
    pub btype: Fp8InputType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a_register: RegisterOperand,
    /// `b-desc`
    pub b_descriptor: RegisterOperand,
    /// `scale-d`
    pub scale_d: PredicateRegister,
    /// `imm-scale-a`
    pub imm_scale_a: ScaleImmediate,
    /// `imm-scale-b`
    pub imm_scale_b: ScaleImmediate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerDescriptor {
    /// `.shape = {.m64n8k32, ...}`
    pub shape: IntegerShape,
    /// `.satfinite`
    pub satfinite: bool,
    /// `.atype = {.s8, .u8}`
    pub atype: IntegerInputType,
    /// `.btype = {.s8, .u8}`
    pub btype: IntegerInputType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a-desc`
    pub a_descriptor: RegisterOperand,
    /// `b-desc`
    pub b_descriptor: RegisterOperand,
    /// `scale-d`
    pub scale_d: PredicateRegister,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerRegister {
    /// `.shape = {.m64n8k32, ...}`
    pub shape: IntegerShape,
    /// `.satfinite`
    pub satfinite: bool,
    /// `.atype = {.s8, .u8}`
    pub atype: IntegerInputType,
    /// `.btype = {.s8, .u8}`
    pub btype: IntegerInputType,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a_register: RegisterOperand,
    /// `b-desc`
    pub b_descriptor: RegisterOperand,
    /// `scale-d`
    pub scale_d: PredicateRegister,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleBitDescriptor {
    /// `.shape = {.m64n8k256, ...}`
    pub shape: BitShape,
    /// `.op = {.and}`
    pub operation: BitOperation,
    /// `d`
    pub destination: RegisterOperand,
    /// `a-desc`
    pub a_descriptor: RegisterOperand,
    /// `b-desc`
    pub b_descriptor: RegisterOperand,
    /// `scale-d`
    pub scale_d: PredicateRegister,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleBitRegister {
    /// `.shape = {.m64n8k256, ...}`
    pub shape: BitShape,
    /// `.op = {.and}`
    pub operation: BitOperation,
    /// `d`
    pub destination: RegisterOperand,
    /// `a`
    pub a_register: RegisterOperand,
    /// `b-desc`
    pub b_descriptor: RegisterOperand,
    /// `scale-d`
    pub scale_d: PredicateRegister,
}

/// `.shape = {.m64n8k16, .m64n16k16, .m64n24k16, .m64n32k16, .m64n40k16, .m64n48k16, .m64n56k16, .m64n64k16, .m64n72k16, .m64n80k16, .m64n88k16, .m64n96k16, .m64n104k16, .m64n112k16, .m64n120k16, .m64n128k16, .m64n136k16, .m64n144k16, .m64n152k16, .m64n160k16, .m64n168k16, .m64n176k16, .m64n184k16, .m64n192k16, .m64n200k16, .m64n208k16, .m64n216k16, .m64n224k16, .m64n232k16, .m64n240k16, .m64n248k16, .m64n256k16}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeK16 {
    /// `.m64n8k16`
    M64N8K16,
    /// `.m64n16k16`
    M64N16K16,
    /// `.m64n24k16`
    M64N24K16,
    /// `.m64n32k16`
    M64N32K16,
    /// `.m64n40k16`
    M64N40K16,
    /// `.m64n48k16`
    M64N48K16,
    /// `.m64n56k16`
    M64N56K16,
    /// `.m64n64k16`
    M64N64K16,
    /// `.m64n72k16`
    M64N72K16,
    /// `.m64n80k16`
    M64N80K16,
    /// `.m64n88k16`
    M64N88K16,
    /// `.m64n96k16`
    M64N96K16,
    /// `.m64n104k16`
    M64N104K16,
    /// `.m64n112k16`
    M64N112K16,
    /// `.m64n120k16`
    M64N120K16,
    /// `.m64n128k16`
    M64N128K16,
    /// `.m64n136k16`
    M64N136K16,
    /// `.m64n144k16`
    M64N144K16,
    /// `.m64n152k16`
    M64N152K16,
    /// `.m64n160k16`
    M64N160K16,
    /// `.m64n168k16`
    M64N168K16,
    /// `.m64n176k16`
    M64N176K16,
    /// `.m64n184k16`
    M64N184K16,
    /// `.m64n192k16`
    M64N192K16,
    /// `.m64n200k16`
    M64N200K16,
    /// `.m64n208k16`
    M64N208K16,
    /// `.m64n216k16`
    M64N216K16,
    /// `.m64n224k16`
    M64N224K16,
    /// `.m64n232k16`
    M64N232K16,
    /// `.m64n240k16`
    M64N240K16,
    /// `.m64n248k16`
    M64N248K16,
    /// `.m64n256k16`
    M64N256K16,
}

/// `.shape = {.m64n8k8, .m64n16k8, .m64n24k8, .m64n32k8, .m64n40k8, .m64n48k8, .m64n56k8, .m64n64k8, .m64n72k8, .m64n80k8, .m64n88k8, .m64n96k8, .m64n104k8, .m64n112k8, .m64n120k8, .m64n128k8, .m64n136k8, .m64n144k8, .m64n152k8, .m64n160k8, .m64n168k8, .m64n176k8, .m64n184k8, .m64n192k8, .m64n200k8, .m64n208k8, .m64n216k8, .m64n224k8, .m64n232k8, .m64n240k8, .m64n248k8, .m64n256k8}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeK8 {
    /// `.m64n8k8`
    M64N8K8,
    /// `.m64n16k8`
    M64N16K8,
    /// `.m64n24k8`
    M64N24K8,
    /// `.m64n32k8`
    M64N32K8,
    /// `.m64n40k8`
    M64N40K8,
    /// `.m64n48k8`
    M64N48K8,
    /// `.m64n56k8`
    M64N56K8,
    /// `.m64n64k8`
    M64N64K8,
    /// `.m64n72k8`
    M64N72K8,
    /// `.m64n80k8`
    M64N80K8,
    /// `.m64n88k8`
    M64N88K8,
    /// `.m64n96k8`
    M64N96K8,
    /// `.m64n104k8`
    M64N104K8,
    /// `.m64n112k8`
    M64N112K8,
    /// `.m64n120k8`
    M64N120K8,
    /// `.m64n128k8`
    M64N128K8,
    /// `.m64n136k8`
    M64N136K8,
    /// `.m64n144k8`
    M64N144K8,
    /// `.m64n152k8`
    M64N152K8,
    /// `.m64n160k8`
    M64N160K8,
    /// `.m64n168k8`
    M64N168K8,
    /// `.m64n176k8`
    M64N176K8,
    /// `.m64n184k8`
    M64N184K8,
    /// `.m64n192k8`
    M64N192K8,
    /// `.m64n200k8`
    M64N200K8,
    /// `.m64n208k8`
    M64N208K8,
    /// `.m64n216k8`
    M64N216K8,
    /// `.m64n224k8`
    M64N224K8,
    /// `.m64n232k8`
    M64N232K8,
    /// `.m64n240k8`
    M64N240K8,
    /// `.m64n248k8`
    M64N248K8,
    /// `.m64n256k8`
    M64N256K8,
}

/// `.shape = {.m64n8k32, .m64n16k32, .m64n24k32, .m64n32k32, .m64n40k32, .m64n48k32, .m64n56k32, .m64n64k32, .m64n72k32, .m64n80k32, .m64n88k32, .m64n96k32, .m64n104k32, .m64n112k32, .m64n120k32, .m64n128k32, .m64n136k32, .m64n144k32, .m64n152k32, .m64n160k32, .m64n168k32, .m64n176k32, .m64n184k32, .m64n192k32, .m64n200k32, .m64n208k32, .m64n216k32, .m64n224k32, .m64n232k32, .m64n240k32, .m64n248k32, .m64n256k32}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeK32 {
    /// `.m64n8k32`
    M64N8K32,
    /// `.m64n16k32`
    M64N16K32,
    /// `.m64n24k32`
    M64N24K32,
    /// `.m64n32k32`
    M64N32K32,
    /// `.m64n40k32`
    M64N40K32,
    /// `.m64n48k32`
    M64N48K32,
    /// `.m64n56k32`
    M64N56K32,
    /// `.m64n64k32`
    M64N64K32,
    /// `.m64n72k32`
    M64N72K32,
    /// `.m64n80k32`
    M64N80K32,
    /// `.m64n88k32`
    M64N88K32,
    /// `.m64n96k32`
    M64N96K32,
    /// `.m64n104k32`
    M64N104K32,
    /// `.m64n112k32`
    M64N112K32,
    /// `.m64n120k32`
    M64N120K32,
    /// `.m64n128k32`
    M64N128K32,
    /// `.m64n136k32`
    M64N136K32,
    /// `.m64n144k32`
    M64N144K32,
    /// `.m64n152k32`
    M64N152K32,
    /// `.m64n160k32`
    M64N160K32,
    /// `.m64n168k32`
    M64N168K32,
    /// `.m64n176k32`
    M64N176K32,
    /// `.m64n184k32`
    M64N184K32,
    /// `.m64n192k32`
    M64N192K32,
    /// `.m64n200k32`
    M64N200K32,
    /// `.m64n208k32`
    M64N208K32,
    /// `.m64n216k32`
    M64N216K32,
    /// `.m64n224k32`
    M64N224K32,
    /// `.m64n232k32`
    M64N232K32,
    /// `.m64n240k32`
    M64N240K32,
    /// `.m64n248k32`
    M64N248K32,
    /// `.m64n256k32`
    M64N256K32,
}

/// `.shape = {.m64n8k32, .m64n16k32, .m64n24k32, .m64n32k32, .m64n48k32, .m64n64k32, .m64n80k32, .m64n96k32, .m64n112k32, .m64n128k32, .m64n144k32, .m64n160k32, .m64n176k32, .m64n192k32, .m64n208k32, .m64n224k32}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerShape {
    /// `.m64n8k32`
    M64N8K32,
    /// `.m64n16k32`
    M64N16K32,
    /// `.m64n24k32`
    M64N24K32,
    /// `.m64n32k32`
    M64N32K32,
    /// `.m64n48k32`
    M64N48K32,
    /// `.m64n64k32`
    M64N64K32,
    /// `.m64n80k32`
    M64N80K32,
    /// `.m64n96k32`
    M64N96K32,
    /// `.m64n112k32`
    M64N112K32,
    /// `.m64n128k32`
    M64N128K32,
    /// `.m64n144k32`
    M64N144K32,
    /// `.m64n160k32`
    M64N160K32,
    /// `.m64n176k32`
    M64N176K32,
    /// `.m64n192k32`
    M64N192K32,
    /// `.m64n208k32`
    M64N208K32,
    /// `.m64n224k32`
    M64N224K32,
}

/// `.shape = {.m64n8k256, .m64n16k256, .m64n24k256, .m64n32k256, .m64n48k256, .m64n64k256, .m64n80k256, .m64n96k256, .m64n112k256, .m64n128k256, .m64n144k256, .m64n160k256, .m64n176k256, .m64n192k256, .m64n208k256, .m64n224k256, .m64n240k256, .m64n256k256}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitShape {
    /// `.m64n8k256`
    M64N8K256,
    /// `.m64n16k256`
    M64N16K256,
    /// `.m64n24k256`
    M64N24K256,
    /// `.m64n32k256`
    M64N32K256,
    /// `.m64n48k256`
    M64N48K256,
    /// `.m64n64k256`
    M64N64K256,
    /// `.m64n80k256`
    M64N80K256,
    /// `.m64n96k256`
    M64N96K256,
    /// `.m64n112k256`
    M64N112K256,
    /// `.m64n128k256`
    M64N128K256,
    /// `.m64n144k256`
    M64N144K256,
    /// `.m64n160k256`
    M64N160K256,
    /// `.m64n176k256`
    M64N176K256,
    /// `.m64n192k256`
    M64N192K256,
    /// `.m64n208k256`
    M64N208K256,
    /// `.m64n224k256`
    M64N224K256,
    /// `.m64n240k256`
    M64N240K256,
    /// `.m64n256k256`
    M64N256K256,
}

/// `.dtype = {.f16, .f32}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalfAccumulatorType {
    /// `.f16`
    F16,
    /// `.f32`
    F32,
}

/// `.dtype = {.f32}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bf16AccumulatorType {
    /// `.f32`
    F32,
}

/// `.dtype = {.f32}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tf32AccumulatorType {
    /// `.f32`
    F32,
}

/// `.dtype = {.f16, .f32}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fp8AccumulatorType {
    /// `.f16`
    F16,
    /// `.f32`
    F32,
}

/// `.atype = {.e4m3, .e5m2}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fp8InputType {
    /// `.e4m3`
    E4M3,
    /// `.e5m2`
    E5M2,
}

/// `.atype = {.s8, .u8}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerInputType {
    /// `.s8`
    S8,
    /// `.u8`
    U8,
}

/// `.op = {.and}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitOperation {
    /// `.and`
    And,
}

/// `{imm-scale-a, imm-scale-b}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleImmediate {
    /// `-1`
    MinusOne,
    /// `1`
    PlusOne,
}

/// `{imm-trans-a, imm-trans-b}`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransposeImmediate {
    /// `0`
    Identity,
    /// `1`
    Transpose,
}
