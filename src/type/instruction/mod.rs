use crate::{
    parser::{PtxParseError, PtxParser, PtxTokenStream, unexpected_value},
    r#type::common::*,
};
/// A PTX instruction with optional predicate and modifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub predicate: Option<PredicateRegister>,
    pub opcode: InstructionOpcode,
    pub comment: Option<String>,
    pub raw: String,
}

pub mod abs;
pub mod activemask;
pub mod add;
pub mod addc;
pub mod alloca;
pub mod and;
pub mod applypriority;
pub mod atom;
pub mod bar;
pub mod barrier;
pub mod bfe;
pub mod bfi;
pub mod bfind;
pub mod bmsk;
pub mod bra;
pub mod brev;
pub mod brkpt;
pub mod brx;
pub mod call;
pub mod clusterlaunchcontrol;
pub mod clz;
pub mod cnot;
pub mod copysign;
pub mod cos;
pub mod cp;
pub mod createpolicy;
pub mod cvt;
pub mod cvta;
pub mod discard;
pub mod div;
pub mod dp2a;
pub mod dp4a;
pub mod elect;
pub mod ex2;
pub mod exit;
pub mod fence;
pub mod fma;
pub mod fns;
pub mod getctarank;
pub mod griddepcontrol;
pub mod isspacep;
pub mod istypep;
pub mod ld;
pub mod ldmatrix;
pub mod ldu;
pub mod lg2;
pub mod lop3;
pub mod mad;
pub mod mad24;
pub mod madc;
pub mod mapa;
pub mod r#match;
pub mod max;
pub mod mbarrier;
pub mod membar;
pub mod min;
pub mod mma;
pub mod mov;
pub mod movmatrix;
pub mod mul;
pub mod mul24;
pub mod multimem;
pub mod nanosleep;
pub mod neg;
pub mod not;
pub mod or;
pub mod pmevent;
pub mod popc;
pub mod prefetch;
pub mod prefetchu;
pub mod prmt;
pub mod rcp;
pub mod red;
pub mod redux;
pub mod rem;
pub mod ret;
pub mod rsqrt;
pub mod sad;
pub mod selp;
pub mod set;
pub mod setmaxnreg;
pub mod setp;
pub mod shf;
pub mod shfl;
pub mod shl;
pub mod shr;
pub mod sin;
pub mod slct;
pub mod sqrt;
pub mod st;
pub mod stackrestore;
pub mod stacksave;
pub mod stmatrix;
pub mod sub;
pub mod subc;
pub mod suld;
pub mod suq;
pub mod sured;
pub mod sust;
pub mod szext;
pub mod tanh;
pub mod tcgen05;
pub mod tensormap;
pub mod testp;
pub mod tex;
pub mod tld4;
pub mod trap;
pub mod txq;
pub mod vmad;
pub mod vop;
pub mod vop2;
pub mod vop4;
pub mod vote;
pub mod vset;
pub mod vset2;
pub mod vset4;
pub mod vsh;
pub mod wgmma;
pub mod wmma;
pub mod xor;

