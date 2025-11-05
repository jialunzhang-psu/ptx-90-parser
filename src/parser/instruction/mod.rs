// Auto-generated module declarations
// DO NOT EDIT MANUALLY
#![allow(unused)]

use crate::parser::{PtxParser, PtxParseError, PtxTokenStream, Span};
use crate::r#type::instruction::Instruction;
use crate::r#type::common::Operand;
use crate::lexer::PtxToken;

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

/// Parse instruction without label or predicate
fn parse_instruction_inner(stream: &mut PtxTokenStream) -> Result<Instruction, PtxParseError> {
    let start_pos = stream.position();
    
    // Peek at the opcode to determine which parser to try
    let opcode = if let Ok((PtxToken::Identifier(name), _)) = stream.peek() {
        name.as_str()
    } else {
        let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(0..0);
        return Err(crate::parser::unexpected_value(span, &["instruction opcode"], "not an identifier"));
    };
    
    // Dispatch based on opcode
    match opcode {
        "subc" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::subc::section_0::SubcCcType as PtxParser>::parse(stream) {
                return Ok(Instruction::SubcCcType(inst));
            }
        }
        "sust" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sust::section_0::SustBDimCopVecCtypeMode as PtxParser>::parse(stream) {
                return Ok(Instruction::SustBDimCopVecCtypeMode(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sust::section_0::SustPDimVecB32Mode as PtxParser>::parse(stream) {
                return Ok(Instruction::SustPDimVecB32Mode(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sust::section_0::SustBAdimCopVecCtypeMode as PtxParser>::parse(stream) {
                return Ok(Instruction::SustBAdimCopVecCtypeMode(inst));
            }
        }
        "isspacep" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::isspacep::section_0::IsspacepSpace as PtxParser>::parse(stream) {
                return Ok(Instruction::IsspacepSpace(inst));
            }
        }
        "popc" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::popc::section_0::PopcType as PtxParser>::parse(stream) {
                return Ok(Instruction::PopcType(inst));
            }
        }
        "sad" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sad::section_0::SadType as PtxParser>::parse(stream) {
                return Ok(Instruction::SadType(inst));
            }
        }
        "shl" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::shl::section_0::ShlType as PtxParser>::parse(stream) {
                return Ok(Instruction::ShlType(inst));
            }
        }
        "tex" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tex::section_0::TexGeomV4DtypeCtype as PtxParser>::parse(stream) {
                return Ok(Instruction::TexGeomV4DtypeCtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tex::section_0::TexGeomV4DtypeCtype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::TexGeomV4DtypeCtype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tex::section_0::TexGeomV2F16x2Ctype as PtxParser>::parse(stream) {
                return Ok(Instruction::TexGeomV2F16x2Ctype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tex::section_0::TexGeomV2F16x2Ctype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::TexGeomV2F16x2Ctype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tex::section_0::TexBaseGeomV4DtypeCtype as PtxParser>::parse(stream) {
                return Ok(Instruction::TexBaseGeomV4DtypeCtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tex::section_0::TexLevelGeomV4DtypeCtype as PtxParser>::parse(stream) {
                return Ok(Instruction::TexLevelGeomV4DtypeCtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tex::section_0::TexGradGeomV4DtypeCtype as PtxParser>::parse(stream) {
                return Ok(Instruction::TexGradGeomV4DtypeCtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tex::section_0::TexBaseGeomV2F16x2Ctype as PtxParser>::parse(stream) {
                return Ok(Instruction::TexBaseGeomV2F16x2Ctype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tex::section_0::TexLevelGeomV2F16x2Ctype as PtxParser>::parse(stream) {
                return Ok(Instruction::TexLevelGeomV2F16x2Ctype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tex::section_0::TexGradGeomV2F16x2Ctype as PtxParser>::parse(stream) {
                return Ok(Instruction::TexGradGeomV2F16x2Ctype(inst));
            }
        }
        "vote" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vote_sync::section_0::VoteSyncModePred as PtxParser>::parse(stream) {
                return Ok(Instruction::VoteSyncModePred(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vote_sync::section_0::VoteSyncBallotB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::VoteSyncBallotB32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vote::section_0::VoteModePred as PtxParser>::parse(stream) {
                return Ok(Instruction::VoteModePred(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vote::section_0::VoteBallotB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::VoteBallotB32(inst));
            }
        }
        "fma" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::fma::section_0::FmaRndFtzSatF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::FmaRndFtzSatF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::fma::section_0::FmaRndFtzF32x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::FmaRndFtzF32x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::fma::section_0::FmaRndF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::FmaRndF64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::fma::section_1::FmaRndFtzSatF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::FmaRndFtzSatF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::fma::section_1::FmaRndFtzSatF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::FmaRndFtzSatF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::fma::section_1::FmaRndFtzReluF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::FmaRndFtzReluF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::fma::section_1::FmaRndFtzReluF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::FmaRndFtzReluF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::fma::section_1::FmaRndReluBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::FmaRndReluBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::fma::section_1::FmaRndReluBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::FmaRndReluBf16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::fma::section_1::FmaRndOobReluType as PtxParser>::parse(stream) {
                return Ok(Instruction::FmaRndOobReluType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::fma::section_2::FmaRndSatF32Abtype as PtxParser>::parse(stream) {
                return Ok(Instruction::FmaRndSatF32Abtype(inst));
            }
        }
        "movmatrix" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::movmatrix::section_0::MovmatrixSyncAlignedShapeTransType as PtxParser>::parse(stream) {
                return Ok(Instruction::MovmatrixSyncAlignedShapeTransType(inst));
            }
        }
        "nanosleep" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::nanosleep::section_0::NanosleepU32 as PtxParser>::parse(stream) {
                return Ok(Instruction::NanosleepU32(inst));
            }
        }
        "istypep" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::istypep::section_0::IstypepType as PtxParser>::parse(stream) {
                return Ok(Instruction::IstypepType(inst));
            }
        }
        "membar" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceSemScope as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceSemScope(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceAcquireSyncRestrictSharedClusterCluster as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceAcquireSyncRestrictSharedClusterCluster(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceReleaseSyncRestrictSharedCtaCluster as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceReleaseSyncRestrictSharedCtaCluster(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceOpRestrictReleaseCluster as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceOpRestrictReleaseCluster(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceProxyProxykind as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceProxyProxykind(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceProxyToProxykindFromProxykindReleaseScope as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceProxyToProxykindFromProxykindReleaseScope(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceProxyToProxykindFromProxykindAcquireScope as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceProxyToProxykindFromProxykindAcquireScope(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::MembarLevel as PtxParser>::parse(stream) {
                return Ok(Instruction::MembarLevel(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::MembarProxyProxykind as PtxParser>::parse(stream) {
                return Ok(Instruction::MembarProxyProxykind(inst));
            }
        }
        "mad24" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mad24::section_0::Mad24ModeType as PtxParser>::parse(stream) {
                return Ok(Instruction::Mad24ModeType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mad24::section_0::Mad24HiSatS32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Mad24HiSatS32(inst));
            }
        }
        "madc" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::madc::section_0::MadcHiloCcType as PtxParser>::parse(stream) {
                return Ok(Instruction::MadcHiloCcType(inst));
            }
        }
        "max" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::max::section_0::MaxAtype as PtxParser>::parse(stream) {
                return Ok(Instruction::MaxAtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::max::section_0::MaxReluBtype as PtxParser>::parse(stream) {
                return Ok(Instruction::MaxReluBtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::max::section_0::MaxFtzNanXorsignAbsF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MaxFtzNanXorsignAbsF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::max::section_0::MaxFtzNanAbsF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MaxFtzNanAbsF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::max::section_0::MaxF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MaxF64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::max::section_0::MaxFtzNanXorsignAbsF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::MaxFtzNanXorsignAbsF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::max::section_0::MaxFtzNanXorsignAbsF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::MaxFtzNanXorsignAbsF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::max::section_0::MaxNanXorsignAbsBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::MaxNanXorsignAbsBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::max::section_0::MaxNanXorsignAbsBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::MaxNanXorsignAbsBf16x2(inst));
            }
        }
        "setp" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::setp::section_0::SetpCmpopFtzType as PtxParser>::parse(stream) {
                return Ok(Instruction::SetpCmpopFtzType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::setp::section_0::SetpCmpopBoolopFtzType as PtxParser>::parse(stream) {
                return Ok(Instruction::SetpCmpopBoolopFtzType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::setp::section_1::SetpCmpopFtzF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetpCmpopFtzF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::setp::section_1::SetpCmpopBoolopFtzF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetpCmpopBoolopFtzF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::setp::section_1::SetpCmpopFtzF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetpCmpopFtzF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::setp::section_1::SetpCmpopBoolopFtzF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetpCmpopBoolopFtzF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::setp::section_1::SetpCmpopBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetpCmpopBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::setp::section_1::SetpCmpopBoolopBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetpCmpopBoolopBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::setp::section_1::SetpCmpopBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetpCmpopBf16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::setp::section_1::SetpCmpopBoolopBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetpCmpopBoolopBf16x2(inst));
            }
        }
        "slct" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::slct::section_0::SlctDtypeS32 as PtxParser>::parse(stream) {
                return Ok(Instruction::SlctDtypeS32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::slct::section_0::SlctFtzDtypeF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::SlctFtzDtypeF32(inst));
            }
        }
        "set" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_0::SetCmpopFtzDtypeStype as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopFtzDtypeStype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_0::SetCmpopBoolopFtzDtypeStype as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopBoolopFtzDtypeStype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_1::SetCmpopFtzF16Stype as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopFtzF16Stype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_1::SetCmpopBoolopFtzF16Stype as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopBoolopFtzF16Stype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_1::SetCmpopBf16Stype as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopBf16Stype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_1::SetCmpopBoolopBf16Stype as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopBoolopBf16Stype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_1::SetCmpopFtzDtypeF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopFtzDtypeF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_1::SetCmpopBoolopFtzDtypeF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopBoolopFtzDtypeF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_2::SetCmpopDtypeBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopDtypeBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_2::SetCmpopBoolopDtypeBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopBoolopDtypeBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_3::SetCmpopFtzDtypeF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopFtzDtypeF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_3::SetCmpopBoolopFtzDtypeF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopBoolopFtzDtypeF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_4::SetCmpopDtypeBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopDtypeBf16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::set::section_4::SetCmpopBoolopDtypeBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetCmpopBoolopDtypeBf16x2(inst));
            }
        }
        "mul24" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mul24::section_0::Mul24ModeType as PtxParser>::parse(stream) {
                return Ok(Instruction::Mul24ModeType(inst));
            }
        }
        "prefetch" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::prefetch::section_0::PrefetchSpaceLevel as PtxParser>::parse(stream) {
                return Ok(Instruction::PrefetchSpaceLevel(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::prefetch::section_0::PrefetchGlobalLevelEvictionPriority as PtxParser>::parse(stream) {
                return Ok(Instruction::PrefetchGlobalLevelEvictionPriority(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::prefetch::section_0::PrefetchuL1 as PtxParser>::parse(stream) {
                return Ok(Instruction::PrefetchuL1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::prefetch::section_0::PrefetchTensormapSpaceTensormap as PtxParser>::parse(stream) {
                return Ok(Instruction::PrefetchTensormapSpaceTensormap(inst));
            }
        }
        "rcp" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::rcp_approx_ftz_f64::section_0::RcpApproxFtzF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::RcpApproxFtzF64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::rcp::section_0::RcpApproxFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::RcpApproxFtzF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::rcp::section_0::RcpRndFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::RcpRndFtzF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::rcp::section_0::RcpRndF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::RcpRndF64(inst));
            }
        }
        "stackrestore" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::stackrestore::section_0::StackrestoreType as PtxParser>::parse(stream) {
                return Ok(Instruction::StackrestoreType(inst));
            }
        }
        "szext" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::szext::section_0::SzextModeType as PtxParser>::parse(stream) {
                return Ok(Instruction::SzextModeType(inst));
            }
        }
        "barrier" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarrierCtaSyncAligned as PtxParser>::parse(stream) {
                return Ok(Instruction::BarrierCtaSyncAligned(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarrierCtaArriveAligned as PtxParser>::parse(stream) {
                return Ok(Instruction::BarrierCtaArriveAligned(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarrierCtaRedPopcAlignedU32 as PtxParser>::parse(stream) {
                return Ok(Instruction::BarrierCtaRedPopcAlignedU32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarrierCtaRedOpAlignedPred as PtxParser>::parse(stream) {
                return Ok(Instruction::BarrierCtaRedOpAlignedPred(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarCtaSync as PtxParser>::parse(stream) {
                return Ok(Instruction::BarCtaSync(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarCtaArrive as PtxParser>::parse(stream) {
                return Ok(Instruction::BarCtaArrive(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarCtaRedPopcU32 as PtxParser>::parse(stream) {
                return Ok(Instruction::BarCtaRedPopcU32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarCtaRedOpPred as PtxParser>::parse(stream) {
                return Ok(Instruction::BarCtaRedOpPred(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::barrier_cluster::section_0::BarrierClusterArriveSemAligned as PtxParser>::parse(stream) {
                return Ok(Instruction::BarrierClusterArriveSemAligned(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::barrier_cluster::section_0::BarrierClusterWaitAcquireAligned as PtxParser>::parse(stream) {
                return Ok(Instruction::BarrierClusterWaitAcquireAligned(inst));
            }
        }
        "vmad" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vmad::section_0::VmadDtypeAtypeBtypeSatScale as PtxParser>::parse(stream) {
                return Ok(Instruction::VmadDtypeAtypeBtypeSatScale(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vmad::section_0::VmadDtypeAtypeBtypePoSatScale as PtxParser>::parse(stream) {
                return Ok(Instruction::VmadDtypeAtypeBtypePoSatScale(inst));
            }
        }
        "not" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::not::section_0::NotType as PtxParser>::parse(stream) {
                return Ok(Instruction::NotType(inst));
            }
        }
        "createpolicy" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::createpolicy::section_0::CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::createpolicy::section_0::CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::createpolicy::section_0::CreatepolicyCvtL2B64 as PtxParser>::parse(stream) {
                return Ok(Instruction::CreatepolicyCvtL2B64(inst));
            }
        }
        "shfl" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::shfl_sync::section_0::ShflSyncModeB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::ShflSyncModeB32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::shfl::section_0::ShflModeB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::ShflModeB32(inst));
            }
        }
        "getctarank" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::getctarank::section_0::GetctarankSpaceType as PtxParser>::parse(stream) {
                return Ok(Instruction::GetctarankSpaceType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::getctarank::section_0::GetctarankSharedClusterType as PtxParser>::parse(stream) {
                return Ok(Instruction::GetctarankSharedClusterType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::getctarank::section_0::GetctarankType as PtxParser>::parse(stream) {
                return Ok(Instruction::GetctarankType(inst));
            }
        }
        "tanh" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tanh::section_0::TanhApproxType as PtxParser>::parse(stream) {
                return Ok(Instruction::TanhApproxType(inst));
            }
        }
        "stacksave" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::stacksave::section_0::StacksaveType as PtxParser>::parse(stream) {
                return Ok(Instruction::StacksaveType(inst));
            }
        }
        "clusterlaunchcontrol" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::clusterlaunchcontrol_query_cancel::section_0::ClusterlaunchcontrolQueryCancelIsCanceledPredB128 as PtxParser>::parse(stream) {
                return Ok(Instruction::ClusterlaunchcontrolQueryCancelIsCanceledPredB128(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::clusterlaunchcontrol_query_cancel::section_0::ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128 as PtxParser>::parse(stream) {
                return Ok(Instruction::ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::clusterlaunchcontrol_query_cancel::section_0::ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128 as PtxParser>::parse(stream) {
                return Ok(Instruction::ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::clusterlaunchcontrol_try_cancel::section_0::ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128 as PtxParser>::parse(stream) {
                return Ok(Instruction::ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128(inst));
            }
        }
        "suld" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::suld::section_0::SuldBGeomCopVecDtypeMode as PtxParser>::parse(stream) {
                return Ok(Instruction::SuldBGeomCopVecDtypeMode(inst));
            }
        }
        "sured" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sured::section_0::SuredBOpGeomCtypeMode as PtxParser>::parse(stream) {
                return Ok(Instruction::SuredBOpGeomCtypeMode(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sured::section_1::SuredPOpGeomCtypeMode as PtxParser>::parse(stream) {
                return Ok(Instruction::SuredPOpGeomCtypeMode(inst));
            }
        }
        "bfind" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bfind::section_0::BfindType as PtxParser>::parse(stream) {
                return Ok(Instruction::BfindType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bfind::section_0::BfindShiftamtType as PtxParser>::parse(stream) {
                return Ok(Instruction::BfindShiftamtType(inst));
            }
        }
        "lg2" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::lg2::section_0::Lg2ApproxFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Lg2ApproxFtzF32(inst));
            }
        }
        "elect" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::elect_sync::section_0::ElectSync as PtxParser>::parse(stream) {
                return Ok(Instruction::ElectSync(inst));
            }
        }
        "mad" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mad_cc::section_0::MadHiloCcType as PtxParser>::parse(stream) {
                return Ok(Instruction::MadHiloCcType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mad::section_0::MadModeType as PtxParser>::parse(stream) {
                return Ok(Instruction::MadModeType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mad::section_0::MadHiSatS32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MadHiSatS32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mad::section_0::MadFtzSatF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MadFtzSatF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mad::section_0::MadRndFtzSatF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MadRndFtzSatF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mad::section_0::MadRndF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MadRndF64(inst));
            }
        }
        "bfi" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bfi::section_0::BfiType as PtxParser>::parse(stream) {
                return Ok(Instruction::BfiType(inst));
            }
        }
        "min" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::min::section_0::MinAtype as PtxParser>::parse(stream) {
                return Ok(Instruction::MinAtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::min::section_0::MinReluBtype as PtxParser>::parse(stream) {
                return Ok(Instruction::MinReluBtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::min::section_0::MinFtzNanXorsignAbsF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MinFtzNanXorsignAbsF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::min::section_0::MinFtzNanAbsF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MinFtzNanAbsF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::min::section_0::MinF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MinF64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::min::section_0::MinFtzNanXorsignAbsF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::MinFtzNanXorsignAbsF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::min::section_0::MinFtzNanXorsignAbsF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::MinFtzNanXorsignAbsF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::min::section_0::MinNanXorsignAbsBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::MinNanXorsignAbsBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::min::section_0::MinNanXorsignAbsBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::MinNanXorsignAbsBf16x2(inst));
            }
        }
        "vop2" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vop2::section_0::Vop2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                return Ok(Instruction::Vop2DtypeAtypeBtypeSat(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vop2::section_0::Vop2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                return Ok(Instruction::Vop2DtypeAtypeBtypeAdd(inst));
            }
        }
        "xor" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::xor::section_0::XorType as PtxParser>::parse(stream) {
                return Ok(Instruction::XorType(inst));
            }
        }
        "ret" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ret::section_0::RetUni as PtxParser>::parse(stream) {
                return Ok(Instruction::RetUni(inst));
            }
        }
        "brev" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::brev::section_0::BrevType as PtxParser>::parse(stream) {
                return Ok(Instruction::BrevType(inst));
            }
        }
        "vset2" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vset2::section_0::Vset2AtypeBtypeCmp as PtxParser>::parse(stream) {
                return Ok(Instruction::Vset2AtypeBtypeCmp(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vset2::section_0::Vset2AtypeBtypeCmpAdd as PtxParser>::parse(stream) {
                return Ok(Instruction::Vset2AtypeBtypeCmpAdd(inst));
            }
        }
        "bar" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarrierCtaSyncAligned as PtxParser>::parse(stream) {
                return Ok(Instruction::BarrierCtaSyncAligned(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarrierCtaArriveAligned as PtxParser>::parse(stream) {
                return Ok(Instruction::BarrierCtaArriveAligned(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarrierCtaRedPopcAlignedU32 as PtxParser>::parse(stream) {
                return Ok(Instruction::BarrierCtaRedPopcAlignedU32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarrierCtaRedOpAlignedPred as PtxParser>::parse(stream) {
                return Ok(Instruction::BarrierCtaRedOpAlignedPred(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarCtaSync as PtxParser>::parse(stream) {
                return Ok(Instruction::BarCtaSync(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarCtaArrive as PtxParser>::parse(stream) {
                return Ok(Instruction::BarCtaArrive(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarCtaRedPopcU32 as PtxParser>::parse(stream) {
                return Ok(Instruction::BarCtaRedPopcU32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar::section_0::BarCtaRedOpPred as PtxParser>::parse(stream) {
                return Ok(Instruction::BarCtaRedOpPred(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bar_warp_sync::section_0::BarWarpSync as PtxParser>::parse(stream) {
                return Ok(Instruction::BarWarpSync(inst));
            }
        }
        "bfe" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bfe::section_0::BfeType as PtxParser>::parse(stream) {
                return Ok(Instruction::BfeType(inst));
            }
        }
        "setmaxnreg" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::setmaxnreg::section_0::SetmaxnregActionSyncAlignedU32 as PtxParser>::parse(stream) {
                return Ok(Instruction::SetmaxnregActionSyncAlignedU32(inst));
            }
        }
        "sub" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sub_cc::section_0::SubCcType as PtxParser>::parse(stream) {
                return Ok(Instruction::SubCcType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sub::section_0::SubType as PtxParser>::parse(stream) {
                return Ok(Instruction::SubType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sub::section_0::SubSatS32 as PtxParser>::parse(stream) {
                return Ok(Instruction::SubSatS32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sub::section_1::SubRndFtzSatF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::SubRndFtzSatF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sub::section_1::SubRndFtzF32x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::SubRndFtzF32x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sub::section_1::SubRndF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::SubRndF64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sub::section_2::SubRndFtzSatF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::SubRndFtzSatF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sub::section_2::SubRndFtzSatF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::SubRndFtzSatF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sub::section_2::SubRndBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::SubRndBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sub::section_2::SubRndBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::SubRndBf16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sub::section_3::SubRndSatF32Atype as PtxParser>::parse(stream) {
                return Ok(Instruction::SubRndSatF32Atype(inst));
            }
        }
        "brkpt" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::brkpt::section_0::Brkpt as PtxParser>::parse(stream) {
                return Ok(Instruction::Brkpt(inst));
            }
        }
        "tensormap" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tensormap_cp_fenceproxy::section_0::TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned as PtxParser>::parse(stream) {
                return Ok(Instruction::TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tensormap_replace::section_0::TensormapReplaceModeField1SsB1024Type as PtxParser>::parse(stream) {
                return Ok(Instruction::TensormapReplaceModeField1SsB1024Type(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tensormap_replace::section_0::TensormapReplaceModeField2SsB1024Type as PtxParser>::parse(stream) {
                return Ok(Instruction::TensormapReplaceModeField2SsB1024Type(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tensormap_replace::section_0::TensormapReplaceModeField3SsB1024Type as PtxParser>::parse(stream) {
                return Ok(Instruction::TensormapReplaceModeField3SsB1024Type(inst));
            }
        }
        "brx" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::brx_idx::section_0::BrxIdxUni as PtxParser>::parse(stream) {
                return Ok(Instruction::BrxIdxUni(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::brx_idx::section_0::BrxIdxUni1 as PtxParser>::parse(stream) {
                return Ok(Instruction::BrxIdxUni1(inst));
            }
        }
        "stmatrix" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::stmatrix::section_0::StmatrixSyncAlignedShapeNumTransSsType as PtxParser>::parse(stream) {
                return Ok(Instruction::StmatrixSyncAlignedShapeNumTransSsType(inst));
            }
        }
        "trap" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::trap::section_0::Trap as PtxParser>::parse(stream) {
                return Ok(Instruction::Trap(inst));
            }
        }
        "exit" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::exit::section_0::Exit as PtxParser>::parse(stream) {
                return Ok(Instruction::Exit(inst));
            }
        }
        "ldmatrix" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ldmatrix::section_0::LdmatrixSyncAlignedShapeNumTransSsType as PtxParser>::parse(stream) {
                return Ok(Instruction::LdmatrixSyncAlignedShapeNumTransSsType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ldmatrix::section_0::LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt as PtxParser>::parse(stream) {
                return Ok(Instruction::LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ldmatrix::section_0::LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt as PtxParser>::parse(stream) {
                return Ok(Instruction::LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt(inst));
            }
        }
        "cp" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_bulk_commit_group::section_0::CpAsyncBulkCommitGroup as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncBulkCommitGroup(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_bulk_prefetch_tensor::section_0::CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_bulk_prefetch::section_0::CpAsyncBulkPrefetchL2SrcLevelCacheHint as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncBulkPrefetchL2SrcLevelCacheHint(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_bulk_tensor::section_0::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_bulk_tensor::section_1::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_bulk_tensor::section_2::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_bulk::section_0::CpAsyncBulkDstSrcCompletionMechanismLevelCacheHint as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncBulkDstSrcCompletionMechanismLevelCacheHint(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_bulk::section_1::CpAsyncBulkDstSrcCompletionMechanismMulticastLevelCacheHint as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncBulkDstSrcCompletionMechanismMulticastLevelCacheHint(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_bulk::section_2::CpAsyncBulkDstSrcCompletionMechanism as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncBulkDstSrcCompletionMechanism(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_bulk::section_3::CpAsyncBulkDstSrcCompletionMechanismLevelCacheHintCpMask as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncBulkDstSrcCompletionMechanismLevelCacheHintCpMask(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_bulk_wait_group::section_0::CpAsyncBulkWaitGroupRead as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncBulkWaitGroupRead(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_commit_group::section_0::CpAsyncCommitGroup as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncCommitGroup(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_mbarrier_arrive::section_0::CpAsyncMbarrierArriveNoincStateB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncMbarrierArriveNoincStateB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async::section_0::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async::section_0::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async::section_0::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1 as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async::section_0::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1 as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_wait_group::section_0::CpAsyncWaitGroup as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncWaitGroup(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_async_wait_group::section_0::CpAsyncWaitAll as PtxParser>::parse(stream) {
                return Ok(Instruction::CpAsyncWaitAll(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_reduce_async_bulk_tensor::section_0::CpReduceAsyncBulkTensorDimDstSrcRedopLoadModeCompletionMechanismLevelCacheHint as PtxParser>::parse(stream) {
                return Ok(Instruction::CpReduceAsyncBulkTensorDimDstSrcRedopLoadModeCompletionMechanismLevelCacheHint(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_reduce_async_bulk::section_0::CpReduceAsyncBulkDstSrcCompletionMechanismRedopType as PtxParser>::parse(stream) {
                return Ok(Instruction::CpReduceAsyncBulkDstSrcCompletionMechanismRedopType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_reduce_async_bulk::section_1::CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType as PtxParser>::parse(stream) {
                return Ok(Instruction::CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cp_reduce_async_bulk::section_2::CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType as PtxParser>::parse(stream) {
                return Ok(Instruction::CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType(inst));
            }
        }
        "dp4a" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::dp4a::section_0::Dp4aAtypeBtype as PtxParser>::parse(stream) {
                return Ok(Instruction::Dp4aAtypeBtype(inst));
            }
        }
        "and" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::and::section_0::AndType as PtxParser>::parse(stream) {
                return Ok(Instruction::AndType(inst));
            }
        }
        "bra" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bra::section_0::BraUni as PtxParser>::parse(stream) {
                return Ok(Instruction::BraUni(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bra::section_0::BraUni1 as PtxParser>::parse(stream) {
                return Ok(Instruction::BraUni1(inst));
            }
        }
        "addc" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::addc::section_0::AddcCcType as PtxParser>::parse(stream) {
                return Ok(Instruction::AddcCcType(inst));
            }
        }
        "tcgen05" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_alloc::section_0::Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_alloc::section_0::Tcgen05DeallocCtaGroupSyncAlignedB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05DeallocCtaGroupSyncAlignedB32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_alloc::section_0::Tcgen05RelinquishAllocPermitCtaGroupSyncAligned as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05RelinquishAllocPermitCtaGroupSyncAligned(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_commit::section_0::Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_cp::section_0::Tcgen05CpCtaGroupShapeMulticastDstSrcFmt as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05CpCtaGroupShapeMulticastDstSrcFmt(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_fence::section_0::Tcgen05FenceBeforeThreadSync as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05FenceBeforeThreadSync(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_fence::section_0::Tcgen05FenceAfterThreadSync as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05FenceAfterThreadSync(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_ld::section_0::Tcgen05LdSyncAlignedShape1NumPackB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05LdSyncAlignedShape1NumPackB32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_ld::section_0::Tcgen05LdSyncAlignedShape2NumPackB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05LdSyncAlignedShape2NumPackB32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_ld::section_0::Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_ld::section_0::Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_ld::section_0::Tcgen05LdRedSyncAlignedShape3NumRedopType as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05LdRedSyncAlignedShape3NumRedopType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_ld::section_0::Tcgen05LdRedSyncAlignedShape4NumRedopType as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05LdRedSyncAlignedShape4NumRedopType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_0::Tcgen05MmaSpCtaGroupKind as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKind(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_0::Tcgen05MmaSpCtaGroupKind1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKind1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_1::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_1::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_2::Tcgen05MmaSpCtaGroupKindCollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKindCollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_2::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_2::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_3::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_3::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_4::Tcgen05MmaSpCtaGroupKindI8 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKindI8(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_4::Tcgen05MmaSpCtaGroupKindI81 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKindI81(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_5::Tcgen05MmaSpCtaGroupKindI8CollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKindI8CollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_5::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_sp::section_5::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_0::Tcgen05MmaCtaGroupKind as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKind(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_0::Tcgen05MmaCtaGroupKind1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKind1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_1::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_1::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_2::Tcgen05MmaCtaGroupKindCollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKindCollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_2::Tcgen05MmaCtaGroupKindAshiftCollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKindAshiftCollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_2::Tcgen05MmaCtaGroupKindAshiftCollectorUsage1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKindAshiftCollectorUsage1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_3::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_3::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_4::Tcgen05MmaCtaGroupKindI8 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKindI8(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_4::Tcgen05MmaCtaGroupKindI81 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKindI81(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_5::Tcgen05MmaCtaGroupKindI8CollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKindI8CollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_5::Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma::section_5::Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_ws_sp::section_0::Tcgen05MmaWsSpCtaGroup1KindCollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaWsSpCtaGroup1KindCollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_ws_sp::section_0::Tcgen05MmaWsSpCtaGroup1KindCollectorUsage1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaWsSpCtaGroup1KindCollectorUsage1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_ws_sp::section_1::Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_ws_sp::section_1::Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_ws::section_0::Tcgen05MmaWsCtaGroup1KindCollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaWsCtaGroup1KindCollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_ws::section_0::Tcgen05MmaWsCtaGroup1KindCollectorUsage1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaWsCtaGroup1KindCollectorUsage1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_ws::section_1::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_mma_ws::section_1::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage1 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_shift::section_0::Tcgen05ShiftCtaGroupDown as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05ShiftCtaGroupDown(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_st::section_0::Tcgen05StSyncAlignedShape1NumUnpackB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05StSyncAlignedShape1NumUnpackB32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_st::section_0::Tcgen05StSyncAlignedShape2NumUnpackB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05StSyncAlignedShape2NumUnpackB32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tcgen05_wait::section_0::Tcgen05WaitOperationSyncAligned as PtxParser>::parse(stream) {
                return Ok(Instruction::Tcgen05WaitOperationSyncAligned(inst));
            }
        }
        "shr" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::shr::section_0::ShrType as PtxParser>::parse(stream) {
                return Ok(Instruction::ShrType(inst));
            }
        }
        "suq" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::suq::section_0::SuqQueryB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::SuqQueryB32(inst));
            }
        }
        "pmevent" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::pmevent::section_0::Pmevent as PtxParser>::parse(stream) {
                return Ok(Instruction::Pmevent(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::pmevent::section_0::PmeventMask as PtxParser>::parse(stream) {
                return Ok(Instruction::PmeventMask(inst));
            }
        }
        "testp" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::testp::section_0::TestpOpType as PtxParser>::parse(stream) {
                return Ok(Instruction::TestpOpType(inst));
            }
        }
        "shf" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::shf::section_0::ShfLModeB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::ShfLModeB32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::shf::section_0::ShfRModeB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::ShfRModeB32(inst));
            }
        }
        "tld4" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tld4::section_0::Tld4Comp2dV4DtypeF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tld4Comp2dV4DtypeF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::tld4::section_0::Tld4CompGeomV4DtypeF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Tld4CompGeomV4DtypeF32(inst));
            }
        }
        "vset4" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vset4::section_0::Vset4AtypeBtypeCmp as PtxParser>::parse(stream) {
                return Ok(Instruction::Vset4AtypeBtypeCmp(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vset4::section_0::Vset4AtypeBtypeCmpAdd as PtxParser>::parse(stream) {
                return Ok(Instruction::Vset4AtypeBtypeCmpAdd(inst));
            }
        }
        "atom" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceOpLevelCacheHintType as PtxParser>::parse(stream) {
                return Ok(Instruction::AtomSemScopeSpaceOpLevelCacheHintType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceOpType as PtxParser>::parse(stream) {
                return Ok(Instruction::AtomSemScopeSpaceOpType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceCasB16 as PtxParser>::parse(stream) {
                return Ok(Instruction::AtomSemScopeSpaceCasB16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceCasB128 as PtxParser>::parse(stream) {
                return Ok(Instruction::AtomSemScopeSpaceCasB128(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceExchLevelCacheHintB128 as PtxParser>::parse(stream) {
                return Ok(Instruction::AtomSemScopeSpaceExchLevelCacheHintB128(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceAddNoftzLevelCacheHintF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::AtomSemScopeSpaceAddNoftzLevelCacheHintF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceAddNoftzLevelCacheHintBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::AtomSemScopeSpaceAddNoftzLevelCacheHintBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::atom::section_1::AtomSemScopeGlobalAddLevelCacheHintVec32BitF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::AtomSemScopeGlobalAddLevelCacheHintVec32BitF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::atom::section_1::AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType as PtxParser>::parse(stream) {
                return Ok(Instruction::AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::atom::section_1::AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType as PtxParser>::parse(stream) {
                return Ok(Instruction::AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType(inst));
            }
        }
        "fns" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::fns::section_0::FnsB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::FnsB32(inst));
            }
        }
        "red" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red_async::section_0::RedAsyncSemScopeSsCompletionMechanismOpType as PtxParser>::parse(stream) {
                return Ok(Instruction::RedAsyncSemScopeSsCompletionMechanismOpType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red_async::section_1::RedAsyncSemScopeSsCompletionMechanismOpType1 as PtxParser>::parse(stream) {
                return Ok(Instruction::RedAsyncSemScopeSsCompletionMechanismOpType1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red_async::section_2::RedAsyncSemScopeSsCompletionMechanismOpType2 as PtxParser>::parse(stream) {
                return Ok(Instruction::RedAsyncSemScopeSsCompletionMechanismOpType2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red_async::section_3::RedAsyncSemScopeSsCompletionMechanismAddType as PtxParser>::parse(stream) {
                return Ok(Instruction::RedAsyncSemScopeSsCompletionMechanismAddType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red_async::section_4::RedAsyncMmioSemScopeSsAddType as PtxParser>::parse(stream) {
                return Ok(Instruction::RedAsyncMmioSemScopeSsAddType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red::section_0::RedOpSpaceSemScopeLevelCacheHintType as PtxParser>::parse(stream) {
                return Ok(Instruction::RedOpSpaceSemScopeLevelCacheHintType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red::section_0::RedAddSpaceSemScopeNoftzLevelCacheHintF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::RedAddSpaceSemScopeNoftzLevelCacheHintF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red::section_0::RedAddSpaceSemScopeNoftzLevelCacheHintF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::RedAddSpaceSemScopeNoftzLevelCacheHintF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red::section_0::RedAddSpaceSemScopeNoftzLevelCacheHintBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::RedAddSpaceSemScopeNoftzLevelCacheHintBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red::section_0::RedAddSpaceSemScopeNoftzLevelCacheHintBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::RedAddSpaceSemScopeNoftzLevelCacheHintBf16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red::section_1::RedAddSpaceSemScopeLevelCacheHintVec32BitF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::RedAddSpaceSemScopeLevelCacheHintVec32BitF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red::section_1::RedOpSpaceSemScopeNoftzLevelCacheHintVec16BitHalfWordType as PtxParser>::parse(stream) {
                return Ok(Instruction::RedOpSpaceSemScopeNoftzLevelCacheHintVec16BitHalfWordType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::red::section_1::RedOpSpaceSemScopeNoftzLevelCacheHintVec32BitPackedType as PtxParser>::parse(stream) {
                return Ok(Instruction::RedOpSpaceSemScopeNoftzLevelCacheHintVec32BitPackedType(inst));
            }
        }
        "add" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::add_cc::section_0::AddCcType as PtxParser>::parse(stream) {
                return Ok(Instruction::AddCcType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::add::section_0::AddType as PtxParser>::parse(stream) {
                return Ok(Instruction::AddType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::add::section_0::AddSatS32 as PtxParser>::parse(stream) {
                return Ok(Instruction::AddSatS32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::add::section_1::AddRndFtzSatF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::AddRndFtzSatF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::add::section_1::AddRndFtzF32x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::AddRndFtzF32x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::add::section_1::AddRndF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::AddRndF64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::add::section_2::AddRndFtzSatF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::AddRndFtzSatF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::add::section_2::AddRndFtzSatF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::AddRndFtzSatF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::add::section_2::AddRndBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::AddRndBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::add::section_2::AddRndBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::AddRndBf16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::add::section_3::AddRndSatF32Atype as PtxParser>::parse(stream) {
                return Ok(Instruction::AddRndSatF32Atype(inst));
            }
        }
        "mul" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mul::section_0::MulModeType as PtxParser>::parse(stream) {
                return Ok(Instruction::MulModeType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mul::section_1::MulRndFtzSatF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MulRndFtzSatF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mul::section_1::MulRndFtzF32x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::MulRndFtzF32x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mul::section_1::MulRndF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MulRndF64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mul::section_2::MulRndFtzSatF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::MulRndFtzSatF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mul::section_2::MulRndFtzSatF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::MulRndFtzSatF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mul::section_2::MulRndBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::MulRndBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mul::section_2::MulRndBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::MulRndBf16x2(inst));
            }
        }
        "rsqrt" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::rsqrt_approx_ftz_f64::section_0::RsqrtApproxFtzF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::RsqrtApproxFtzF64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::rsqrt::section_0::RsqrtApproxFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::RsqrtApproxFtzF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::rsqrt::section_0::RsqrtApproxF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::RsqrtApproxF64(inst));
            }
        }
        "selp" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::selp::section_0::SelpType as PtxParser>::parse(stream) {
                return Ok(Instruction::SelpType(inst));
            }
        }
        "wgmma" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_commit_group::section_0::WgmmaCommitGroupSyncAligned as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaCommitGroupSyncAligned(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_fence::section_0::WgmmaFenceSyncAligned as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaFenceSyncAligned(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async_sp::section_0::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F16 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async_sp::section_0::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F161 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F161(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async_sp::section_1::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async_sp::section_1::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf161 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf161(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async_sp::section_2::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf32 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async_sp::section_2::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf321 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf321(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async_sp::section_3::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async_sp::section_3::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async_sp::section_4::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async_sp::section_4::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async::section_0::WgmmaMmaAsyncSyncAlignedShapeDtypeF16F16 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSyncAlignedShapeDtypeF16F16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async::section_0::WgmmaMmaAsyncSyncAlignedShapeDtypeF16F161 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSyncAlignedShapeDtypeF16F161(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async::section_1::WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async::section_1::WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf161 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf161(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async::section_2::WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf32 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async::section_2::WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf321 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf321(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async::section_3::WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async::section_3::WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async::section_4::WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async::section_4::WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async::section_5::WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_mma_async::section_5::WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc1 as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wgmma_wait_group::section_0::WgmmaWaitGroupSyncAligned as PtxParser>::parse(stream) {
                return Ok(Instruction::WgmmaWaitGroupSyncAligned(inst));
            }
        }
        "activemask" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::activemask::section_0::ActivemaskB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::ActivemaskB32(inst));
            }
        }
        "griddepcontrol" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::griddepcontrol::section_0::GriddepcontrolAction as PtxParser>::parse(stream) {
                return Ok(Instruction::GriddepcontrolAction(inst));
            }
        }
        "or" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::or::section_0::OrType as PtxParser>::parse(stream) {
                return Ok(Instruction::OrType(inst));
            }
        }
        "lop3" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::lop3::section_0::Lop3B32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Lop3B32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::lop3::section_0::Lop3BoolopB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Lop3BoolopB32(inst));
            }
        }
        "rem" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::rem::section_0::RemType as PtxParser>::parse(stream) {
                return Ok(Instruction::RemType(inst));
            }
        }
        "prmt" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::prmt::section_0::PrmtB32Mode as PtxParser>::parse(stream) {
                return Ok(Instruction::PrmtB32Mode(inst));
            }
        }
        "wmma" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_0::WmmaLoadASyncAlignedLayoutShapeSsAtype as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadASyncAlignedLayoutShapeSsAtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_0::WmmaLoadBSyncAlignedLayoutShapeSsBtype as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadBSyncAlignedLayoutShapeSsBtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_0::WmmaLoadCSyncAlignedLayoutShapeSsCtype as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadCSyncAlignedLayoutShapeSsCtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_1::WmmaLoadASyncAlignedLayoutShapeSsAtype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadASyncAlignedLayoutShapeSsAtype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_1::WmmaLoadBSyncAlignedLayoutShapeSsBtype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadBSyncAlignedLayoutShapeSsBtype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_1::WmmaLoadCSyncAlignedLayoutShapeSsCtype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadCSyncAlignedLayoutShapeSsCtype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_2::WmmaLoadASyncAlignedLayoutShapeSsAtype2 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadASyncAlignedLayoutShapeSsAtype2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_2::WmmaLoadBSyncAlignedLayoutShapeSsBtype2 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadBSyncAlignedLayoutShapeSsBtype2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_2::WmmaLoadCSyncAlignedLayoutShapeSsCtype2 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadCSyncAlignedLayoutShapeSsCtype2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_3::WmmaLoadASyncAlignedLayoutShapeSsAtype3 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadASyncAlignedLayoutShapeSsAtype3(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_3::WmmaLoadBSyncAlignedLayoutShapeSsBtype3 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadBSyncAlignedLayoutShapeSsBtype3(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_3::WmmaLoadCSyncAlignedLayoutShapeSsCtype3 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadCSyncAlignedLayoutShapeSsCtype3(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_4::WmmaLoadASyncAlignedRowShapeSsAtype as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadASyncAlignedRowShapeSsAtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_4::WmmaLoadBSyncAlignedColShapeSsBtype as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadBSyncAlignedColShapeSsBtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_4::WmmaLoadCSyncAlignedLayoutShapeSsCtype4 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadCSyncAlignedLayoutShapeSsCtype4(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_5::WmmaLoadASyncAlignedRowShapeSsAtype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadASyncAlignedRowShapeSsAtype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_5::WmmaLoadBSyncAlignedColShapeSsBtype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadBSyncAlignedColShapeSsBtype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_load::section_5::WmmaLoadCSyncAlignedLayoutShapeSsCtype5 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaLoadCSyncAlignedLayoutShapeSsCtype5(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_mma::section_0::WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_mma::section_1::WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_mma::section_2::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_mma::section_3::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_mma::section_4::WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_mma::section_5::WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_mma::section_6::WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_store::section_0::WmmaStoreDSyncAlignedLayoutShapeSsType as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaStoreDSyncAlignedLayoutShapeSsType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_store::section_1::WmmaStoreDSyncAlignedLayoutShapeSsType1 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaStoreDSyncAlignedLayoutShapeSsType1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_store::section_2::WmmaStoreDSyncAlignedLayoutShapeSsType2 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaStoreDSyncAlignedLayoutShapeSsType2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::wmma_store::section_3::WmmaStoreDSyncAlignedLayoutShapeSsType3 as PtxParser>::parse(stream) {
                return Ok(Instruction::WmmaStoreDSyncAlignedLayoutShapeSsType3(inst));
            }
        }
        "discard" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::discard::section_0::DiscardGlobalLevel as PtxParser>::parse(stream) {
                return Ok(Instruction::DiscardGlobalLevel(inst));
            }
        }
        "match" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::match_sync::section_0::MatchAnySyncType as PtxParser>::parse(stream) {
                return Ok(Instruction::MatchAnySyncType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::match_sync::section_0::MatchAllSyncType as PtxParser>::parse(stream) {
                return Ok(Instruction::MatchAllSyncType(inst));
            }
        }
        "ld" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ld_global_nc::section_0::LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType as PtxParser>::parse(stream) {
                return Ok(Instruction::LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ld_global_nc::section_0::LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ld_global_nc::section_0::LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType as PtxParser>::parse(stream) {
                return Ok(Instruction::LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ld_global_nc::section_0::LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ld::section_0::LdWeakSsCopLevelCacheHintLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::LdWeakSsCopLevelCacheHintLevelPrefetchSizeVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ld::section_0::LdWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::LdWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ld::section_0::LdVolatileSsLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::LdVolatileSsLevelPrefetchSizeVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ld::section_0::LdRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::LdRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ld::section_0::LdAcquireScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::LdAcquireScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ld::section_0::LdMmioRelaxedSysGlobalType as PtxParser>::parse(stream) {
                return Ok(Instruction::LdMmioRelaxedSysGlobalType(inst));
            }
        }
        "call" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::call::section_0::CallUni as PtxParser>::parse(stream) {
                return Ok(Instruction::CallUni(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::call::section_0::CallUni1 as PtxParser>::parse(stream) {
                return Ok(Instruction::CallUni1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::call::section_0::CallUni2 as PtxParser>::parse(stream) {
                return Ok(Instruction::CallUni2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::call::section_0::CallUni3 as PtxParser>::parse(stream) {
                return Ok(Instruction::CallUni3(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::call::section_0::CallUni4 as PtxParser>::parse(stream) {
                return Ok(Instruction::CallUni4(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::call::section_0::CallUni5 as PtxParser>::parse(stream) {
                return Ok(Instruction::CallUni5(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::call::section_0::CallUni6 as PtxParser>::parse(stream) {
                return Ok(Instruction::CallUni6(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::call::section_0::CallUni7 as PtxParser>::parse(stream) {
                return Ok(Instruction::CallUni7(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::call::section_0::CallUni8 as PtxParser>::parse(stream) {
                return Ok(Instruction::CallUni8(inst));
            }
        }
        "clz" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::clz::section_0::ClzType as PtxParser>::parse(stream) {
                return Ok(Instruction::ClzType(inst));
            }
        }
        "alloca" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::alloca::section_0::AllocaType as PtxParser>::parse(stream) {
                return Ok(Instruction::AllocaType(inst));
            }
        }
        "cvt" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt_pack::section_0::CvtPackSatConverttypeAbtype as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtPackSatConverttypeAbtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt_pack::section_1::CvtPackSatConverttypeAbtypeCtype as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtPackSatConverttypeAbtypeCtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtIrndFtzSatDtypeAtype as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtIrndFtzSatDtypeAtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtFrndFtzSatDtypeAtype as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtFrndFtzSatDtypeAtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtFrnd2ReluSatfiniteF16F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtFrnd2ReluSatfiniteF16F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtFrnd2ReluSatfiniteF16x2F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtFrnd2ReluSatfiniteF16x2F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRsReluSatfiniteF16x2F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRsReluSatfiniteF16x2F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtFrnd2ReluSatfiniteBf16F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtFrnd2ReluSatfiniteBf16F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtFrnd2ReluSatfiniteBf16x2F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtFrnd2ReluSatfiniteBf16x2F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRsReluSatfiniteBf16x2F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRsReluSatfiniteBf16x2F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRnaSatfiniteTf32F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRnaSatfiniteTf32F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtFrnd2SatfiniteReluTf32F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtFrnd2SatfiniteReluTf32F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRnSatfiniteReluF8x2typeF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRnSatfiniteReluF8x2typeF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRnSatfiniteReluF8x2typeF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRnSatfiniteReluF8x2typeF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRnReluF16x2F8x2type as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRnReluF16x2F8x2type(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRsReluSatfiniteF8x4typeF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRsReluSatfiniteF8x4typeF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRnSatfiniteReluF4x2typeF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRnSatfiniteReluF4x2typeF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRnReluF16x2F4x2type as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRnReluF16x2F4x2type(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRsReluSatfiniteF4x4typeF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRsReluSatfiniteF4x4typeF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRnSatfiniteReluF6x2typeF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRnSatfiniteReluF6x2typeF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRnReluF16x2F6x2type as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRnReluF16x2F6x2type(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRsReluSatfiniteF6x4typeF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRsReluSatfiniteF6x4typeF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtFrnd3SatfiniteUe8m0x2F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtFrnd3SatfiniteUe8m0x2F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtFrnd3SatfiniteUe8m0x2Bf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtFrnd3SatfiniteUe8m0x2Bf16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvt::section_0::CvtRnBf16x2Ue8m0x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtRnBf16x2Ue8m0x2(inst));
            }
        }
        "mbarrier" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_arrive::section_0::MbarrierArriveSemScopeStateB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierArriveSemScopeStateB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_arrive::section_0::MbarrierArriveSemScopeSharedClusterB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierArriveSemScopeSharedClusterB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_arrive::section_0::MbarrierArriveExpectTxSemScopeStateB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierArriveExpectTxSemScopeStateB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_arrive::section_0::MbarrierArriveExpectTxSemScopeSharedClusterB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierArriveExpectTxSemScopeSharedClusterB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_arrive::section_0::MbarrierArriveNocompleteReleaseCtaStateB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierArriveNocompleteReleaseCtaStateB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_arrive_drop::section_0::MbarrierArriveDropSemScopeStateB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierArriveDropSemScopeStateB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_arrive_drop::section_0::MbarrierArriveDropSemScopeSharedClusterB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierArriveDropSemScopeSharedClusterB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_arrive_drop::section_0::MbarrierArriveDropExpectTxStateSemScopeB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierArriveDropExpectTxStateSemScopeB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_arrive_drop::section_0::MbarrierArriveDropExpectTxSharedClusterSemScopeB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierArriveDropExpectTxSharedClusterSemScopeB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_arrive_drop::section_0::MbarrierArriveDropNocompleteReleaseCtaStateB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierArriveDropNocompleteReleaseCtaStateB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_complete_tx::section_0::MbarrierCompleteTxSemScopeSpaceB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierCompleteTxSemScopeSpaceB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_expect_tx::section_0::MbarrierExpectTxSemScopeSpaceB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierExpectTxSemScopeSpaceB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_init::section_0::MbarrierInitStateB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierInitStateB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_inval::section_0::MbarrierInvalStateB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierInvalStateB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_pending_count::section_0::MbarrierPendingCountB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierPendingCountB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_test_wait::section_0::MbarrierTestWaitSemScopeStateB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierTestWaitSemScopeStateB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_test_wait::section_0::MbarrierTestWaitParitySemScopeStateB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierTestWaitParitySemScopeStateB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_test_wait::section_0::MbarrierTryWaitSemScopeStateB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierTryWaitSemScopeStateB64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mbarrier_test_wait::section_0::MbarrierTryWaitParitySemScopeStateB64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MbarrierTryWaitParitySemScopeStateB64(inst));
            }
        }
        "prefetchu" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::prefetch::section_0::PrefetchSpaceLevel as PtxParser>::parse(stream) {
                return Ok(Instruction::PrefetchSpaceLevel(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::prefetch::section_0::PrefetchGlobalLevelEvictionPriority as PtxParser>::parse(stream) {
                return Ok(Instruction::PrefetchGlobalLevelEvictionPriority(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::prefetch::section_0::PrefetchuL1 as PtxParser>::parse(stream) {
                return Ok(Instruction::PrefetchuL1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::prefetch::section_0::PrefetchTensormapSpaceTensormap as PtxParser>::parse(stream) {
                return Ok(Instruction::PrefetchTensormapSpaceTensormap(inst));
            }
        }
        "redux" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::redux_sync::section_0::ReduxSyncOpType as PtxParser>::parse(stream) {
                return Ok(Instruction::ReduxSyncOpType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::redux_sync::section_1::ReduxSyncOpB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::ReduxSyncOpB32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::redux_sync::section_2::ReduxSyncOpAbsNanF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::ReduxSyncOpAbsNanF32(inst));
            }
        }
        "neg" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::neg::section_0::NegType as PtxParser>::parse(stream) {
                return Ok(Instruction::NegType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::neg::section_0::NegFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::NegFtzF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::neg::section_0::NegF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::NegF64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::neg::section_0::NegFtzF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::NegFtzF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::neg::section_0::NegFtzF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::NegFtzF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::neg::section_0::NegBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::NegBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::neg::section_0::NegBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::NegBf16x2(inst));
            }
        }
        "sqrt" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sqrt::section_0::SqrtApproxFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::SqrtApproxFtzF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sqrt::section_0::SqrtRndFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::SqrtRndFtzF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sqrt::section_0::SqrtRndF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::SqrtRndF64(inst));
            }
        }
        "vop4" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vop4::section_0::Vop4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                return Ok(Instruction::Vop4DtypeAtypeBtypeSat(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vop4::section_0::Vop4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                return Ok(Instruction::Vop4DtypeAtypeBtypeAdd(inst));
            }
        }
        "sin" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::sin::section_0::SinApproxFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::SinApproxFtzF32(inst));
            }
        }
        "vop" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vop::section_0::VopDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                return Ok(Instruction::VopDtypeAtypeBtypeSat(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vop::section_0::VopDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                return Ok(Instruction::VopDtypeAtypeBtypeSatOp2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vop::section_0::VopDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                return Ok(Instruction::VopDtypeAtypeBtypeSat1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vsh::section_0::VopDtypeAtypeU32SatMode as PtxParser>::parse(stream) {
                return Ok(Instruction::VopDtypeAtypeU32SatMode(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vsh::section_0::VopDtypeAtypeU32SatModeOp2 as PtxParser>::parse(stream) {
                return Ok(Instruction::VopDtypeAtypeU32SatModeOp2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vsh::section_0::VopDtypeAtypeU32SatMode1 as PtxParser>::parse(stream) {
                return Ok(Instruction::VopDtypeAtypeU32SatMode1(inst));
            }
        }
        "cos" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cos::section_0::CosApproxFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::CosApproxFtzF32(inst));
            }
        }
        "bmsk" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::bmsk::section_0::BmskModeB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::BmskModeB32(inst));
            }
        }
        "cnot" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cnot::section_0::CnotType as PtxParser>::parse(stream) {
                return Ok(Instruction::CnotType(inst));
            }
        }
        "multimem" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::multimem_ld_reduce::section_0::MultimemLdReduceLdsemScopeSsOpType as PtxParser>::parse(stream) {
                return Ok(Instruction::MultimemLdReduceLdsemScopeSsOpType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::multimem_ld_reduce::section_0::MultimemLdReduceWeakSsOpType as PtxParser>::parse(stream) {
                return Ok(Instruction::MultimemLdReduceWeakSsOpType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::multimem_ld_reduce::section_0::MultimemStStsemScopeSsType as PtxParser>::parse(stream) {
                return Ok(Instruction::MultimemStStsemScopeSsType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::multimem_ld_reduce::section_0::MultimemStWeakSsType as PtxParser>::parse(stream) {
                return Ok(Instruction::MultimemStWeakSsType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::multimem_ld_reduce::section_0::MultimemRedRedsemScopeSsOpType as PtxParser>::parse(stream) {
                return Ok(Instruction::MultimemRedRedsemScopeSsOpType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::multimem_ld_reduce::section_1::MultimemLdReduceLdsemScopeSsOpAccPrecVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::MultimemLdReduceLdsemScopeSsOpAccPrecVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::multimem_ld_reduce::section_1::MultimemLdReduceWeakSsOpAccPrecVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::MultimemLdReduceWeakSsOpAccPrecVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::multimem_ld_reduce::section_1::MultimemStStsemScopeSsVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::MultimemStStsemScopeSsVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::multimem_ld_reduce::section_1::MultimemStWeakSsVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::MultimemStWeakSsVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::multimem_ld_reduce::section_1::MultimemRedRedsemScopeSsRedopVecRedtype as PtxParser>::parse(stream) {
                return Ok(Instruction::MultimemRedRedsemScopeSsRedopVecRedtype(inst));
            }
        }
        "mov" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mov::section_0::MovType as PtxParser>::parse(stream) {
                return Ok(Instruction::MovType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mov::section_0::MovU32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MovU32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mov::section_0::MovU64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MovU64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mov::section_0::MovU321 as PtxParser>::parse(stream) {
                return Ok(Instruction::MovU321(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mov::section_0::MovU641 as PtxParser>::parse(stream) {
                return Ok(Instruction::MovU641(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mov::section_1::MovType1 as PtxParser>::parse(stream) {
                return Ok(Instruction::MovType1(inst));
            }
        }
        "applypriority" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::applypriority::section_0::ApplypriorityGlobalLevelEvictionPriority as PtxParser>::parse(stream) {
                return Ok(Instruction::ApplypriorityGlobalLevelEvictionPriority(inst));
            }
        }
        "mapa" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mapa::section_0::MapaSpaceType as PtxParser>::parse(stream) {
                return Ok(Instruction::MapaSpaceType(inst));
            }
        }
        "div" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::div::section_0::DivType as PtxParser>::parse(stream) {
                return Ok(Instruction::DivType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::div::section_0::DivApproxFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::DivApproxFtzF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::div::section_0::DivFullFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::DivFullFtzF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::div::section_0::DivRndFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::DivRndFtzF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::div::section_0::DivRndF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::DivRndF64(inst));
            }
        }
        "mma" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_0::MmaSpvariantSyncAlignedM16n8k16RowColDtypeF16F16Ctype as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpvariantSyncAlignedM16n8k16RowColDtypeF16F16Ctype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_0::MmaSpvariantSyncAlignedM16n8k32RowColDtypeF16F16Ctype as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpvariantSyncAlignedM16n8k32RowColDtypeF16F16Ctype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k16RowColF32Bf16Bf16F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpvariantSyncAlignedM16n8k16RowColF32Bf16Bf16F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k32RowColF32Bf16Bf16F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpvariantSyncAlignedM16n8k32RowColF32Bf16Bf16F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k8RowColF32Tf32Tf32F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpvariantSyncAlignedM16n8k8RowColF32Tf32Tf32F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k16RowColF32Tf32Tf32F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpvariantSyncAlignedM16n8k16RowColF32Tf32Tf32F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k64RowColF32F8typeF8typeF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpvariantSyncAlignedM16n8k64RowColF32F8typeF8typeF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_1::MmaSpOrderedMetadataSyncAlignedM16n8k64RowColKindDtypeF8f6f4typeF8f6f4typeCtype as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpOrderedMetadataSyncAlignedM16n8k64RowColKindDtypeF8f6f4typeF8f6f4typeCtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_2::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_3::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_4::MmaSpvariantSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpvariantSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_5::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma_sp::section_6::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_0::MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_0::MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_0::MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_1::MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_1::MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_1::MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_1::MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_1::MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_2::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_3::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_4::MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_5::MmaSyncAlignedShapeRowColF64F64F64F64 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedShapeRowColF64F64F64F64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_6::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_7::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321 as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::mma::section_8::MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc as PtxParser>::parse(stream) {
                return Ok(Instruction::MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc(inst));
            }
        }
        "st" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::st_async::section_0::StAsyncSemScopeSsCompletionMechanismVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::StAsyncSemScopeSsCompletionMechanismVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::st_async::section_1::StAsyncMmioSemScopeSsType as PtxParser>::parse(stream) {
                return Ok(Instruction::StAsyncMmioSemScopeSsType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::st_bulk::section_0::StBulkWeakSharedCta as PtxParser>::parse(stream) {
                return Ok(Instruction::StBulkWeakSharedCta(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::st::section_0::StWeakSsCopLevelCacheHintVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::StWeakSsCopLevelCacheHintVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::st::section_0::StWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::StWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::st::section_0::StVolatileSsVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::StVolatileSsVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::st::section_0::StRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::StRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::st::section_0::StReleaseScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::StReleaseScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::st::section_0::StMmioRelaxedSysGlobalType as PtxParser>::parse(stream) {
                return Ok(Instruction::StMmioRelaxedSysGlobalType(inst));
            }
        }
        "abs" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::abs::section_0::AbsType as PtxParser>::parse(stream) {
                return Ok(Instruction::AbsType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::abs::section_0::AbsFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::AbsFtzF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::abs::section_0::AbsF64 as PtxParser>::parse(stream) {
                return Ok(Instruction::AbsF64(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::abs::section_0::AbsFtzF16 as PtxParser>::parse(stream) {
                return Ok(Instruction::AbsFtzF16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::abs::section_0::AbsFtzF16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::AbsFtzF16x2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::abs::section_0::AbsBf16 as PtxParser>::parse(stream) {
                return Ok(Instruction::AbsBf16(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::abs::section_0::AbsBf16x2 as PtxParser>::parse(stream) {
                return Ok(Instruction::AbsBf16x2(inst));
            }
        }
        "fence" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceSemScope as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceSemScope(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceAcquireSyncRestrictSharedClusterCluster as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceAcquireSyncRestrictSharedClusterCluster(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceReleaseSyncRestrictSharedCtaCluster as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceReleaseSyncRestrictSharedCtaCluster(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceOpRestrictReleaseCluster as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceOpRestrictReleaseCluster(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceProxyProxykind as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceProxyProxykind(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceProxyToProxykindFromProxykindReleaseScope as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceProxyToProxykindFromProxykindReleaseScope(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceProxyToProxykindFromProxykindAcquireScope as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceProxyToProxykindFromProxykindAcquireScope(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster as PtxParser>::parse(stream) {
                return Ok(Instruction::FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::MembarLevel as PtxParser>::parse(stream) {
                return Ok(Instruction::MembarLevel(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::membar::section_0::MembarProxyProxykind as PtxParser>::parse(stream) {
                return Ok(Instruction::MembarProxyProxykind(inst));
            }
        }
        "txq" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::txq::section_0::TxqTqueryB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::TxqTqueryB32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::txq::section_0::TxqLevelTlqueryB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::TxqLevelTlqueryB32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::txq::section_0::TxqSqueryB32 as PtxParser>::parse(stream) {
                return Ok(Instruction::TxqSqueryB32(inst));
            }
        }
        "vset" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vset::section_0::VsetAtypeBtypeCmp as PtxParser>::parse(stream) {
                return Ok(Instruction::VsetAtypeBtypeCmp(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vset::section_0::VsetAtypeBtypeCmpOp2 as PtxParser>::parse(stream) {
                return Ok(Instruction::VsetAtypeBtypeCmpOp2(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::vset::section_0::VsetAtypeBtypeCmp1 as PtxParser>::parse(stream) {
                return Ok(Instruction::VsetAtypeBtypeCmp1(inst));
            }
        }
        "ldu" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ldu::section_0::LduSsType as PtxParser>::parse(stream) {
                return Ok(Instruction::LduSsType(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ldu::section_0::LduSsVecType as PtxParser>::parse(stream) {
                return Ok(Instruction::LduSsVecType(inst));
            }
        }
        "cvta" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvta::section_0::CvtaSpaceSize as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtaSpaceSize(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::cvta::section_0::CvtaToSpaceSize as PtxParser>::parse(stream) {
                return Ok(Instruction::CvtaToSpaceSize(inst));
            }
        }
        "copysign" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::copysign::section_0::CopysignType as PtxParser>::parse(stream) {
                return Ok(Instruction::CopysignType(inst));
            }
        }
        "dp2a" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::dp2a::section_0::Dp2aModeAtypeBtype as PtxParser>::parse(stream) {
                return Ok(Instruction::Dp2aModeAtypeBtype(inst));
            }
        }
        "ex2" => {
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ex2::section_0::Ex2ApproxFtzF32 as PtxParser>::parse(stream) {
                return Ok(Instruction::Ex2ApproxFtzF32(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ex2::section_0::Ex2ApproxAtype as PtxParser>::parse(stream) {
                return Ok(Instruction::Ex2ApproxAtype(inst));
            }
            stream.set_position(start_pos);
            if let Ok(inst) = <crate::r#type::instruction::ex2::section_0::Ex2ApproxFtzBtype as PtxParser>::parse(stream) {
                return Ok(Instruction::Ex2ApproxFtzBtype(inst));
            }
        }
        _ => {}
    }
    
    // If no parser matched, return error
    let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(0..0);
    Err(crate::parser::unexpected_value(span, &["valid PTX instruction"], "no matching instruction format"))
}

impl PtxParser for Instruction {
    /// Parse a PTX instruction with optional label and predicate
    ///
    /// Format: [label:] [@{!}pred] instruction
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        // Optional label (ends with colon)
        // Labels are handled by the module parser, not here
        
        // Optional predicate: @{!}pred or @!pred
        let _predicate = if stream.check(|t| matches!(t, PtxToken::At)) {
            stream.consume()?; // consume @
            
            // Optional negation
            let _negated = stream.consume_if(|t| matches!(t, PtxToken::Exclaim)).is_some();

            // Predicate operand (can be register %p1 or identifier p)
            let _pred = Operand::parse(stream)?;

            Some(())
        } else {
            None
        };
        
        // Parse the actual instruction
        parse_instruction_inner(stream)
    }
}
