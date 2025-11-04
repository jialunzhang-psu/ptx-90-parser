// Auto-generated module declarations
// DO NOT EDIT MANUALLY
#![allow(unused)]

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

/// Top-level instruction type encompassing all PTX instructions
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    AbsType(abs::section_0::AbsType),
    AbsFtzF32(abs::section_0::AbsFtzF32),
    AbsF64(abs::section_0::AbsF64),
    AbsFtzF16(abs::section_0::AbsFtzF16),
    AbsFtzF16x2(abs::section_0::AbsFtzF16x2),
    AbsBf16(abs::section_0::AbsBf16),
    AbsBf16x2(abs::section_0::AbsBf16x2),
    ActivemaskB32(activemask::section_0::ActivemaskB32),
    AddCcType(add_cc::section_0::AddCcType),
    AddType(add::section_0::AddType),
    AddSatS32(add::section_0::AddSatS32),
    AddRndFtzSatF32(add::section_1::AddRndFtzSatF32),
    AddRndFtzF32x2(add::section_1::AddRndFtzF32x2),
    AddRndF64(add::section_1::AddRndF64),
    AddRndFtzSatF16(add::section_2::AddRndFtzSatF16),
    AddRndFtzSatF16x2(add::section_2::AddRndFtzSatF16x2),
    AddRndBf16(add::section_2::AddRndBf16),
    AddRndBf16x2(add::section_2::AddRndBf16x2),
    AddRndSatF32Atype(add::section_3::AddRndSatF32Atype),
    AddcCcType(addc::section_0::AddcCcType),
    AllocaType(alloca::section_0::AllocaType),
    AndType(and::section_0::AndType),
    ApplypriorityGlobalLevelEvictionPriority(applypriority::section_0::ApplypriorityGlobalLevelEvictionPriority),
    AtomSemScopeSpaceOpLevelCacheHintType(atom::section_0::AtomSemScopeSpaceOpLevelCacheHintType),
    AtomSemScopeSpaceOpType(atom::section_0::AtomSemScopeSpaceOpType),
    AtomSemScopeSpaceCasB16(atom::section_0::AtomSemScopeSpaceCasB16),
    AtomSemScopeSpaceCasB128(atom::section_0::AtomSemScopeSpaceCasB128),
    AtomSemScopeSpaceExchLevelCacheHintB128(atom::section_0::AtomSemScopeSpaceExchLevelCacheHintB128),
    AtomSemScopeSpaceAddNoftzLevelCacheHintF16(atom::section_0::AtomSemScopeSpaceAddNoftzLevelCacheHintF16),
    AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2(atom::section_0::AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2),
    AtomSemScopeSpaceAddNoftzLevelCacheHintBf16(atom::section_0::AtomSemScopeSpaceAddNoftzLevelCacheHintBf16),
    AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2(atom::section_0::AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2),
    AtomSemScopeGlobalAddLevelCacheHintVec32BitF32(atom::section_1::AtomSemScopeGlobalAddLevelCacheHintVec32BitF32),
    AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType(atom::section_1::AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType),
    AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType(atom::section_1::AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType),
    BarrierCtaSyncAligned(bar::section_0::BarrierCtaSyncAligned),
    BarrierCtaArriveAligned(bar::section_0::BarrierCtaArriveAligned),
    BarrierCtaRedPopcAlignedU32(bar::section_0::BarrierCtaRedPopcAlignedU32),
    BarrierCtaRedOpAlignedPred(bar::section_0::BarrierCtaRedOpAlignedPred),
    BarCtaSync(bar::section_0::BarCtaSync),
    BarCtaArrive(bar::section_0::BarCtaArrive),
    BarCtaRedPopcU32(bar::section_0::BarCtaRedPopcU32),
    BarCtaRedOpPred(bar::section_0::BarCtaRedOpPred),
    BarWarpSync(bar_warp_sync::section_0::BarWarpSync),
    BarrierClusterArriveSemAligned(barrier_cluster::section_0::BarrierClusterArriveSemAligned),
    BarrierClusterWaitAcquireAligned(barrier_cluster::section_0::BarrierClusterWaitAcquireAligned),
    BfeType(bfe::section_0::BfeType),
    BfiType(bfi::section_0::BfiType),
    BfindType(bfind::section_0::BfindType),
    BfindShiftamtType(bfind::section_0::BfindShiftamtType),
    BmskModeB32(bmsk::section_0::BmskModeB32),
    BraUni(bra::section_0::BraUni),
    BraUni1(bra::section_0::BraUni1),
    BrevType(brev::section_0::BrevType),
    Brkpt(brkpt::section_0::Brkpt),
    BrxIdxUni(brx_idx::section_0::BrxIdxUni),
    BrxIdxUni1(brx_idx::section_0::BrxIdxUni1),
    CallUni(call::section_0::CallUni),
    CallUni1(call::section_0::CallUni1),
    CallUni2(call::section_0::CallUni2),
    CallUni3(call::section_0::CallUni3),
    CallUni4(call::section_0::CallUni4),
    CallUni5(call::section_0::CallUni5),
    CallUni6(call::section_0::CallUni6),
    CallUni7(call::section_0::CallUni7),
    CallUni8(call::section_0::CallUni8),
    ClusterlaunchcontrolQueryCancelIsCanceledPredB128(clusterlaunchcontrol_query_cancel::section_0::ClusterlaunchcontrolQueryCancelIsCanceledPredB128),
    ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128(clusterlaunchcontrol_query_cancel::section_0::ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128),
    ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128(clusterlaunchcontrol_query_cancel::section_0::ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128),
    ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128(clusterlaunchcontrol_try_cancel::section_0::ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128),
    ClzType(clz::section_0::ClzType),
    CnotType(cnot::section_0::CnotType),
    CopysignType(copysign::section_0::CopysignType),
    CosApproxFtzF32(cos::section_0::CosApproxFtzF32),
    CpAsyncBulkCommitGroup(cp_async_bulk_commit_group::section_0::CpAsyncBulkCommitGroup),
    CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint(cp_async_bulk_prefetch_tensor::section_0::CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint),
    CpAsyncBulkPrefetchL2SrcLevelCacheHint(cp_async_bulk_prefetch::section_0::CpAsyncBulkPrefetchL2SrcLevelCacheHint),
    CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint(cp_async_bulk_tensor::section_0::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint),
    CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint(cp_async_bulk_tensor::section_1::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint),
    CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint(cp_async_bulk_tensor::section_2::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint),
    CpAsyncBulkDstSrcCompletionMechanismLevelCacheHint(cp_async_bulk::section_0::CpAsyncBulkDstSrcCompletionMechanismLevelCacheHint),
    CpAsyncBulkDstSrcCompletionMechanismMulticastLevelCacheHint(cp_async_bulk::section_1::CpAsyncBulkDstSrcCompletionMechanismMulticastLevelCacheHint),
    CpAsyncBulkDstSrcCompletionMechanism(cp_async_bulk::section_2::CpAsyncBulkDstSrcCompletionMechanism),
    CpAsyncBulkDstSrcCompletionMechanismLevelCacheHintCpMask(cp_async_bulk::section_3::CpAsyncBulkDstSrcCompletionMechanismLevelCacheHintCpMask),
    CpAsyncBulkWaitGroupRead(cp_async_bulk_wait_group::section_0::CpAsyncBulkWaitGroupRead),
    CpAsyncCommitGroup(cp_async_commit_group::section_0::CpAsyncCommitGroup),
    CpAsyncMbarrierArriveNoincStateB64(cp_async_mbarrier_arrive::section_0::CpAsyncMbarrierArriveNoincStateB64),
    CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize(cp_async::section_0::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize),
    CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize(cp_async::section_0::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize),
    CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1(cp_async::section_0::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1),
    CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1(cp_async::section_0::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1),
    CpAsyncWaitGroup(cp_async_wait_group::section_0::CpAsyncWaitGroup),
    CpAsyncWaitAll(cp_async_wait_group::section_0::CpAsyncWaitAll),
    CpReduceAsyncBulkTensorDimDstSrcRedopLoadModeCompletionMechanismLevelCacheHint(cp_reduce_async_bulk_tensor::section_0::CpReduceAsyncBulkTensorDimDstSrcRedopLoadModeCompletionMechanismLevelCacheHint),
    CpReduceAsyncBulkDstSrcCompletionMechanismRedopType(cp_reduce_async_bulk::section_0::CpReduceAsyncBulkDstSrcCompletionMechanismRedopType),
    CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType(cp_reduce_async_bulk::section_1::CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType),
    CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType(cp_reduce_async_bulk::section_2::CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType),
    CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64(createpolicy::section_0::CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64),
    CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64(createpolicy::section_0::CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64),
    CreatepolicyCvtL2B64(createpolicy::section_0::CreatepolicyCvtL2B64),
    CvtPackSatConverttypeAbtype(cvt_pack::section_0::CvtPackSatConverttypeAbtype),
    CvtPackSatConverttypeAbtypeCtype(cvt_pack::section_1::CvtPackSatConverttypeAbtypeCtype),
    CvtIrndFtzSatDtypeAtype(cvt::section_0::CvtIrndFtzSatDtypeAtype),
    CvtFrndFtzSatDtypeAtype(cvt::section_0::CvtFrndFtzSatDtypeAtype),
    CvtFrnd2ReluSatfiniteF16F32(cvt::section_0::CvtFrnd2ReluSatfiniteF16F32),
    CvtFrnd2ReluSatfiniteF16x2F32(cvt::section_0::CvtFrnd2ReluSatfiniteF16x2F32),
    CvtRsReluSatfiniteF16x2F32(cvt::section_0::CvtRsReluSatfiniteF16x2F32),
    CvtFrnd2ReluSatfiniteBf16F32(cvt::section_0::CvtFrnd2ReluSatfiniteBf16F32),
    CvtFrnd2ReluSatfiniteBf16x2F32(cvt::section_0::CvtFrnd2ReluSatfiniteBf16x2F32),
    CvtRsReluSatfiniteBf16x2F32(cvt::section_0::CvtRsReluSatfiniteBf16x2F32),
    CvtRnaSatfiniteTf32F32(cvt::section_0::CvtRnaSatfiniteTf32F32),
    CvtFrnd2SatfiniteReluTf32F32(cvt::section_0::CvtFrnd2SatfiniteReluTf32F32),
    CvtRnSatfiniteReluF8x2typeF32(cvt::section_0::CvtRnSatfiniteReluF8x2typeF32),
    CvtRnSatfiniteReluF8x2typeF16x2(cvt::section_0::CvtRnSatfiniteReluF8x2typeF16x2),
    CvtRnReluF16x2F8x2type(cvt::section_0::CvtRnReluF16x2F8x2type),
    CvtRsReluSatfiniteF8x4typeF32(cvt::section_0::CvtRsReluSatfiniteF8x4typeF32),
    CvtRnSatfiniteReluF4x2typeF32(cvt::section_0::CvtRnSatfiniteReluF4x2typeF32),
    CvtRnReluF16x2F4x2type(cvt::section_0::CvtRnReluF16x2F4x2type),
    CvtRsReluSatfiniteF4x4typeF32(cvt::section_0::CvtRsReluSatfiniteF4x4typeF32),
    CvtRnSatfiniteReluF6x2typeF32(cvt::section_0::CvtRnSatfiniteReluF6x2typeF32),
    CvtRnReluF16x2F6x2type(cvt::section_0::CvtRnReluF16x2F6x2type),
    CvtRsReluSatfiniteF6x4typeF32(cvt::section_0::CvtRsReluSatfiniteF6x4typeF32),
    CvtFrnd3SatfiniteUe8m0x2F32(cvt::section_0::CvtFrnd3SatfiniteUe8m0x2F32),
    CvtFrnd3SatfiniteUe8m0x2Bf16x2(cvt::section_0::CvtFrnd3SatfiniteUe8m0x2Bf16x2),
    CvtRnBf16x2Ue8m0x2(cvt::section_0::CvtRnBf16x2Ue8m0x2),
    CvtaSpaceSize(cvta::section_0::CvtaSpaceSize),
    CvtaToSpaceSize(cvta::section_0::CvtaToSpaceSize),
    DiscardGlobalLevel(discard::section_0::DiscardGlobalLevel),
    DivType(div::section_0::DivType),
    DivApproxFtzF32(div::section_0::DivApproxFtzF32),
    DivFullFtzF32(div::section_0::DivFullFtzF32),
    DivRndFtzF32(div::section_0::DivRndFtzF32),
    DivRndF64(div::section_0::DivRndF64),
    Dp2aModeAtypeBtype(dp2a::section_0::Dp2aModeAtypeBtype),
    Dp4aAtypeBtype(dp4a::section_0::Dp4aAtypeBtype),
    ElectSync(elect_sync::section_0::ElectSync),
    Ex2ApproxFtzF32(ex2::section_0::Ex2ApproxFtzF32),
    Ex2ApproxAtype(ex2::section_0::Ex2ApproxAtype),
    Ex2ApproxFtzBtype(ex2::section_0::Ex2ApproxFtzBtype),
    Exit(exit::section_0::Exit),
    FmaRndFtzSatF32(fma::section_0::FmaRndFtzSatF32),
    FmaRndFtzF32x2(fma::section_0::FmaRndFtzF32x2),
    FmaRndF64(fma::section_0::FmaRndF64),
    FmaRndFtzSatF16(fma::section_1::FmaRndFtzSatF16),
    FmaRndFtzSatF16x2(fma::section_1::FmaRndFtzSatF16x2),
    FmaRndFtzReluF16(fma::section_1::FmaRndFtzReluF16),
    FmaRndFtzReluF16x2(fma::section_1::FmaRndFtzReluF16x2),
    FmaRndReluBf16(fma::section_1::FmaRndReluBf16),
    FmaRndReluBf16x2(fma::section_1::FmaRndReluBf16x2),
    FmaRndOobReluType(fma::section_1::FmaRndOobReluType),
    FmaRndSatF32Abtype(fma::section_2::FmaRndSatF32Abtype),
    FnsB32(fns::section_0::FnsB32),
    GetctarankSpaceType(getctarank::section_0::GetctarankSpaceType),
    GetctarankSharedClusterType(getctarank::section_0::GetctarankSharedClusterType),
    GetctarankType(getctarank::section_0::GetctarankType),
    GriddepcontrolAction(griddepcontrol::section_0::GriddepcontrolAction),
    IsspacepSpace(isspacep::section_0::IsspacepSpace),
    IstypepType(istypep::section_0::IstypepType),
    LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType(ld_global_nc::section_0::LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType),
    LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType(ld_global_nc::section_0::LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType),
    LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType(ld_global_nc::section_0::LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType),
    LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(ld_global_nc::section_0::LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType),
    LdWeakSsCopLevelCacheHintLevelPrefetchSizeVecType(ld::section_0::LdWeakSsCopLevelCacheHintLevelPrefetchSizeVecType),
    LdWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(ld::section_0::LdWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType),
    LdVolatileSsLevelPrefetchSizeVecType(ld::section_0::LdVolatileSsLevelPrefetchSizeVecType),
    LdRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(ld::section_0::LdRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType),
    LdAcquireScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(ld::section_0::LdAcquireScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType),
    LdMmioRelaxedSysGlobalType(ld::section_0::LdMmioRelaxedSysGlobalType),
    LdmatrixSyncAlignedShapeNumTransSsType(ldmatrix::section_0::LdmatrixSyncAlignedShapeNumTransSsType),
    LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt(ldmatrix::section_0::LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt),
    LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt(ldmatrix::section_0::LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt),
    LduSsType(ldu::section_0::LduSsType),
    LduSsVecType(ldu::section_0::LduSsVecType),
    Lg2ApproxFtzF32(lg2::section_0::Lg2ApproxFtzF32),
    Lop3B32(lop3::section_0::Lop3B32),
    Lop3BoolopB32(lop3::section_0::Lop3BoolopB32),
    MadHiloCcType(mad_cc::section_0::MadHiloCcType),
    MadModeType(mad::section_0::MadModeType),
    MadHiSatS32(mad::section_0::MadHiSatS32),
    MadFtzSatF32(mad::section_0::MadFtzSatF32),
    MadRndFtzSatF32(mad::section_0::MadRndFtzSatF32),
    MadRndF64(mad::section_0::MadRndF64),
    Mad24ModeType(mad24::section_0::Mad24ModeType),
    Mad24HiSatS32(mad24::section_0::Mad24HiSatS32),
    MadcHiloCcType(madc::section_0::MadcHiloCcType),
    MapaSpaceType(mapa::section_0::MapaSpaceType),
    MatchAnySyncType(match_sync::section_0::MatchAnySyncType),
    MatchAllSyncType(match_sync::section_0::MatchAllSyncType),
    MaxAtype(max::section_0::MaxAtype),
    MaxReluBtype(max::section_0::MaxReluBtype),
    MaxFtzNanXorsignAbsF32(max::section_0::MaxFtzNanXorsignAbsF32),
    MaxFtzNanAbsF32(max::section_0::MaxFtzNanAbsF32),
    MaxF64(max::section_0::MaxF64),
    MaxFtzNanXorsignAbsF16(max::section_0::MaxFtzNanXorsignAbsF16),
    MaxFtzNanXorsignAbsF16x2(max::section_0::MaxFtzNanXorsignAbsF16x2),
    MaxNanXorsignAbsBf16(max::section_0::MaxNanXorsignAbsBf16),
    MaxNanXorsignAbsBf16x2(max::section_0::MaxNanXorsignAbsBf16x2),
    MbarrierArriveSemScopeStateB64(mbarrier_arrive::section_0::MbarrierArriveSemScopeStateB64),
    MbarrierArriveSemScopeSharedClusterB64(mbarrier_arrive::section_0::MbarrierArriveSemScopeSharedClusterB64),
    MbarrierArriveExpectTxSemScopeStateB64(mbarrier_arrive::section_0::MbarrierArriveExpectTxSemScopeStateB64),
    MbarrierArriveExpectTxSemScopeSharedClusterB64(mbarrier_arrive::section_0::MbarrierArriveExpectTxSemScopeSharedClusterB64),
    MbarrierArriveNocompleteReleaseCtaStateB64(mbarrier_arrive::section_0::MbarrierArriveNocompleteReleaseCtaStateB64),
    MbarrierArriveDropSemScopeStateB64(mbarrier_arrive_drop::section_0::MbarrierArriveDropSemScopeStateB64),
    MbarrierArriveDropSemScopeSharedClusterB64(mbarrier_arrive_drop::section_0::MbarrierArriveDropSemScopeSharedClusterB64),
    MbarrierArriveDropExpectTxStateSemScopeB64(mbarrier_arrive_drop::section_0::MbarrierArriveDropExpectTxStateSemScopeB64),
    MbarrierArriveDropExpectTxSharedClusterSemScopeB64(mbarrier_arrive_drop::section_0::MbarrierArriveDropExpectTxSharedClusterSemScopeB64),
    MbarrierArriveDropNocompleteReleaseCtaStateB64(mbarrier_arrive_drop::section_0::MbarrierArriveDropNocompleteReleaseCtaStateB64),
    MbarrierCompleteTxSemScopeSpaceB64(mbarrier_complete_tx::section_0::MbarrierCompleteTxSemScopeSpaceB64),
    MbarrierExpectTxSemScopeSpaceB64(mbarrier_expect_tx::section_0::MbarrierExpectTxSemScopeSpaceB64),
    MbarrierInitStateB64(mbarrier_init::section_0::MbarrierInitStateB64),
    MbarrierInvalStateB64(mbarrier_inval::section_0::MbarrierInvalStateB64),
    MbarrierPendingCountB64(mbarrier_pending_count::section_0::MbarrierPendingCountB64),
    MbarrierTestWaitSemScopeStateB64(mbarrier_test_wait::section_0::MbarrierTestWaitSemScopeStateB64),
    MbarrierTestWaitParitySemScopeStateB64(mbarrier_test_wait::section_0::MbarrierTestWaitParitySemScopeStateB64),
    MbarrierTryWaitSemScopeStateB64(mbarrier_test_wait::section_0::MbarrierTryWaitSemScopeStateB64),
    MbarrierTryWaitParitySemScopeStateB64(mbarrier_test_wait::section_0::MbarrierTryWaitParitySemScopeStateB64),
    FenceSemScope(membar::section_0::FenceSemScope),
    FenceAcquireSyncRestrictSharedClusterCluster(membar::section_0::FenceAcquireSyncRestrictSharedClusterCluster),
    FenceReleaseSyncRestrictSharedCtaCluster(membar::section_0::FenceReleaseSyncRestrictSharedCtaCluster),
    FenceOpRestrictReleaseCluster(membar::section_0::FenceOpRestrictReleaseCluster),
    FenceProxyProxykind(membar::section_0::FenceProxyProxykind),
    FenceProxyToProxykindFromProxykindReleaseScope(membar::section_0::FenceProxyToProxykindFromProxykindReleaseScope),
    FenceProxyToProxykindFromProxykindAcquireScope(membar::section_0::FenceProxyToProxykindFromProxykindAcquireScope),
    FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster(membar::section_0::FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster),
    FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster(membar::section_0::FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster),
    MembarLevel(membar::section_0::MembarLevel),
    MembarProxyProxykind(membar::section_0::MembarProxyProxykind),
    MinAtype(min::section_0::MinAtype),
    MinReluBtype(min::section_0::MinReluBtype),
    MinFtzNanXorsignAbsF32(min::section_0::MinFtzNanXorsignAbsF32),
    MinFtzNanAbsF32(min::section_0::MinFtzNanAbsF32),
    MinF64(min::section_0::MinF64),
    MinFtzNanXorsignAbsF16(min::section_0::MinFtzNanXorsignAbsF16),
    MinFtzNanXorsignAbsF16x2(min::section_0::MinFtzNanXorsignAbsF16x2),
    MinNanXorsignAbsBf16(min::section_0::MinNanXorsignAbsBf16),
    MinNanXorsignAbsBf16x2(min::section_0::MinNanXorsignAbsBf16x2),
    MmaSpvariantSyncAlignedM16n8k16RowColDtypeF16F16Ctype(mma_sp::section_0::MmaSpvariantSyncAlignedM16n8k16RowColDtypeF16F16Ctype),
    MmaSpvariantSyncAlignedM16n8k32RowColDtypeF16F16Ctype(mma_sp::section_0::MmaSpvariantSyncAlignedM16n8k32RowColDtypeF16F16Ctype),
    MmaSpvariantSyncAlignedM16n8k16RowColF32Bf16Bf16F32(mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k16RowColF32Bf16Bf16F32),
    MmaSpvariantSyncAlignedM16n8k32RowColF32Bf16Bf16F32(mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k32RowColF32Bf16Bf16F32),
    MmaSpvariantSyncAlignedM16n8k8RowColF32Tf32Tf32F32(mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k8RowColF32Tf32Tf32F32),
    MmaSpvariantSyncAlignedM16n8k16RowColF32Tf32Tf32F32(mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k16RowColF32Tf32Tf32F32),
    MmaSpvariantSyncAlignedM16n8k64RowColF32F8typeF8typeF32(mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k64RowColF32F8typeF8typeF32),
    MmaSpOrderedMetadataSyncAlignedM16n8k64RowColKindDtypeF8f6f4typeF8f6f4typeCtype(mma_sp::section_1::MmaSpOrderedMetadataSyncAlignedM16n8k64RowColKindDtypeF8f6f4typeF8f6f4typeCtype),
    MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype(mma_sp::section_2::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype),
    MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1(mma_sp::section_3::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1),
    MmaSpvariantSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype(mma_sp::section_4::MmaSpvariantSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype),
    MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32(mma_sp::section_5::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32),
    MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321(mma_sp::section_6::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321),
    MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype(mma::section_0::MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype),
    MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype(mma::section_0::MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype),
    MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype(mma::section_0::MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype),
    MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32(mma::section_1::MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32),
    MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32(mma::section_1::MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32),
    MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32(mma::section_1::MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32),
    MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype(mma::section_1::MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype),
    MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype(mma::section_1::MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype),
    MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype(mma::section_2::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype),
    MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1(mma::section_3::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1),
    MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype(mma::section_4::MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype),
    MmaSyncAlignedShapeRowColF64F64F64F64(mma::section_5::MmaSyncAlignedShapeRowColF64F64F64F64),
    MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32(mma::section_6::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32),
    MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321(mma::section_7::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321),
    MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc(mma::section_8::MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc),
    MovType(mov::section_0::MovType),
    MovU32(mov::section_0::MovU32),
    MovU64(mov::section_0::MovU64),
    MovU321(mov::section_0::MovU321),
    MovU641(mov::section_0::MovU641),
    MovType1(mov::section_1::MovType1),
    MovmatrixSyncAlignedShapeTransType(movmatrix::section_0::MovmatrixSyncAlignedShapeTransType),
    MulModeType(mul::section_0::MulModeType),
    MulRndFtzSatF32(mul::section_1::MulRndFtzSatF32),
    MulRndFtzF32x2(mul::section_1::MulRndFtzF32x2),
    MulRndF64(mul::section_1::MulRndF64),
    MulRndFtzSatF16(mul::section_2::MulRndFtzSatF16),
    MulRndFtzSatF16x2(mul::section_2::MulRndFtzSatF16x2),
    MulRndBf16(mul::section_2::MulRndBf16),
    MulRndBf16x2(mul::section_2::MulRndBf16x2),
    Mul24ModeType(mul24::section_0::Mul24ModeType),
    MultimemLdReduceLdsemScopeSsOpType(multimem_ld_reduce::section_0::MultimemLdReduceLdsemScopeSsOpType),
    MultimemLdReduceWeakSsOpType(multimem_ld_reduce::section_0::MultimemLdReduceWeakSsOpType),
    MultimemStStsemScopeSsType(multimem_ld_reduce::section_0::MultimemStStsemScopeSsType),
    MultimemStWeakSsType(multimem_ld_reduce::section_0::MultimemStWeakSsType),
    MultimemRedRedsemScopeSsOpType(multimem_ld_reduce::section_0::MultimemRedRedsemScopeSsOpType),
    MultimemLdReduceLdsemScopeSsOpAccPrecVecType(multimem_ld_reduce::section_1::MultimemLdReduceLdsemScopeSsOpAccPrecVecType),
    MultimemLdReduceWeakSsOpAccPrecVecType(multimem_ld_reduce::section_1::MultimemLdReduceWeakSsOpAccPrecVecType),
    MultimemStStsemScopeSsVecType(multimem_ld_reduce::section_1::MultimemStStsemScopeSsVecType),
    MultimemStWeakSsVecType(multimem_ld_reduce::section_1::MultimemStWeakSsVecType),
    MultimemRedRedsemScopeSsRedopVecRedtype(multimem_ld_reduce::section_1::MultimemRedRedsemScopeSsRedopVecRedtype),
    NanosleepU32(nanosleep::section_0::NanosleepU32),
    NegType(neg::section_0::NegType),
    NegFtzF32(neg::section_0::NegFtzF32),
    NegF64(neg::section_0::NegF64),
    NegFtzF16(neg::section_0::NegFtzF16),
    NegFtzF16x2(neg::section_0::NegFtzF16x2),
    NegBf16(neg::section_0::NegBf16),
    NegBf16x2(neg::section_0::NegBf16x2),
    NotType(not::section_0::NotType),
    OrType(or::section_0::OrType),
    Pmevent(pmevent::section_0::Pmevent),
    PmeventMask(pmevent::section_0::PmeventMask),
    PopcType(popc::section_0::PopcType),
    PrefetchSpaceLevel(prefetch::section_0::PrefetchSpaceLevel),
    PrefetchGlobalLevelEvictionPriority(prefetch::section_0::PrefetchGlobalLevelEvictionPriority),
    PrefetchuL1(prefetch::section_0::PrefetchuL1),
    PrefetchTensormapSpaceTensormap(prefetch::section_0::PrefetchTensormapSpaceTensormap),
    PrmtB32Mode(prmt::section_0::PrmtB32Mode),
    RcpApproxFtzF64(rcp_approx_ftz_f64::section_0::RcpApproxFtzF64),
    RcpApproxFtzF32(rcp::section_0::RcpApproxFtzF32),
    RcpRndFtzF32(rcp::section_0::RcpRndFtzF32),
    RcpRndF64(rcp::section_0::RcpRndF64),
    RedAsyncSemScopeSsCompletionMechanismOpType(red_async::section_0::RedAsyncSemScopeSsCompletionMechanismOpType),
    RedAsyncSemScopeSsCompletionMechanismOpType1(red_async::section_1::RedAsyncSemScopeSsCompletionMechanismOpType1),
    RedAsyncSemScopeSsCompletionMechanismOpType2(red_async::section_2::RedAsyncSemScopeSsCompletionMechanismOpType2),
    RedAsyncSemScopeSsCompletionMechanismAddType(red_async::section_3::RedAsyncSemScopeSsCompletionMechanismAddType),
    RedAsyncMmioSemScopeSsAddType(red_async::section_4::RedAsyncMmioSemScopeSsAddType),
    RedOpSpaceSemScopeLevelCacheHintType(red::section_0::RedOpSpaceSemScopeLevelCacheHintType),
    RedAddSpaceSemScopeNoftzLevelCacheHintF16(red::section_0::RedAddSpaceSemScopeNoftzLevelCacheHintF16),
    RedAddSpaceSemScopeNoftzLevelCacheHintF16x2(red::section_0::RedAddSpaceSemScopeNoftzLevelCacheHintF16x2),
    RedAddSpaceSemScopeNoftzLevelCacheHintBf16(red::section_0::RedAddSpaceSemScopeNoftzLevelCacheHintBf16),
    RedAddSpaceSemScopeNoftzLevelCacheHintBf16x2(red::section_0::RedAddSpaceSemScopeNoftzLevelCacheHintBf16x2),
    RedAddSpaceSemScopeLevelCacheHintVec32BitF32(red::section_1::RedAddSpaceSemScopeLevelCacheHintVec32BitF32),
    RedOpSpaceSemScopeNoftzLevelCacheHintVec16BitHalfWordType(red::section_1::RedOpSpaceSemScopeNoftzLevelCacheHintVec16BitHalfWordType),
    RedOpSpaceSemScopeNoftzLevelCacheHintVec32BitPackedType(red::section_1::RedOpSpaceSemScopeNoftzLevelCacheHintVec32BitPackedType),
    ReduxSyncOpType(redux_sync::section_0::ReduxSyncOpType),
    ReduxSyncOpB32(redux_sync::section_1::ReduxSyncOpB32),
    ReduxSyncOpAbsNanF32(redux_sync::section_2::ReduxSyncOpAbsNanF32),
    RemType(rem::section_0::RemType),
    RetUni(ret::section_0::RetUni),
    RsqrtApproxFtzF64(rsqrt_approx_ftz_f64::section_0::RsqrtApproxFtzF64),
    RsqrtApproxFtzF32(rsqrt::section_0::RsqrtApproxFtzF32),
    RsqrtApproxF64(rsqrt::section_0::RsqrtApproxF64),
    SadType(sad::section_0::SadType),
    SelpType(selp::section_0::SelpType),
    SetCmpopFtzDtypeStype(set::section_0::SetCmpopFtzDtypeStype),
    SetCmpopBoolopFtzDtypeStype(set::section_0::SetCmpopBoolopFtzDtypeStype),
    SetCmpopFtzF16Stype(set::section_1::SetCmpopFtzF16Stype),
    SetCmpopBoolopFtzF16Stype(set::section_1::SetCmpopBoolopFtzF16Stype),
    SetCmpopBf16Stype(set::section_1::SetCmpopBf16Stype),
    SetCmpopBoolopBf16Stype(set::section_1::SetCmpopBoolopBf16Stype),
    SetCmpopFtzDtypeF16(set::section_1::SetCmpopFtzDtypeF16),
    SetCmpopBoolopFtzDtypeF16(set::section_1::SetCmpopBoolopFtzDtypeF16),
    SetCmpopDtypeBf16(set::section_2::SetCmpopDtypeBf16),
    SetCmpopBoolopDtypeBf16(set::section_2::SetCmpopBoolopDtypeBf16),
    SetCmpopFtzDtypeF16x2(set::section_3::SetCmpopFtzDtypeF16x2),
    SetCmpopBoolopFtzDtypeF16x2(set::section_3::SetCmpopBoolopFtzDtypeF16x2),
    SetCmpopDtypeBf16x2(set::section_4::SetCmpopDtypeBf16x2),
    SetCmpopBoolopDtypeBf16x2(set::section_4::SetCmpopBoolopDtypeBf16x2),
    SetmaxnregActionSyncAlignedU32(setmaxnreg::section_0::SetmaxnregActionSyncAlignedU32),
    SetpCmpopFtzType(setp::section_0::SetpCmpopFtzType),
    SetpCmpopBoolopFtzType(setp::section_0::SetpCmpopBoolopFtzType),
    SetpCmpopFtzF16(setp::section_1::SetpCmpopFtzF16),
    SetpCmpopBoolopFtzF16(setp::section_1::SetpCmpopBoolopFtzF16),
    SetpCmpopFtzF16x2(setp::section_1::SetpCmpopFtzF16x2),
    SetpCmpopBoolopFtzF16x2(setp::section_1::SetpCmpopBoolopFtzF16x2),
    SetpCmpopBf16(setp::section_1::SetpCmpopBf16),
    SetpCmpopBoolopBf16(setp::section_1::SetpCmpopBoolopBf16),
    SetpCmpopBf16x2(setp::section_1::SetpCmpopBf16x2),
    SetpCmpopBoolopBf16x2(setp::section_1::SetpCmpopBoolopBf16x2),
    ShfLModeB32(shf::section_0::ShfLModeB32),
    ShfRModeB32(shf::section_0::ShfRModeB32),
    ShflSyncModeB32(shfl_sync::section_0::ShflSyncModeB32),
    ShflModeB32(shfl::section_0::ShflModeB32),
    ShlType(shl::section_0::ShlType),
    ShrType(shr::section_0::ShrType),
    SinApproxFtzF32(sin::section_0::SinApproxFtzF32),
    SlctDtypeS32(slct::section_0::SlctDtypeS32),
    SlctFtzDtypeF32(slct::section_0::SlctFtzDtypeF32),
    SqrtApproxFtzF32(sqrt::section_0::SqrtApproxFtzF32),
    SqrtRndFtzF32(sqrt::section_0::SqrtRndFtzF32),
    SqrtRndF64(sqrt::section_0::SqrtRndF64),
    StAsyncSemScopeSsCompletionMechanismVecType(st_async::section_0::StAsyncSemScopeSsCompletionMechanismVecType),
    StAsyncMmioSemScopeSsType(st_async::section_1::StAsyncMmioSemScopeSsType),
    StBulkWeakSharedCta(st_bulk::section_0::StBulkWeakSharedCta),
    StWeakSsCopLevelCacheHintVecType(st::section_0::StWeakSsCopLevelCacheHintVecType),
    StWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType(st::section_0::StWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType),
    StVolatileSsVecType(st::section_0::StVolatileSsVecType),
    StRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType(st::section_0::StRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType),
    StReleaseScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType(st::section_0::StReleaseScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType),
    StMmioRelaxedSysGlobalType(st::section_0::StMmioRelaxedSysGlobalType),
    StackrestoreType(stackrestore::section_0::StackrestoreType),
    StacksaveType(stacksave::section_0::StacksaveType),
    StmatrixSyncAlignedShapeNumTransSsType(stmatrix::section_0::StmatrixSyncAlignedShapeNumTransSsType),
    SubCcType(sub_cc::section_0::SubCcType),
    SubType(sub::section_0::SubType),
    SubSatS32(sub::section_0::SubSatS32),
    SubRndFtzSatF32(sub::section_1::SubRndFtzSatF32),
    SubRndFtzF32x2(sub::section_1::SubRndFtzF32x2),
    SubRndF64(sub::section_1::SubRndF64),
    SubRndFtzSatF16(sub::section_2::SubRndFtzSatF16),
    SubRndFtzSatF16x2(sub::section_2::SubRndFtzSatF16x2),
    SubRndBf16(sub::section_2::SubRndBf16),
    SubRndBf16x2(sub::section_2::SubRndBf16x2),
    SubRndSatF32Atype(sub::section_3::SubRndSatF32Atype),
    SubcCcType(subc::section_0::SubcCcType),
    SuldBGeomCopVecDtypeMode(suld::section_0::SuldBGeomCopVecDtypeMode),
    SuqQueryB32(suq::section_0::SuqQueryB32),
    SuredBOpGeomCtypeMode(sured::section_0::SuredBOpGeomCtypeMode),
    SuredPOpGeomCtypeMode(sured::section_1::SuredPOpGeomCtypeMode),
    SustBDimCopVecCtypeMode(sust::section_0::SustBDimCopVecCtypeMode),
    SustPDimVecB32Mode(sust::section_0::SustPDimVecB32Mode),
    SustBAdimCopVecCtypeMode(sust::section_0::SustBAdimCopVecCtypeMode),
    SzextModeType(szext::section_0::SzextModeType),
    TanhApproxType(tanh::section_0::TanhApproxType),
    Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32(tcgen05_alloc::section_0::Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32),
    Tcgen05DeallocCtaGroupSyncAlignedB32(tcgen05_alloc::section_0::Tcgen05DeallocCtaGroupSyncAlignedB32),
    Tcgen05RelinquishAllocPermitCtaGroupSyncAligned(tcgen05_alloc::section_0::Tcgen05RelinquishAllocPermitCtaGroupSyncAligned),
    Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64(tcgen05_commit::section_0::Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64),
    Tcgen05CpCtaGroupShapeMulticastDstSrcFmt(tcgen05_cp::section_0::Tcgen05CpCtaGroupShapeMulticastDstSrcFmt),
    Tcgen05FenceBeforeThreadSync(tcgen05_fence::section_0::Tcgen05FenceBeforeThreadSync),
    Tcgen05FenceAfterThreadSync(tcgen05_fence::section_0::Tcgen05FenceAfterThreadSync),
    Tcgen05LdSyncAlignedShape1NumPackB32(tcgen05_ld::section_0::Tcgen05LdSyncAlignedShape1NumPackB32),
    Tcgen05LdSyncAlignedShape2NumPackB32(tcgen05_ld::section_0::Tcgen05LdSyncAlignedShape2NumPackB32),
    Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32(tcgen05_ld::section_0::Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32),
    Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32(tcgen05_ld::section_0::Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32),
    Tcgen05LdRedSyncAlignedShape3NumRedopType(tcgen05_ld::section_0::Tcgen05LdRedSyncAlignedShape3NumRedopType),
    Tcgen05LdRedSyncAlignedShape4NumRedopType(tcgen05_ld::section_0::Tcgen05LdRedSyncAlignedShape4NumRedopType),
    Tcgen05MmaSpCtaGroupKind(tcgen05_mma_sp::section_0::Tcgen05MmaSpCtaGroupKind),
    Tcgen05MmaSpCtaGroupKind1(tcgen05_mma_sp::section_0::Tcgen05MmaSpCtaGroupKind1),
    Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize(tcgen05_mma_sp::section_1::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize),
    Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize1(tcgen05_mma_sp::section_1::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize1),
    Tcgen05MmaSpCtaGroupKindCollectorUsage(tcgen05_mma_sp::section_2::Tcgen05MmaSpCtaGroupKindCollectorUsage),
    Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage(tcgen05_mma_sp::section_2::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage),
    Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage1(tcgen05_mma_sp::section_2::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage1),
    Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage(tcgen05_mma_sp::section_3::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage),
    Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1(tcgen05_mma_sp::section_3::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1),
    Tcgen05MmaSpCtaGroupKindI8(tcgen05_mma_sp::section_4::Tcgen05MmaSpCtaGroupKindI8),
    Tcgen05MmaSpCtaGroupKindI81(tcgen05_mma_sp::section_4::Tcgen05MmaSpCtaGroupKindI81),
    Tcgen05MmaSpCtaGroupKindI8CollectorUsage(tcgen05_mma_sp::section_5::Tcgen05MmaSpCtaGroupKindI8CollectorUsage),
    Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage(tcgen05_mma_sp::section_5::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage),
    Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage1(tcgen05_mma_sp::section_5::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage1),
    Tcgen05MmaCtaGroupKind(tcgen05_mma::section_0::Tcgen05MmaCtaGroupKind),
    Tcgen05MmaCtaGroupKind1(tcgen05_mma::section_0::Tcgen05MmaCtaGroupKind1),
    Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize(tcgen05_mma::section_1::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize),
    Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize1(tcgen05_mma::section_1::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize1),
    Tcgen05MmaCtaGroupKindCollectorUsage(tcgen05_mma::section_2::Tcgen05MmaCtaGroupKindCollectorUsage),
    Tcgen05MmaCtaGroupKindAshiftCollectorUsage(tcgen05_mma::section_2::Tcgen05MmaCtaGroupKindAshiftCollectorUsage),
    Tcgen05MmaCtaGroupKindAshiftCollectorUsage1(tcgen05_mma::section_2::Tcgen05MmaCtaGroupKindAshiftCollectorUsage1),
    Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage(tcgen05_mma::section_3::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage),
    Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1(tcgen05_mma::section_3::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1),
    Tcgen05MmaCtaGroupKindI8(tcgen05_mma::section_4::Tcgen05MmaCtaGroupKindI8),
    Tcgen05MmaCtaGroupKindI81(tcgen05_mma::section_4::Tcgen05MmaCtaGroupKindI81),
    Tcgen05MmaCtaGroupKindI8CollectorUsage(tcgen05_mma::section_5::Tcgen05MmaCtaGroupKindI8CollectorUsage),
    Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage(tcgen05_mma::section_5::Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage),
    Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage1(tcgen05_mma::section_5::Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage1),
    Tcgen05MmaWsSpCtaGroup1KindCollectorUsage(tcgen05_mma_ws_sp::section_0::Tcgen05MmaWsSpCtaGroup1KindCollectorUsage),
    Tcgen05MmaWsSpCtaGroup1KindCollectorUsage1(tcgen05_mma_ws_sp::section_0::Tcgen05MmaWsSpCtaGroup1KindCollectorUsage1),
    Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage(tcgen05_mma_ws_sp::section_1::Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage),
    Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage1(tcgen05_mma_ws_sp::section_1::Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage1),
    Tcgen05MmaWsCtaGroup1KindCollectorUsage(tcgen05_mma_ws::section_0::Tcgen05MmaWsCtaGroup1KindCollectorUsage),
    Tcgen05MmaWsCtaGroup1KindCollectorUsage1(tcgen05_mma_ws::section_0::Tcgen05MmaWsCtaGroup1KindCollectorUsage1),
    Tcgen05MmaWsCtaGroup1KindI8CollectorUsage(tcgen05_mma_ws::section_1::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage),
    Tcgen05MmaWsCtaGroup1KindI8CollectorUsage1(tcgen05_mma_ws::section_1::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage1),
    Tcgen05ShiftCtaGroupDown(tcgen05_shift::section_0::Tcgen05ShiftCtaGroupDown),
    Tcgen05StSyncAlignedShape1NumUnpackB32(tcgen05_st::section_0::Tcgen05StSyncAlignedShape1NumUnpackB32),
    Tcgen05StSyncAlignedShape2NumUnpackB32(tcgen05_st::section_0::Tcgen05StSyncAlignedShape2NumUnpackB32),
    Tcgen05WaitOperationSyncAligned(tcgen05_wait::section_0::Tcgen05WaitOperationSyncAligned),
    TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned(tensormap_cp_fenceproxy::section_0::TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned),
    TensormapReplaceModeField1SsB1024Type(tensormap_replace::section_0::TensormapReplaceModeField1SsB1024Type),
    TensormapReplaceModeField2SsB1024Type(tensormap_replace::section_0::TensormapReplaceModeField2SsB1024Type),
    TensormapReplaceModeField3SsB1024Type(tensormap_replace::section_0::TensormapReplaceModeField3SsB1024Type),
    TestpOpType(testp::section_0::TestpOpType),
    TexGeomV4DtypeCtype(tex::section_0::TexGeomV4DtypeCtype),
    TexGeomV4DtypeCtype1(tex::section_0::TexGeomV4DtypeCtype1),
    TexGeomV2F16x2Ctype(tex::section_0::TexGeomV2F16x2Ctype),
    TexGeomV2F16x2Ctype1(tex::section_0::TexGeomV2F16x2Ctype1),
    TexBaseGeomV4DtypeCtype(tex::section_0::TexBaseGeomV4DtypeCtype),
    TexLevelGeomV4DtypeCtype(tex::section_0::TexLevelGeomV4DtypeCtype),
    TexGradGeomV4DtypeCtype(tex::section_0::TexGradGeomV4DtypeCtype),
    TexBaseGeomV2F16x2Ctype(tex::section_0::TexBaseGeomV2F16x2Ctype),
    TexLevelGeomV2F16x2Ctype(tex::section_0::TexLevelGeomV2F16x2Ctype),
    TexGradGeomV2F16x2Ctype(tex::section_0::TexGradGeomV2F16x2Ctype),
    Tld4Comp2dV4DtypeF32(tld4::section_0::Tld4Comp2dV4DtypeF32),
    Tld4CompGeomV4DtypeF32(tld4::section_0::Tld4CompGeomV4DtypeF32),
    Trap(trap::section_0::Trap),
    TxqTqueryB32(txq::section_0::TxqTqueryB32),
    TxqLevelTlqueryB32(txq::section_0::TxqLevelTlqueryB32),
    TxqSqueryB32(txq::section_0::TxqSqueryB32),
    VmadDtypeAtypeBtypeSatScale(vmad::section_0::VmadDtypeAtypeBtypeSatScale),
    VmadDtypeAtypeBtypePoSatScale(vmad::section_0::VmadDtypeAtypeBtypePoSatScale),
    VopDtypeAtypeBtypeSat(vop::section_0::VopDtypeAtypeBtypeSat),
    VopDtypeAtypeBtypeSatOp2(vop::section_0::VopDtypeAtypeBtypeSatOp2),
    VopDtypeAtypeBtypeSat1(vop::section_0::VopDtypeAtypeBtypeSat1),
    Vop2DtypeAtypeBtypeSat(vop2::section_0::Vop2DtypeAtypeBtypeSat),
    Vop2DtypeAtypeBtypeAdd(vop2::section_0::Vop2DtypeAtypeBtypeAdd),
    Vop4DtypeAtypeBtypeSat(vop4::section_0::Vop4DtypeAtypeBtypeSat),
    Vop4DtypeAtypeBtypeAdd(vop4::section_0::Vop4DtypeAtypeBtypeAdd),
    VoteSyncModePred(vote_sync::section_0::VoteSyncModePred),
    VoteSyncBallotB32(vote_sync::section_0::VoteSyncBallotB32),
    VoteModePred(vote::section_0::VoteModePred),
    VoteBallotB32(vote::section_0::VoteBallotB32),
    VsetAtypeBtypeCmp(vset::section_0::VsetAtypeBtypeCmp),
    VsetAtypeBtypeCmpOp2(vset::section_0::VsetAtypeBtypeCmpOp2),
    VsetAtypeBtypeCmp1(vset::section_0::VsetAtypeBtypeCmp1),
    Vset2AtypeBtypeCmp(vset2::section_0::Vset2AtypeBtypeCmp),
    Vset2AtypeBtypeCmpAdd(vset2::section_0::Vset2AtypeBtypeCmpAdd),
    Vset4AtypeBtypeCmp(vset4::section_0::Vset4AtypeBtypeCmp),
    Vset4AtypeBtypeCmpAdd(vset4::section_0::Vset4AtypeBtypeCmpAdd),
    VopDtypeAtypeU32SatMode(vsh::section_0::VopDtypeAtypeU32SatMode),
    VopDtypeAtypeU32SatModeOp2(vsh::section_0::VopDtypeAtypeU32SatModeOp2),
    VopDtypeAtypeU32SatMode1(vsh::section_0::VopDtypeAtypeU32SatMode1),
    WgmmaCommitGroupSyncAligned(wgmma_commit_group::section_0::WgmmaCommitGroupSyncAligned),
    WgmmaFenceSyncAligned(wgmma_fence::section_0::WgmmaFenceSyncAligned),
    WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F16(wgmma_mma_async_sp::section_0::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F16),
    WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F161(wgmma_mma_async_sp::section_0::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F161),
    WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf16(wgmma_mma_async_sp::section_1::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf16),
    WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf161(wgmma_mma_async_sp::section_1::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf161),
    WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf32(wgmma_mma_async_sp::section_2::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf32),
    WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf321(wgmma_mma_async_sp::section_2::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf321),
    WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype(wgmma_mma_async_sp::section_3::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype),
    WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype1(wgmma_mma_async_sp::section_3::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype1),
    WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype(wgmma_mma_async_sp::section_4::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype),
    WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype1(wgmma_mma_async_sp::section_4::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype1),
    WgmmaMmaAsyncSyncAlignedShapeDtypeF16F16(wgmma_mma_async::section_0::WgmmaMmaAsyncSyncAlignedShapeDtypeF16F16),
    WgmmaMmaAsyncSyncAlignedShapeDtypeF16F161(wgmma_mma_async::section_0::WgmmaMmaAsyncSyncAlignedShapeDtypeF16F161),
    WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf16(wgmma_mma_async::section_1::WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf16),
    WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf161(wgmma_mma_async::section_1::WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf161),
    WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf32(wgmma_mma_async::section_2::WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf32),
    WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf321(wgmma_mma_async::section_2::WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf321),
    WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype(wgmma_mma_async::section_3::WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype),
    WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype1(wgmma_mma_async::section_3::WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype1),
    WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype(wgmma_mma_async::section_4::WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype),
    WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype1(wgmma_mma_async::section_4::WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype1),
    WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc(wgmma_mma_async::section_5::WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc),
    WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc1(wgmma_mma_async::section_5::WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc1),
    WgmmaWaitGroupSyncAligned(wgmma_wait_group::section_0::WgmmaWaitGroupSyncAligned),
    WmmaLoadASyncAlignedLayoutShapeSsAtype(wmma_load::section_0::WmmaLoadASyncAlignedLayoutShapeSsAtype),
    WmmaLoadBSyncAlignedLayoutShapeSsBtype(wmma_load::section_0::WmmaLoadBSyncAlignedLayoutShapeSsBtype),
    WmmaLoadCSyncAlignedLayoutShapeSsCtype(wmma_load::section_0::WmmaLoadCSyncAlignedLayoutShapeSsCtype),
    WmmaLoadASyncAlignedLayoutShapeSsAtype1(wmma_load::section_1::WmmaLoadASyncAlignedLayoutShapeSsAtype1),
    WmmaLoadBSyncAlignedLayoutShapeSsBtype1(wmma_load::section_1::WmmaLoadBSyncAlignedLayoutShapeSsBtype1),
    WmmaLoadCSyncAlignedLayoutShapeSsCtype1(wmma_load::section_1::WmmaLoadCSyncAlignedLayoutShapeSsCtype1),
    WmmaLoadASyncAlignedLayoutShapeSsAtype2(wmma_load::section_2::WmmaLoadASyncAlignedLayoutShapeSsAtype2),
    WmmaLoadBSyncAlignedLayoutShapeSsBtype2(wmma_load::section_2::WmmaLoadBSyncAlignedLayoutShapeSsBtype2),
    WmmaLoadCSyncAlignedLayoutShapeSsCtype2(wmma_load::section_2::WmmaLoadCSyncAlignedLayoutShapeSsCtype2),
    WmmaLoadASyncAlignedLayoutShapeSsAtype3(wmma_load::section_3::WmmaLoadASyncAlignedLayoutShapeSsAtype3),
    WmmaLoadBSyncAlignedLayoutShapeSsBtype3(wmma_load::section_3::WmmaLoadBSyncAlignedLayoutShapeSsBtype3),
    WmmaLoadCSyncAlignedLayoutShapeSsCtype3(wmma_load::section_3::WmmaLoadCSyncAlignedLayoutShapeSsCtype3),
    WmmaLoadASyncAlignedRowShapeSsAtype(wmma_load::section_4::WmmaLoadASyncAlignedRowShapeSsAtype),
    WmmaLoadBSyncAlignedColShapeSsBtype(wmma_load::section_4::WmmaLoadBSyncAlignedColShapeSsBtype),
    WmmaLoadCSyncAlignedLayoutShapeSsCtype4(wmma_load::section_4::WmmaLoadCSyncAlignedLayoutShapeSsCtype4),
    WmmaLoadASyncAlignedRowShapeSsAtype1(wmma_load::section_5::WmmaLoadASyncAlignedRowShapeSsAtype1),
    WmmaLoadBSyncAlignedColShapeSsBtype1(wmma_load::section_5::WmmaLoadBSyncAlignedColShapeSsBtype1),
    WmmaLoadCSyncAlignedLayoutShapeSsCtype5(wmma_load::section_5::WmmaLoadCSyncAlignedLayoutShapeSsCtype5),
    WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype(wmma_mma::section_0::WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype),
    WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite(wmma_mma::section_1::WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite),
    WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32(wmma_mma::section_2::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32),
    WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321(wmma_mma::section_3::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321),
    WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64(wmma_mma::section_4::WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64),
    WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite(wmma_mma::section_5::WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite),
    WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32(wmma_mma::section_6::WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32),
    WmmaStoreDSyncAlignedLayoutShapeSsType(wmma_store::section_0::WmmaStoreDSyncAlignedLayoutShapeSsType),
    WmmaStoreDSyncAlignedLayoutShapeSsType1(wmma_store::section_1::WmmaStoreDSyncAlignedLayoutShapeSsType1),
    WmmaStoreDSyncAlignedLayoutShapeSsType2(wmma_store::section_2::WmmaStoreDSyncAlignedLayoutShapeSsType2),
    WmmaStoreDSyncAlignedLayoutShapeSsType3(wmma_store::section_3::WmmaStoreDSyncAlignedLayoutShapeSsType3),
    XorType(xor::section_0::XorType),
}