/// All supported PTX opcodes with their dedicated operand/modifier payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionOpcode {
    Abs(abs::Abs),
    Activemask(activemask::Activemask),
    Add(add::Add),
    Addc(addc::Addc),
    Alloca(alloca::Alloca),
    And(and::And),
    Applypriority(applypriority::Applypriority),
    Atom(atom::Atom),
    Bar(bar::Bar),
    Barrier(barrier::Barrier),
    Bfe(bfe::Bfe),
    Bfi(bfi::Bfi),
    Bfind(bfind::Bfind),
    Bmsk(bmsk::Bmsk),
    Brev(brev::Brev),
    Bra(bra::Bra),
    Brkpt(brkpt::Brkpt),
    Brx(brx::Brx),
    Call(call::Call),
    Clz(clz::Clz),
    Clusterlaunchcontrol(clusterlaunchcontrol::TryCancel),
    Cnot(cnot::Cnot),
    Copysign(copysign::Copysign),
    Cos(cos::Cos),
    Cp(cp::CpOpcode),
    Createpolicy(createpolicy::Createpolicy),
    Cvt(cvt::Cvt),
    Cvta(cvta::Cvta),
    Div(div::Div),
    Discard(discard::Discard),
    Dp2a(dp2a::Dp2a),
    Dp4a(dp4a::Dp4a),
    Elect(elect::Elect),
    Ex2(ex2::Ex2),
    Exit(exit::Exit),
    Fence(fence::FenceInstruction),
    Fma(fma::Fma),
    Fns(fns::Fns),
    Getctarank(getctarank::Getctarank),
    Griddepcontrol(griddepcontrol::Griddepcontrol),
    Isspacep(isspacep::Isspacep),
    Istypep(istypep::Istypep),
    Ld(ld::Ld),
    Ldmatrix(ldmatrix::Ldmatrix),
    Ldu(ldu::Ldu),
    Lg2(lg2::Lg2),
    Lop3(lop3::Lop3),
    Mad(mad::Mad),
    Mad24(mad24::Mad24),
    Madc(madc::Madc),
    Mapa(mapa::Mapa),
    Match(r#match::Match),
    Max(max::Max),
    Mbarrier(mbarrier::Mbarrier),
    Membar(membar::Membar),
    Min(min::Min),
    Mov(mov::Mov),
    Movmatrix(movmatrix::Movmatrix),
    Mma(mma::MmaInstruction),
    Mul(mul::Mul),
    Mul24(mul24::Mul24),
    Multimem(multimem::Instruction),
    Nanosleep(nanosleep::Nanosleep),
    Neg(neg::Neg),
    Not(not::Not),
    Or(or::Or),
    Pmevent(pmevent::Pmevent),
    Popc(popc::Popc),
    Prefetch(prefetch::Prefetch),
    Prefetchu(prefetchu::Prefetchu),
    Prmt(prmt::Prmt),
    Rcp(rcp::Rcp),
    Red(red::RedOpcode),
    Redux(redux::Redux),
    Rem(rem::Rem),
    Rsqrt(rsqrt::Rsqrt),
    Sad(sad::Sad),
    Selp(selp::Selp),
    Set(set::Set),
    Setmaxnreg(setmaxnreg::Setmaxnreg),
    Setp(setp::Setp),
    Shf(shf::Shf),
    Shfl(shfl::Shfl),
    Shl(shl::Shl),
    Shr(shr::Shr),
    Sin(sin::Sin),
    Slct(slct::Slct),
    Sqrt(sqrt::Sqrt),
    Stackrestore(stackrestore::Stackrestore),
    Stacksave(stacksave::Stacksave),
    St(st::St),
    Stmatrix(stmatrix::Stmatrix),
    Sub(sub::Sub),
    Subc(subc::Subc),
    Suq(suq::Suq),
    Suld(suld::Suld),
    Sured(sured::Sured),
    Sust(sust::Sust),
    Szext(szext::Szext),
    Tanh(tanh::Tanh),
    Tcgen05(tcgen05::Tcgen05),
    Tensormap(tensormap::TensormapOpcode),
    Tex(tex::Tex),
    Testp(testp::Testp),
    Tld4(tld4::Tld4),
    Trap(trap::Trap),
    Txq(txq::Txq),
    Vabsdiff(vop::Vop),
    Vabsdiff2(vop2::Vop2),
    Vabsdiff4(vop4::Vop4),
    Vadd(vop::Vop),
    Vadd2(vop2::Vop2),
    Vadd4(vop4::Vop4),
    Vavrg2(vop2::Vop2),
    Vavrg4(vop4::Vop4),
    Vmad(vmad::Vmad),
    Vmax(vop::Vop),
    Vmax2(vop2::Vop2),
    Vmax4(vop4::Vop4),
    Vmin(vop::Vop),
    Vmin2(vop2::Vop2),
    Vmin4(vop4::Vop4),
    Vset(vset::Vset),
    Vset2(vset2::Vset2),
    Vset4(vset4::Vset4),
    Vshl(vsh::Vsh),
    Vshr(vsh::Vsh),
    Vsub(vop::Vop),
    Vsub2(vop2::Vop2),
    Vsub4(vop4::Vop4),
    Vote(vote::Vote),
    Wgmma(wgmma::Wgmma),
    Wmma(wmma::Instruction),
    Xor(xor::Xor),
    Ret(ret::Ret),
}

