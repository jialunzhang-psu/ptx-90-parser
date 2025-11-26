// TreeDisplay implementations for variable types (src/type/variable.rs)

use super::{TreeDisplay, TreeFormatter};
use crate::r#type::variable::*;

impl TreeDisplay for ModuleVariableDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            ModuleVariableDirective::Tex { directive, span } => {
                f.root(&format!(
                    "ModuleVariableDirective::Tex [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            ModuleVariableDirective::Shared { directive, span } => {
                f.root(&format!(
                    "ModuleVariableDirective::Shared [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            ModuleVariableDirective::Global { directive, span } => {
                f.root(&format!(
                    "ModuleVariableDirective::Global [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
            ModuleVariableDirective::Const { directive, span } => {
                f.root(&format!(
                    "ModuleVariableDirective::Const [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "directive", directive, source)
            }
        }
    }
}

impl TreeDisplay for VariableDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        f.root(&format!(
            "VariableDirective [{}]",
            f.format_raw(self.span, source)
        ))?;
        f.field_vec(false, "attributes", &self.attributes, source)?;
        f.field_vec(false, "modifiers", &self.modifiers, source)?;
        f.field_with_child(false, "ty", &self.ty, source)?;
        f.field(false, "name", &self.name.val)?;
        f.field(false, "array_dims", &format!("{:?}", self.array_dims))?;
        f.field_option(true, "initializer", &self.initializer, source)
    }
}

impl TreeDisplay for VariableModifier {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            VariableModifier::Vector { value, span } => {
                f.root(&format!(
                    "VariableModifier::Vector [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(true, "value", &value.to_string())
            }
            VariableModifier::Alignment { value, span } => {
                f.root(&format!(
                    "VariableModifier::Alignment [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(true, "value", &value.to_string())
            }
            VariableModifier::Ptr { span } => f.root(&format!(
                "VariableModifier::Ptr [{}]",
                f.format_raw(*span, source)
            )),
        }
    }
}

impl TreeDisplay for ParameterDirective {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            ParameterDirective::Register { ty, name, span } => {
                f.root(&format!(
                    "ParameterDirective::Register [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(false, "ty", ty, source)?;
                f.field(true, "name", &name.val)
            }
            ParameterDirective::Parameter {
                align,
                ptr,
                space,
                ty,
                name,
                array,
                span,
            } => {
                f.root(&format!(
                    "ParameterDirective::Parameter [{}]",
                    f.format_raw(*span, source)
                ))?;
                match align {
                    Some(a) => f.field(false, "align", &format!("Some({})", a))?,
                    None => f.field(false, "align", "None")?,
                }
                f.field(false, "ptr", &ptr.to_string())?;
                f.field_option(false, "space", space, source)?;
                f.field_with_child(false, "ty", ty, source)?;
                f.field(false, "name", &name.val)?;
                f.field(true, "array", &format!("{:?}", array))
            }
        }
    }
}

impl TreeDisplay for ParamStateSpace {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        let (variant, span) = match self {
            ParamStateSpace::Const { span } => ("Const", span),
            ParamStateSpace::Global { span } => ("Global", span),
            ParamStateSpace::Local { span } => ("Local", span),
            ParamStateSpace::Shared { span } => ("Shared", span),
        };
        f.root(&format!(
            "ParamStateSpace::{} [{}]",
            variant,
            f.format_raw(*span, source)
        ))
    }
}

impl TreeDisplay for InitializerValue {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            InitializerValue::NumericLiteral { value, span } => {
                f.root(&format!(
                    "InitializerValue::NumericLiteral [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(true, "value", &format!("{:?}", value))
            }
            InitializerValue::FunctionSymbol { name, span } => {
                f.root(&format!(
                    "InitializerValue::FunctionSymbol [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(true, "name", &name.val)
            }
            InitializerValue::StringLiteral { value, span } => {
                f.root(&format!(
                    "InitializerValue::StringLiteral [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field(true, "value", &format!("\"{}\"", value))
            }
        }
    }
}

impl TreeDisplay for GlobalInitializer {
    fn tree_display(&self, f: &mut TreeFormatter, source: &str) -> std::fmt::Result {
        match self {
            GlobalInitializer::Scalar { value, span } => {
                f.root(&format!(
                    "GlobalInitializer::Scalar [{}]",
                    f.format_raw(*span, source)
                ))?;
                f.field_with_child(true, "value", value, source)
            }
            GlobalInitializer::Aggregate { values, span } => f.root(&format!(
                "GlobalInitializer::Aggregate [{}] ({} values)",
                f.format_raw(*span, source),
                values.len()
            )),
        }
    }
}
