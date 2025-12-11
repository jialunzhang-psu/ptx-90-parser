// TreeDisplay implementations for function types (src/type/function.rs)

use super::{TreeDisplay, TreeFormatter};
use crate::r#type::function::*;

impl TreeDisplay for AliasFunctionDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "AliasFunctionDirective [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field(false, "alias", &self.alias.val)?;
        f.field(true, "target", &self.target.val)
    }
}

impl TreeDisplay for FuncFunctionDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "FuncFunctionDirective [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field_vec(false, "attributes", &self.attributes, source)?;
        f.field_option(false, "return_param", &self.return_param, source)?;
        f.field(false, "name", &self.name.val)?;
        f.field_vec(false, "params", &self.params, source)?;
        f.field_vec(false, "directives", &self.directives, source)?;
        f.field_option(true, "body", &self.body, source)
    }
}

impl TreeDisplay for EntryFunctionDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "EntryFunctionDirective [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field(false, "name", &self.name.val)?;
        f.field_vec(false, "params", &self.params, source)?;
        f.field_vec(false, "directives", &self.directives, source)?;
        f.field_option(true, "body", &self.body, source)
    }
}

impl TreeDisplay for FuncFunctionHeaderDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            FuncFunctionHeaderDirective::NoReturn { span } => f.root(&format!(
                "FuncFunctionHeaderDirective::NoReturn [{}]",
                f.format_raw(*span, source)
            )),
            FuncFunctionHeaderDirective::Pragma { args, span } => {
                f.root(&format!(
                    "FuncFunctionHeaderDirective::Pragma [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_vec(true, "args", args, source)
            }
            FuncFunctionHeaderDirective::AbiPreserve { value, span } => {
                f.root(&format!(
                    "FuncFunctionHeaderDirective::AbiPreserve [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(true, "value", &value.to_string())
            }
            FuncFunctionHeaderDirective::AbiPreserveControl { value, span } => {
                f.root(&format!(
                    "FuncFunctionHeaderDirective::AbiPreserveControl [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(true, "value", &value.to_string())
            }
        }
    }
}

impl TreeDisplay for EntryFunctionHeaderDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            EntryFunctionHeaderDirective::MaxNReg { value, span } => {
                f.root(&format!(
                    "EntryFunctionHeaderDirective::MaxNReg [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(true, "value", &value.to_string())
            }
            EntryFunctionHeaderDirective::MaxNTid { dim, span } => {
                f.root(&format!(
                    "EntryFunctionHeaderDirective::MaxNTid [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "dim", dim, source)
            }
            EntryFunctionHeaderDirective::ReqNTid { dim, span } => {
                f.root(&format!(
                    "EntryFunctionHeaderDirective::ReqNTid [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "dim", dim, source)
            }
            EntryFunctionHeaderDirective::MinNCtaPerSm { value, span } => {
                f.root(&format!(
                    "EntryFunctionHeaderDirective::MinNCtaPerSm [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(true, "value", &value.to_string())
            }
            EntryFunctionHeaderDirective::MaxNCtaPerSm { value, span } => {
                f.root(&format!(
                    "EntryFunctionHeaderDirective::MaxNCtaPerSm [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(true, "value", &value.to_string())
            }
            EntryFunctionHeaderDirective::Pragma { args, span } => {
                f.root(&format!(
                    "EntryFunctionHeaderDirective::Pragma [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_vec(true, "args", args, source)
            }
            EntryFunctionHeaderDirective::ReqNctaPerCluster { dim, span } => {
                f.root(&format!(
                    "EntryFunctionHeaderDirective::ReqNctaPerCluster [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "dim", dim, source)
            }
            EntryFunctionHeaderDirective::ExplicitCluster { span } => f.root(&format!(
                "EntryFunctionHeaderDirective::ExplicitCluster [{}]",
                f.format_raw(*span, source)
            )),
            EntryFunctionHeaderDirective::MaxClusterRank { value, span } => {
                f.root(&format!(
                    "EntryFunctionHeaderDirective::MaxClusterRank [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(true, "value", &value.to_string())
            }
            EntryFunctionHeaderDirective::BlocksAreClusters { span } => f.root(&format!(
                "EntryFunctionHeaderDirective::BlocksAreClusters [{}]",
                f.format_raw(*span, source)
            )),
        }
    }
}

impl TreeDisplay for FunctionBody {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "FunctionBody [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field_vec(true, "statements", &self.statements, source)
    }
}

impl TreeDisplay for FunctionStatement {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            FunctionStatement::Label { label, span } => {
                f.root(&format!(
                    "FunctionStatement::Label [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "label", label, source)
            }
            FunctionStatement::Directive { directive, span } => {
                f.root(&format!(
                    "FunctionStatement::Directive [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            FunctionStatement::Instruction {
                instruction,
                span: _,
            } => {
                // Display instruction inline since it contains all the details
                instruction.tree_display(f, source)
            }
            FunctionStatement::Block { statements, span } => {
                f.root(&format!(
                    "FunctionStatement::Block [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_vec(true, "statements", statements, source)
            }
        }
    }
}

impl TreeDisplay for RegisterDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "RegisterDirective [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field_with_child(false, "ty", &self.ty, source)?;
        f.field_vec(true, "registers", &self.registers, source)
    }
}

impl TreeDisplay for RegisterTarget {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "RegisterTarget [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field(false, "name", &self.name.val)?;
        match self.range {
            Some(r) => f.field(true, "range", &format!("Some({})", r))?,
            None => f.field(true, "range", "None")?,
        };
        Ok(())
    }
}

impl TreeDisplay for StatementDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            StatementDirective::Loc { directive, span } => {
                f.root(&format!(
                    "StatementDirective::Loc [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            StatementDirective::Pragma { directive, span } => {
                f.root(&format!(
                    "StatementDirective::Pragma [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            StatementDirective::Section { directive, span } => {
                f.root(&format!(
                    "StatementDirective::Section [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            StatementDirective::Reg { directive, span } => {
                f.root(&format!(
                    "StatementDirective::Reg [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            StatementDirective::Local { directive, span } => {
                f.root(&format!(
                    "StatementDirective::Local [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            StatementDirective::Param { directive, span } => {
                f.root(&format!(
                    "StatementDirective::Param [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            StatementDirective::Shared { directive, span } => {
                f.root(&format!(
                    "StatementDirective::Shared [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            StatementDirective::Dwarf { directive, span } => {
                f.root(&format!(
                    "StatementDirective::Dwarf [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            StatementDirective::BranchTargets { directive, span } => {
                f.root(&format!(
                    "StatementDirective::BranchTargets [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            StatementDirective::CallTargets { directive, span } => {
                f.root(&format!(
                    "StatementDirective::CallTargets [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            StatementDirective::CallPrototype { directive, span } => {
                f.root(&format!(
                    "StatementDirective::CallPrototype [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
        }
    }
}

impl TreeDisplay for DwarfDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "DwarfDirective [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field(true, "kind", &format!("{:?}", self.kind))
    }
}

impl TreeDisplay for SectionDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "SectionDirective [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field(false, "name", &format!("\"{}\"", self.name))?;
        f.field(
            true,
            "entries",
            &format!("<{} entries>", self.entries.len()),
        )
    }
}

impl TreeDisplay for LocationDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "LocationDirective [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field(false, "file_index", &self.file_index.to_string())?;
        f.field(false, "line", &self.line.to_string())?;
        f.field(false, "column", &self.column.to_string())?;
        f.field_option(true, "inlined_at", &self.inlined_at, source)
    }
}

impl TreeDisplay for LocationInlinedAt {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "LocationInlinedAt [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field(false, "file_index", &self.file_index.to_string())?;
        f.field(false, "line", &self.line.to_string())?;
        f.field(false, "column", &self.column.to_string())?;
        f.field(false, "function_name", &self.function_name.val)?;
        f.field_with_child(false, "label", &self.label, source)?;
        match self.label_offset {
            Some(offset) => f.field(true, "label_offset", &format!("Some({})", offset)),
            None => f.field(true, "label_offset", "None"),
        }
    }
}

impl TreeDisplay for PragmaDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "PragmaDirective [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field(true, "kind", &format!("{:?}", self.kind))
    }
}

impl TreeDisplay for BranchTargetsDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "BranchTargetsDirective [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field_vec(true, "labels", &self.labels, source)
    }
}

impl TreeDisplay for CallTargetsDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "CallTargetsDirective [{}]",
            f.format_raw(self.span, source)
        ))?;
        let targets: Vec<String> = self.targets.iter().map(|t| t.val.clone()).collect();
        f.field_vec(true, "targets", &targets, source)
    }
}

impl TreeDisplay for CallPrototypeDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "CallPrototypeDirective [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field(false, "return_spec", &format!("{:?}", self.return_spec))?;
        f.field_vec(false, "params", &self.params, source)?;
        f.field(false, "noreturn", &self.noreturn.to_string())?;
        match self.abi_preserve {
            Some(v) => f.field(false, "abi_preserve", &format!("Some({})", v))?,
            None => f.field(false, "abi_preserve", "None")?,
        }
        match self.abi_preserve_control {
            Some(v) => f.field(true, "abi_preserve_control", &format!("Some({})", v)),
            None => f.field(true, "abi_preserve_control", "None"),
        }
    }
}

impl TreeDisplay for FunctionDim {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            FunctionDim::X { x, span } => {
                f.root(&format!("FunctionDim::X [{}]", f.format_raw(*span, source)))?;
                f.field(true, "x", &x.to_string())
            }
            FunctionDim::XY { x, y, span } => {
                f.root(&format!(
                    "FunctionDim::XY [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(false, "x", &x.to_string())?;
                f.field(true, "y", &y.to_string())
            }
            FunctionDim::XYZ { x, y, z, span } => {
                f.root(&format!(
                    "FunctionDim::XYZ [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(false, "x", &x.to_string())?;
                f.field(false, "y", &y.to_string())?;
                f.field(true, "z", &z.to_string())
            }
        }
    }
}
