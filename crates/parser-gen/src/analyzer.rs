use crate::naming;
use crate::r#type::{
    IdentifierToken, InstructionRule, Modifier, Operand, OperandElement, OperatorToken, Rule,
    Section,
};
use std::collections::{BTreeSet, HashMap};

pub type RustName = String;

/// Get the text representation of a modifier for length comparison
fn modifier_text_length(modifier: &AnalyzedModifier) -> usize {
    match modifier {
        AnalyzedModifier::Atom((text, _)) => text.len(),
        AnalyzedModifier::Sequence(items) => {
            items.iter().map(|(m, _)| modifier_text_length(m)).sum()
        }
        AnalyzedModifier::Choice { options, .. } => {
            // For choices, use the maximum length among options
            options
                .iter()
                .map(|(m, _)| modifier_text_length(m))
                .max()
                .unwrap_or(0)
        }
        AnalyzedModifier::Optional(inner) => modifier_text_length(&inner.0),
    }
}

pub struct AnalyzedSection {
    pub instructions: Vec<AnalyzedInstruction>,
    pub opcodes: BTreeSet<IdentifierToken>,
}

#[derive(Debug, Clone)]
pub struct AnalyzedInstruction {
    pub rust_name: RustName,
    pub head: AnalyzedInstructionHead,
    pub operands: Vec<AnalyzedOperand>,
    pub raw: String,
}

#[derive(Debug, Clone)]
pub struct AnalyzedInstructionHead {
    pub opcode: IdentifierToken,
    pub modifiers: Vec<(AnalyzedModifier, RustName)>,
}

#[derive(Debug, Clone)]
pub struct AnalyzedOperand {
    pub operator: Option<(OperatorToken, RustName)>,
    pub operand: AnalyzedOperandElement,
    pub modifier: Option<(AnalyzedModifier, RustName)>,
}

#[derive(Debug, Clone)]
pub enum AnalyzedModifier {
    Atom((IdentifierToken, RustName)),
    Sequence(Vec<(AnalyzedModifier, RustName)>),
    Optional(Box<(AnalyzedModifier, RustName)>),
    Choice {
        base: (IdentifierToken, RustName),
        options: Vec<(AnalyzedModifier, RustName)>,
    },
}

/// Underlying operand element (identifier or nested group).
#[derive(Debug, Clone)]
pub enum AnalyzedOperandElement {
    /// a::b, ::n, etc.
    Item((IdentifierToken, RustName)),
    /// {xdim, ydim, zdim, ...}
    CurlyGroup(Vec<(IdentifierToken, RustName)>),
    /// [xdim, ydim, zdim, ...]
    /// The boolean flag indicates whether the operand is optional within the group.
    SquareGroup(Vec<(IdentifierToken, RustName, bool)>),
    /// (param-list)
    ParamList(RustName),
    /// (ret-param) - Always contains a simple identifier.
    ParenthesizedOperand((IdentifierToken, RustName)),
    /// a|p
    PipeChoice(((IdentifierToken, RustName), (IdentifierToken, RustName))),
    /// a{|p}
    PipeOptionalChoice(((IdentifierToken, RustName), (IdentifierToken, RustName))),
    /// [a] - Always contains a simple identifier.
    Address((IdentifierToken, RustName)),
    /// {, cache-policy} - Always contains a simple identifier.
    Optional((IdentifierToken, RustName)),
    /// Immediate number
    ImmediateNumber((String, RustName)),
    /// Choice, cp-size = { 4, 8, 16 }
    /// First field is the base name, second is the options with their variant names
    Choice {
        base: (IdentifierToken, RustName),
        options: Vec<(IdentifierToken, RustName)>,
    },
}

