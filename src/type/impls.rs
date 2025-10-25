use std::fmt;

use super::*;

impl ParameterQualifiers {
    pub fn is_empty(&self) -> bool {
        !self.is_const
            && !self.is_volatile
            && !self.is_restrict
            && !self.is_noalias
            && self.pointer.is_none()
    }
}

impl ParameterSpecifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AddressSign {
    pub fn negate(self) -> Self {
        match self {
            AddressSign::Positive => AddressSign::Negative,
            AddressSign::Negative => AddressSign::Positive,
        }
    }
}

impl VariableQualifier {
    pub fn width(&self) -> u32 {
        match self {
            VariableQualifier::Vector(width) => *width,
            VariableQualifier::Sampler => 1,
        }
    }
}

impl fmt::Display for StateSpaceModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateSpaceModifier::Param => write!(f, "param"),
            StateSpaceModifier::Global => write!(f, "global"),
            StateSpaceModifier::Local => write!(f, "local"),
            StateSpaceModifier::Shared => write!(f, "shared"),
            StateSpaceModifier::Const => write!(f, "const"),
            StateSpaceModifier::Generic => write!(f, "generic"),
        }
    }
}

impl fmt::Display for OpcodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpcodeKind::Abs => write!(f, "abs"),
            OpcodeKind::Activemask => write!(f, "activemask"),
            OpcodeKind::Add => write!(f, "add"),
            OpcodeKind::Addc => write!(f, "addc"),
            OpcodeKind::Alloca => write!(f, "alloca"),
            OpcodeKind::And => write!(f, "and"),
            OpcodeKind::Applypriority => write!(f, "applypriority"),
            OpcodeKind::Atom => write!(f, "atom"),
            OpcodeKind::Bar => write!(f, "bar"),
            OpcodeKind::Barrier => write!(f, "barrier"),
            OpcodeKind::Bfe => write!(f, "bfe"),
            OpcodeKind::Bfi => write!(f, "bfi"),
            OpcodeKind::Bfind => write!(f, "bfind"),
            OpcodeKind::Bmsk => write!(f, "bmsk"),
            OpcodeKind::Brev => write!(f, "brev"),
            OpcodeKind::Bra => write!(f, "bra"),
            OpcodeKind::Brkpt => write!(f, "brkpt"),
            OpcodeKind::Brx => write!(f, "brx"),
            OpcodeKind::Call => write!(f, "call"),
            OpcodeKind::Clz => write!(f, "clz"),
            OpcodeKind::Clusterlaunchcontrol => write!(f, "clusterlaunchcontrol"),
            OpcodeKind::Cnot => write!(f, "cnot"),
            OpcodeKind::Copysign => write!(f, "copysign"),
            OpcodeKind::Cos => write!(f, "cos"),
            OpcodeKind::Cp => write!(f, "cp"),
            OpcodeKind::Createpolicy => write!(f, "createpolicy"),
            OpcodeKind::Cvt => write!(f, "cvt"),
            OpcodeKind::Cvta => write!(f, "cvta"),
            OpcodeKind::Div => write!(f, "div"),
            OpcodeKind::Discard => write!(f, "discard"),
            OpcodeKind::Dp2a => write!(f, "dp2a"),
            OpcodeKind::Dp4a => write!(f, "dp4a"),
            OpcodeKind::Elect => write!(f, "elect"),
            OpcodeKind::Ex2 => write!(f, "ex2"),
            OpcodeKind::Exit => write!(f, "exit"),
            OpcodeKind::Fence => write!(f, "fence"),
            OpcodeKind::Fma => write!(f, "fma"),
            OpcodeKind::Fns => write!(f, "fns"),
            OpcodeKind::Getctarank => write!(f, "getctarank"),
            OpcodeKind::Griddepcontrol => write!(f, "griddepcontrol"),
            OpcodeKind::Isspacep => write!(f, "isspacep"),
            OpcodeKind::Istypep => write!(f, "istypep"),
            OpcodeKind::Ld => write!(f, "ld"),
            OpcodeKind::Ldmatrix => write!(f, "ldmatrix"),
            OpcodeKind::Ldu => write!(f, "ldu"),
            OpcodeKind::Lg2 => write!(f, "lg2"),
            OpcodeKind::Lop3 => write!(f, "lop3"),
            OpcodeKind::Mad => write!(f, "mad"),
            OpcodeKind::Mad24 => write!(f, "mad24"),
            OpcodeKind::Madc => write!(f, "madc"),
            OpcodeKind::Mapa => write!(f, "mapa"),
            OpcodeKind::Match => write!(f, "match"),
            OpcodeKind::Max => write!(f, "max"),
            OpcodeKind::Mbarrier => write!(f, "mbarrier"),
            OpcodeKind::Membar => write!(f, "membar"),
            OpcodeKind::Min => write!(f, "min"),
            OpcodeKind::Mov => write!(f, "mov"),
            OpcodeKind::Movmatrix => write!(f, "movmatrix"),
            OpcodeKind::Mma => write!(f, "mma"),
            OpcodeKind::Mul => write!(f, "mul"),
            OpcodeKind::Mul24 => write!(f, "mul24"),
            OpcodeKind::Multimem => write!(f, "multimem"),
            OpcodeKind::Nanosleep => write!(f, "nanosleep"),
            OpcodeKind::Neg => write!(f, "neg"),
            OpcodeKind::Not => write!(f, "not"),
            OpcodeKind::Or => write!(f, "or"),
            OpcodeKind::Pmevent => write!(f, "pmevent"),
            OpcodeKind::Popc => write!(f, "popc"),
            OpcodeKind::Prefetch => write!(f, "prefetch"),
            OpcodeKind::Prefetchu => write!(f, "prefetchu"),
            OpcodeKind::Prmt => write!(f, "prmt"),
            OpcodeKind::Rcp => write!(f, "rcp"),
            OpcodeKind::Red => write!(f, "red"),
            OpcodeKind::Redux => write!(f, "redux"),
            OpcodeKind::Rem => write!(f, "rem"),
            OpcodeKind::Rsqrt => write!(f, "rsqrt"),
            OpcodeKind::Sad => write!(f, "sad"),
            OpcodeKind::Selp => write!(f, "selp"),
            OpcodeKind::Set => write!(f, "set"),
            OpcodeKind::Setmaxnreg => write!(f, "setmaxnreg"),
            OpcodeKind::Setp => write!(f, "setp"),
            OpcodeKind::Shf => write!(f, "shf"),
            OpcodeKind::Shfl => write!(f, "shfl"),
            OpcodeKind::Shl => write!(f, "shl"),
            OpcodeKind::Shr => write!(f, "shr"),
            OpcodeKind::Sin => write!(f, "sin"),
            OpcodeKind::Slct => write!(f, "slct"),
            OpcodeKind::Sqrt => write!(f, "sqrt"),
            OpcodeKind::Stackrestore => write!(f, "stackrestore"),
            OpcodeKind::Stacksave => write!(f, "stacksave"),
            OpcodeKind::St => write!(f, "st"),
            OpcodeKind::Stmatrix => write!(f, "stmatrix"),
            OpcodeKind::Sub => write!(f, "sub"),
            OpcodeKind::Subc => write!(f, "subc"),
            OpcodeKind::Suq => write!(f, "suq"),
            OpcodeKind::Suld => write!(f, "suld"),
            OpcodeKind::Sured => write!(f, "sured"),
            OpcodeKind::Sust => write!(f, "sust"),
            OpcodeKind::Szext => write!(f, "szext"),
            OpcodeKind::Tanh => write!(f, "tanh"),
            OpcodeKind::Tcgen05 => write!(f, "tcgen05"),
            OpcodeKind::Tensormap => write!(f, "tensormap"),
            OpcodeKind::Tex => write!(f, "tex"),
            OpcodeKind::Testp => write!(f, "testp"),
            OpcodeKind::Tld4 => write!(f, "tld4"),
            OpcodeKind::Trap => write!(f, "trap"),
            OpcodeKind::Txq => write!(f, "txq"),
            OpcodeKind::Vabsdiff => write!(f, "vabsdiff"),
            OpcodeKind::Vabsdiff2 => write!(f, "vabsdiff2"),
            OpcodeKind::Vabsdiff4 => write!(f, "vabsdiff4"),
            OpcodeKind::Vadd => write!(f, "vadd"),
            OpcodeKind::Vadd2 => write!(f, "vadd2"),
            OpcodeKind::Vadd4 => write!(f, "vadd4"),
            OpcodeKind::Vavrg2 => write!(f, "vavrg2"),
            OpcodeKind::Vavrg4 => write!(f, "vavrg4"),
            OpcodeKind::Vmad => write!(f, "vmad"),
            OpcodeKind::Vmax => write!(f, "vmax"),
            OpcodeKind::Vmax2 => write!(f, "vmax2"),
            OpcodeKind::Vmax4 => write!(f, "vmax4"),
            OpcodeKind::Vmin => write!(f, "vmin"),
            OpcodeKind::Vmin2 => write!(f, "vmin2"),
            OpcodeKind::Vmin4 => write!(f, "vmin4"),
            OpcodeKind::Vset => write!(f, "vset"),
            OpcodeKind::Vset2 => write!(f, "vset2"),
            OpcodeKind::Vset4 => write!(f, "vset4"),
            OpcodeKind::Vshl => write!(f, "vshl"),
            OpcodeKind::Vshr => write!(f, "vshr"),
            OpcodeKind::Vsub => write!(f, "vsub"),
            OpcodeKind::Vsub2 => write!(f, "vsub2"),
            OpcodeKind::Vsub4 => write!(f, "vsub4"),
            OpcodeKind::Vote => write!(f, "vote"),
            OpcodeKind::Wgmma => write!(f, "wgmma"),
            OpcodeKind::Wmma => write!(f, "wmma"),
            OpcodeKind::Xor => write!(f, "xor"),
            OpcodeKind::Ret => write!(f, "ret"),
        }
    }
}

