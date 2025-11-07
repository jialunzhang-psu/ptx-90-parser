// Auto-generated module declarations
// DO NOT EDIT MANUALLY
#![allow(unused)]

use crate::parser::{PtxParser, PtxParseError, PtxTokenStream, Span};
use crate::r#type::instruction::Inst;
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
pub(crate) fn parse_instruction_inner(stream: &mut PtxTokenStream) -> Result<Inst, PtxParseError> {
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
        "abs" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::abs::section_0::AbsType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AbsType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::abs::section_0::AbsFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AbsFtzF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::abs::section_0::AbsF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AbsF64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::abs::section_0::AbsFtzF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AbsFtzF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::abs::section_0::AbsFtzF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AbsFtzF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::abs::section_0::AbsBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AbsBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::abs::section_0::AbsBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AbsBf16x2(inst)),
                Err(_) => {}
            }
        }
        "activemask" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::activemask::section_0::ActivemaskB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ActivemaskB32(inst)),
                Err(_) => {}
            }
        }
        "add" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::add_cc::section_0::AddCcType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AddCcType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::add::section_0::AddType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AddType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::add::section_0::AddSatS32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AddSatS32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::add::section_1::AddRndFtzSatF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AddRndFtzSatF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::add::section_1::AddRndFtzF32x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AddRndFtzF32x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::add::section_1::AddRndF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AddRndF64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::add::section_2::AddRndFtzSatF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AddRndFtzSatF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::add::section_2::AddRndFtzSatF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AddRndFtzSatF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::add::section_2::AddRndBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AddRndBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::add::section_2::AddRndBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AddRndBf16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::add::section_3::AddRndSatF32Atype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AddRndSatF32Atype(inst)),
                Err(_) => {}
            }
        }
        "addc" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::addc::section_0::AddcCcType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AddcCcType(inst)),
                Err(_) => {}
            }
        }
        "alloca" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::alloca::section_0::AllocaType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AllocaType(inst)),
                Err(_) => {}
            }
        }
        "and" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::and::section_0::AndType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AndType(inst)),
                Err(_) => {}
            }
        }
        "applypriority" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::applypriority::section_0::ApplypriorityGlobalLevelEvictionPriority as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ApplypriorityGlobalLevelEvictionPriority(inst)),
                Err(_) => {}
            }
        }
        "atom" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceOpLevelCacheHintType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AtomSemScopeSpaceOpLevelCacheHintType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceOpType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AtomSemScopeSpaceOpType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceCasB16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AtomSemScopeSpaceCasB16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceCasB128 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AtomSemScopeSpaceCasB128(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceExchLevelCacheHintB128 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AtomSemScopeSpaceExchLevelCacheHintB128(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceAddNoftzLevelCacheHintF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AtomSemScopeSpaceAddNoftzLevelCacheHintF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AtomSemScopeSpaceAddNoftzLevelCacheHintF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceAddNoftzLevelCacheHintBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AtomSemScopeSpaceAddNoftzLevelCacheHintBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::atom::section_0::AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AtomSemScopeSpaceAddNoftzLevelCacheHintBf16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::atom::section_1::AtomSemScopeGlobalAddLevelCacheHintVec32BitF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AtomSemScopeGlobalAddLevelCacheHintVec32BitF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::atom::section_1::AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AtomSemScopeGlobalOpNoftzLevelCacheHintVec16BitHalfWordType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::atom::section_1::AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::AtomSemScopeGlobalOpNoftzLevelCacheHintVec32BitPackedType(inst)),
                Err(_) => {}
            }
        }
        "bar" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarrierCtaSyncAligned as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarrierCtaSyncAligned(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarrierCtaArriveAligned as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarrierCtaArriveAligned(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarrierCtaRedPopcAlignedU32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarrierCtaRedPopcAlignedU32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarrierCtaRedOpAlignedPred as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarrierCtaRedOpAlignedPred(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarCtaSync as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarCtaSync(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarCtaArrive as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarCtaArrive(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarCtaRedPopcU32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarCtaRedPopcU32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarCtaRedOpPred as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarCtaRedOpPred(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar_warp_sync::section_0::BarWarpSync as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarWarpSync(inst)),
                Err(_) => {}
            }
        }
        "barrier" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarrierCtaSyncAligned as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarrierCtaSyncAligned(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarrierCtaArriveAligned as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarrierCtaArriveAligned(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarrierCtaRedPopcAlignedU32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarrierCtaRedPopcAlignedU32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarrierCtaRedOpAlignedPred as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarrierCtaRedOpAlignedPred(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarCtaSync as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarCtaSync(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarCtaArrive as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarCtaArrive(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarCtaRedPopcU32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarCtaRedPopcU32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bar::section_0::BarCtaRedOpPred as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarCtaRedOpPred(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::barrier_cluster::section_0::BarrierClusterArriveSemAligned as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarrierClusterArriveSemAligned(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::barrier_cluster::section_0::BarrierClusterWaitAcquireAligned as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BarrierClusterWaitAcquireAligned(inst)),
                Err(_) => {}
            }
        }
        "bfe" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bfe::section_0::BfeType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BfeType(inst)),
                Err(_) => {}
            }
        }
        "bfi" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bfi::section_0::BfiType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BfiType(inst)),
                Err(_) => {}
            }
        }
        "bfind" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bfind::section_0::BfindType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BfindType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bfind::section_0::BfindShiftamtType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BfindShiftamtType(inst)),
                Err(_) => {}
            }
        }
        "bmsk" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bmsk::section_0::BmskModeB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BmskModeB32(inst)),
                Err(_) => {}
            }
        }
        "bra" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bra::section_0::BraUni as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BraUni(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::bra::section_0::BraUni1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BraUni1(inst)),
                Err(_) => {}
            }
        }
        "brev" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::brev::section_0::BrevType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BrevType(inst)),
                Err(_) => {}
            }
        }
        "brkpt" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::brkpt::section_0::Brkpt as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Brkpt(inst)),
                Err(_) => {}
            }
        }
        "brx" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::brx_idx::section_0::BrxIdxUni as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BrxIdxUni(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::brx_idx::section_0::BrxIdxUni1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::BrxIdxUni1(inst)),
                Err(_) => {}
            }
        }
        "call" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::call::section_0::CallUni as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CallUni(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::call::section_0::CallUni1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CallUni1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::call::section_0::CallUni2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CallUni2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::call::section_0::CallUni3 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CallUni3(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::call::section_0::CallUni4 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CallUni4(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::call::section_0::CallUni5 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CallUni5(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::call::section_0::CallUni6 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CallUni6(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::call::section_0::CallUni7 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CallUni7(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::call::section_0::CallUni8 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CallUni8(inst)),
                Err(_) => {}
            }
        }
        "clusterlaunchcontrol" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::clusterlaunchcontrol_query_cancel::section_0::ClusterlaunchcontrolQueryCancelIsCanceledPredB128 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ClusterlaunchcontrolQueryCancelIsCanceledPredB128(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::clusterlaunchcontrol_query_cancel::section_0::ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ClusterlaunchcontrolQueryCancelGetFirstCtaidV4B32B128(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::clusterlaunchcontrol_query_cancel::section_0::ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ClusterlaunchcontrolQueryCancelGetFirstCtaidDimensionB32B128(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::clusterlaunchcontrol_try_cancel::section_0::ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ClusterlaunchcontrolTryCancelAsyncSpaceCompletionMechanismMulticastClusterAllB128(inst)),
                Err(_) => {}
            }
        }
        "clz" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::clz::section_0::ClzType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ClzType(inst)),
                Err(_) => {}
            }
        }
        "cnot" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cnot::section_0::CnotType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CnotType(inst)),
                Err(_) => {}
            }
        }
        "copysign" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::copysign::section_0::CopysignType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CopysignType(inst)),
                Err(_) => {}
            }
        }
        "cos" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cos::section_0::CosApproxFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CosApproxFtzF32(inst)),
                Err(_) => {}
            }
        }
        "cp" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_bulk_commit_group::section_0::CpAsyncBulkCommitGroup as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncBulkCommitGroup(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_bulk_prefetch_tensor::section_0::CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncBulkPrefetchTensorDimL2SrcLoadModeLevelCacheHint(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_bulk_prefetch::section_0::CpAsyncBulkPrefetchL2SrcLevelCacheHint as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncBulkPrefetchL2SrcLevelCacheHint(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_bulk_tensor::section_0::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismCtaGroupLevelCacheHint(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_bulk_tensor::section_1::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismMulticastCtaGroupLevelCacheHint(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_bulk_tensor::section_2::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncBulkTensorDimDstSrcLoadModeCompletionMechanismLevelCacheHint(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_bulk::section_0::CpAsyncBulkDstSrcCompletionMechanismLevelCacheHint as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncBulkDstSrcCompletionMechanismLevelCacheHint(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_bulk::section_1::CpAsyncBulkDstSrcCompletionMechanismMulticastLevelCacheHint as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncBulkDstSrcCompletionMechanismMulticastLevelCacheHint(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_bulk::section_2::CpAsyncBulkDstSrcCompletionMechanism as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncBulkDstSrcCompletionMechanism(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_bulk::section_3::CpAsyncBulkDstSrcCompletionMechanismLevelCacheHintCpMask as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncBulkDstSrcCompletionMechanismLevelCacheHintCpMask(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_bulk_wait_group::section_0::CpAsyncBulkWaitGroupRead as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncBulkWaitGroupRead(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_commit_group::section_0::CpAsyncCommitGroup as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncCommitGroup(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_mbarrier_arrive::section_0::CpAsyncMbarrierArriveNoincStateB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncMbarrierArriveNoincStateB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async::section_0::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async::section_0::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async::section_0::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncCaStateGlobalLevelCacheHintLevelPrefetchSize1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async::section_0::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncCgStateGlobalLevelCacheHintLevelPrefetchSize1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_wait_group::section_0::CpAsyncWaitGroup as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncWaitGroup(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_async_wait_group::section_0::CpAsyncWaitAll as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpAsyncWaitAll(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_reduce_async_bulk_tensor::section_0::CpReduceAsyncBulkTensorDimDstSrcRedopLoadModeCompletionMechanismLevelCacheHint as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpReduceAsyncBulkTensorDimDstSrcRedopLoadModeCompletionMechanismLevelCacheHint(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_reduce_async_bulk::section_0::CpReduceAsyncBulkDstSrcCompletionMechanismRedopType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpReduceAsyncBulkDstSrcCompletionMechanismRedopType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_reduce_async_bulk::section_1::CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintRedopType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cp_reduce_async_bulk::section_2::CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CpReduceAsyncBulkDstSrcCompletionMechanismLevelCacheHintAddNoftzType(inst)),
                Err(_) => {}
            }
        }
        "createpolicy" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::createpolicy::section_0::CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CreatepolicyRangeGlobalLevelPrimaryPriorityLevelSecondaryPriorityB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::createpolicy::section_0::CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CreatepolicyFractionalLevelPrimaryPriorityLevelSecondaryPriorityB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::createpolicy::section_0::CreatepolicyCvtL2B64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CreatepolicyCvtL2B64(inst)),
                Err(_) => {}
            }
        }
        "cvt" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt_pack::section_0::CvtPackSatConverttypeAbtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtPackSatConverttypeAbtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt_pack::section_1::CvtPackSatConverttypeAbtypeCtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtPackSatConverttypeAbtypeCtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtIrndFtzSatDtypeAtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtIrndFtzSatDtypeAtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtFrndFtzSatDtypeAtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtFrndFtzSatDtypeAtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtFrnd2ReluSatfiniteF16F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtFrnd2ReluSatfiniteF16F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtFrnd2ReluSatfiniteF16x2F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtFrnd2ReluSatfiniteF16x2F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRsReluSatfiniteF16x2F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRsReluSatfiniteF16x2F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtFrnd2ReluSatfiniteBf16F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtFrnd2ReluSatfiniteBf16F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtFrnd2ReluSatfiniteBf16x2F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtFrnd2ReluSatfiniteBf16x2F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRsReluSatfiniteBf16x2F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRsReluSatfiniteBf16x2F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRnaSatfiniteTf32F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRnaSatfiniteTf32F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtFrnd2SatfiniteReluTf32F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtFrnd2SatfiniteReluTf32F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRnSatfiniteReluF8x2typeF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRnSatfiniteReluF8x2typeF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRnSatfiniteReluF8x2typeF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRnSatfiniteReluF8x2typeF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRnReluF16x2F8x2type as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRnReluF16x2F8x2type(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRsReluSatfiniteF8x4typeF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRsReluSatfiniteF8x4typeF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRnSatfiniteReluF4x2typeF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRnSatfiniteReluF4x2typeF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRnReluF16x2F4x2type as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRnReluF16x2F4x2type(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRsReluSatfiniteF4x4typeF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRsReluSatfiniteF4x4typeF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRnSatfiniteReluF6x2typeF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRnSatfiniteReluF6x2typeF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRnReluF16x2F6x2type as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRnReluF16x2F6x2type(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRsReluSatfiniteF6x4typeF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRsReluSatfiniteF6x4typeF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtFrnd3SatfiniteUe8m0x2F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtFrnd3SatfiniteUe8m0x2F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtFrnd3SatfiniteUe8m0x2Bf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtFrnd3SatfiniteUe8m0x2Bf16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvt::section_0::CvtRnBf16x2Ue8m0x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtRnBf16x2Ue8m0x2(inst)),
                Err(_) => {}
            }
        }
        "cvta" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvta::section_0::CvtaSpaceSize as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtaSpaceSize(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::cvta::section_0::CvtaToSpaceSize as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::CvtaToSpaceSize(inst)),
                Err(_) => {}
            }
        }
        "discard" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::discard::section_0::DiscardGlobalLevel as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::DiscardGlobalLevel(inst)),
                Err(_) => {}
            }
        }
        "div" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::div::section_0::DivType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::DivType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::div::section_0::DivApproxFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::DivApproxFtzF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::div::section_0::DivFullFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::DivFullFtzF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::div::section_0::DivRndFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::DivRndFtzF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::div::section_0::DivRndF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::DivRndF64(inst)),
                Err(_) => {}
            }
        }
        "dp2a" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::dp2a::section_0::Dp2aModeAtypeBtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Dp2aModeAtypeBtype(inst)),
                Err(_) => {}
            }
        }
        "dp4a" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::dp4a::section_0::Dp4aAtypeBtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Dp4aAtypeBtype(inst)),
                Err(_) => {}
            }
        }
        "elect" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::elect_sync::section_0::ElectSync as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ElectSync(inst)),
                Err(_) => {}
            }
        }
        "ex2" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ex2::section_0::Ex2ApproxFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Ex2ApproxFtzF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ex2::section_0::Ex2ApproxAtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Ex2ApproxAtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ex2::section_0::Ex2ApproxFtzBtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Ex2ApproxFtzBtype(inst)),
                Err(_) => {}
            }
        }
        "exit" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::exit::section_0::Exit as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Exit(inst)),
                Err(_) => {}
            }
        }
        "fence" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceSemScope as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceSemScope(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceAcquireSyncRestrictSharedClusterCluster as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceAcquireSyncRestrictSharedClusterCluster(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceReleaseSyncRestrictSharedCtaCluster as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceReleaseSyncRestrictSharedCtaCluster(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceOpRestrictReleaseCluster as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceOpRestrictReleaseCluster(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceProxyProxykind as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceProxyProxykind(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceProxyToProxykindFromProxykindReleaseScope as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceProxyToProxykindFromProxykindReleaseScope(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceProxyToProxykindFromProxykindAcquireScope as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceProxyToProxykindFromProxykindAcquireScope(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::MembarLevel as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MembarLevel(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::MembarProxyProxykind as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MembarProxyProxykind(inst)),
                Err(_) => {}
            }
        }
        "fma" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::fma::section_0::FmaRndFtzSatF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FmaRndFtzSatF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::fma::section_0::FmaRndFtzF32x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FmaRndFtzF32x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::fma::section_0::FmaRndF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FmaRndF64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::fma::section_1::FmaRndFtzSatF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FmaRndFtzSatF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::fma::section_1::FmaRndFtzSatF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FmaRndFtzSatF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::fma::section_1::FmaRndFtzReluF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FmaRndFtzReluF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::fma::section_1::FmaRndFtzReluF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FmaRndFtzReluF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::fma::section_1::FmaRndReluBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FmaRndReluBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::fma::section_1::FmaRndReluBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FmaRndReluBf16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::fma::section_1::FmaRndOobReluType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FmaRndOobReluType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::fma::section_2::FmaRndSatF32Abtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FmaRndSatF32Abtype(inst)),
                Err(_) => {}
            }
        }
        "fns" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::fns::section_0::FnsB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FnsB32(inst)),
                Err(_) => {}
            }
        }
        "getctarank" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::getctarank::section_0::GetctarankSpaceType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::GetctarankSpaceType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::getctarank::section_0::GetctarankSharedClusterType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::GetctarankSharedClusterType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::getctarank::section_0::GetctarankType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::GetctarankType(inst)),
                Err(_) => {}
            }
        }
        "griddepcontrol" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::griddepcontrol::section_0::GriddepcontrolAction as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::GriddepcontrolAction(inst)),
                Err(_) => {}
            }
        }
        "isspacep" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::isspacep::section_0::IsspacepSpace as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::IsspacepSpace(inst)),
                Err(_) => {}
            }
        }
        "istypep" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::istypep::section_0::IstypepType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::IstypepType(inst)),
                Err(_) => {}
            }
        }
        "ld" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ld_global_nc::section_0::LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdGlobalCopNcLevelCacheHintLevelPrefetchSizeType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ld_global_nc::section_0::LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdGlobalCopNcLevelCacheHintLevelPrefetchSizeVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ld_global_nc::section_0::LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ld_global_nc::section_0::LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdGlobalNcLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ld::section_0::LdWeakSsCopLevelCacheHintLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdWeakSsCopLevelCacheHintLevelPrefetchSizeVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ld::section_0::LdWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ld::section_0::LdVolatileSsLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdVolatileSsLevelPrefetchSizeVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ld::section_0::LdRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ld::section_0::LdAcquireScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdAcquireScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintLevelPrefetchSizeVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ld::section_0::LdMmioRelaxedSysGlobalType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdMmioRelaxedSysGlobalType(inst)),
                Err(_) => {}
            }
        }
        "ldmatrix" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ldmatrix::section_0::LdmatrixSyncAlignedShapeNumTransSsType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdmatrixSyncAlignedShapeNumTransSsType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ldmatrix::section_0::LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdmatrixSyncAlignedM8n16NumSsDstFmtSrcFmt(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ldmatrix::section_0::LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LdmatrixSyncAlignedM16n16NumTransSsDstFmtSrcFmt(inst)),
                Err(_) => {}
            }
        }
        "ldu" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ldu::section_0::LduSsType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LduSsType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ldu::section_0::LduSsVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::LduSsVecType(inst)),
                Err(_) => {}
            }
        }
        "lg2" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::lg2::section_0::Lg2ApproxFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Lg2ApproxFtzF32(inst)),
                Err(_) => {}
            }
        }
        "lop3" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::lop3::section_0::Lop3B32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Lop3B32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::lop3::section_0::Lop3BoolopB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Lop3BoolopB32(inst)),
                Err(_) => {}
            }
        }
        "mad" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mad_cc::section_0::MadHiloCcType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MadHiloCcType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mad::section_0::MadModeType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MadModeType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mad::section_0::MadHiSatS32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MadHiSatS32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mad::section_0::MadFtzSatF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MadFtzSatF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mad::section_0::MadRndFtzSatF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MadRndFtzSatF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mad::section_0::MadRndF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MadRndF64(inst)),
                Err(_) => {}
            }
        }
        "mad24" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mad24::section_0::Mad24ModeType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Mad24ModeType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mad24::section_0::Mad24HiSatS32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Mad24HiSatS32(inst)),
                Err(_) => {}
            }
        }
        "madc" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::madc::section_0::MadcHiloCcType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MadcHiloCcType(inst)),
                Err(_) => {}
            }
        }
        "mapa" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mapa::section_0::MapaSpaceType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MapaSpaceType(inst)),
                Err(_) => {}
            }
        }
        "match" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::match_sync::section_0::MatchAnySyncType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MatchAnySyncType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::match_sync::section_0::MatchAllSyncType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MatchAllSyncType(inst)),
                Err(_) => {}
            }
        }
        "max" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::max::section_0::MaxAtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MaxAtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::max::section_0::MaxReluBtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MaxReluBtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::max::section_0::MaxFtzNanXorsignAbsF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MaxFtzNanXorsignAbsF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::max::section_0::MaxFtzNanAbsF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MaxFtzNanAbsF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::max::section_0::MaxF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MaxF64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::max::section_0::MaxFtzNanXorsignAbsF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MaxFtzNanXorsignAbsF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::max::section_0::MaxFtzNanXorsignAbsF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MaxFtzNanXorsignAbsF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::max::section_0::MaxNanXorsignAbsBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MaxNanXorsignAbsBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::max::section_0::MaxNanXorsignAbsBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MaxNanXorsignAbsBf16x2(inst)),
                Err(_) => {}
            }
        }
        "mbarrier" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_arrive::section_0::MbarrierArriveSemScopeStateB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierArriveSemScopeStateB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_arrive::section_0::MbarrierArriveSemScopeSharedClusterB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierArriveSemScopeSharedClusterB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_arrive::section_0::MbarrierArriveExpectTxSemScopeStateB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierArriveExpectTxSemScopeStateB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_arrive::section_0::MbarrierArriveExpectTxSemScopeSharedClusterB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierArriveExpectTxSemScopeSharedClusterB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_arrive::section_0::MbarrierArriveNocompleteReleaseCtaStateB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierArriveNocompleteReleaseCtaStateB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_arrive_drop::section_0::MbarrierArriveDropSemScopeStateB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierArriveDropSemScopeStateB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_arrive_drop::section_0::MbarrierArriveDropSemScopeSharedClusterB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierArriveDropSemScopeSharedClusterB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_arrive_drop::section_0::MbarrierArriveDropExpectTxStateSemScopeB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierArriveDropExpectTxStateSemScopeB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_arrive_drop::section_0::MbarrierArriveDropExpectTxSharedClusterSemScopeB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierArriveDropExpectTxSharedClusterSemScopeB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_arrive_drop::section_0::MbarrierArriveDropNocompleteReleaseCtaStateB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierArriveDropNocompleteReleaseCtaStateB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_complete_tx::section_0::MbarrierCompleteTxSemScopeSpaceB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierCompleteTxSemScopeSpaceB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_expect_tx::section_0::MbarrierExpectTxSemScopeSpaceB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierExpectTxSemScopeSpaceB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_init::section_0::MbarrierInitStateB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierInitStateB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_inval::section_0::MbarrierInvalStateB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierInvalStateB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_pending_count::section_0::MbarrierPendingCountB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierPendingCountB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_test_wait::section_0::MbarrierTestWaitSemScopeStateB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierTestWaitSemScopeStateB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_test_wait::section_0::MbarrierTestWaitParitySemScopeStateB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierTestWaitParitySemScopeStateB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_test_wait::section_0::MbarrierTryWaitSemScopeStateB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierTryWaitSemScopeStateB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mbarrier_test_wait::section_0::MbarrierTryWaitParitySemScopeStateB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MbarrierTryWaitParitySemScopeStateB64(inst)),
                Err(_) => {}
            }
        }
        "membar" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceSemScope as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceSemScope(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceAcquireSyncRestrictSharedClusterCluster as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceAcquireSyncRestrictSharedClusterCluster(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceReleaseSyncRestrictSharedCtaCluster as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceReleaseSyncRestrictSharedCtaCluster(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceOpRestrictReleaseCluster as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceOpRestrictReleaseCluster(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceProxyProxykind as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceProxyProxykind(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceProxyToProxykindFromProxykindReleaseScope as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceProxyToProxykindFromProxykindReleaseScope(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceProxyToProxykindFromProxykindAcquireScope as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceProxyToProxykindFromProxykindAcquireScope(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceProxyAsyncGenericAcquireSyncRestrictSharedClusterCluster(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::FenceProxyAsyncGenericReleaseSyncRestrictSharedCtaCluster(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::MembarLevel as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MembarLevel(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::membar::section_0::MembarProxyProxykind as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MembarProxyProxykind(inst)),
                Err(_) => {}
            }
        }
        "min" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::min::section_0::MinAtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MinAtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::min::section_0::MinReluBtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MinReluBtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::min::section_0::MinFtzNanXorsignAbsF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MinFtzNanXorsignAbsF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::min::section_0::MinFtzNanAbsF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MinFtzNanAbsF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::min::section_0::MinF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MinF64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::min::section_0::MinFtzNanXorsignAbsF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MinFtzNanXorsignAbsF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::min::section_0::MinFtzNanXorsignAbsF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MinFtzNanXorsignAbsF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::min::section_0::MinNanXorsignAbsBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MinNanXorsignAbsBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::min::section_0::MinNanXorsignAbsBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MinNanXorsignAbsBf16x2(inst)),
                Err(_) => {}
            }
        }
        "mma" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_0::MmaSpvariantSyncAlignedM16n8k16RowColDtypeF16F16Ctype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpvariantSyncAlignedM16n8k16RowColDtypeF16F16Ctype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_0::MmaSpvariantSyncAlignedM16n8k32RowColDtypeF16F16Ctype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpvariantSyncAlignedM16n8k32RowColDtypeF16F16Ctype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k16RowColF32Bf16Bf16F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpvariantSyncAlignedM16n8k16RowColF32Bf16Bf16F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k32RowColF32Bf16Bf16F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpvariantSyncAlignedM16n8k32RowColF32Bf16Bf16F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k8RowColF32Tf32Tf32F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpvariantSyncAlignedM16n8k8RowColF32Tf32Tf32F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k16RowColF32Tf32Tf32F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpvariantSyncAlignedM16n8k16RowColF32Tf32Tf32F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_1::MmaSpvariantSyncAlignedM16n8k64RowColF32F8typeF8typeF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpvariantSyncAlignedM16n8k64RowColF32F8typeF8typeF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_1::MmaSpOrderedMetadataSyncAlignedM16n8k64RowColKindDtypeF8f6f4typeF8f6f4typeCtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpOrderedMetadataSyncAlignedM16n8k64RowColKindDtypeF8f6f4typeF8f6f4typeCtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_2::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_3::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpvariantSyncAlignedM16n8k128RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_4::MmaSpvariantSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpvariantSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_5::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma_sp::section_6::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSpvariantSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_0::MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedM8n8k4AlayoutBlayoutDtypeF16F16Ctype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_0::MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedM16n8k8RowColDtypeF16F16Ctype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_0::MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedM16n8k16RowColDtypeF16F16Ctype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_1::MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedM16n8k4RowColF32Tf32Tf32F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_1::MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedM16n8k8RowColF32AtypeBtypeF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_1::MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedM16n8k16RowColF32Bf16Bf16F32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_1::MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedShapeRowColDtypeF8typeF8typeCtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_1::MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedM16n8k32RowColKindDtypeF8f6f4typeF8f6f4typeCtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_2::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_3::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedM16n8k64RowColKindBlockScaleScaleVecSizeF32E2m1E2m1F32Stype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_4::MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedM16n8k32RowColKindBlockScaleScaleVecSizeF32F8f6f4typeF8f6f4typeF32Stype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_5::MmaSyncAlignedShapeRowColF64F64F64F64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedShapeRowColF64F64F64F64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_6::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_7::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedShapeRowColSatfiniteS32AtypeBtypeS321(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mma::section_8::MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MmaSyncAlignedShapeRowColS32B1B1S32BitopPopc(inst)),
                Err(_) => {}
            }
        }
        "mov" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mov::section_0::MovType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MovType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mov::section_0::MovU32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MovU32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mov::section_0::MovU64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MovU64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mov::section_0::MovU321 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MovU321(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mov::section_0::MovU641 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MovU641(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mov::section_1::MovType1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MovType1(inst)),
                Err(_) => {}
            }
        }
        "movmatrix" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::movmatrix::section_0::MovmatrixSyncAlignedShapeTransType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MovmatrixSyncAlignedShapeTransType(inst)),
                Err(_) => {}
            }
        }
        "mul" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mul::section_0::MulModeType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MulModeType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mul::section_1::MulRndFtzSatF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MulRndFtzSatF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mul::section_1::MulRndFtzF32x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MulRndFtzF32x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mul::section_1::MulRndF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MulRndF64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mul::section_2::MulRndFtzSatF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MulRndFtzSatF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mul::section_2::MulRndFtzSatF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MulRndFtzSatF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mul::section_2::MulRndBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MulRndBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mul::section_2::MulRndBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MulRndBf16x2(inst)),
                Err(_) => {}
            }
        }
        "mul24" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::mul24::section_0::Mul24ModeType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Mul24ModeType(inst)),
                Err(_) => {}
            }
        }
        "multimem" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::multimem_ld_reduce::section_0::MultimemLdReduceLdsemScopeSsOpType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MultimemLdReduceLdsemScopeSsOpType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::multimem_ld_reduce::section_0::MultimemLdReduceWeakSsOpType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MultimemLdReduceWeakSsOpType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::multimem_ld_reduce::section_0::MultimemStStsemScopeSsType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MultimemStStsemScopeSsType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::multimem_ld_reduce::section_0::MultimemStWeakSsType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MultimemStWeakSsType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::multimem_ld_reduce::section_0::MultimemRedRedsemScopeSsOpType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MultimemRedRedsemScopeSsOpType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::multimem_ld_reduce::section_1::MultimemLdReduceLdsemScopeSsOpAccPrecVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MultimemLdReduceLdsemScopeSsOpAccPrecVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::multimem_ld_reduce::section_1::MultimemLdReduceWeakSsOpAccPrecVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MultimemLdReduceWeakSsOpAccPrecVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::multimem_ld_reduce::section_1::MultimemStStsemScopeSsVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MultimemStStsemScopeSsVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::multimem_ld_reduce::section_1::MultimemStWeakSsVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MultimemStWeakSsVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::multimem_ld_reduce::section_1::MultimemRedRedsemScopeSsRedopVecRedtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::MultimemRedRedsemScopeSsRedopVecRedtype(inst)),
                Err(_) => {}
            }
        }
        "nanosleep" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::nanosleep::section_0::NanosleepU32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::NanosleepU32(inst)),
                Err(_) => {}
            }
        }
        "neg" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::neg::section_0::NegType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::NegType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::neg::section_0::NegFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::NegFtzF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::neg::section_0::NegF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::NegF64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::neg::section_0::NegFtzF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::NegFtzF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::neg::section_0::NegFtzF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::NegFtzF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::neg::section_0::NegBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::NegBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::neg::section_0::NegBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::NegBf16x2(inst)),
                Err(_) => {}
            }
        }
        "not" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::not::section_0::NotType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::NotType(inst)),
                Err(_) => {}
            }
        }
        "or" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::or::section_0::OrType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::OrType(inst)),
                Err(_) => {}
            }
        }
        "pmevent" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::pmevent::section_0::Pmevent as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Pmevent(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::pmevent::section_0::PmeventMask as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::PmeventMask(inst)),
                Err(_) => {}
            }
        }
        "popc" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::popc::section_0::PopcType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::PopcType(inst)),
                Err(_) => {}
            }
        }
        "prefetch" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::prefetch::section_0::PrefetchSpaceLevel as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::PrefetchSpaceLevel(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::prefetch::section_0::PrefetchGlobalLevelEvictionPriority as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::PrefetchGlobalLevelEvictionPriority(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::prefetch::section_0::PrefetchuL1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::PrefetchuL1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::prefetch::section_0::PrefetchTensormapSpaceTensormap as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::PrefetchTensormapSpaceTensormap(inst)),
                Err(_) => {}
            }
        }
        "prefetchu" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::prefetch::section_0::PrefetchSpaceLevel as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::PrefetchSpaceLevel(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::prefetch::section_0::PrefetchGlobalLevelEvictionPriority as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::PrefetchGlobalLevelEvictionPriority(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::prefetch::section_0::PrefetchuL1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::PrefetchuL1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::prefetch::section_0::PrefetchTensormapSpaceTensormap as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::PrefetchTensormapSpaceTensormap(inst)),
                Err(_) => {}
            }
        }
        "prmt" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::prmt::section_0::PrmtB32Mode as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::PrmtB32Mode(inst)),
                Err(_) => {}
            }
        }
        "rcp" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::rcp_approx_ftz_f64::section_0::RcpApproxFtzF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RcpApproxFtzF64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::rcp::section_0::RcpApproxFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RcpApproxFtzF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::rcp::section_0::RcpRndFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RcpRndFtzF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::rcp::section_0::RcpRndF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RcpRndF64(inst)),
                Err(_) => {}
            }
        }
        "red" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red_async::section_0::RedAsyncSemScopeSsCompletionMechanismOpType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedAsyncSemScopeSsCompletionMechanismOpType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red_async::section_1::RedAsyncSemScopeSsCompletionMechanismOpType1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedAsyncSemScopeSsCompletionMechanismOpType1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red_async::section_2::RedAsyncSemScopeSsCompletionMechanismOpType2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedAsyncSemScopeSsCompletionMechanismOpType2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red_async::section_3::RedAsyncSemScopeSsCompletionMechanismAddType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedAsyncSemScopeSsCompletionMechanismAddType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red_async::section_4::RedAsyncMmioSemScopeSsAddType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedAsyncMmioSemScopeSsAddType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red::section_0::RedOpSpaceSemScopeLevelCacheHintType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedOpSpaceSemScopeLevelCacheHintType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red::section_0::RedAddSpaceSemScopeNoftzLevelCacheHintF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedAddSpaceSemScopeNoftzLevelCacheHintF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red::section_0::RedAddSpaceSemScopeNoftzLevelCacheHintF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedAddSpaceSemScopeNoftzLevelCacheHintF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red::section_0::RedAddSpaceSemScopeNoftzLevelCacheHintBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedAddSpaceSemScopeNoftzLevelCacheHintBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red::section_0::RedAddSpaceSemScopeNoftzLevelCacheHintBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedAddSpaceSemScopeNoftzLevelCacheHintBf16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red::section_1::RedAddSpaceSemScopeLevelCacheHintVec32BitF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedAddSpaceSemScopeLevelCacheHintVec32BitF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red::section_1::RedOpSpaceSemScopeNoftzLevelCacheHintVec16BitHalfWordType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedOpSpaceSemScopeNoftzLevelCacheHintVec16BitHalfWordType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::red::section_1::RedOpSpaceSemScopeNoftzLevelCacheHintVec32BitPackedType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RedOpSpaceSemScopeNoftzLevelCacheHintVec32BitPackedType(inst)),
                Err(_) => {}
            }
        }
        "redux" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::redux_sync::section_0::ReduxSyncOpType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ReduxSyncOpType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::redux_sync::section_1::ReduxSyncOpB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ReduxSyncOpB32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::redux_sync::section_2::ReduxSyncOpAbsNanF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ReduxSyncOpAbsNanF32(inst)),
                Err(_) => {}
            }
        }
        "rem" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::rem::section_0::RemType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RemType(inst)),
                Err(_) => {}
            }
        }
        "ret" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::ret::section_0::RetUni as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RetUni(inst)),
                Err(_) => {}
            }
        }
        "rsqrt" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::rsqrt_approx_ftz_f64::section_0::RsqrtApproxFtzF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RsqrtApproxFtzF64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::rsqrt::section_0::RsqrtApproxFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RsqrtApproxFtzF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::rsqrt::section_0::RsqrtApproxF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::RsqrtApproxF64(inst)),
                Err(_) => {}
            }
        }
        "sad" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sad::section_0::SadType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SadType(inst)),
                Err(_) => {}
            }
        }
        "selp" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::selp::section_0::SelpType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SelpType(inst)),
                Err(_) => {}
            }
        }
        "set" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_0::SetCmpopFtzDtypeStype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopFtzDtypeStype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_0::SetCmpopBoolopFtzDtypeStype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopBoolopFtzDtypeStype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_1::SetCmpopFtzF16Stype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopFtzF16Stype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_1::SetCmpopBoolopFtzF16Stype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopBoolopFtzF16Stype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_1::SetCmpopBf16Stype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopBf16Stype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_1::SetCmpopBoolopBf16Stype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopBoolopBf16Stype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_1::SetCmpopFtzDtypeF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopFtzDtypeF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_1::SetCmpopBoolopFtzDtypeF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopBoolopFtzDtypeF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_2::SetCmpopDtypeBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopDtypeBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_2::SetCmpopBoolopDtypeBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopBoolopDtypeBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_3::SetCmpopFtzDtypeF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopFtzDtypeF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_3::SetCmpopBoolopFtzDtypeF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopBoolopFtzDtypeF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_4::SetCmpopDtypeBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopDtypeBf16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::set::section_4::SetCmpopBoolopDtypeBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetCmpopBoolopDtypeBf16x2(inst)),
                Err(_) => {}
            }
        }
        "setmaxnreg" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::setmaxnreg::section_0::SetmaxnregActionSyncAlignedU32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetmaxnregActionSyncAlignedU32(inst)),
                Err(_) => {}
            }
        }
        "setp" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::setp::section_0::SetpCmpopFtzType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetpCmpopFtzType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::setp::section_0::SetpCmpopBoolopFtzType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetpCmpopBoolopFtzType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::setp::section_1::SetpCmpopFtzF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetpCmpopFtzF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::setp::section_1::SetpCmpopBoolopFtzF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetpCmpopBoolopFtzF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::setp::section_1::SetpCmpopFtzF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetpCmpopFtzF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::setp::section_1::SetpCmpopBoolopFtzF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetpCmpopBoolopFtzF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::setp::section_1::SetpCmpopBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetpCmpopBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::setp::section_1::SetpCmpopBoolopBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetpCmpopBoolopBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::setp::section_1::SetpCmpopBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetpCmpopBf16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::setp::section_1::SetpCmpopBoolopBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SetpCmpopBoolopBf16x2(inst)),
                Err(_) => {}
            }
        }
        "shf" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::shf::section_0::ShfLModeB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ShfLModeB32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::shf::section_0::ShfRModeB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ShfRModeB32(inst)),
                Err(_) => {}
            }
        }
        "shfl" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::shfl_sync::section_0::ShflSyncModeB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ShflSyncModeB32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::shfl::section_0::ShflModeB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ShflModeB32(inst)),
                Err(_) => {}
            }
        }
        "shl" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::shl::section_0::ShlType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ShlType(inst)),
                Err(_) => {}
            }
        }
        "shr" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::shr::section_0::ShrType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::ShrType(inst)),
                Err(_) => {}
            }
        }
        "sin" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sin::section_0::SinApproxFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SinApproxFtzF32(inst)),
                Err(_) => {}
            }
        }
        "slct" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::slct::section_0::SlctDtypeS32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SlctDtypeS32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::slct::section_0::SlctFtzDtypeF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SlctFtzDtypeF32(inst)),
                Err(_) => {}
            }
        }
        "sqrt" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sqrt::section_0::SqrtApproxFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SqrtApproxFtzF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sqrt::section_0::SqrtRndFtzF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SqrtRndFtzF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sqrt::section_0::SqrtRndF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SqrtRndF64(inst)),
                Err(_) => {}
            }
        }
        "st" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::st_async::section_0::StAsyncSemScopeSsCompletionMechanismVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::StAsyncSemScopeSsCompletionMechanismVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::st_async::section_1::StAsyncMmioSemScopeSsType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::StAsyncMmioSemScopeSsType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::st_bulk::section_0::StBulkWeakSharedCta as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::StBulkWeakSharedCta(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::st::section_0::StWeakSsCopLevelCacheHintVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::StWeakSsCopLevelCacheHintVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::st::section_0::StWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::StWeakSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::st::section_0::StVolatileSsVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::StVolatileSsVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::st::section_0::StRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::StRelaxedScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::st::section_0::StReleaseScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::StReleaseScopeSsLevel1EvictionPriorityLevel2EvictionPriorityLevelCacheHintVecType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::st::section_0::StMmioRelaxedSysGlobalType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::StMmioRelaxedSysGlobalType(inst)),
                Err(_) => {}
            }
        }
        "stackrestore" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::stackrestore::section_0::StackrestoreType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::StackrestoreType(inst)),
                Err(_) => {}
            }
        }
        "stacksave" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::stacksave::section_0::StacksaveType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::StacksaveType(inst)),
                Err(_) => {}
            }
        }
        "stmatrix" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::stmatrix::section_0::StmatrixSyncAlignedShapeNumTransSsType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::StmatrixSyncAlignedShapeNumTransSsType(inst)),
                Err(_) => {}
            }
        }
        "sub" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sub_cc::section_0::SubCcType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SubCcType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sub::section_0::SubType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SubType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sub::section_0::SubSatS32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SubSatS32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sub::section_1::SubRndFtzSatF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SubRndFtzSatF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sub::section_1::SubRndFtzF32x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SubRndFtzF32x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sub::section_1::SubRndF64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SubRndF64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sub::section_2::SubRndFtzSatF16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SubRndFtzSatF16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sub::section_2::SubRndFtzSatF16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SubRndFtzSatF16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sub::section_2::SubRndBf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SubRndBf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sub::section_2::SubRndBf16x2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SubRndBf16x2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sub::section_3::SubRndSatF32Atype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SubRndSatF32Atype(inst)),
                Err(_) => {}
            }
        }
        "subc" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::subc::section_0::SubcCcType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SubcCcType(inst)),
                Err(_) => {}
            }
        }
        "suld" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::suld::section_0::SuldBGeomCopVecDtypeMode as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SuldBGeomCopVecDtypeMode(inst)),
                Err(_) => {}
            }
        }
        "suq" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::suq::section_0::SuqQueryB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SuqQueryB32(inst)),
                Err(_) => {}
            }
        }
        "sured" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sured::section_0::SuredBOpGeomCtypeMode as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SuredBOpGeomCtypeMode(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sured::section_1::SuredPOpGeomCtypeMode as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SuredPOpGeomCtypeMode(inst)),
                Err(_) => {}
            }
        }
        "sust" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sust::section_0::SustBDimCopVecCtypeMode as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SustBDimCopVecCtypeMode(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sust::section_0::SustPDimVecB32Mode as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SustPDimVecB32Mode(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::sust::section_0::SustBAdimCopVecCtypeMode as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SustBAdimCopVecCtypeMode(inst)),
                Err(_) => {}
            }
        }
        "szext" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::szext::section_0::SzextModeType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::SzextModeType(inst)),
                Err(_) => {}
            }
        }
        "tanh" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tanh::section_0::TanhApproxType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TanhApproxType(inst)),
                Err(_) => {}
            }
        }
        "tcgen05" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_alloc::section_0::Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05AllocCtaGroupSyncAlignedSharedCtaB32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_alloc::section_0::Tcgen05DeallocCtaGroupSyncAlignedB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05DeallocCtaGroupSyncAlignedB32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_alloc::section_0::Tcgen05RelinquishAllocPermitCtaGroupSyncAligned as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05RelinquishAllocPermitCtaGroupSyncAligned(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_commit::section_0::Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05CommitCtaGroupCompletionMechanismSharedClusterMulticastB64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_cp::section_0::Tcgen05CpCtaGroupShapeMulticastDstSrcFmt as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05CpCtaGroupShapeMulticastDstSrcFmt(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_fence::section_0::Tcgen05FenceBeforeThreadSync as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05FenceBeforeThreadSync(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_fence::section_0::Tcgen05FenceAfterThreadSync as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05FenceAfterThreadSync(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_ld::section_0::Tcgen05LdSyncAlignedShape1NumPackB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05LdSyncAlignedShape1NumPackB32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_ld::section_0::Tcgen05LdSyncAlignedShape2NumPackB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05LdSyncAlignedShape2NumPackB32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_ld::section_0::Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05LdRedSyncAlignedShape3NumRedopAbsNanF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_ld::section_0::Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05LdRedSyncAlignedShape4NumRedopAbsNanF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_ld::section_0::Tcgen05LdRedSyncAlignedShape3NumRedopType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05LdRedSyncAlignedShape3NumRedopType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_ld::section_0::Tcgen05LdRedSyncAlignedShape4NumRedopType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05LdRedSyncAlignedShape4NumRedopType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_0::Tcgen05MmaSpCtaGroupKind as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKind(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_0::Tcgen05MmaSpCtaGroupKind1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKind1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_1::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_1::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsize1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_2::Tcgen05MmaSpCtaGroupKindCollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKindCollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_2::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_2::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKindAshiftCollectorUsage1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_3::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_3::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_4::Tcgen05MmaSpCtaGroupKindI8 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKindI8(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_4::Tcgen05MmaSpCtaGroupKindI81 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKindI81(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_5::Tcgen05MmaSpCtaGroupKindI8CollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKindI8CollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_5::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_sp::section_5::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaSpCtaGroupKindI8AshiftCollectorUsage1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_0::Tcgen05MmaCtaGroupKind as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKind(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_0::Tcgen05MmaCtaGroupKind1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKind1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_1::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_1::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsize1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_2::Tcgen05MmaCtaGroupKindCollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKindCollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_2::Tcgen05MmaCtaGroupKindAshiftCollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKindAshiftCollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_2::Tcgen05MmaCtaGroupKindAshiftCollectorUsage1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKindAshiftCollectorUsage1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_3::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_3::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKindBlockScaleScaleVectorsizeCollectorUsage1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_4::Tcgen05MmaCtaGroupKindI8 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKindI8(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_4::Tcgen05MmaCtaGroupKindI81 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKindI81(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_5::Tcgen05MmaCtaGroupKindI8CollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKindI8CollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_5::Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma::section_5::Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaCtaGroupKindI8AshiftCollectorUsage1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_ws_sp::section_0::Tcgen05MmaWsSpCtaGroup1KindCollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaWsSpCtaGroup1KindCollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_ws_sp::section_0::Tcgen05MmaWsSpCtaGroup1KindCollectorUsage1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaWsSpCtaGroup1KindCollectorUsage1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_ws_sp::section_1::Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_ws_sp::section_1::Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaWsSpCtaGroup1KindI8CollectorUsage1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_ws::section_0::Tcgen05MmaWsCtaGroup1KindCollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaWsCtaGroup1KindCollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_ws::section_0::Tcgen05MmaWsCtaGroup1KindCollectorUsage1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaWsCtaGroup1KindCollectorUsage1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_ws::section_1::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_mma_ws::section_1::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05MmaWsCtaGroup1KindI8CollectorUsage1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_shift::section_0::Tcgen05ShiftCtaGroupDown as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05ShiftCtaGroupDown(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_st::section_0::Tcgen05StSyncAlignedShape1NumUnpackB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05StSyncAlignedShape1NumUnpackB32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_st::section_0::Tcgen05StSyncAlignedShape2NumUnpackB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05StSyncAlignedShape2NumUnpackB32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tcgen05_wait::section_0::Tcgen05WaitOperationSyncAligned as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tcgen05WaitOperationSyncAligned(inst)),
                Err(_) => {}
            }
        }
        "tensormap" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tensormap_cp_fenceproxy::section_0::TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TensormapCpFenceproxyCpQualifiersFenceQualifiersSyncAligned(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tensormap_replace::section_0::TensormapReplaceModeField1SsB1024Type as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TensormapReplaceModeField1SsB1024Type(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tensormap_replace::section_0::TensormapReplaceModeField2SsB1024Type as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TensormapReplaceModeField2SsB1024Type(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tensormap_replace::section_0::TensormapReplaceModeField3SsB1024Type as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TensormapReplaceModeField3SsB1024Type(inst)),
                Err(_) => {}
            }
        }
        "testp" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::testp::section_0::TestpOpType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TestpOpType(inst)),
                Err(_) => {}
            }
        }
        "tex" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tex::section_0::TexGeomV4DtypeCtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TexGeomV4DtypeCtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tex::section_0::TexGeomV4DtypeCtype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TexGeomV4DtypeCtype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tex::section_0::TexGeomV2F16x2Ctype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TexGeomV2F16x2Ctype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tex::section_0::TexGeomV2F16x2Ctype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TexGeomV2F16x2Ctype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tex::section_0::TexBaseGeomV4DtypeCtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TexBaseGeomV4DtypeCtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tex::section_0::TexLevelGeomV4DtypeCtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TexLevelGeomV4DtypeCtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tex::section_0::TexGradGeomV4DtypeCtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TexGradGeomV4DtypeCtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tex::section_0::TexBaseGeomV2F16x2Ctype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TexBaseGeomV2F16x2Ctype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tex::section_0::TexLevelGeomV2F16x2Ctype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TexLevelGeomV2F16x2Ctype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tex::section_0::TexGradGeomV2F16x2Ctype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TexGradGeomV2F16x2Ctype(inst)),
                Err(_) => {}
            }
        }
        "tld4" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tld4::section_0::Tld4Comp2dV4DtypeF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tld4Comp2dV4DtypeF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::tld4::section_0::Tld4CompGeomV4DtypeF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Tld4CompGeomV4DtypeF32(inst)),
                Err(_) => {}
            }
        }
        "trap" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::trap::section_0::Trap as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Trap(inst)),
                Err(_) => {}
            }
        }
        "txq" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::txq::section_0::TxqTqueryB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TxqTqueryB32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::txq::section_0::TxqLevelTlqueryB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TxqLevelTlqueryB32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::txq::section_0::TxqSqueryB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::TxqSqueryB32(inst)),
                Err(_) => {}
            }
        }
        "vabsdiff" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
        }
        "vabsdiff2" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vadd2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vsub2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vavrg2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vabsdiff2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmin2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmax2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vadd2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vsub2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vavrg2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vabsdiff2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmin2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmax2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
        }
        "vabsdiff4" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vadd4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vsub4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vavrg4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vabsdiff4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmin4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmax4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vadd4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vsub4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vavrg4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vabsdiff4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmin4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmax4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
        }
        "vadd" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
        }
        "vadd2" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vadd2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vsub2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vavrg2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vabsdiff2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmin2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmax2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vadd2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vsub2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vavrg2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vabsdiff2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmin2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmax2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
        }
        "vadd4" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vadd4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vsub4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vavrg4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vabsdiff4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmin4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmax4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vadd4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vsub4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vavrg4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vabsdiff4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmin4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmax4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
        }
        "vavrg2" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vadd2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vsub2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vavrg2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vabsdiff2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmin2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmax2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vadd2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vsub2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vavrg2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vabsdiff2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmin2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmax2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
        }
        "vavrg4" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vadd4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vsub4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vavrg4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vabsdiff4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmin4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmax4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vadd4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vsub4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vavrg4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vabsdiff4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmin4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmax4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
        }
        "vmad" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vmad::section_0::VmadDtypeAtypeBtypeSatScale as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmadDtypeAtypeBtypeSatScale(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vmad::section_0::VmadDtypeAtypeBtypePoSatScale as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmadDtypeAtypeBtypePoSatScale(inst)),
                Err(_) => {}
            }
        }
        "vmax" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
        }
        "vmax2" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vadd2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vsub2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vavrg2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vabsdiff2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmin2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmax2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vadd2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vsub2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vavrg2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vabsdiff2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmin2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmax2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
        }
        "vmax4" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vadd4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vsub4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vavrg4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vabsdiff4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmin4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmax4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vadd4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vsub4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vavrg4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vabsdiff4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmin4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmax4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
        }
        "vmin" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
        }
        "vmin2" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vadd2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vsub2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vavrg2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vabsdiff2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmin2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmax2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vadd2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vsub2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vavrg2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vabsdiff2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmin2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmax2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
        }
        "vmin4" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vadd4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vsub4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vavrg4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vabsdiff4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmin4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmax4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vadd4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vsub4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vavrg4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vabsdiff4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmin4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmax4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
        }
        "vote" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vote_sync::section_0::VoteSyncModePred as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VoteSyncModePred(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vote_sync::section_0::VoteSyncBallotB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VoteSyncBallotB32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vote::section_0::VoteModePred as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VoteModePred(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vote::section_0::VoteBallotB32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VoteBallotB32(inst)),
                Err(_) => {}
            }
        }
        "vset" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vset::section_0::VsetAtypeBtypeCmp as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsetAtypeBtypeCmp(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vset::section_0::VsetAtypeBtypeCmpOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsetAtypeBtypeCmpOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vset::section_0::VsetAtypeBtypeCmp1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsetAtypeBtypeCmp1(inst)),
                Err(_) => {}
            }
        }
        "vset2" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vset2::section_0::Vset2AtypeBtypeCmp as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vset2AtypeBtypeCmp(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vset2::section_0::Vset2AtypeBtypeCmpAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vset2AtypeBtypeCmpAdd(inst)),
                Err(_) => {}
            }
        }
        "vset4" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vset4::section_0::Vset4AtypeBtypeCmp as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vset4AtypeBtypeCmp(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vset4::section_0::Vset4AtypeBtypeCmpAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vset4AtypeBtypeCmpAdd(inst)),
                Err(_) => {}
            }
        }
        "vshl" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vsh::section_0::VshlDtypeAtypeU32SatMode as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VshlDtypeAtypeU32SatMode(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vsh::section_0::VshrDtypeAtypeU32SatMode as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VshrDtypeAtypeU32SatMode(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vsh::section_0::VshlDtypeAtypeU32SatModeOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VshlDtypeAtypeU32SatModeOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vsh::section_0::VshrDtypeAtypeU32SatModeOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VshrDtypeAtypeU32SatModeOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vsh::section_0::VshlDtypeAtypeU32SatMode1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VshlDtypeAtypeU32SatMode1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vsh::section_0::VshrDtypeAtypeU32SatMode1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VshrDtypeAtypeU32SatMode1(inst)),
                Err(_) => {}
            }
        }
        "vshr" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vsh::section_0::VshlDtypeAtypeU32SatMode as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VshlDtypeAtypeU32SatMode(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vsh::section_0::VshrDtypeAtypeU32SatMode as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VshrDtypeAtypeU32SatMode(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vsh::section_0::VshlDtypeAtypeU32SatModeOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VshlDtypeAtypeU32SatModeOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vsh::section_0::VshrDtypeAtypeU32SatModeOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VshrDtypeAtypeU32SatModeOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vsh::section_0::VshlDtypeAtypeU32SatMode1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VshlDtypeAtypeU32SatMode1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vsh::section_0::VshrDtypeAtypeU32SatMode1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VshrDtypeAtypeU32SatMode1(inst)),
                Err(_) => {}
            }
        }
        "vsub" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSatOp2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSatOp2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VaddDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VaddDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VsubDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VsubDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VabsdiffDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VabsdiffDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VminDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VminDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop::section_0::VmaxDtypeAtypeBtypeSat1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::VmaxDtypeAtypeBtypeSat1(inst)),
                Err(_) => {}
            }
        }
        "vsub2" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vadd2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vsub2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vavrg2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vabsdiff2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmin2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmax2DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax2DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vadd2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vsub2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vavrg2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vabsdiff2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmin2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop2::section_0::Vmax2DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax2DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
        }
        "vsub4" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vadd4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vsub4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vavrg4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vabsdiff4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmin4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmax4DtypeAtypeBtypeSat as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax4DtypeAtypeBtypeSat(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vadd4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vadd4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vsub4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vsub4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vavrg4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vavrg4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vabsdiff4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vabsdiff4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmin4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmin4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::vop4::section_0::Vmax4DtypeAtypeBtypeAdd as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::Vmax4DtypeAtypeBtypeAdd(inst)),
                Err(_) => {}
            }
        }
        "wgmma" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_commit_group::section_0::WgmmaCommitGroupSyncAligned as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaCommitGroupSyncAligned(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_fence::section_0::WgmmaFenceSyncAligned as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaFenceSyncAligned(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async_sp::section_0::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async_sp::section_0::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F161 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeF16F161(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async_sp::section_1::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async_sp::section_1::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf161 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeBf16Bf161(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async_sp::section_2::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async_sp::section_2::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf321 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeTf32Tf321(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async_sp::section_3::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async_sp::section_3::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSpSyncAlignedShapeDtypeAtypeBtype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async_sp::section_4::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async_sp::section_4::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSpSyncAlignedShapeSatfiniteS32AtypeBtype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async::section_0::WgmmaMmaAsyncSyncAlignedShapeDtypeF16F16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeF16F16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async::section_0::WgmmaMmaAsyncSyncAlignedShapeDtypeF16F161 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeF16F161(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async::section_1::WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf16 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf16(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async::section_1::WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf161 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeBf16Bf161(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async::section_2::WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async::section_2::WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf321 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeTf32Tf321(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async::section_3::WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async::section_3::WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSyncAlignedShapeDtypeAtypeBtype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async::section_4::WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async::section_4::WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSyncAlignedShapeSatfiniteS32AtypeBtype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async::section_5::WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_mma_async::section_5::WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaMmaAsyncSyncAlignedShapeS32B1B1OpPopc1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wgmma_wait_group::section_0::WgmmaWaitGroupSyncAligned as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WgmmaWaitGroupSyncAligned(inst)),
                Err(_) => {}
            }
        }
        "wmma" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_0::WmmaLoadASyncAlignedLayoutShapeSsAtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadASyncAlignedLayoutShapeSsAtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_0::WmmaLoadBSyncAlignedLayoutShapeSsBtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadBSyncAlignedLayoutShapeSsBtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_0::WmmaLoadCSyncAlignedLayoutShapeSsCtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadCSyncAlignedLayoutShapeSsCtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_1::WmmaLoadASyncAlignedLayoutShapeSsAtype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadASyncAlignedLayoutShapeSsAtype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_1::WmmaLoadBSyncAlignedLayoutShapeSsBtype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadBSyncAlignedLayoutShapeSsBtype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_1::WmmaLoadCSyncAlignedLayoutShapeSsCtype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadCSyncAlignedLayoutShapeSsCtype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_2::WmmaLoadASyncAlignedLayoutShapeSsAtype2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadASyncAlignedLayoutShapeSsAtype2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_2::WmmaLoadBSyncAlignedLayoutShapeSsBtype2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadBSyncAlignedLayoutShapeSsBtype2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_2::WmmaLoadCSyncAlignedLayoutShapeSsCtype2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadCSyncAlignedLayoutShapeSsCtype2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_3::WmmaLoadASyncAlignedLayoutShapeSsAtype3 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadASyncAlignedLayoutShapeSsAtype3(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_3::WmmaLoadBSyncAlignedLayoutShapeSsBtype3 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadBSyncAlignedLayoutShapeSsBtype3(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_3::WmmaLoadCSyncAlignedLayoutShapeSsCtype3 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadCSyncAlignedLayoutShapeSsCtype3(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_4::WmmaLoadASyncAlignedRowShapeSsAtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadASyncAlignedRowShapeSsAtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_4::WmmaLoadBSyncAlignedColShapeSsBtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadBSyncAlignedColShapeSsBtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_4::WmmaLoadCSyncAlignedLayoutShapeSsCtype4 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadCSyncAlignedLayoutShapeSsCtype4(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_5::WmmaLoadASyncAlignedRowShapeSsAtype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadASyncAlignedRowShapeSsAtype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_5::WmmaLoadBSyncAlignedColShapeSsBtype1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadBSyncAlignedColShapeSsBtype1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_load::section_5::WmmaLoadCSyncAlignedLayoutShapeSsCtype5 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaLoadCSyncAlignedLayoutShapeSsCtype5(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_mma::section_0::WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaMmaSyncAlignedAlayoutBlayoutShapeDtypeCtype(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_mma::section_1::WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaMmaSyncAlignedAlayoutBlayoutShapeS32AtypeBtypeS32Satfinite(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_mma::section_2::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_mma::section_3::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaMmaSyncAlignedAlayoutBlayoutShapeF32AtypeBtypeF321(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_mma::section_4::WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaMmaSyncAlignedAlayoutBlayoutShapeRndF64F64F64F64(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_mma::section_5::WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaMmaSyncAlignedRowColShapeS32AtypeBtypeS32Satfinite(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_mma::section_6::WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaMmaOpPopcSyncAlignedRowColShapeS32AtypeBtypeS32(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_store::section_0::WmmaStoreDSyncAlignedLayoutShapeSsType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaStoreDSyncAlignedLayoutShapeSsType(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_store::section_1::WmmaStoreDSyncAlignedLayoutShapeSsType1 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaStoreDSyncAlignedLayoutShapeSsType1(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_store::section_2::WmmaStoreDSyncAlignedLayoutShapeSsType2 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaStoreDSyncAlignedLayoutShapeSsType2(inst)),
                Err(_) => {}
            }
            stream.set_position(start_pos);
            match <crate::r#type::instruction::wmma_store::section_3::WmmaStoreDSyncAlignedLayoutShapeSsType3 as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::WmmaStoreDSyncAlignedLayoutShapeSsType3(inst)),
                Err(_) => {}
            }
        }
        "xor" => {
            stream.set_position(start_pos);
            match <crate::r#type::instruction::xor::section_0::XorType as PtxParser>::parse(stream) {
                Ok(inst) => return Ok(Inst::XorType(inst)),
                Err(_) => {}
            }
        }
        _ => {}
    }
    
    // If no parser matched, return error
    let span = stream.peek().map(|(_, s)| s.clone()).unwrap_or(0..0);
    Err(crate::parser::unexpected_value(span, &["valid PTX instruction"], "no matching instruction format"))
}