pub struct Analyzer {
    /// Map from parameter name to its choices
    parameters: HashMap<IdentifierToken, Vec<Modifier>>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
        }
    }

    /// Analyze multiple sections and compute rust names for all of them
    /// Names are made unique across ALL sections to avoid conflicts when re-exported
    pub fn analyze_sections(&mut self, sections: &[Section]) -> Vec<AnalyzedSection> {
        let mut analyzed_sections = Vec::new();

        for section in sections {
            let analyzed = self.analyze(section);

            if !analyzed.instructions.is_empty() {
                analyzed_sections.push(analyzed);
            }
        }

        // Compute rust names globally across all sections
        self.compute_rust_names_globally(&mut analyzed_sections);

        analyzed_sections
    }

    /// Compute rust names across all sections, ensuring global uniqueness
    fn compute_rust_names_globally(&mut self, sections: &mut [AnalyzedSection]) {
        // Track instruction names globally across all sections
        let mut instruction_names: HashMap<String, usize> = HashMap::new();

        for section in sections {
            for instr in &mut section.instructions {
                // Compute instruction struct name from opcode and modifiers
                let base_name = self.compute_struct_name_from_head(&instr.head);

                // Ensure uniqueness by tracking name usage globally
                let count = instruction_names.entry(base_name.clone()).or_insert(0);
                instr.rust_name = if *count == 0 {
                    base_name.clone()
                } else {
                    format!("{}{}", base_name, count)
                };
                *count += 1;

                // Track field names within this instruction scope
                let mut field_names: HashMap<String, usize> = HashMap::new();

                // Process modifiers
                for (modifier, rust_name) in &mut instr.head.modifiers {
                    let base = self.compute_modifier_base_name(modifier);
                    let sanitized = naming::sanitize_field_name(&base);
                    let count = field_names.entry(sanitized.clone()).or_insert(0);
                    *rust_name = if *count == 0 {
                        sanitized
                    } else {
                        format!("{}{}", sanitized, count)
                    };
                    *count += 1;

                    // Recursively compute names within the modifier
                    self.compute_modifier_rust_names(modifier, &mut field_names);
                }

                // Process operands
                for operand in &mut instr.operands {
                    // Operator prefix
                    if let Some((_op, rust_name)) = &mut operand.operator {
                        let base = format!(
                            "{}_op",
                            self.compute_operand_element_base_name(&operand.operand)
                        );
                        let sanitized = self.sanitize_field_name(&base);
                        let count = field_names.entry(sanitized.clone()).or_insert(0);
                        *rust_name = if *count == 0 {
                            sanitized
                        } else {
                            format!("{}{}", sanitized, count)
                        };
                        *count += 1;
                    }

                    // Operand element
                    self.compute_operand_element_rust_names(&mut operand.operand, &mut field_names);

                    // Operand modifier
                    if let Some((modifier, rust_name)) = &mut operand.modifier {
                        let base = self.compute_modifier_base_name(modifier);
                        let sanitized = self.sanitize_field_name(&base);
                        let count = field_names.entry(sanitized.clone()).or_insert(0);
                        *rust_name = if *count == 0 {
                            sanitized
                        } else {
                            format!("{}{}", sanitized, count)
                        };
                        *count += 1;

                        self.compute_modifier_rust_names(modifier, &mut field_names);
                    }
                }
            }
        }
    }

    pub fn analyze(&mut self, section: &Section) -> AnalyzedSection {
        // First pass: collect all parameter rules
        for rule in &section.rules {
            if let Rule::Parameter(param) = rule {
                self.parameters
                    .insert(param.name.clone(), param.choices.clone());
            }
        }

        // Second pass: process instructions
        let instructions: Vec<AnalyzedInstruction> = section
            .rules
            .iter()
            .filter_map(|rule| {
                if let Rule::Instruction(instr) = rule {
                    Some(self.analyze_instruction(instr))
                } else {
                    None
                }
            })
            .collect();

        let mut expanded = Vec::new();
        for instr in instructions {
            if let Some(choices) = self.parameters.get(&instr.head.opcode) {
                let mut emitted = false;
                for choice in choices {
                    if let Modifier::Atom(choice_ident) = choice {
                        let mut cloned = instr.clone();
                        cloned.head.opcode = choice_ident.clone();
                        expanded.push(cloned);
                        emitted = true;
                    }
                }
                if !emitted {
                    expanded.push(instr);
                }
            } else {
                expanded.push(instr);
            }
        }

        let opcodes = expanded
            .iter()
            .map(|instr| instr.head.opcode.clone())
            .collect();

        AnalyzedSection {
            instructions: expanded,
            opcodes,
        }
    }

    fn analyze_instruction(&self, instr: &InstructionRule) -> AnalyzedInstruction {
        AnalyzedInstruction {
            rust_name: "".to_string(),
            head: AnalyzedInstructionHead {
                opcode: instr.head.opcode.clone(),
                modifiers: instr
                    .head
                    .modifiers
                    .iter()
                    .map(|m| (self.analyze_modifier(m), "".to_string()))
                    .collect(),
            },
            operands: instr
                .operands
                .iter()
                .map(|op| self.analyze_operand(op))
                .collect(),
            raw: instr.raw.clone(),
        }
    }

    fn analyze_operand(&self, operand: &Operand) -> AnalyzedOperand {
        AnalyzedOperand {
            operator: operand
                .operator
                .as_ref()
                .map(|x| (x.clone(), "".to_string())),
            operand: self.analyze_operand_element(&operand.operand),
            modifier: operand
                .modifier
                .as_ref()
                .map(|m| (self.analyze_modifier(m), "".to_string())),
        }
    }

    fn analyze_operand_element(&self, element: &OperandElement) -> AnalyzedOperandElement {
        use OperandElement::*;
        match element {
            Item(ident) => {
                // Check if this identifier has a parameter rule
                if let Some(choices) = self.parameters.get(ident) {
                    // Extract identifier tokens from the choices and create variant names
                    let mut options: Vec<(IdentifierToken, RustName)> = choices
                        .iter()
                        .filter_map(|choice| match choice {
                            Modifier::Atom(id) => Some((id.clone(), "".to_string())),
                            _ => None,
                        })
                        .collect();

                    if !options.is_empty() {
                        // Sort options by length (descending) to try longest matches first
                        options.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

                        AnalyzedOperandElement::Choice {
                            base: (ident.clone(), "".to_string()),
                            options,
                        }
                    } else {
                        AnalyzedOperandElement::Item((ident.clone(), "".to_string()))
                    }
                } else {
                    AnalyzedOperandElement::Item((ident.clone(), "".to_string()))
                }
            }
            Address(ident) => AnalyzedOperandElement::Address((ident.clone(), "".to_string())),
            Optional(ident) => AnalyzedOperandElement::Optional((ident.clone(), "".to_string())),
            ParenthesizedOperand(ident) => {
                AnalyzedOperandElement::ParenthesizedOperand((ident.clone(), "".to_string()))
            }
            PipeChoice(pair) => AnalyzedOperandElement::PipeChoice((
                (pair.0.clone(), "".to_string()),
                (pair.1.clone(), "".to_string()),
            )),
            PipeOptionalChoice(pair) => AnalyzedOperandElement::PipeOptionalChoice((
                (pair.0.clone(), "".to_string()),
                (pair.1.clone(), "".to_string()),
            )),
            CurlyGroup(operands) => {
                // CurlyGroup can contain simple items or nested structures
                let items: Vec<(IdentifierToken, RustName, bool)> = operands
                    .iter()
                    .enumerate()
                    .map(|(idx, op)| match &op.operand {
                        Item(ident) => (ident.clone(), "".to_string(), false),
                        Optional(ident) => (ident.clone(), "".to_string(), true),
                        _ => (format!("elem{}", idx), "".to_string(), false),
                    })
                    .collect();
                AnalyzedOperandElement::CurlyGroup(
                    items
                        .into_iter()
                        .map(|(ident, rust, _)| (ident, rust))
                        .collect(),
                )
            }
            SquareGroup(operands) => {
                // SquareGroup can contain simple items or nested structures like {b,}
                let items: Vec<(IdentifierToken, RustName, bool)> = operands
                    .iter()
                    .enumerate()
                    .map(|(idx, op)| match &op.operand {
                        Item(ident) => (ident.clone(), "".to_string(), false),
                        Optional(ident) => (ident.clone(), "".to_string(), true),
                        _ => (format!("elem{}", idx), "".to_string(), false),
                    })
                    .collect();
                AnalyzedOperandElement::SquareGroup(items)
            }
            ParamList => AnalyzedOperandElement::ParamList("".to_string()),
            ImmediateNumber(num) => {
                AnalyzedOperandElement::ImmediateNumber((num.clone(), "".to_string()))
            }
        }
    }

    fn analyze_modifier(&self, modifier: &Modifier) -> AnalyzedModifier {
        match modifier {
            Modifier::Atom(ident) => {
                // Check if this identifier has a parameter rule
                if let Some(choices) = self.parameters.get(ident) {
                    // Create a Choice with recursively processed options
                    let mut options: Vec<_> = choices
                        .iter()
                        .map(|choice| (self.analyze_modifier(choice), "".to_string()))
                        .collect();

                    // Sort by text length (descending) to try longest matches first
                    // This prevents ".s16" from matching before ".s16x2"
                    options.sort_by(|a, b| {
                        let len_a = modifier_text_length(&a.0);
                        let len_b = modifier_text_length(&b.0);
                        len_b.cmp(&len_a) // Descending order
                    });

                    AnalyzedModifier::Choice {
                        base: (ident.clone(), "".to_string()),
                        options,
                    }
                } else {
                    AnalyzedModifier::Atom((ident.clone(), "".to_string()))
                }
            }
            Modifier::Sequence(items) => {
                // Recursively process each identifier in the sequence
                AnalyzedModifier::Sequence(
                    items
                        .iter()
                        .map(|ident| {
                            // Check if this identifier has a parameter rule
                            if let Some(choices) = self.parameters.get(ident) {
                                let mut options: Vec<_> = choices
                                    .iter()
                                    .map(|choice| (self.analyze_modifier(choice), "".to_string()))
                                    .collect();

                                // Sort by text length (descending)
                                options.sort_by(|a, b| {
                                    let len_a = modifier_text_length(&a.0);
                                    let len_b = modifier_text_length(&b.0);
                                    len_b.cmp(&len_a)
                                });

                                (
                                    AnalyzedModifier::Choice {
                                        base: (ident.clone(), "".to_string()),
                                        options,
                                    },
                                    "".to_string(),
                                )
                            } else {
                                (
                                    AnalyzedModifier::Atom((ident.clone(), "".to_string())),
                                    "".to_string(),
                                )
                            }
                        })
                        .collect(),
                )
            }
            Modifier::Optional(ident) => {
                // Recursively check the identifier inside
                let inner = if let Some(choices) = self.parameters.get(ident) {
                    let mut options: Vec<_> = choices
                        .iter()
                        .map(|choice| (self.analyze_modifier(choice), "".to_string()))
                        .collect();

                    // Sort by text length (descending)
                    options.sort_by(|a, b| {
                        let len_a = modifier_text_length(&a.0);
                        let len_b = modifier_text_length(&b.0);
                        len_b.cmp(&len_a)
                    });

                    AnalyzedModifier::Choice {
                        base: (ident.clone(), "".to_string()),
                        options,
                    }
                } else {
                    AnalyzedModifier::Atom((ident.clone(), "".to_string()))
                };
                AnalyzedModifier::Optional(Box::new((inner, "".to_string())))
            }
        }
    }
}