impl fmt::Display for InstructionOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl fmt::Display for TypeModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeModifier::F16 => write!(f, "f16"),
            TypeModifier::F32 => write!(f, "f32"),
            TypeModifier::F64 => write!(f, "f64"),
            TypeModifier::F128 => write!(f, "f128"),
            TypeModifier::B8 => write!(f, "b8"),
            TypeModifier::B16 => write!(f, "b16"),
            TypeModifier::B32 => write!(f, "b32"),
            TypeModifier::B64 => write!(f, "b64"),
            TypeModifier::S8 => write!(f, "s8"),
            TypeModifier::S16 => write!(f, "s16"),
            TypeModifier::S32 => write!(f, "s32"),
            TypeModifier::S64 => write!(f, "s64"),
            TypeModifier::U8 => write!(f, "u8"),
            TypeModifier::U16 => write!(f, "u16"),
            TypeModifier::U32 => write!(f, "u32"),
            TypeModifier::U64 => write!(f, "u64"),
            TypeModifier::Pred => write!(f, "pred"),
        }
    }
}

impl fmt::Display for ConditionModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConditionModifier::Eq => write!(f, "eq"),
            ConditionModifier::Ne => write!(f, "ne"),
            ConditionModifier::Lt => write!(f, "lt"),
            ConditionModifier::Le => write!(f, "le"),
            ConditionModifier::Gt => write!(f, "gt"),
            ConditionModifier::Ge => write!(f, "ge"),
            ConditionModifier::Lo => write!(f, "lo"),
            ConditionModifier::Hi => write!(f, "hi"),
            ConditionModifier::Ls => write!(f, "ls"),
            ConditionModifier::Hs => write!(f, "hs"),
        }
    }
}

