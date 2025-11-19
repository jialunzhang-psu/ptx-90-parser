/// TreeDisplay implementations for common types (src/type/common.rs)

use crate::r#type::common::*;
use super::{TreeDisplay, TreeFormatter};

impl TreeDisplay for CodeLinkage {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        let (variant, span) = match self {
            CodeLinkage::Visible { span } => ("Visible", span),
            CodeLinkage::Extern { span } => ("Extern", span),
            CodeLinkage::Weak { span } => ("Weak", span),
        };
        f.root(&format!("CodeLinkage::{} [{}]", variant, f.format_raw(*span, source)))
    }
}

impl TreeDisplay for DataLinkage {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        let (variant, span) = match self {
            DataLinkage::Visible { span } => ("Visible", span),
            DataLinkage::Extern { span } => ("Extern", span),
            DataLinkage::Weak { span } => ("Weak", span),
            DataLinkage::Common { span } => ("Common", span),
        };
        f.root(&format!("DataLinkage::{} [{}]", variant, f.format_raw(*span, source)))
    }
}

impl TreeDisplay for AttributeDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            AttributeDirective::Unified { uuid1, uuid2, span } => {
                f.root(&format!("AttributeDirective::Unified [{}]", f.format_raw(*span, source)))?;
                f.field(false, "uuid1", &uuid1.to_string())?;
                f.field(true, "uuid2", &uuid2.to_string())
            }
            AttributeDirective::Managed { span } => {
                f.root(&format!("AttributeDirective::Managed [{}]", f.format_raw(*span, source)))
            }
        }
    }
}

impl TreeDisplay for DataType {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        let (variant, span) = match self {
            DataType::U8 { span } => ("U8", span),
            DataType::U16 { span } => ("U16", span),
            DataType::U32 { span } => ("U32", span),
            DataType::U64 { span } => ("U64", span),
            DataType::S8 { span } => ("S8", span),
            DataType::S16 { span } => ("S16", span),
            DataType::S32 { span } => ("S32", span),
            DataType::S64 { span } => ("S64", span),
            DataType::F16 { span } => ("F16", span),
            DataType::F16x2 { span } => ("F16x2", span),
            DataType::F32 { span } => ("F32", span),
            DataType::F64 { span } => ("F64", span),
            DataType::B8 { span } => ("B8", span),
            DataType::B16 { span } => ("B16", span),
            DataType::B32 { span } => ("B32", span),
            DataType::B64 { span } => ("B64", span),
            DataType::B128 { span } => ("B128", span),
            DataType::Pred { span } => ("Pred", span),
            DataType::TexRef { span } => ("TexRef", span),
            DataType::SamplerRef { span } => ("SamplerRef", span),
            DataType::SurfRef { span } => ("SurfRef", span),
        };
        f.root(&format!("DataType::{} [{}]", variant, f.format_raw(*span, source)))
    }
}

impl TreeDisplay for Sign {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        let (variant, span) = match self {
            Sign::Negative { span } => ("Negative", span),
            Sign::Positive { span } => ("Positive", span),
        };
        f.root(&format!("Sign::{} [{}]", variant, f.format_raw(*span, source)))
    }
}

impl TreeDisplay for Axis {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        let (variant, span) = match self {
            Axis::None { span } => ("None", span),
            Axis::X { span } => ("X", span),
            Axis::Y { span } => ("Y", span),
            Axis::Z { span } => ("Z", span),
        };
        f.root(&format!("Axis::{} [{}]", variant, f.format_raw(*span, source)))
    }
}