impl Analyzer {
    fn compute_modifier_rust_names(
        &self,
        modifier: &mut AnalyzedModifier,
        field_names: &mut HashMap<String, usize>,
    ) {
        match modifier {
            AnalyzedModifier::Atom((ident, rust_name)) => {
                let sanitized = self.sanitize_field_name(ident);
                let count = field_names.entry(sanitized.clone()).or_insert(0);
                *rust_name = if *count == 0 {
                    sanitized
                } else {
                    format!("{}{}", sanitized, count)
                };
                *count += 1;
            }
            AnalyzedModifier::Sequence(items) => {
                for (item, rust_name) in items {
                    let base = self.compute_modifier_base_name(item);
                    let sanitized = self.sanitize_field_name(&base);
                    let count = field_names.entry(sanitized.clone()).or_insert(0);
                    *rust_name = if *count == 0 {
                        sanitized
                    } else {
                        format!("{}{}", sanitized, count)
                    };
                    *count += 1;

                    self.compute_modifier_rust_names(item, field_names);
                }
            }
            AnalyzedModifier::Optional(inner) => {
                let (inner_mod, rust_name) = inner.as_mut();
                let base = self.compute_modifier_base_name(inner_mod);
                let sanitized = self.sanitize_field_name(&base);
                let count = field_names.entry(sanitized.clone()).or_insert(0);
                *rust_name = if *count == 0 {
                    sanitized
                } else {
                    format!("{}{}", sanitized, count)
                };
                *count += 1;

                self.compute_modifier_rust_names(inner_mod, field_names);
            }
            AnalyzedModifier::Choice { base, options } => {
                let (base_ident, base_rust_name) = base;
                // Enum names should be PascalCase
                let sanitized = naming::sanitize_enum_name(base_ident);
                *base_rust_name = sanitized;

                // Variant names within enum scope
                let mut variant_names: HashMap<String, usize> = HashMap::new();
                for (option, variant_name) in options {
                    let base = self.compute_modifier_base_name(option);
                    let sanitized = self.sanitize_variant_name(&base);
                    let count = variant_names.entry(sanitized.clone()).or_insert(0);
                    *variant_name = if *count == 0 {
                        sanitized
                    } else {
                        format!("{}{}", sanitized, count)
                    };
                    *count += 1;

                    // Recursively compute names for nested modifiers within the variant
                    self.compute_modifier_rust_names(option, &mut variant_names);
                }
            }
        }
    }

