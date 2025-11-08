// ============================================================================
// PTX Specification Parser
// ============================================================================
//
// This module parses PTX instruction specification strings into structured AST nodes.
//
// ## Overview
//
// PTX specifications are text files that define instruction syntax patterns.
// They consist of:
// - **Sections**: Separated by lines of 5+ dashes (-----)
// - **Rules**: Statements ending with semicolons (;)
// - **Parameter Rules**: Define parameter sets (e.g., `.type = { .u32, .s32 }`)
// - **Instruction Rules**: Define instruction patterns (e.g., `add.type d, a, b`)
//
// ## Parsing Pipeline
//
// 1. **Section Splitting**: Split input by separator lines (-----)
// 2. **Rule Splitting**: Split each section by semicolons (;)
// 3. **Rule Classification**: Determine if rule is parameter or instruction (by '=')
// 4. **Identifier Collection**: Gather all parameter names for context-aware parsing
// 5. **Parsing**: Parse each rule into structured AST nodes
//
// ## Example
//
// ```ptx
// add.type d, a, b;
// .type = { .u32, .s32, .f32 };
// ```
//
// This parses into:
// - InstructionRule: `add.type d, a, b`
// - ParameterRule: `.type` with choices `[.u32, .s32, .f32]`

use crate::lexer::{PtxSpecToken, Span, token_to_string, tokenize};
use crate::r#type::{
    InstructionHead, InstructionRule, Modifier, Operand, OperandElement, OperatorToken,
    ParameterRule, Rule, Section,
};
use std::collections::HashSet;
use std::fmt;

// ============================================================================
// Error Types
// ============================================================================

/// Error type for specification parsing failures
#[derive(Debug, Clone)]
pub struct SpecParseError {
    pub message: String,
    pub span: Option<Span>,
}