impl fmt::Display for DataDirectiveKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataDirectiveKind::B8 => write!(f, "b8"),
            DataDirectiveKind::B16 => write!(f, "b16"),
            DataDirectiveKind::B32 => write!(f, "b32"),
            DataDirectiveKind::B64 => write!(f, "b64"),
        }
    }
}

impl fmt::Display for FunctionVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionVisibility::Visible => write!(f, "visible"),
            FunctionVisibility::Hidden => write!(f, "hidden"),
        }
    }
}

impl fmt::Display for FunctionLinkage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionLinkage::Extern => write!(f, "extern"),
            FunctionLinkage::Weak => write!(f, "weak"),
            FunctionLinkage::WeakExtern => write!(f, "weak_extern"),
        }
    }
}

impl fmt::Display for PointerAddressSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PointerAddressSpace::Generic => write!(f, "generic"),
            PointerAddressSpace::Global => write!(f, "global"),
            PointerAddressSpace::Shared => write!(f, "shared"),
            PointerAddressSpace::Local => write!(f, "local"),
            PointerAddressSpace::Const => write!(f, "const"),
        }
    }
}

impl fmt::Display for RoundingModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoundingModifier::Rn => write!(f, "rn"),
            RoundingModifier::Rz => write!(f, "rz"),
            RoundingModifier::Rm => write!(f, "rm"),
            RoundingModifier::Rp => write!(f, "rp"),
        }
    }
}