    fn compute_operand_element_rust_names(
        &self,
        element: &mut AnalyzedOperandElement,
        field_names: &mut HashMap<String, usize>,
    ) {
        match element {
            AnalyzedOperandElement::Item((ident, rust_name))
            | AnalyzedOperandElement::Address((ident, rust_name))
            | AnalyzedOperandElement::Optional((ident, rust_name))
            | AnalyzedOperandElement::ParenthesizedOperand((ident, rust_name)) => {
                let sanitized = self.sanitize_field_name(ident);
                let count = field_names.entry(sanitized.clone()).or_insert(0);
                *rust_name = if *count == 0 {
                    sanitized
                } else {
                    format!("{}{}", sanitized, count)
                };
                *count += 1;
            }
            AnalyzedOperandElement::PipeChoice(((first, first_name), (second, second_name)))
            | AnalyzedOperandElement::PipeOptionalChoice((
                (first, first_name),
                (second, second_name),
            )) => {
                for (ident, rust_name) in [(first, first_name), (second, second_name)] {
                    let sanitized = self.sanitize_field_name(ident);
                    let count = field_names.entry(sanitized.clone()).or_insert(0);
                    *rust_name = if *count == 0 {
                        sanitized
                    } else {
                        format!("{}{}", sanitized, count)
                    };
                    *count += 1;
                }
            }
            AnalyzedOperandElement::CurlyGroup(items) => {
                for (ident, rust_name) in items {
                    let sanitized = self.sanitize_field_name(ident);
                    let count = field_names.entry(sanitized.clone()).or_insert(0);
                    *rust_name = if *count == 0 {
                        sanitized
                    } else {
                        format!("{}{}", sanitized, count)
                    };
                    *count += 1;
                }
            }
            AnalyzedOperandElement::SquareGroup(items) => {
                for (ident, rust_name, _) in items {
                    let sanitized = self.sanitize_field_name(ident);
                    let count = field_names.entry(sanitized.clone()).or_insert(0);
                    *rust_name = if *count == 0 {
                        sanitized
                    } else {
                        format!("{}{}", sanitized, count)
                    };
                    *count += 1;
                }
            }
            AnalyzedOperandElement::ImmediateNumber((num, rust_name)) => {
                let base = format!("imm_{}", num.replace("0x", "").replace("-", "neg"));
                let sanitized = self.sanitize_field_name(&base);
                let count = field_names.entry(sanitized.clone()).or_insert(0);
                *rust_name = if *count == 0 {
                    sanitized
                } else {
                    format!("{}{}", sanitized, count)
                };
                *count += 1;
            }
            AnalyzedOperandElement::Choice { base, options } => {
                let (base_ident, base_rust_name) = base;
                // Enum names should be PascalCase
                let sanitized = self.sanitize_enum_name(base_ident);
                let count = field_names.entry(sanitized.clone()).or_insert(0);
                *base_rust_name = if *count == 0 {
                    sanitized
                } else {
                    format!("{}{}", sanitized, count)
                };
                *count += 1;

                // Compute variant names for each option
                let mut variant_names: HashMap<String, usize> = HashMap::new();
                for (option_ident, variant_name) in options {
                    let sanitized = self.sanitize_variant_name(option_ident);
                    let count = variant_names.entry(sanitized.clone()).or_insert(0);
                    *variant_name = if *count == 0 {
                        sanitized
                    } else {
                        format!("{}{}", sanitized, count)
                    };
                    *count += 1;
                }
            }
            AnalyzedOperandElement::ParamList(rust_name) => {
                // ParamList always gets the name "param_list"
                let sanitized = self.sanitize_field_name("param_list");
                let count = field_names.entry(sanitized.clone()).or_insert(0);
                *rust_name = if *count == 0 {
                    sanitized
                } else {
                    format!("{}{}", sanitized, count)
                };
                *count += 1;
            }
        }
    }

