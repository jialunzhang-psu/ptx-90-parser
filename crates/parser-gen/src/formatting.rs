use tree_sitter::format_sexp;

// ============================================================================
// Display implementations for AST types
// ============================================================================

use crate::r#type::*;
use std::fmt;

fn join_display<T: fmt::Display>(items: &[T], sep: &str) -> String {
    let mut out = String::new();
    for (idx, item) in items.iter().enumerate() {
        if idx > 0 {
            out.push_str(sep);
        }
        fmt::write(&mut out, format_args!("{}", item)).unwrap();
    }
    out
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Modifier::Atom(id) => write!(f, "{id}"),
            Modifier::Sequence(items) => write!(f, "Sequence({})", items.join(", ")),
            Modifier::Optional(id) => write!(f, "Optional({id})"),
        }
    }
}

impl fmt::Display for OperandElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperandElement::Item(ident) => write!(f, "{ident}"),
            OperandElement::CurlyGroup(items) => write!(f, "{{{}}}", join_display(items, ", ")),
            OperandElement::SquareGroup(items) => write!(f, "[{}]", join_display(items, ", ")),
            OperandElement::ParamList => write!(f, "(param-list)"),
            OperandElement::ParenthesizedOperand(id) => write!(f, "({id})"),
            OperandElement::PipeChoice((a, b)) => write!(f, "{}|{}", a, b),
            OperandElement::PipeOptionalChoice((a, b)) => write!(f, "{}{{|{}}}", a, b),
            OperandElement::Optional(id) => write!(f, "{{?{id}}}"),
            OperandElement::Address(id) => write!(f, "[{id}]"),
            OperandElement::ImmediateNumber(num) => write!(f, "{}", num),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(op) = &self.operator {
            match op {
                OperatorToken::Negate => write!(f, "-")?,
                OperatorToken::LogicalNot => write!(f, "!")?,
            }
        }
        write!(f, "{}", self.operand)?;
        if let Some(modifier) = &self.modifier {
            write!(f, " {{mod: {modifier}}}")?;
        }
        Ok(())
    }
}

impl fmt::Display for InstructionHead {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "opcode: {}, modifiers:[{}]",
            self.opcode,
            join_display(&self.modifiers, ", ")
        )
    }
}

impl fmt::Display for InstructionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Instruction{{opcode: {}, modifiers:[{}], operands:[{}]}}",
            self.head.opcode,
            join_display(&self.head.modifiers, ", "),
            join_display(&self.operands, ", ")
        )
    }
}

impl fmt::Display for ParameterRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parameter{{name: {}, choices:[{}]}}",
            self.name,
            join_display(&self.choices, ", ")
        )
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::Instruction(instr) => write!(f, "{}", instr),
            Rule::Parameter(param) => write!(f, "{}", param),
        }
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TopLevel[{}]", join_display(&self.rules, ", "))
    }
}

// ============================================================================
// Tree-sitter S-expression builders
// ============================================================================

impl Section {
    /// Render the AST as a Tree-sitter style S-expression with indentation.
    pub fn to_treesitter_pretty(&self) -> String {
        let root = build_top_level_node(self);
        Tree::new(root).to_sexp_pretty()
    }
}

fn build_top_level_node(top: &Section) -> Node {
    let mut root = Node::new("source_file");
    for rule in &top.rules {
        root.push_child(build_rule_node(rule));
    }
    root
}

fn build_rule_node(rule: &Rule) -> Node {
    match rule {
        Rule::Instruction(instr) => build_instruction_rule_node(instr),
        Rule::Parameter(param) => build_parameter_rule_node(param),
    }
}

fn build_instruction_rule_node(instr: &InstructionRule) -> Node {
    Node::new("instruction_rule")
        .with_child(build_instruction_head_node(&instr.head))
        .with_child(build_operands_node(&instr.operands))
}

fn build_parameter_rule_node(param: &ParameterRule) -> Node {
    let mut node = Node::new("parameter_rule");
    node.push_child(build_identifier_node("name", &param.name));

    let mut choices = Node::new("choices");
    for modifier in &param.choices {
        choices.push_child(build_modifier_node(modifier));
    }
    node.push_child(choices);
    node
}

fn build_instruction_head_node(head: &InstructionHead) -> Node {
    let mut node = Node::new("instruction_head");
    node.push_child(build_identifier_node("opcode", &head.opcode));

    let mut modifiers = Node::new("modifiers");
    for modifier in &head.modifiers {
        modifiers.push_child(build_modifier_node(modifier));
    }
    node.push_child(modifiers);
    node
}

