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

use crate::{lexer::PtxToken, r#type::instruction::*, unparser::PtxUnparser};

impl PtxUnparser for InstructionOpcode {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        match self {
            InstructionOpcode::Abs(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Activemask(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Add(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Addc(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Alloca(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::And(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Applypriority(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Atom(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Bar(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Barrier(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Bfe(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Bfi(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Bfind(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Bmsk(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Brev(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Bra(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Brkpt(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Brx(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Call(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Clz(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Clusterlaunchcontrol(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Cnot(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Copysign(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Cos(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Cp(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Createpolicy(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Cvt(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Cvta(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Div(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Discard(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Dp2a(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Dp4a(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Elect(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Ex2(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Exit(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Fence(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Fma(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Fns(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Getctarank(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Griddepcontrol(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Isspacep(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Istypep(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Ld(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Ldmatrix(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Ldu(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Lg2(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Lop3(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Mad(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Mad24(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Madc(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Mapa(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Match(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Max(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Mbarrier(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Membar(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Min(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Mov(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Movmatrix(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Mma(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Mul(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Mul24(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Multimem(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Nanosleep(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Neg(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Not(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Or(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Pmevent(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Popc(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Prefetch(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Prefetchu(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Prmt(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Rcp(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Red(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Redux(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Rem(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Rsqrt(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Sad(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Selp(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Set(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Setmaxnreg(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Setp(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Shf(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Shfl(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Shl(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Shr(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Sin(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Slct(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Sqrt(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Stackrestore(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Stacksave(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::St(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Stmatrix(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Sub(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Subc(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Suq(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Suld(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Sured(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Sust(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Szext(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Tanh(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Tcgen05(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Tensormap(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Tex(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Testp(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Tld4(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Trap(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Txq(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Vabsdiff(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vabsdiff2(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vabsdiff4(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vadd(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vadd2(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vadd4(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vavrg2(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vavrg4(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vmad(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vmax(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vmax2(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vmax4(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vmin(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vmin2(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vmin4(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vset(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vset2(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vset4(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vshl(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vshr(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vsub(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vsub2(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vsub4(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Vote(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Wgmma(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Wmma(instruction) => {
                PtxUnparser::unparse_tokens(instruction, tokens)
            }
            InstructionOpcode::Xor(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
            InstructionOpcode::Ret(instruction) => PtxUnparser::unparse_tokens(instruction, tokens),
        }
    }
}

impl PtxUnparser for Instruction {
    fn unparse_tokens(&self, tokens: &mut Vec<PtxToken>) {
        if let Some(predicate) = &self.predicate {
            tokens.push(PtxToken::At);
            predicate.unparse_tokens(tokens);
        }
        self.opcode.unparse_tokens(tokens);
    }
}