    fn compute_modifier_base_name(&self, modifier: &AnalyzedModifier) -> String {
        match modifier {
            AnalyzedModifier::Atom((ident, _)) => ident.clone(),
            AnalyzedModifier::Choice { base, .. } => base.0.clone(),
            AnalyzedModifier::Sequence(items) => items
                .iter()
                .map(|(item, _)| self.compute_modifier_base_name(item))
                .collect::<Vec<_>>()
                .join("_"),
            AnalyzedModifier::Optional(inner) => self.compute_modifier_base_name(&inner.0),
        }
    }

    fn compute_operand_element_base_name(&self, element: &AnalyzedOperandElement) -> String {
        match element {
            AnalyzedOperandElement::Item((ident, _))
            | AnalyzedOperandElement::Address((ident, _))
            | AnalyzedOperandElement::Optional((ident, _))
            | AnalyzedOperandElement::ParenthesizedOperand((ident, _)) => ident.clone(),
            AnalyzedOperandElement::PipeChoice(((first, _), _))
            | AnalyzedOperandElement::PipeOptionalChoice(((first, _), _)) => first.clone(),
            AnalyzedOperandElement::CurlyGroup(items) => items
                .iter()
                .map(|(ident, _)| ident.as_str())
                .collect::<Vec<_>>()
                .join("_"),
            AnalyzedOperandElement::SquareGroup(items) => items
                .iter()
                .map(|(ident, _, _)| ident.as_str())
                .collect::<Vec<_>>()
                .join("_"),
            AnalyzedOperandElement::ParamList(_) => "param_list".to_string(),
            AnalyzedOperandElement::ImmediateNumber((num, _)) => {
                format!("imm_{}", num.replace("0x", "").replace("-", "neg"))
            }
            AnalyzedOperandElement::Choice { base, .. } => base.0.clone(),
        }
    }