impl fmt::Display for SpecParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(span) = &self.span {
            write!(f, "{} at {:?}", self.message, span)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for SpecParseError {}

// ============================================================================
// Public API
// ============================================================================

/// Parse a PTX specification string into a list of Section AST nodes.
///
/// # Arguments
/// * `source` - The PTX specification source code
///
/// # Returns
/// * `Ok(Vec<Section>)` - Parsed sections containing rules
/// * `Err(SpecParseError)` - Parsing error with message and optional span
///
/// # Example
/// ```ignore
/// let spec = "add.type d, a, b;\n.type = { .u32, .s32 };";
/// let sections = parse_spec(spec)?;
/// ```
pub fn parse_spec(source: &str) -> Result<Vec<Section>, SpecParseError> {
    parse_spec_with_name(source, "<input>")
}

/// Parse a PTX specification string with a file name for error reporting.
///
/// # Arguments
/// * `source` - The PTX specification source code
/// * `_name` - The file name (currently unused, reserved for future error reporting)
pub fn parse_spec_with_name(source: &str, _name: &str) -> Result<Vec<Section>, SpecParseError> {
    // Step 1: Split input into sections using "-----" as separator
    let sections = split_into_sections(source);

    let mut top_levels = Vec::new();

    for section in sections {
        if section.trim().is_empty() {
            continue;
        }

        // Step 2: Split each section into rules using ";" as separator
        let rule_strings = split_into_rules(&section)?;

        if rule_strings.is_empty() {
            continue;
        }

        // Step 3: Determine whether each rule is parameterlist or instruction by checking "="
        let classified_rules = classify_rules(&rule_strings)?;

        // Step 4: For parameterlist, collect all the LHS identifiers
        let identifiers = collect_parameter_identifiers(&classified_rules);

        // Step 5: Parse all the RHS, knowing how to punctuate with the set of identifiers
        let mut rules = Vec::new();

        for classified_rule in classified_rules {
            match classified_rule {
                ClassifiedRule::Parameter { lhs_list, rhs } => {
                    // Parse the RHS choices, excluding the LHS identifiers to avoid self-recursion
                    let choices = parse_parameter_rhs(&rhs, &identifiers, &lhs_list)?;

                    // Create a ParameterRule for each LHS
                    for lhs in lhs_list {
                        rules.push(Rule::Parameter(ParameterRule {
                            name: lhs,
                            choices: choices.clone(),
                        }));
                    }
                }
                ClassifiedRule::Instruction { text } => {
                    // Step 6: Parse instructions by first parsing all the modifiers one by one,
                    // then all the operands one by one
                    let instruction = parse_instruction(&text, &identifiers)?;
                    rules.push(Rule::Instruction(instruction));
                }
            }
        }

        top_levels.push(Section { rules });
    }

    if top_levels.is_empty() {
        top_levels.push(Section { rules: Vec::new() });
    }

    Ok(top_levels)
}

// ============================================================================
// Section Splitting (Step 1)
// ============================================================================
//
// Split the input source into sections using lines of 5+ dashes as separators.
// Each section can contain multiple rules.
//
// Example:
// ```
// add.type d, a, b;
// .type = { .u32, .s32 };
// -----
// sub.type d, a, b;
// ```
// Results in 2 sections.

fn split_into_sections(source: &str) -> Vec<String> {
    let mut sections = Vec::new();
    let mut current_section = String::new();

    for line in source.lines() {
        // Check if this line is a separator (5+ dashes, possibly with whitespace)
        let trimmed = line.trim();
        if trimmed.len() >= 5 && trimmed.chars().all(|c| c == '-') {
            // This is a separator line
            if !current_section.is_empty() {
                sections.push(current_section);
                current_section = String::new();
            }
        } else {
            // Regular line, add to current section
            if !current_section.is_empty() {
                current_section.push('\n');
            }
            current_section.push_str(line);
        }
    }

    // Don't forget the last section
    if !current_section.is_empty() {
        sections.push(current_section);
    }

    sections
}

// ============================================================================
// Rule Splitting (Step 2)
// ============================================================================
//
// Split a section into individual rules using semicolons as separators.
// Tokenizes the section first to properly handle semicolons in context.
//
// Example:
// ```
// add.type d, a, b; sub.type d, a, b;
// ```
// Results in 2 rules: ["add.type d, a, b", "sub.type d, a, b"]

fn split_into_rules(section: &str) -> Result<Vec<String>, SpecParseError> {
    let tokens = tokenize(section).map_err(|e| SpecParseError {
        message: format!("Lexer error: {:?}", e),
        span: Some(e.span),
    })?;

    let mut rules = Vec::new();
    let mut current_rule = Vec::new();

    for (token, span) in tokens {
        if matches!(token, PtxSpecToken::Semicolon) {
            if !current_rule.is_empty() {
                rules.push(current_rule);
                current_rule = Vec::new();
            }
        } else {
            current_rule.push((token, span));
        }
    }

    // Don't forget the last rule if it doesn't end with semicolon
    if !current_rule.is_empty() {
        rules.push(current_rule);
    }

    Ok(rules
        .into_iter()
        .map(|tokens| {
            tokens
                .iter()
                .map(|(t, _)| token_to_string(t))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect())
}

// ============================================================================
// Step 3: Classify rules
// ============================================================================

enum ClassifiedRule {
    Parameter { lhs_list: Vec<String>, rhs: String },
    Instruction { text: String },
}

fn classify_rules(rule_strings: &[String]) -> Result<Vec<ClassifiedRule>, SpecParseError> {
    let mut classified = Vec::new();

    for rule_str in rule_strings {
        if rule_str.trim().is_empty() {
            continue;
        }

        // Check if it contains "=" to determine if it's a parameter rule
        if rule_str.contains('=') {
            // It's a parameter rule, re-tokenize to get proper names without extra spaces
            let tokens = tokenize(rule_str).map_err(|e| SpecParseError {
                message: format!("Lexer error: {:?}", e),
                span: Some(e.span),
            })?;

            let mut lhs_list = Vec::new();
            let mut rhs_start = 0;

            // Parse LHS names: collect tokens before '='
            // Need to handle patterns like:
            // - .directive = ...
            // - ::identifier = ...
            // - .directive::identifier = ...
            let mut i = 0;
            while i < tokens.len() {
                // Find the next '='
                let mut eq_pos = i;
                while eq_pos < tokens.len() && !matches!(tokens[eq_pos].0, PtxSpecToken::Equals) {
                    eq_pos += 1;
                }

                if eq_pos >= tokens.len() {
                    break; // No more '=' found
                }

                // Collect tokens from i to eq_pos-1 as a single LHS name
                if eq_pos > i {
                    let mut name = String::new();
                    for j in i..eq_pos {
                        name.push_str(&token_to_string(&tokens[j].0));
                    }
                    lhs_list.push(name);
                    i = eq_pos + 1; // Skip past the '='
                    rhs_start = i;
                } else {
                    i += 1;
                }
            }

            // RHS is everything after the last '='
            let rhs: String = tokens[rhs_start..]
                .iter()
                .map(|(t, _)| token_to_string(t))
                .collect::<Vec<_>>()
                .join(" ");

            if lhs_list.is_empty() {
                return Err(SpecParseError {
                    message: format!("No LHS found in parameter rule: {}", rule_str),
                    span: None,
                });
            }

            classified.push(ClassifiedRule::Parameter { lhs_list, rhs });
        } else {
            // It's an instruction rule
            classified.push(ClassifiedRule::Instruction {
                text: rule_str.clone(),
            });
        }
    }

    Ok(classified)
}

// ============================================================================
// Step 4: Collect parameter identifiers
// ============================================================================

fn collect_parameter_identifiers(classified_rules: &[ClassifiedRule]) -> HashSet<String> {
    let mut identifiers = HashSet::new();

    for rule in classified_rules {
        if let ClassifiedRule::Parameter { lhs_list, .. } = rule {
            for lhs in lhs_list {
                identifiers.insert(lhs.clone());
            }
        }
    }

    identifiers
}

// ============================================================================
// Step 5: Parse parameter RHS
// ============================================================================

fn parse_parameter_rhs(
    rhs: &str,
    identifiers: &HashSet<String>,
    exclude_lhs: &[String],
) -> Result<Vec<Modifier>, SpecParseError> {
    let tokens = tokenize(rhs).map_err(|e| SpecParseError {
        message: format!("Lexer error in RHS: {:?}", e),
        span: Some(e.span),
    })?;

    // Create a filtered identifier set that excludes the LHS to avoid self-recursion
    let mut filtered_identifiers = identifiers.clone();
    for lhs in exclude_lhs {
        filtered_identifiers.remove(lhs);
    }

    let mut parser = TokenParser::new(tokens, &filtered_identifiers);

    // RHS should be: { modifier, modifier, ... }
    parser.expect_token(&PtxSpecToken::LBrace)?;

    let mut choices = Vec::new();

    loop {
        if parser.peek_is(&PtxSpecToken::RBrace) {
            parser.advance();
            break;
        }

        let modifier = parser.parse_parameter_choice()?;
        choices.push(modifier);

        if parser.peek_is(&PtxSpecToken::Comma) {
            parser.advance();
        } else if parser.peek_is(&PtxSpecToken::RBrace) {
            parser.advance();
            break;
        } else {
            return Err(SpecParseError {
                message: format!("Expected ',' or '}}' in parameter RHS"),
                span: parser.current_span(),
            });
        }
    }

    Ok(choices)
}

// ============================================================================
// Step 6: Parse instruction
// ============================================================================

fn parse_instruction(
    text: &str,
    identifiers: &HashSet<String>,
) -> Result<InstructionRule, SpecParseError> {
    let tokens = tokenize(text).map_err(|e| SpecParseError {
        message: format!("Lexer error in instruction: {:?}", e),
        span: Some(e.span),
    })?;

    let mut parser = TokenParser::new(tokens, identifiers);

    // Parse opcode (first identifier)
    let opcode = parser.expect_identifier()?;

    // Parse all modifiers one by one
    let mut modifiers = Vec::new();
    while parser.peek_is_modifier_start() {
        match parser.parse_modifier() {
            Ok(modifier) => modifiers.push(modifier),
            Err(_) => break, // Not a modifier, stop parsing modifiers
        }
    }

    // Parse all operands one by one
    let mut operands = Vec::new();
    while !parser.is_eof() {
        let operand = parser.parse_operand()?;
        operands.push(operand);

        // Consume optional comma
        if parser.peek_is(&PtxSpecToken::Comma) {
            parser.advance();
        }
    }

    Ok(InstructionRule {
        head: InstructionHead { opcode, modifiers },
        operands,
        raw: text.to_string(),
    })
}

// ============================================================================
// Token Parser Helper
// ============================================================================

struct TokenParser<'a> {
    tokens: Vec<(PtxSpecToken, Span)>,
    pos: usize,
    identifiers: &'a HashSet<String>,
}

impl<'a> TokenParser<'a> {
    fn new(tokens: Vec<(PtxSpecToken, Span)>, identifiers: &'a HashSet<String>) -> Self {
        Self {
            tokens,
            pos: 0,
            identifiers,
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn peek(&self) -> Option<&PtxSpecToken> {
        self.tokens.get(self.pos).map(|(t, _)| t)
    }

    fn current_span(&self) -> Option<Span> {
        self.tokens.get(self.pos).map(|(_, s)| s.clone())
    }

    fn peek_is(&self, expected: &PtxSpecToken) -> bool {
        if let Some(token) = self.peek() {
            std::mem::discriminant(token) == std::mem::discriminant(expected)
        } else {
            false
        }
    }

    fn advance(&mut self) -> Option<(PtxSpecToken, Span)> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect_token(&mut self, expected: &PtxSpecToken) -> Result<(), SpecParseError> {
        if self.peek_is(expected) {
            self.advance();
            Ok(())
        } else {
            Err(SpecParseError {
                message: format!("Expected {:?}, got {:?}", expected, self.peek()),
                span: self.current_span(),
            })
        }
    }

    fn expect_identifier(&mut self) -> Result<String, SpecParseError> {
        // Only collect a single identifier token (not chained identifiers)
        if let Some(token) = self.peek() {
            match token {
                PtxSpecToken::Identifier(s) => {
                    let result = s.clone();
                    self.advance();
                    Ok(result)
                }
                _ => Err(SpecParseError {
                    message: format!("Expected identifier, found {:?}", token),
                    span: self.current_span(),
                }),
            }
        } else {
            Err(SpecParseError {
                message: "Expected identifier, found end of input".to_string(),
                span: self.current_span(),
            })
        }
    }

    fn peek_is_modifier_start(&self) -> bool {
        if let Some(token) = self.peek() {
            matches!(token, PtxSpecToken::Dot | PtxSpecToken::LBrace)
        } else {
            false
        }
    }

    fn parse_modifier(&mut self) -> Result<Modifier, SpecParseError> {
        if self.peek_is(&PtxSpecToken::LBrace) {
            // Check if this is really an optional modifier {.something}
            // or just an operand curly group {a, b, c}
            let saved_pos = self.pos;
            self.advance(); // consume '{'

            // Check if next token is a modifier start
            if self.peek_is_modifier_start() {
                let inner = self.parse_modifier_atom_with_concatenation()?;
                self.expect_token(&PtxSpecToken::RBrace)?;

                // Extract the identifier from the inner modifier (must be Atom)
                let id = match inner {
                    Modifier::Atom(id) => id,
                    _ => panic!(
                        "Optional modifier must contain a simple Atom, got: {:?}",
                        inner
                    ),
                };

                Ok(Modifier::Optional(id))
            } else {
                // Not a modifier, restore position
                self.pos = saved_pos;
                return Err(SpecParseError {
                    message: "Not a modifier".to_string(),
                    span: self.current_span(),
                });
            }
        } else {
            self.parse_modifier_atom()
        }
    }

    fn parse_parameter_choice(&mut self) -> Result<Modifier, SpecParseError> {
        // Parse a parameter choice which might be:
        // - .collector::buffer::op where ::buffer, ::op are identifiers -> Sequence([.collector, ::buffer, ::op])
        // - .b.n.n.n.n where .n is a known identifier -> Sequence([".b", ".n", ".n", ".n", ".n"])
        // - .dtype where .dtype is an identifier -> Atom(.dtype) (parameter reference)
        // - aa::item3 where neither are identifiers -> Atom(aa::item3) (literal)
        // - 0, 1, 2 (numbers in parameter context) -> Atom("0"), Atom("1"), etc.
        //
        // Algorithm:
        // 1. Collect tokens until delimiter into a string representation
        // 2. Try to match token sequences from the identifier set
        // 3. Split into segments and return Atom or Sequence

        let start_pos = self.pos;

        // Find the end position (next delimiter)
        let mut end_pos = self.pos;
        while end_pos < self.tokens.len() {
            match &self.tokens[end_pos].0 {
                PtxSpecToken::Comma
                | PtxSpecToken::RBrace
                | PtxSpecToken::RBracket
                | PtxSpecToken::RParen => break,
                _ => end_pos += 1,
            }
        }

        if start_pos >= end_pos {
            return Err(SpecParseError {
                message: "Expected parameter choice".to_string(),
                span: self.current_span(),
            });
        }

        // Helper: convert token range to string
        let tokens_to_string = |start: usize, end: usize| -> String {
            self.tokens[start..end]
                .iter()
                .map(|(t, _)| token_to_string(t))
                .collect::<Vec<_>>()
                .join("")
        };

        // Check if entire range is a known identifier
        let full_string = tokens_to_string(start_pos, end_pos);
        if self.identifiers.contains(&full_string) {
            self.pos = end_pos;
            return Ok(Modifier::Atom(full_string));
        }

        // Try to split by matching token sequences against identifier set
        let mut segments = Vec::new();
        let mut pos = start_pos;

        while pos < end_pos {
            let mut matched = false;

            // Try longest match first
            for try_end in (pos + 1..=end_pos).rev() {
                let candidate = tokens_to_string(pos, try_end);

                if self.identifiers.contains(&candidate) {
                    segments.push(candidate);
                    pos = try_end;
                    matched = true;
                    break;
                }
            }

            if !matched {
                // Collect unmatched tokens until we find a match
                let segment_start = pos;
                pos += 1;

                while pos < end_pos {
                    let mut has_match = false;
                    for try_end in (pos + 1..=end_pos).rev() {
                        if self.identifiers.contains(&tokens_to_string(pos, try_end)) {
                            has_match = true;
                            break;
                        }
                    }
                    if has_match {
                        break;
                    }
                    pos += 1;
                }

                segments.push(tokens_to_string(segment_start, pos));
            }
        }

        self.pos = end_pos;

        if segments.is_empty() {
            Ok(Modifier::Atom(full_string))
        } else if segments.len() == 1 {
            Ok(Modifier::Atom(segments.into_iter().next().unwrap()))
        } else {
            Ok(Modifier::Sequence(segments))
        }
    }

    fn parse_modifier_atom_with_concatenation(&mut self) -> Result<Modifier, SpecParseError> {
        // Parse a modifier like .b8x16.b6x16_p32 (for instruction modifiers)
        // Collects all tokens until we hit a delimiter or operand-like token

        let start_pos = self.pos;

        // Find the end position (next delimiter or operand-like token)
        let mut end_pos = self.pos;
        while end_pos < self.tokens.len() {
            match &self.tokens[end_pos].0 {
                // Stop at delimiters
                PtxSpecToken::Comma
                | PtxSpecToken::RBrace
                | PtxSpecToken::RBracket
                | PtxSpecToken::RParen
                | PtxSpecToken::LBrace
                | PtxSpecToken::LBracket
                | PtxSpecToken::LParen
                | PtxSpecToken::Pipe
                | PtxSpecToken::Semicolon => break,
                _ => end_pos += 1,
            }
        }

        if start_pos >= end_pos {
            return Err(SpecParseError {
                message: "Expected modifier".to_string(),
                span: self.current_span(),
            });
        }

        // Convert token range to string
        let result = self.tokens[start_pos..end_pos]
            .iter()
            .map(|(t, _)| token_to_string(t))
            .collect::<Vec<_>>()
            .join("");

        self.pos = end_pos;
        Ok(Modifier::Atom(result))
    }

    fn parse_modifier_atom(&mut self) -> Result<Modifier, SpecParseError> {
        // Parse a single modifier - must start with .
        // :: is NOT a separator, it's part of the modifier content
        // Collect everything after . until we hit:
        // - Another . (next modifier)
        // - A consecutive identifier (one that comes after an identifier, not after . or ::)
        // - A delimiter (comma, brace, etc.)

        // Check if this is an immediate number (no prefix) - treat as Atom for parameter choices
        if let Some(token) = self.peek() {
            match token {
                PtxSpecToken::DecimalInteger(_) | PtxSpecToken::HexInteger(_) => {
                    if let Some((tok, _)) = self.advance() {
                        let num_str = match tok {
                            PtxSpecToken::DecimalInteger(s) => s,
                            PtxSpecToken::HexInteger(s) => s,
                            _ => unreachable!(),
                        };
                        return Ok(Modifier::Atom(num_str));
                    }
                }
                _ => {}
            }
        }

        // Must start with .
        if !self.peek_is(&PtxSpecToken::Dot) {
            return Err(SpecParseError {
                message: "Modifier must start with '.'".to_string(),
                span: self.current_span(),
            });
        }

        let mut result = String::new();
        let mut last_was_identifier; // Track if last consumed token was an identifier

        // Consume the initial dot
        result.push('.');
        self.advance();
        last_was_identifier = false;

        // Now collect everything until we hit a stopping point
        loop {
            if let Some(token) = self.peek() {
                match token {
                    PtxSpecToken::Identifier(s) => {
                        // If the last thing we consumed was an identifier, this is a consecutive identifier
                        // This means we've reached the operands, so stop here
                        if last_was_identifier {
                            break;
                        }
                        result.push_str(s);
                        self.advance();
                        last_was_identifier = true;
                    }
                    PtxSpecToken::DecimalInteger(s) => {
                        result.push_str(s);
                        self.advance();
                        last_was_identifier = false;
                    }
                    PtxSpecToken::HexInteger(s) => {
                        result.push_str(s);
                        self.advance();
                        last_was_identifier = false;
                    }
                    PtxSpecToken::DoubleColon => {
                        result.push_str("::");
                        self.advance();
                        last_was_identifier = false;
                    }
                    PtxSpecToken::Star => {
                        result.push('*');
                        self.advance();
                        last_was_identifier = false;
                    }
                    // Stop at next dot (another modifier)
                    PtxSpecToken::Dot => break,
                    // Stop at delimiters
                    _ => break,
                }
            } else {
                break;
            }
        }

        if result == "." {
            return Err(SpecParseError {
                message: "Invalid modifier: '.'".to_string(),
                span: self.current_span(),
            });
        }

        Ok(Modifier::Atom(result))
    }

    fn parse_operand(&mut self) -> Result<Operand, SpecParseError> {
        // Check for {, operand} pattern at the start (optional operand)
        if self.peek_is(&PtxSpecToken::LBrace) {
            let saved_pos = self.pos;
            self.advance();

            if self.peek_is(&PtxSpecToken::Comma) {
                // This is {, operand}, an optional operand
                self.advance(); // consume comma
                let inner_operand = self.parse_operand()?;
                self.expect_token(&PtxSpecToken::RBrace)?;

                // Extract the identifier (must be simple)
                let id = match inner_operand {
                    Operand {
                        operator: None,
                        operand: OperandElement::Item(id),
                        modifier: None,
                    } => id,
                    _ => panic!(
                        "Optional operand {{, ...}} must contain a simple identifier, got: {:?}",
                        inner_operand
                    ),
                };

                return Ok(Operand {
                    operator: None,
                    operand: OperandElement::Optional(id),
                    modifier: None,
                });
            }

            // Not {, pattern, restore position
            self.pos = saved_pos;
        }

        let mut operator = None;
        let mut modifier = None;

        // Check for optional operator prefix: {!} or {-}
        if self.peek_is(&PtxSpecToken::LBrace) {
            let saved_pos = self.pos;
            self.advance();

            if self.peek_is(&PtxSpecToken::Exclaim) {
                self.advance();
                self.expect_token(&PtxSpecToken::RBrace)?;
                operator = Some(OperatorToken::LogicalNot);
            } else if self.peek_is(&PtxSpecToken::Minus) {
                self.advance();
                self.expect_token(&PtxSpecToken::RBrace)?;
                operator = Some(OperatorToken::Negate);
            } else {
                // Not an operator, restore position
                self.pos = saved_pos;
            }
        }

        // Parse the operand element
        let mut operand = self.parse_operand_element()?;

        // Check for pipe choice without braces: d|p
        if self.peek_is(&PtxSpecToken::Pipe) {
            self.advance(); // consume |

            // Extract first identifier
            let first = match operand {
                OperandElement::Item(id) => id,
                _ => panic!(
                    "PipeChoice must have simple identifiers, got: {:?}",
                    operand
                ),
            };

            // Parse second identifier
            let second = self.expect_identifier()?;

            operand = OperandElement::PipeChoice((first, second));
        }

        // Check for optional modifier: {.something} or pipe optional choice {|p}
        if self.peek_is(&PtxSpecToken::LBrace) {
            let saved_pos = self.pos;
            self.advance();

            if self.peek_is_modifier_start() {
                let inner = self.parse_modifier_atom_with_concatenation()?;
                self.expect_token(&PtxSpecToken::RBrace)?;

                // Extract the identifier from the inner modifier (must be Atom)
                let id = match inner {
                    Modifier::Atom(id) => id,
                    _ => panic!("Operand modifier must be a simple Atom, got: {:?}", inner),
                };

                modifier = Some(Modifier::Optional(id));
            } else if self.peek_is(&PtxSpecToken::Pipe) {
                // This is d{|p}, a pipe optional choice
                self.advance(); // consume |

                // Extract first identifier
                let first = match operand {
                    OperandElement::Item(id) => id,
                    _ => panic!(
                        "PipeOptionalChoice must have simple identifiers, got: {:?}",
                        operand
                    ),
                };

                // Parse second identifier
                let second = self.expect_identifier()?;

                self.expect_token(&PtxSpecToken::RBrace)?;

                return Ok(Operand {
                    operator,
                    operand: OperandElement::PipeOptionalChoice((first, second)),
                    modifier: None,
                });
            } else {
                // Not a modifier or pipe choice, restore position
                self.pos = saved_pos;
            }
        }

        if modifier.is_none() {
            if self.peek_is(&PtxSpecToken::Dot) {
                let parsed_modifier = self.parse_modifier_atom_with_concatenation()?;
                modifier = Some(parsed_modifier);
            }
        }

        Ok(Operand {
            operator,
            operand,
            modifier,
        })
    }

    fn parse_operand_element(&mut self) -> Result<OperandElement, SpecParseError> {
        if self.peek_is(&PtxSpecToken::LBrace) {
            self.parse_curly_group()
        } else if self.peek_is(&PtxSpecToken::LBracket) {
            self.parse_square_group_or_address()
        } else if self.peek_is(&PtxSpecToken::LParen) {
            self.parse_paren_group()
        } else {
            // Check for immediate numbers
            if let Some(token) = self.peek() {
                match token {
                    PtxSpecToken::DecimalInteger(_) | PtxSpecToken::HexInteger(_) => {
                        if let Some((tok, _)) = self.advance() {
                            let num_str = match tok {
                                PtxSpecToken::DecimalInteger(s) => s,
                                PtxSpecToken::HexInteger(s) => s,
                                _ => unreachable!(),
                            };
                            return Ok(OperandElement::ImmediateNumber(num_str));
                        }
                    }
                    _ => {}
                }
            }

            // Simple identifier
            let ident = self.expect_identifier()?;
            Ok(OperandElement::Item(ident))
        }
    }

    fn parse_curly_group(&mut self) -> Result<OperandElement, SpecParseError> {
        self.expect_token(&PtxSpecToken::LBrace)?;

        let mut operands = Vec::new();

        loop {
            if self.peek_is(&PtxSpecToken::RBrace) {
                self.advance();
                break;
            }

            let operand = self.parse_operand()?;
            operands.push(operand);

            if self.peek_is(&PtxSpecToken::Comma) {
                self.advance();

                // Check if this is the pattern {operand,} (optional operand)
                if self.peek_is(&PtxSpecToken::RBrace) && operands.len() == 1 {
                    // This is {operand,} which means optional operand
                    self.advance(); // consume }

                    let inner_operand = operands.into_iter().next().unwrap();

                    // Extract the identifier (must be simple)
                    let id = match inner_operand {
                        Operand {
                            operator: None,
                            operand: OperandElement::Item(id),
                            modifier: None,
                        } => id,
                        _ => panic!(
                            "Optional operand {{...,}} must contain a simple identifier, got: {:?}",
                            inner_operand
                        ),
                    };

                    return Ok(OperandElement::Optional(id));
                }
            } else if self.peek_is(&PtxSpecToken::RBrace) {
                self.advance();
                break;
            } else {
                return Err(SpecParseError {
                    message: "Expected ',' or '}' in curly group".to_string(),
                    span: self.current_span(),
                });
            }
        }

        Ok(OperandElement::CurlyGroup(operands))
    }

    fn parse_square_group_or_address(&mut self) -> Result<OperandElement, SpecParseError> {
        self.expect_token(&PtxSpecToken::LBracket)?;

        if self.peek_is(&PtxSpecToken::RBracket) {
            self.advance();
            return Ok(OperandElement::SquareGroup(Vec::new()));
        }

        let first_operand = self.parse_operand()?;

        // Check if there's a comma (SquareGroup) or just closing bracket (Address)
        if self.peek_is(&PtxSpecToken::Comma) {
            // It's a SquareGroup with multiple operands
            let mut operands = vec![first_operand];

            loop {
                if self.peek_is(&PtxSpecToken::RBracket) {
                    break;
                }

                // Consume comma if present
                if self.peek_is(&PtxSpecToken::Comma) {
                    self.advance();
                }

                if self.peek_is(&PtxSpecToken::RBracket) {
                    break; // trailing comma
                }

                // Parse next operand (could be an identifier or {something,})
                let operand = self.parse_operand()?;
                operands.push(operand);
            }

            self.expect_token(&PtxSpecToken::RBracket)?;
            Ok(OperandElement::SquareGroup(operands))
        } else if self.peek_is(&PtxSpecToken::RBracket) {
            // Single operand, it's an Address
            self.advance();

            // Extract the identifier (must be simple)
            let id = match first_operand {
                Operand {
                    operator: None,
                    operand: OperandElement::Item(id),
                    modifier: None,
                } => id,
                _ => panic!(
                    "Address [a] must contain a simple identifier, got: {:?}",
                    first_operand
                ),
            };

            Ok(OperandElement::Address(id))
        } else {
            Err(SpecParseError {
                message: format!("Expected ',' or ']' in square group"),
                span: self.current_span(),
            })
        }
    }

    fn parse_paren_group(&mut self) -> Result<OperandElement, SpecParseError> {
        self.expect_token(&PtxSpecToken::LParen)?;

        // Check if it's (param-list)
        if let Some(token) = self.peek() {
            if let PtxSpecToken::Identifier(s) = token {
                if s == "param-list" {
                    self.advance();
                    self.expect_token(&PtxSpecToken::RParen)?;
                    return Ok(OperandElement::ParamList);
                }
            }
        }

        // Otherwise it's a parenthesized operand
        let operand = self.parse_operand()?;
        self.expect_token(&PtxSpecToken::RParen)?;

        // Extract the identifier (must be simple)
        let id = match operand {
            Operand {
                operator: None,
                operand: OperandElement::Item(id),
                modifier: None,
            } => id,
            _ => panic!(
                "ParenthesizedOperand (a) must contain a simple identifier, got: {:?}",
                operand
            ),
        };

        Ok(OperandElement::ParenthesizedOperand(id))
    }
}