impl TreeDisplay for SpecialRegister {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            SpecialRegister::AggrSmemSize { span } => {
                f.root(&format!("SpecialRegister::AggrSmemSize [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::DynamicSmemSize { span } => {
                f.root(&format!("SpecialRegister::DynamicSmemSize [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::LanemaskGt { span } => {
                f.root(&format!("SpecialRegister::LanemaskGt [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::ReservedSmemOffsetBegin { span } => {
                f.root(&format!("SpecialRegister::ReservedSmemOffsetBegin [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Clock { span } => {
                f.root(&format!("SpecialRegister::Clock [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Envreg { index, span } => {
                f.root(&format!("SpecialRegister::Envreg [{}]", f.format_raw(*span, source)))?;
                f.field(true, "index", &index.to_string())
            }
            SpecialRegister::LanemaskLe { span } => {
                f.root(&format!("SpecialRegister::LanemaskLe [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::ReservedSmemOffsetCap { span } => {
                f.root(&format!("SpecialRegister::ReservedSmemOffsetCap [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Clock64 { span } => {
                f.root(&format!("SpecialRegister::Clock64 [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Globaltimer { span } => {
                f.root(&format!("SpecialRegister::Globaltimer [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::LanemaskLt { span } => {
                f.root(&format!("SpecialRegister::LanemaskLt [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::ReservedSmemOffsetEnd { span } => {
                f.root(&format!("SpecialRegister::ReservedSmemOffsetEnd [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::ClusterCtaid { axis, span } => {
                f.root(&format!("SpecialRegister::ClusterCtaid [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "axis", axis, source)
            }
            SpecialRegister::GlobaltimerHi { span } => {
                f.root(&format!("SpecialRegister::GlobaltimerHi [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Nclusterid { span } => {
                f.root(&format!("SpecialRegister::Nclusterid [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Smid { span } => {
                f.root(&format!("SpecialRegister::Smid [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::ClusterCtarank { axis, span } => {
                f.root(&format!("SpecialRegister::ClusterCtarank [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "axis", axis, source)
            }
            SpecialRegister::GlobaltimerLo { span } => {
                f.root(&format!("SpecialRegister::GlobaltimerLo [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Nctaid { axis, span } => {
                f.root(&format!("SpecialRegister::Nctaid [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "axis", axis, source)
            }
            SpecialRegister::Tid { axis, span } => {
                f.root(&format!("SpecialRegister::Tid [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "axis", axis, source)
            }
            SpecialRegister::ClusterNctaid { axis, span } => {
                f.root(&format!("SpecialRegister::ClusterNctaid [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "axis", axis, source)
            }
            SpecialRegister::Gridid { span } => {
                f.root(&format!("SpecialRegister::Gridid [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Nsmid { span } => {
                f.root(&format!("SpecialRegister::Nsmid [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::TotalSmemSize { span } => {
                f.root(&format!("SpecialRegister::TotalSmemSize [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::ClusterNctarank { axis, span } => {
                f.root(&format!("SpecialRegister::ClusterNctarank [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "axis", axis, source)
            }
            SpecialRegister::IsExplicitCluster { span } => {
                f.root(&format!("SpecialRegister::IsExplicitCluster [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Ntid { axis, span } => {
                f.root(&format!("SpecialRegister::Ntid [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "axis", axis, source)
            }
            SpecialRegister::Warpid { span } => {
                f.root(&format!("SpecialRegister::Warpid [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Clusterid { span } => {
                f.root(&format!("SpecialRegister::Clusterid [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Laneid { span } => {
                f.root(&format!("SpecialRegister::Laneid [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Nwarpid { span } => {
                f.root(&format!("SpecialRegister::Nwarpid [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::WARPSZ { span } => {
                f.root(&format!("SpecialRegister::WARPSZ [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Ctaid { axis, span } => {
                f.root(&format!("SpecialRegister::Ctaid [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "axis", axis, source)
            }
            SpecialRegister::LanemaskEq { span } => {
                f.root(&format!("SpecialRegister::LanemaskEq [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::Pm { index, span } => {
                f.root(&format!("SpecialRegister::Pm [{}]", f.format_raw(*span, source)))?;
                f.field(true, "index", &index.to_string())
            }
            SpecialRegister::Pm64 { index, span } => {
                f.root(&format!("SpecialRegister::Pm64 [{}]", f.format_raw(*span, source)))?;
                f.field(true, "index", &index.to_string())
            }
            SpecialRegister::CurrentGraphExec { span } => {
                f.root(&format!("SpecialRegister::CurrentGraphExec [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::LanemaskGe { span } => {
                f.root(&format!("SpecialRegister::LanemaskGe [{}]", f.format_raw(*span, source)))
            }
            SpecialRegister::ReservedSmemOffset { index, span } => {
                f.root(&format!("SpecialRegister::ReservedSmemOffset [{}]", f.format_raw(*span, source)))?;
                f.field(true, "index", &index.to_string())
            }
        }
    }
}

impl TreeDisplay for Label {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!("Label [{}]", f.format_raw(self.span, source)))?;
        f.field(true, "val", &self.val)
    }
}

impl TreeDisplay for Predicate {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!("Predicate [{}]", f.format_raw(self.span, source)))?;
        f.field(false, "negated", &self.negated.to_string())?;
        f.field_with_child(true, "operand", &self.operand, source)
    }
}

impl TreeDisplay for Instruction {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        let pred_str = match &self.predicate {
            Some(p) => format!("@{}{} ", if p.negated { "!" } else { "" }, format!("{:?}", p.operand)),
            None => String::new(),
        };
        f.root(&format!("Instruction [{}] {}{:?}", f.format_raw(self.span, source), pred_str, self.inst))
    }
}

impl TreeDisplay for Operand {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            Operand::Register { operand, span } => {
                f.root(&format!("Operand::Register [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "operand", operand, source)
            }
            Operand::Immediate { operand, span } => {
                f.root(&format!("Operand::Immediate [{}]", f.format_raw(*span, source)))?;
                f.field_with_child(true, "operand", operand, source)
            }
            Operand::Symbol { name, span } => {
                f.root(&format!("Operand::Symbol [{}]", f.format_raw(*span, source)))?;
                f.field(true, "name", name)
            }
            Operand::SymbolOffset { symbol, offset, span } => {
                f.root(&format!("Operand::SymbolOffset [{}]", f.format_raw(*span, source)))?;
                f.field(false, "symbol", symbol)?;
                f.field_with_child(true, "offset", offset, source)
            }
        }
    }
}

impl TreeDisplay for RegisterOperand {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!("RegisterOperand [{}]", f.format_raw(self.span, source)))?;
        f.field(false, "name", &self.name)?;
        match &self.component {
            Some(c) => f.field(true, "component", &format!("Some({})", c)),
            None => f.field(true, "component", "None"),
        }
    }
}

impl TreeDisplay for Immediate {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!("Immediate [{}]", f.format_raw(self.span, source)))?;
        f.field(true, "value", &self.value)
    }
}