fn build_modifier_node(modifier: &Modifier) -> Node {
    match modifier {
        Modifier::Atom(ident) => build_identifier_node("modifier_atom", ident),
        Modifier::Sequence(items) => {
            let mut node = Node::new("modifier_sequence");
            for item in items {
                node.push_child(build_identifier_node("modifier_atom", item));
            }
            node
        }
        Modifier::Optional(id) => {
            let mut node = Node::new("modifier_optional");
            node.push_child(build_identifier_node("modifier_atom", id));
            node
        }
    }
}

fn build_operands_node(operands: &[Operand]) -> Node {
    let mut node = Node::new("operands");
    for operand in operands {
        node.push_child(build_operand_node(operand));
    }
    node
}

fn build_operand_node(operand: &Operand) -> Node {
    let mut node = Node::new("operand");
    if let Some(op_node) = build_operator_node(&operand.operator) {
        node.push_child(op_node);
    }
    node.push_child(build_operand_element_node(&operand.operand));
    if let Some(modifier) = &operand.modifier {
        node.push_child(build_modifier_node(modifier));
    }
    node
}

fn build_operator_node(operator: &Option<OperatorToken>) -> Option<Node> {
    match operator {
        Some(OperatorToken::LogicalNot) => Some(Node::leaf("operator_token", "!")),
        Some(OperatorToken::Negate) => Some(Node::leaf("operator_token", "-")),
        None => None,
    }
}

fn build_operand_element_node(element: &OperandElement) -> Node {
    match element {
        OperandElement::Item(ident) => build_identifier_node("item", ident),
        OperandElement::CurlyGroup(items) => {
            let mut node = Node::new("curly_group");
            for item in items {
                node.push_child(build_operand_node(item));
            }
            node
        }
        OperandElement::SquareGroup(items) => {
            let mut node = Node::new("square_group");
            for item in items {
                node.push_child(build_operand_node(item));
            }
            node
        }
        OperandElement::ParamList => Node::new("param_list"),
        OperandElement::ParenthesizedOperand(id) => {
            let mut node = Node::new("parenthesized_operand");
            node.push_child(build_identifier_node("item", id));
            node
        }
        OperandElement::PipeChoice((a, b)) => {
            let mut node = Node::new("pipe_choice");
            node.push_child(build_identifier_node("first", a));
            node.push_child(build_identifier_node("second", b));
            node
        }
        OperandElement::PipeOptionalChoice((a, b)) => {
            let mut node = Node::new("pipe_optional_choice");
            node.push_child(build_identifier_node("first", a));
            node.push_child(build_identifier_node("second", b));
            node
        }
        OperandElement::Address(id) => {
            let mut node = Node::new("address");
            node.push_child(build_identifier_node("item", id));
            node
        }
        OperandElement::Optional(id) => {
            let mut node = Node::new("operand_optional");
            node.push_child(build_identifier_node("item", id));
            node
        }
        OperandElement::ImmediateNumber(num) => Node::leaf("immediate_number", num),
    }
}

fn build_identifier_node(label: &str, ident: &IdentifierToken) -> Node {
    Node::leaf(label, ident)
}

// ============================================================================
// S-expression formatting for debugging
// ============================================================================
#[derive(Debug, Clone)]
pub struct Node {
    kind: String,
    text: Option<String>,
    children: Vec<Node>,
}

impl Node {
    pub fn new(kind: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            text: None,
            children: Vec::new(),
        }
    }

    pub fn leaf(kind: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            text: Some(text.into()),
            children: Vec::new(),
        }
    }

    pub fn push_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn with_child(mut self, child: Node) -> Self {
        self.push_child(child);
        self
    }

    fn write_sexp(&self, out: &mut String) {
        out.push('(');
        out.push_str(&self.kind);

        if let Some(text) = &self.text {
            out.push(' ');
            write_atom(out, text);
        }

        for child in &self.children {
            out.push(' ');
            child.write_sexp(out);
        }

        out.push(')');
    }
}

pub struct Tree {
    root: Node,
}

impl Tree {
    pub fn new(root: Node) -> Self {
        Self { root }
    }

    pub fn to_sexp(&self) -> String {
        let mut buf = String::new();
        self.root.write_sexp(&mut buf);
        buf
    }

    pub fn to_sexp_pretty(&self) -> String {
        format_sexp(&self.to_sexp(), 0)
    }
}

fn write_atom(out: &mut String, text: &str) {
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out.push('"');
}
