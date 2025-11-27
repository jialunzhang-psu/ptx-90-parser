// Auto-generated module declarations
// DO NOT EDIT MANUALLY
#![allow(unused)]

use crate::lexer::PtxToken;
use crate::unparser::PtxUnparser;
use crate::r#type::instruction::Inst;

pub mod abs;
pub mod activemask;
pub mod add_cc;
pub mod add;
pub mod addc;
pub mod alloca;
pub mod and;
pub mod applypriority;
pub mod atom;
pub mod bar;
pub mod bar_warp_sync;
pub mod barrier_cluster;
pub mod bfe;
pub mod bfi;
pub mod bfind;
pub mod bmsk;
pub mod bra;
pub mod brev;
pub mod brkpt;
pub mod brx_idx;
pub mod call;
pub mod clusterlaunchcontrol_query_cancel;
pub mod clusterlaunchcontrol_try_cancel;
pub mod clz;
pub mod cnot;
pub mod copysign;
pub mod cos;
pub mod cp_async_bulk_commit_group;
pub mod cp_async_bulk_prefetch_tensor;
pub mod cp_async_bulk_prefetch;
pub mod cp_async_bulk_tensor;
pub mod cp_async_bulk;
pub mod cp_async_bulk_wait_group;
pub mod cp_async_commit_group;
pub mod cp_async_mbarrier_arrive;
pub mod cp_async;
pub mod cp_async_wait_group;
pub mod cp_reduce_async_bulk_tensor;
pub mod cp_reduce_async_bulk;
pub mod createpolicy;
pub mod cvt_pack;
pub mod cvt;
pub mod cvta;
pub mod discard;
pub mod div;
pub mod dp2a;
pub mod dp4a;
pub mod elect_sync;
pub mod ex2;
pub mod exit;
pub mod fma;
pub mod fns;
pub mod getctarank;
pub mod griddepcontrol;
pub mod isspacep;
pub mod istypep;
pub mod ld_global_nc;
pub mod ld;
pub mod ldmatrix;
pub mod ldu;
pub mod lg2;
pub mod lop3;
pub mod mad_cc;
pub mod mad;
pub mod mad24;
pub mod madc;
pub mod mapa;
pub mod match_sync;
pub mod max;
pub mod mbarrier_arrive;
pub mod mbarrier_arrive_drop;
pub mod mbarrier_complete_tx;
pub mod mbarrier_expect_tx;
pub mod mbarrier_init;
pub mod mbarrier_inval;
pub mod mbarrier_pending_count;
pub mod mbarrier_test_wait;
pub mod membar;
pub mod min;
pub mod mma_sp;
pub mod mma;
pub mod mov;
pub mod movmatrix;
pub mod mul;
pub mod mul24;
pub mod multimem_ld_reduce;
pub mod nanosleep;
pub mod neg;
pub mod not;
pub mod or;
pub mod pmevent;
pub mod popc;
pub mod prefetch;
pub mod prmt;
pub mod rcp_approx_ftz_f64;
pub mod rcp;
pub mod red_async;
pub mod red;
pub mod redux_sync;
pub mod rem;
pub mod ret;
pub mod rsqrt_approx_ftz_f64;
pub mod rsqrt;
pub mod sad;
pub mod selp;
pub mod set;
pub mod setmaxnreg;
pub mod setp;
pub mod shf;
pub mod shfl_sync;
pub mod shfl;
pub mod shl;
pub mod shr;
pub mod sin;
pub mod slct;
pub mod sqrt;
pub mod st_async;
pub mod st_bulk;
pub mod st;
pub mod stackrestore;
pub mod stacksave;
pub mod stmatrix;
pub mod sub_cc;
pub mod sub;
pub mod subc;
pub mod suld;
pub mod suq;
pub mod sured;
pub mod sust;
pub mod szext;
pub mod tanh;
pub mod tcgen05_alloc;
pub mod tcgen05_commit;
pub mod tcgen05_cp;
pub mod tcgen05_fence;
pub mod tcgen05_ld;
pub mod tcgen05_mma_sp;
pub mod tcgen05_mma;
pub mod tcgen05_mma_ws_sp;
pub mod tcgen05_mma_ws;
pub mod tcgen05_shift;
pub mod tcgen05_st;
pub mod tcgen05_wait;
pub mod tensormap_cp_fenceproxy;
pub mod tensormap_replace;
pub mod testp;
pub mod tex;
pub mod tld4;
pub mod trap;
pub mod txq;
pub mod vmad;
pub mod vop;
pub mod vop2;
pub mod vop4;
pub mod vote_sync;
pub mod vote;
pub mod vset;
pub mod vset2;
pub mod vset4;
pub mod vsh;
pub mod wgmma_commit_group;
pub mod wgmma_fence;
pub mod wgmma_mma_async_sp;
pub mod wgmma_mma_async;
pub mod wgmma_wait_group;
pub mod wmma_load;
pub mod wmma_mma;
pub mod wmma_store;
pub mod xor;