    fn sanitize_field_name(&self, s: &str) -> String {
        naming::sanitize_field_name(s)
    }

    fn sanitize_variant_name(&self, s: &str) -> String {
        naming::sanitize_variant_name(s)
    }

    fn sanitize_struct_name(&self, s: &str) -> String {
        naming::sanitize_struct_name(s)
    }

    fn sanitize_enum_name(&self, s: &str) -> String {
        naming::sanitize_enum_name(s)
    }

    /// Generate a unique struct name from the instruction head (opcode + modifiers)
    /// For example: opcode="abs", modifiers=[Atom(".type")] -> "AbsType"
    ///              opcode="abs", modifiers=[Optional(".ftz"), Atom(".f32")] -> "AbsFtzF32"
    fn compute_struct_name_from_head(&self, head: &AnalyzedInstructionHead) -> String {
        let mut parts = vec![head.opcode.clone()];

        // Collect all modifier atoms
        for (modifier, _rust_name) in &head.modifiers {
            self.collect_modifier_parts(modifier, &mut parts);
        }

        // Join all parts and sanitize
        let combined = parts.join("_");
        let cleaned = combined
            .replace('.', "_")
            .replace("::", "_")
            .replace('-', "_")
            .replace('{', "")
            .replace('}', "");

        self.sanitize_struct_name(&cleaned)
    }

    /// Recursively collect identifier parts from modifiers for struct naming
    fn collect_modifier_parts(&self, modifier: &AnalyzedModifier, parts: &mut Vec<String>) {
        match modifier {
            AnalyzedModifier::Atom((ident, _)) => {
                parts.push(ident.clone());
            }
            AnalyzedModifier::Sequence(seq) => {
                for (m, _) in seq {
                    self.collect_modifier_parts(m, parts);
                }
            }
            AnalyzedModifier::Optional(boxed) => {
                let (m, _) = &**boxed;
                self.collect_modifier_parts(m, parts);
            }
            AnalyzedModifier::Choice { base, options: _ } => {
                let (base_ident, _) = base;
                parts.push(base_ident.clone());
                // Note: we don't include option variants in the base struct name
                // as they're already represented by the Choice enum
            }
        }
    }
}