impl fmt::Display for GlobalVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlobalVisibility::Visible => write!(f, "visible"),
            GlobalVisibility::Hidden => write!(f, "hidden"),
        }
    }
}

impl fmt::Display for GlobalLinkage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlobalLinkage::Extern => write!(f, "extern"),
            GlobalLinkage::Weak => write!(f, "weak"),
            GlobalLinkage::WeakExtern => write!(f, "weak_extern"),
        }
    }
}

impl fmt::Display for GlobalAddressSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlobalAddressSpace::Global => write!(f, "global"),
            GlobalAddressSpace::Const => write!(f, "const"),
            GlobalAddressSpace::Shared => write!(f, "shared"),
            GlobalAddressSpace::Local => write!(f, "local"),
            GlobalAddressSpace::Param => write!(f, "param"),
        }
    }
}

impl fmt::Display for GlobalMutability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlobalMutability::Const => write!(f, "const"),
            GlobalMutability::Volatile => write!(f, "volatile"),
        }
    }
}

impl fmt::Display for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalarType::B8 => write!(f, "b8"),
            ScalarType::B16 => write!(f, "b16"),
            ScalarType::B32 => write!(f, "b32"),
            ScalarType::B64 => write!(f, "b64"),
            ScalarType::S8 => write!(f, "s8"),
            ScalarType::S16 => write!(f, "s16"),
            ScalarType::S32 => write!(f, "s32"),
            ScalarType::S64 => write!(f, "s64"),
            ScalarType::U8 => write!(f, "u8"),
            ScalarType::U16 => write!(f, "u16"),
            ScalarType::U32 => write!(f, "u32"),
            ScalarType::U64 => write!(f, "u64"),
            ScalarType::F16 => write!(f, "f16"),
            ScalarType::F32 => write!(f, "f32"),
            ScalarType::F64 => write!(f, "f64"),
            ScalarType::Pred => write!(f, "pred"),
            ScalarType::TexRef => write!(f, "texref"),
            ScalarType::SamplerRef => write!(f, "samplerref"),
            ScalarType::SurfRef => write!(f, "surfref"),
        }
    }
}

impl fmt::Display for MathModeModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathModeModifier::Approx => write!(f, "approx"),
            MathModeModifier::Full => write!(f, "full"),
        }
    }
}