impl PtxUnparser for Inst {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        self.unparse_tokens_mode(tokens, false);
    }
    fn unparse_tokens_mode(&self, tokens: &mut Vec<PtxToken>, spaced: bool) {
        match self {
            Inst::AbsType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AbsFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AbsF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AbsFtzF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AbsFtzF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AbsBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AbsBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ActivemaskB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AddCcType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AddType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AddSatS32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AddRndFtzSatF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AddRndFtzF32x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AddRndF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AddRndFtzSatF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AddRndFtzSatF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AddRndBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AddRndBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AddRndSatF32Atype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AddcCcType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AllocaType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AndType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ApplypriorityGlobalLevelEvictionPriority(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AtomSemScopeSpaceOpLevelCacheHintType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AtomSemScopeSpaceOpType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AtomSemScopeSpaceCasB16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AtomSemScopeSpaceCasB128(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AtomSemScopeSpaceExchLevelCacheHintB128(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AtomSemScopeSpaceAddNoftzLevelCacheHintF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AtomSemScopeSpaceAddNoftzLevelCacheHintBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AtomSemScopeGlobalAddLevelCacheHintVec32BitF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BarrierCtaSyncAligned(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BarrierCtaArriveAligned(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BarrierCtaRedPopcAlignedU32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BarrierCtaRedOpAlignedPred(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BarCtaSync(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BarCtaArrive(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BarCtaRedPopcU32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BarCtaRedOpPred(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BarWarpSync(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BarrierClusterArriveSemAligned(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BarrierClusterWaitAcquireAligned(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BfeType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BfiType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BfindType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BfindShiftamtType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BmskModeB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BraUni(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BraUni1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BrevType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Brkpt(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BrxIdxUni(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::BrxIdxUni1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CallUni(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CallUni1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CallUni2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CallUni3(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CallUni4(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CallUni5(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CallUni6(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CallUni7(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CallUni8(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ClusterlaunchcontrolQueryCancelIsCanceledPredB128(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ClzType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CnotType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CopysignType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CosApproxFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncBulkCommitGroup(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncBulkPrefetchL2SrcLevelCacheHint(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncBulkDstSrcCompletionMechanismLevelCacheHint(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncBulkDstSrcCompletionMechanismMulticastLevelCacheHint(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncBulkDstSrcCompletionMechanism(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncBulkDstSrcCompletionMechanismLevelCacheHintCpMask(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncBulkWaitGroupRead(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncCommitGroup(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncMbarrierArriveNoincStateB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncWaitGroup(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpAsyncWaitAll(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpReduceAsyncBulkTensorDimDstSrcRedopLoadModeCompletionMechanismLevelCacheHint(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpReduceAsyncBulkDstSrcCompletionMechanismRedopType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CreatepolicyCvtL2B64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtPackSatConverttypeAbtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtPackSatConverttypeAbtypeCtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtIrndFtzSatDtypeAtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtFrndFtzSatDtypeAtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtFrnd2ReluSatfiniteF16F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtFrnd2ReluSatfiniteF16x2F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRsReluSatfiniteF16x2F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtFrnd2ReluSatfiniteBf16F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtFrnd2ReluSatfiniteBf16x2F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRsReluSatfiniteBf16x2F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRnaSatfiniteTf32F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtFrnd2SatfiniteReluTf32F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRnSatfiniteReluF8x2typeF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRnSatfiniteReluF8x2typeF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRnReluF16x2F8x2type(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRsReluSatfiniteF8x4typeF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRnSatfiniteReluF4x2typeF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRnReluF16x2F4x2type(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRsReluSatfiniteF4x4typeF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRnSatfiniteReluF6x2typeF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRnReluF16x2F6x2type(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRsReluSatfiniteF6x4typeF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtFrnd3SatfiniteUe8m0x2F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtFrnd3SatfiniteUe8m0x2Bf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtRnBf16x2Ue8m0x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtaSpaceSize(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::CvtaToSpaceSize(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::DiscardGlobalLevel(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::DivType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::DivApproxFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::DivFullFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::DivRndFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::DivRndF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Dp2aModeAtypeBtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Dp4aAtypeBtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ElectSync(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Ex2ApproxFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Ex2ApproxAtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Ex2ApproxFtzBtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Exit(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FmaRndFtzSatF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FmaRndFtzF32x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FmaRndF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FmaRndFtzSatF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FmaRndFtzSatF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FmaRndFtzReluF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FmaRndFtzReluF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FmaRndReluBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FmaRndReluBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FmaRndOobReluType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FmaRndSatF32Abtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FnsB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::GetctarankSpaceType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::GetctarankSharedClusterType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::GetctarankType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::GriddepcontrolAction(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::IsspacepSpace(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::IstypepType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdWeakSsCopLevelCacheHintLevelPrefetchSizeVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdVolatileSsLevelPrefetchSizeVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdAcquireScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdMmioRelaxedSysGlobalType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdmatrixSyncAlignedShapeNumTransSsType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LduSsType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::LduSsVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Lg2ApproxFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Lop3B32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Lop3BoolopB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MadHiloCcType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MadModeType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MadHiSatS32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MadFtzSatF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MadRndFtzSatF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MadRndF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Mad24ModeType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Mad24HiSatS32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MadcHiloCcType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MapaSpaceType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MatchAnySyncType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MatchAllSyncType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MaxAtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MaxReluBtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MaxFtzNanXorsignAbsF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MaxFtzNanAbsF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MaxF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MaxFtzNanXorsignAbsF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MaxFtzNanXorsignAbsF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MaxNanXorsignAbsBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MaxNanXorsignAbsBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierArriveSemScopeStateB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierArriveSemScopeSharedClusterB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierArriveExpectTxSemScopeStateB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierArriveExpectTxSemScopeSharedClusterB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierArriveNocompleteReleaseCtaStateB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierArriveDropSemScopeStateB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierArriveDropSemScopeSharedClusterB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierArriveDropExpectTxStateSemScopeB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierArriveDropExpectTxSharedClusterSemScopeB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierArriveDropNocompleteReleaseCtaStateB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierCompleteTxSemScopeSpaceB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierExpectTxSemScopeSpaceB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierInitStateB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierInvalStateB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierPendingCountB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierTestWaitSemScopeStateB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierTestWaitParitySemScopeStateB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierTryWaitSemScopeStateB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MbarrierTryWaitParitySemScopeStateB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FenceSemScope(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FenceAcquireSyncRestrictSharedClusterCluster(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FenceReleaseSyncRestrictSharedCtaCluster(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FenceOpRestrictReleaseCluster(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FenceProxyProxykind(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FenceProxyToProxykindFromProxykindReleaseScope(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FenceProxyToProxykindFromProxykindAcquireScope(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MembarLevel(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MembarProxyProxykind(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MinAtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MinReluBtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MinFtzNanXorsignAbsF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MinFtzNanAbsF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MinF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MinFtzNanXorsignAbsF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MinFtzNanXorsignAbsF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MinNanXorsignAbsBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MinNanXorsignAbsBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpvariantSyncAlignedM16n8k16RowColDtypeF16F16Ctype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpvariantSyncAlignedM16n8k32RowColDtypeF16F16Ctype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpvariantSyncAlignedM16n8k16RowColF32Bf16Bf16F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpvariantSyncAlignedM16n8k32RowColF32Bf16Bf16F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpvariantSyncAlignedM16n8k8RowColF32Tf32Tf32F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpvariantSyncAlignedM16n8k16RowColF32Tf32Tf32F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpvariantSyncAlignedM16n8k64RowColF32F8typeF8typeF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpOrderedMetadataSyncAlignedM16n8k64RowColKindDtypeF8f6f4typeF8f6f4typeCtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpvariantSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedShapeRowColF64F64F64F64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MovType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MovU32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MovU64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MovU321(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MovU641(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MovType1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MovmatrixSyncAlignedShapeTransType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MulModeType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MulRndFtzSatF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MulRndFtzF32x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MulRndF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MulRndFtzSatF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MulRndFtzSatF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MulRndBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MulRndBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Mul24ModeType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MultimemLdReduceLdsemScopeSsOpType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MultimemLdReduceWeakSsOpType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MultimemStStsemScopeSsType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MultimemStWeakSsType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MultimemRedRedsemScopeSsOpType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MultimemLdReduceLdsemScopeSsOpAccPrecVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MultimemLdReduceWeakSsOpAccPrecVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MultimemStStsemScopeSsVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MultimemStWeakSsVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::MultimemRedRedsemScopeSsRedopVecRedtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::NanosleepU32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::NegType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::NegFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::NegF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::NegFtzF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::NegFtzF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::NegBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::NegBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::NotType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::OrType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Pmevent(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::PmeventMask(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::PopcType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::PrefetchSpaceLevel(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::PrefetchGlobalLevelEvictionPriority(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::PrefetchuL1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::PrefetchTensormapSpaceTensormap(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::PrmtB32Mode(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RcpApproxFtzF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RcpApproxFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RcpRndFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RcpRndF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedAsyncSemScopeSsCompletionMechanismOpType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedAsyncSemScopeSsCompletionMechanismOpType1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedAsyncSemScopeSsCompletionMechanismOpType2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedAsyncSemScopeSsCompletionMechanismAddType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedAsyncMmioSemScopeSsAddType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedOpSpaceSemScopeLevelCacheHintType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedAddSpaceSemScopeNoftzLevelCacheHintF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedAddSpaceSemScopeNoftzLevelCacheHintF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedAddSpaceSemScopeNoftzLevelCacheHintBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedAddSpaceSemScopeNoftzLevelCacheHintBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedAddSpaceSemScopeLevelCacheHintVec32BitF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedOpSpaceSemScopeNoftzLevelCacheHintVec16BitHalfWordType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RedOpSpaceSemScopeNoftzLevelCacheHintVec32BitPackedType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ReduxSyncOpType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ReduxSyncOpB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ReduxSyncOpAbsNanF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RemType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RetUni(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RsqrtApproxFtzF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RsqrtApproxFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::RsqrtApproxF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SadType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SelpType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopFtzDtypeStype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopBoolopFtzDtypeStype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopFtzF16Stype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopBoolopFtzF16Stype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopBf16Stype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopBoolopBf16Stype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopFtzDtypeF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopBoolopFtzDtypeF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopDtypeBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopBoolopDtypeBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopFtzDtypeF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopBoolopFtzDtypeF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopDtypeBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetCmpopBoolopDtypeBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetmaxnregActionSyncAlignedU32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetpCmpopFtzType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetpCmpopBoolopFtzType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetpCmpopFtzF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetpCmpopBoolopFtzF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetpCmpopFtzF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetpCmpopBoolopFtzF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetpCmpopBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetpCmpopBoolopBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetpCmpopBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SetpCmpopBoolopBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ShfLModeB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ShfRModeB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ShflSyncModeB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ShflModeB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ShlType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::ShrType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SinApproxFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SlctDtypeS32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SlctFtzDtypeF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SqrtApproxFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SqrtRndFtzF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SqrtRndF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::StAsyncSemScopeSsCompletionMechanismVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::StAsyncMmioSemScopeSsType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::StBulkWeakSharedCta(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::StWeakSsCopLevelCacheHintVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::StWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::StVolatileSsVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::StRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::StReleaseScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::StMmioRelaxedSysGlobalType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::StackrestoreType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::StacksaveType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::StmatrixSyncAlignedShapeNumTransSsType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SubCcType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SubType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SubSatS32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SubRndFtzSatF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SubRndFtzF32x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SubRndF64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SubRndFtzSatF16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SubRndFtzSatF16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SubRndBf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SubRndBf16x2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SubRndSatF32Atype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SubcCcType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SuldBGeomCopVecDtypeMode(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SuqQueryB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SuredBOpGeomCtypeMode(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SuredPOpGeomCtypeMode(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SustBDimCopVecCtypeMode(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SustPDimVecB32Mode(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SustBAdimCopVecCtypeMode(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::SzextModeType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TanhApproxType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05DeallocCtaGroupSyncAlignedB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05RelinquishAllocPermitCtaGroupSyncAligned(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05CpCtaGroupShapeMulticastDstSrcFmt(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05FenceBeforeThreadSync(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05FenceAfterThreadSync(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05LdSyncAlignedShape1NumPackB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05LdSyncAlignedShape2NumPackB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05LdRedSyncAlignedShape3NumRedopType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05LdRedSyncAlignedShape4NumRedopType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKind(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKind1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKindCollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKindI8(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKindI81(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKindI8CollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKind(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKind1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKindCollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKindAshiftCollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKindAshiftCollectorUsage1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKindI8(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKindI81(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKindI8CollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaWsSpCtaGroup1KindCollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaWsSpCtaGroup1KindCollectorUsage1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaWsCtaGroup1KindCollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaWsCtaGroup1KindCollectorUsage1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05ShiftCtaGroupDown(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05StSyncAlignedShape1NumUnpackB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05StSyncAlignedShape2NumUnpackB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tcgen05WaitOperationSyncAligned(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TensormapReplaceModeField1SsB1024Type(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TensormapReplaceModeField2SsB1024Type(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TensormapReplaceModeField3SsB1024Type(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TestpOpType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TexGeomV4DtypeCtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TexGeomV4DtypeCtype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TexGeomV2F16x2Ctype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TexGeomV2F16x2Ctype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TexBaseGeomV4DtypeCtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TexLevelGeomV4DtypeCtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TexGradGeomV4DtypeCtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TexBaseGeomV2F16x2Ctype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TexLevelGeomV2F16x2Ctype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TexGradGeomV2F16x2Ctype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tld4Comp2dV4DtypeF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Tld4CompGeomV4DtypeF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Trap(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TxqTqueryB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TxqLevelTlqueryB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::TxqSqueryB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VmadDtypeAtypeBtypeSatScale(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VmadDtypeAtypeBtypePoSatScale(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VaddDtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VsubDtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VabsdiffDtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VminDtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VmaxDtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VaddDtypeAtypeBtypeSatOp2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VsubDtypeAtypeBtypeSatOp2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VabsdiffDtypeAtypeBtypeSatOp2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VminDtypeAtypeBtypeSatOp2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VmaxDtypeAtypeBtypeSatOp2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VaddDtypeAtypeBtypeSat1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VsubDtypeAtypeBtypeSat1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VabsdiffDtypeAtypeBtypeSat1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VminDtypeAtypeBtypeSat1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VmaxDtypeAtypeBtypeSat1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vadd2DtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vsub2DtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vavrg2DtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vabsdiff2DtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vmin2DtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vmax2DtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vadd2DtypeAtypeBtypeAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vsub2DtypeAtypeBtypeAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vavrg2DtypeAtypeBtypeAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vabsdiff2DtypeAtypeBtypeAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vmin2DtypeAtypeBtypeAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vmax2DtypeAtypeBtypeAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vadd4DtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vsub4DtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vavrg4DtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vabsdiff4DtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vmin4DtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vmax4DtypeAtypeBtypeSat(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vadd4DtypeAtypeBtypeAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vsub4DtypeAtypeBtypeAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vavrg4DtypeAtypeBtypeAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vabsdiff4DtypeAtypeBtypeAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vmin4DtypeAtypeBtypeAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vmax4DtypeAtypeBtypeAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VoteSyncModePred(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VoteSyncBallotB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VoteModePred(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VoteBallotB32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VsetAtypeBtypeCmp(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VsetAtypeBtypeCmpOp2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VsetAtypeBtypeCmp1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vset2AtypeBtypeCmp(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vset2AtypeBtypeCmpAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vset4AtypeBtypeCmp(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::Vset4AtypeBtypeCmpAdd(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VshlDtypeAtypeU32SatMode(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VshrDtypeAtypeU32SatMode(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VshlDtypeAtypeU32SatModeOp2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VshrDtypeAtypeU32SatModeOp2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VshlDtypeAtypeU32SatMode1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::VshrDtypeAtypeU32SatMode1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaCommitGroupSyncAligned(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaFenceSyncAligned(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F161(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf161(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf321(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeF16F16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeF16F161(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf16(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf161(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf321(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WgmmaWaitGroupSyncAligned(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadASyncAlignedLayoutShapeSsAtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadBSyncAlignedLayoutShapeSsBtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadCSyncAlignedLayoutShapeSsCtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadASyncAlignedLayoutShapeSsAtype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadBSyncAlignedLayoutShapeSsBtype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadCSyncAlignedLayoutShapeSsCtype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadASyncAlignedLayoutShapeSsAtype2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadBSyncAlignedLayoutShapeSsBtype2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadCSyncAlignedLayoutShapeSsCtype2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadASyncAlignedLayoutShapeSsAtype3(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadBSyncAlignedLayoutShapeSsBtype3(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadCSyncAlignedLayoutShapeSsCtype3(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadASyncAlignedRowShapeSsAtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadBSyncAlignedColShapeSsBtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadCSyncAlignedLayoutShapeSsCtype4(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadASyncAlignedRowShapeSsAtype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadBSyncAlignedColShapeSsBtype1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaLoadCSyncAlignedLayoutShapeSsCtype5(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaStoreDSyncAlignedLayoutShapeSsType(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaStoreDSyncAlignedLayoutShapeSsType1(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaStoreDSyncAlignedLayoutShapeSsType2(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::WmmaStoreDSyncAlignedLayoutShapeSsType3(value) => value.unparse_tokens_mode(tokens, spaced),
            Inst::XorType(value) => value.unparse_tokens_mode(tokens, spaced),
        }
    }
}