const INSTRUCTION_NAMES: &[&str] = &[
    "abs",
    "activemask",
    "add",
    "addc",
    "alloca",
    "and",
    "applypriority",
    "atom",
    "bar",
    "barrier",
    "bfe",
    "bfi",
    "bfind",
    "bmsk",
    "brev",
    "bra",
    "brkpt",
    "brx",
    "call",
    "clz",
    "clusterlaunchcontrol",
    "cnot",
    "copysign",
    "cos",
    "cp",
    "createpolicy",
    "cvt",
    "cvta",
    "div",
    "discard",
    "dp2a",
    "dp4a",
    "elect",
    "ex2",
    "exit",
    "fence",
    "fma",
    "fns",
    "getctarank",
    "griddepcontrol",
    "isspacep",
    "istypep",
    "ld",
    "ldmatrix",
    "ldu",
    "lg2",
    "lop3",
    "mad",
    "mad24",
    "madc",
    "mapa",
    "match",
    "max",
    "mbarrier",
    "membar",
    "min",
    "mov",
    "movmatrix",
    "mma",
    "mul",
    "mul24",
    "multimem",
    "nanosleep",
    "neg",
    "not",
    "or",
    "pmevent",
    "popc",
    "prefetch",
    "prefetchu",
    "prmt",
    "rcp",
    "red",
    "redux",
    "rem",
    "ret",
    "rsqrt",
    "sad",
    "selp",
    "set",
    "setmaxnreg",
    "setp",
    "shf",
    "shfl",
    "shl",
    "shr",
    "sin",
    "slct",
    "sqrt",
    "stackrestore",
    "stacksave",
    "st",
    "stmatrix",
    "sub",
    "subc",
    "suld",
    "suq",
    "sured",
    "sust",
    "szext",
    "tanh",
    "tcgen05",
    "tensormap",
    "testp",
    "tex",
    "tld4",
    "trap",
    "txq",
    "vabsdiff",
    "vabsdiff2",
    "vabsdiff4",
    "vadd",
    "vadd2",
    "vadd4",
    "vavrg2",
    "vavrg4",
    "vmad",
    "vmax",
    "vmax2",
    "vmax4",
    "vmin",
    "vmin2",
    "vmin4",
    "vset",
    "vset2",
    "vset4",
    "vshl",
    "vshr",
    "vsub",
    "vsub2",
    "vsub4",
    "vote",
    "wgmma",
    "wmma",
    "xor",
];