impl fmt::Display for SynchronizationModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SynchronizationModifier::Sync => write!(f, "sync"),
            SynchronizationModifier::Async => write!(f, "async"),
        }
    }
}

impl fmt::Display for AsyncGroupModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsyncGroupModifier::CommitGroup => write!(f, "commit_group"),
            AsyncGroupModifier::WaitGroup => write!(f, "wait_group"),
        }
    }
}

impl fmt::Display for ShuffleModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShuffleModifier::Bfly => write!(f, "bfly"),
            ShuffleModifier::Down => write!(f, "down"),
            ShuffleModifier::Up => write!(f, "up"),
            ShuffleModifier::Idx => write!(f, "idx"),
        }
    }
}

impl fmt::Display for CacheModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheModifier::Nc => write!(f, "nc"),
            CacheModifier::Ca => write!(f, "ca"),
            CacheModifier::Cg => write!(f, "cg"),
            CacheModifier::Cs => write!(f, "cs"),
            CacheModifier::Lu => write!(f, "lu"),
        }
    }
}

impl fmt::Display for MemoryScopeModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryScopeModifier::Cta => write!(f, "cta"),
            MemoryScopeModifier::Gl => write!(f, "gl"),
            MemoryScopeModifier::Gpu => write!(f, "gpu"),
            MemoryScopeModifier::Sys => write!(f, "sys"),
        }
    }
}

impl fmt::Display for AtomicOperationModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomicOperationModifier::Cas => write!(f, "cas"),
            AtomicOperationModifier::Add => write!(f, "add"),
            AtomicOperationModifier::Inc => write!(f, "inc"),
            AtomicOperationModifier::Dec => write!(f, "dec"),
            AtomicOperationModifier::Exch => write!(f, "exch"),
            AtomicOperationModifier::Min => write!(f, "min"),
            AtomicOperationModifier::Max => write!(f, "max"),
            AtomicOperationModifier::And => write!(f, "and"),
            AtomicOperationModifier::Or => write!(f, "or"),
            AtomicOperationModifier::Xor => write!(f, "xor"),
        }
    }
}

impl fmt::Display for CallModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallModifier::Uni => write!(f, "uni"),
        }
    }
}

impl fmt::Display for MemoryOrderModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryOrderModifier::Relaxed => write!(f, "relaxed"),
            MemoryOrderModifier::Acquire => write!(f, "acquire"),
            MemoryOrderModifier::Release => write!(f, "release"),
            MemoryOrderModifier::AcqRel => write!(f, "acq_rel"),
            MemoryOrderModifier::Sc => write!(f, "sc"),
        }
    }
}

impl FunctionHeaderDirective {
    pub fn name(&self) -> &'static str {
        match self {
            FunctionHeaderDirective::Visibility(_) => "visibility",
            FunctionHeaderDirective::Linkage(_) => "linkage",
            FunctionHeaderDirective::NoReturn => "noreturn",
            FunctionHeaderDirective::AbiPreserve(_) => "abi_preserve",
            FunctionHeaderDirective::AbiPreserveControl(_) => "abi_preserve_control",
            FunctionHeaderDirective::MaxClusterRank(_) => "maxclusterrank",
            FunctionHeaderDirective::BlocksAreClusters => "blocksareclusters",
            FunctionHeaderDirective::ExplicitCluster(_) => "explicitcluster",
            FunctionHeaderDirective::ReqNctaPerCluster(_) => "reqnctapercluster",
            FunctionHeaderDirective::MaxNReg(_) => "maxnreg",
            FunctionHeaderDirective::MaxNTid(_) => "maxntid",
            FunctionHeaderDirective::MinNCtaPerSm(_) => "minnctapersm",
            FunctionHeaderDirective::ReqNTid(_) => "reqntid",
            FunctionHeaderDirective::MaxNCtaPerSm(_) => "maxnctapersm",
            FunctionHeaderDirective::Pragma(_) => "pragma",
        }
    }
}