impl PtxParser for InstructionOpcode {
    fn parse(stream: &mut PtxTokenStream) -> Result<Self, PtxParseError> {
        let position = stream.position();
        let (identifier, span) = stream.expect_identifier()?;
        stream.set_position(position);

        match identifier.as_str() {
            "abs" => Ok(InstructionOpcode::Abs(self::abs::Abs::parse(stream)?)),
            "activemask" => Ok(InstructionOpcode::Activemask(
                self::activemask::Activemask::parse(stream)?,
            )),
            "add" => Ok(InstructionOpcode::Add(self::add::Add::parse(stream)?)),
            "addc" => Ok(InstructionOpcode::Addc(self::addc::Addc::parse(stream)?)),
            "alloca" => Ok(InstructionOpcode::Alloca(self::alloca::Alloca::parse(
                stream,
            )?)),
            "and" => Ok(InstructionOpcode::And(self::and::And::parse(stream)?)),
            "applypriority" => Ok(InstructionOpcode::Applypriority(
                self::applypriority::Applypriority::parse(stream)?,
            )),
            "atom" => Ok(InstructionOpcode::Atom(self::atom::Atom::parse(stream)?)),
            "bar" => Ok(InstructionOpcode::Bar(self::bar::Bar::parse(stream)?)),
            "barrier" => Ok(InstructionOpcode::Barrier(self::barrier::Barrier::parse(
                stream,
            )?)),
            "bfe" => Ok(InstructionOpcode::Bfe(self::bfe::Bfe::parse(stream)?)),
            "bfi" => Ok(InstructionOpcode::Bfi(self::bfi::Bfi::parse(stream)?)),
            "bfind" => Ok(InstructionOpcode::Bfind(self::bfind::Bfind::parse(stream)?)),
            "bmsk" => Ok(InstructionOpcode::Bmsk(self::bmsk::Bmsk::parse(stream)?)),
            "brev" => Ok(InstructionOpcode::Brev(self::brev::Brev::parse(stream)?)),
            "bra" => Ok(InstructionOpcode::Bra(self::bra::Bra::parse(stream)?)),
            "brkpt" => Ok(InstructionOpcode::Brkpt(self::brkpt::Brkpt::parse(stream)?)),
            "brx" => Ok(InstructionOpcode::Brx(self::brx::Brx::parse(stream)?)),
            "call" => Ok(InstructionOpcode::Call(self::call::Call::parse(stream)?)),
            "clz" => Ok(InstructionOpcode::Clz(self::clz::Clz::parse(stream)?)),
            "clusterlaunchcontrol" => Ok(InstructionOpcode::Clusterlaunchcontrol(
                self::clusterlaunchcontrol::TryCancel::parse(stream)?,
            )),
            "cnot" => Ok(InstructionOpcode::Cnot(self::cnot::Cnot::parse(stream)?)),
            "copysign" => Ok(InstructionOpcode::Copysign(
                self::copysign::Copysign::parse(stream)?,
            )),
            "cos" => Ok(InstructionOpcode::Cos(self::cos::Cos::parse(stream)?)),
            "cp" => Ok(InstructionOpcode::Cp(self::cp::CpOpcode::parse(stream)?)),
            "createpolicy" => Ok(InstructionOpcode::Createpolicy(
                self::createpolicy::Createpolicy::parse(stream)?,
            )),
            "cvt" => Ok(InstructionOpcode::Cvt(self::cvt::Cvt::parse(stream)?)),
            "cvta" => Ok(InstructionOpcode::Cvta(self::cvta::Cvta::parse(stream)?)),
            "div" => Ok(InstructionOpcode::Div(self::div::Div::parse(stream)?)),
            "discard" => Ok(InstructionOpcode::Discard(self::discard::Discard::parse(
                stream,
            )?)),
            "dp2a" => Ok(InstructionOpcode::Dp2a(self::dp2a::Dp2a::parse(stream)?)),
            "dp4a" => Ok(InstructionOpcode::Dp4a(self::dp4a::Dp4a::parse(stream)?)),
            "elect" => Ok(InstructionOpcode::Elect(self::elect::Elect::parse(stream)?)),
            "ex2" => Ok(InstructionOpcode::Ex2(self::ex2::Ex2::parse(stream)?)),
            "exit" => Ok(InstructionOpcode::Exit(self::exit::Exit::parse(stream)?)),
            "fence" => Ok(InstructionOpcode::Fence(
                self::fence::FenceInstruction::parse(stream)?,
            )),
            "fma" => Ok(InstructionOpcode::Fma(self::fma::Fma::parse(stream)?)),
            "fns" => Ok(InstructionOpcode::Fns(self::fns::Fns::parse(stream)?)),
            "getctarank" => Ok(InstructionOpcode::Getctarank(
                self::getctarank::Getctarank::parse(stream)?,
            )),
            "griddepcontrol" => Ok(InstructionOpcode::Griddepcontrol(
                self::griddepcontrol::Griddepcontrol::parse(stream)?,
            )),
            "isspacep" => Ok(InstructionOpcode::Isspacep(
                self::isspacep::Isspacep::parse(stream)?,
            )),
            "istypep" => Ok(InstructionOpcode::Istypep(self::istypep::Istypep::parse(
                stream,
            )?)),
            "ld" => Ok(InstructionOpcode::Ld(self::ld::Ld::parse(stream)?)),
            "ldmatrix" => Ok(InstructionOpcode::Ldmatrix(
                self::ldmatrix::Ldmatrix::parse(stream)?,
            )),
            "ldu" => Ok(InstructionOpcode::Ldu(self::ldu::Ldu::parse(stream)?)),
            "lg2" => Ok(InstructionOpcode::Lg2(self::lg2::Lg2::parse(stream)?)),
            "lop3" => Ok(InstructionOpcode::Lop3(self::lop3::Lop3::parse(stream)?)),
            "mad" => Ok(InstructionOpcode::Mad(self::mad::Mad::parse(stream)?)),
            "mad24" => Ok(InstructionOpcode::Mad24(self::mad24::Mad24::parse(stream)?)),
            "madc" => Ok(InstructionOpcode::Madc(self::madc::Madc::parse(stream)?)),
            "mapa" => Ok(InstructionOpcode::Mapa(self::mapa::Mapa::parse(stream)?)),
            "match" => Ok(InstructionOpcode::Match(self::r#match::Match::parse(
                stream,
            )?)),
            "max" => Ok(InstructionOpcode::Max(self::max::Max::parse(stream)?)),
            "mbarrier" => Ok(InstructionOpcode::Mbarrier(
                self::mbarrier::Mbarrier::parse(stream)?,
            )),
            "membar" => Ok(InstructionOpcode::Membar(self::membar::Membar::parse(
                stream,
            )?)),
            "min" => Ok(InstructionOpcode::Min(self::min::Min::parse(stream)?)),
            "mov" => Ok(InstructionOpcode::Mov(self::mov::Mov::parse(stream)?)),
            "movmatrix" => Ok(InstructionOpcode::Movmatrix(
                self::movmatrix::Movmatrix::parse(stream)?,
            )),
            "mma" => Ok(InstructionOpcode::Mma(self::mma::MmaInstruction::parse(
                stream,
            )?)),
            "mul" => Ok(InstructionOpcode::Mul(self::mul::Mul::parse(stream)?)),
            "mul24" => Ok(InstructionOpcode::Mul24(self::mul24::Mul24::parse(stream)?)),
            "multimem" => Ok(InstructionOpcode::Multimem(
                self::multimem::Instruction::parse(stream)?,
            )),
            "nanosleep" => Ok(InstructionOpcode::Nanosleep(
                self::nanosleep::Nanosleep::parse(stream)?,
            )),
            "neg" => Ok(InstructionOpcode::Neg(self::neg::Neg::parse(stream)?)),
            "not" => Ok(InstructionOpcode::Not(self::not::Not::parse(stream)?)),
            "or" => Ok(InstructionOpcode::Or(self::or::Or::parse(stream)?)),
            "pmevent" => Ok(InstructionOpcode::Pmevent(self::pmevent::Pmevent::parse(
                stream,
            )?)),
            "popc" => Ok(InstructionOpcode::Popc(self::popc::Popc::parse(stream)?)),
            "prefetch" => Ok(InstructionOpcode::Prefetch(
                self::prefetch::Prefetch::parse(stream)?,
            )),
            "prefetchu" => Ok(InstructionOpcode::Prefetchu(
                self::prefetchu::Prefetchu::parse(stream)?,
            )),
            "prmt" => Ok(InstructionOpcode::Prmt(self::prmt::Prmt::parse(stream)?)),
            "rcp" => Ok(InstructionOpcode::Rcp(self::rcp::Rcp::parse(stream)?)),
            "red" => Ok(InstructionOpcode::Red(self::red::RedOpcode::parse(stream)?)),
            "redux" => Ok(InstructionOpcode::Redux(self::redux::Redux::parse(stream)?)),
            "rem" => Ok(InstructionOpcode::Rem(self::rem::Rem::parse(stream)?)),
            "ret" => Ok(InstructionOpcode::Ret(self::ret::Ret::parse(stream)?)),
            "rsqrt" => Ok(InstructionOpcode::Rsqrt(self::rsqrt::Rsqrt::parse(stream)?)),
            "sad" => Ok(InstructionOpcode::Sad(self::sad::Sad::parse(stream)?)),
            "selp" => Ok(InstructionOpcode::Selp(self::selp::Selp::parse(stream)?)),
            "set" => Ok(InstructionOpcode::Set(self::set::Set::parse(stream)?)),
            "setmaxnreg" => Ok(InstructionOpcode::Setmaxnreg(
                self::setmaxnreg::Setmaxnreg::parse(stream)?,
            )),
            "setp" => Ok(InstructionOpcode::Setp(self::setp::Setp::parse(stream)?)),
            "shf" => Ok(InstructionOpcode::Shf(self::shf::Shf::parse(stream)?)),
            "shfl" => Ok(InstructionOpcode::Shfl(self::shfl::Shfl::parse(stream)?)),
            "shl" => Ok(InstructionOpcode::Shl(self::shl::Shl::parse(stream)?)),
            "shr" => Ok(InstructionOpcode::Shr(self::shr::Shr::parse(stream)?)),
            "sin" => Ok(InstructionOpcode::Sin(self::sin::Sin::parse(stream)?)),
            "slct" => Ok(InstructionOpcode::Slct(self::slct::Slct::parse(stream)?)),
            "sqrt" => Ok(InstructionOpcode::Sqrt(self::sqrt::Sqrt::parse(stream)?)),
            "stackrestore" => Ok(InstructionOpcode::Stackrestore(
                self::stackrestore::Stackrestore::parse(stream)?,
            )),
            "stacksave" => Ok(InstructionOpcode::Stacksave(
                self::stacksave::Stacksave::parse(stream)?,
            )),
            "st" => Ok(InstructionOpcode::St(self::st::St::parse(stream)?)),
            "stmatrix" => Ok(InstructionOpcode::Stmatrix(
                self::stmatrix::Stmatrix::parse(stream)?,
            )),
            "sub" => Ok(InstructionOpcode::Sub(self::sub::Sub::parse(stream)?)),
            "subc" => Ok(InstructionOpcode::Subc(self::subc::Subc::parse(stream)?)),
            "suld" => Ok(InstructionOpcode::Suld(self::suld::Suld::parse(stream)?)),
            "suq" => Ok(InstructionOpcode::Suq(self::suq::Suq::parse(stream)?)),
            "sured" => Ok(InstructionOpcode::Sured(self::sured::Sured::parse(stream)?)),
            "sust" => Ok(InstructionOpcode::Sust(self::sust::Sust::parse(stream)?)),
            "szext" => Ok(InstructionOpcode::Szext(self::szext::Szext::parse(stream)?)),
            "tanh" => Ok(InstructionOpcode::Tanh(self::tanh::Tanh::parse(stream)?)),
            "tcgen05" => Ok(InstructionOpcode::Tcgen05(self::tcgen05::Tcgen05::parse(
                stream,
            )?)),
            "tensormap" => Ok(InstructionOpcode::Tensormap(
                self::tensormap::TensormapOpcode::parse(stream)?,
            )),
            "testp" => Ok(InstructionOpcode::Testp(self::testp::Testp::parse(stream)?)),
            "tex" => Ok(InstructionOpcode::Tex(self::tex::Tex::parse(stream)?)),
            "tld4" => Ok(InstructionOpcode::Tld4(self::tld4::Tld4::parse(stream)?)),
            "trap" => Ok(InstructionOpcode::Trap(self::trap::Trap::parse(stream)?)),
            "txq" => Ok(InstructionOpcode::Txq(self::txq::Txq::parse(stream)?)),
            "vabsdiff" => Ok(InstructionOpcode::Vabsdiff(self::vop::Vop::parse(stream)?)),
            "vabsdiff2" => Ok(InstructionOpcode::Vabsdiff2(self::vop2::Vop2::parse(
                stream,
            )?)),
            "vabsdiff4" => Ok(InstructionOpcode::Vabsdiff4(self::vop4::Vop4::parse(
                stream,
            )?)),
            "vadd" => Ok(InstructionOpcode::Vadd(self::vop::Vop::parse(stream)?)),
            "vadd2" => Ok(InstructionOpcode::Vadd2(self::vop2::Vop2::parse(stream)?)),
            "vadd4" => Ok(InstructionOpcode::Vadd4(self::vop4::Vop4::parse(stream)?)),
            "vavrg2" => Ok(InstructionOpcode::Vavrg2(self::vop2::Vop2::parse(stream)?)),
            "vavrg4" => Ok(InstructionOpcode::Vavrg4(self::vop4::Vop4::parse(stream)?)),
            "vmad" => Ok(InstructionOpcode::Vmad(self::vmad::Vmad::parse(stream)?)),
            "vmax" => Ok(InstructionOpcode::Vmax(self::vop::Vop::parse(stream)?)),
            "vmax2" => Ok(InstructionOpcode::Vmax2(self::vop2::Vop2::parse(stream)?)),
            "vmax4" => Ok(InstructionOpcode::Vmax4(self::vop4::Vop4::parse(stream)?)),
            "vmin" => Ok(InstructionOpcode::Vmin(self::vop::Vop::parse(stream)?)),
            "vmin2" => Ok(InstructionOpcode::Vmin2(self::vop2::Vop2::parse(stream)?)),
            "vmin4" => Ok(InstructionOpcode::Vmin4(self::vop4::Vop4::parse(stream)?)),
            "vset" => Ok(InstructionOpcode::Vset(self::vset::Vset::parse(stream)?)),
            "vset2" => Ok(InstructionOpcode::Vset2(self::vset2::Vset2::parse(stream)?)),
            "vset4" => Ok(InstructionOpcode::Vset4(self::vset4::Vset4::parse(stream)?)),
            "vshl" => Ok(InstructionOpcode::Vshl(self::vsh::Vsh::parse(stream)?)),
            "vshr" => Ok(InstructionOpcode::Vshr(self::vsh::Vsh::parse(stream)?)),
            "vsub" => Ok(InstructionOpcode::Vsub(self::vop::Vop::parse(stream)?)),
            "vsub2" => Ok(InstructionOpcode::Vsub2(self::vop2::Vop2::parse(stream)?)),
            "vsub4" => Ok(InstructionOpcode::Vsub4(self::vop4::Vop4::parse(stream)?)),
            "vote" => Ok(InstructionOpcode::Vote(self::vote::Vote::parse(stream)?)),
            "wgmma" => Ok(InstructionOpcode::Wgmma(self::wgmma::Wgmma::parse(stream)?)),
            "wmma" => Ok(InstructionOpcode::Wmma(self::wmma::Instruction::parse(
                stream,
            )?)),
            "xor" => Ok(InstructionOpcode::Xor(self::xor::Xor::parse(stream)?)),
            other => Err(unexpected_value(span, INSTRUCTION_NAMES, other)),
        }
    }
}
