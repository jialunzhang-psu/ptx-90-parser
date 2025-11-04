use crate::lexer::{PtxSpecToken, Span, token_to_string, tokenize};
use crate::r#type::{
    InstructionHead, InstructionRule, Modifier, Operand, OperandElement, OperatorToken,
    ParameterRule, Rule, Section,
};
use std::collections::HashSet;
use std::fmt;

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

/// Parse a PTX specification string into a list of Section AST nodes.
pub fn parse_spec(source: &str) -> Result<Vec<Section>, SpecParseError> {
    parse_spec_with_name(source, "<input>")
}

/// Parse a PTX specification string with a file name for error reporting.
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
                    // Parse the RHS choices
                    let choices = parse_parameter_rhs(&rhs, &identifiers)?;

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
// Step 1: Split into sections
// ============================================================================

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
// Step 2: Split into rules
// ============================================================================

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
) -> Result<Vec<Modifier>, SpecParseError> {
    let tokens = tokenize(rhs).map_err(|e| SpecParseError {
        message: format!("Lexer error in RHS: {:?}", e),
        span: Some(e.span),
    })?;

    let mut parser = TokenParser::new(tokens, identifiers);

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
        if let Some((token, _)) = self.advance() {
            match token {
                PtxSpecToken::Identifier(s) => Ok(s),
                PtxSpecToken::Directive(s) => Ok(format!(".{}", s)),
                _ => Err(SpecParseError {
                    message: format!("Expected identifier, got {:?}", token),
                    span: self.current_span(),
                }),
            }
        } else {
            Err(SpecParseError {
                message: "Unexpected end of input, expected identifier".to_string(),
                span: None,
            })
        }
    }

    fn peek_is_modifier_start(&self) -> bool {
        if let Some(token) = self.peek() {
            matches!(
                token,
                PtxSpecToken::Dot | PtxSpecToken::Directive(_) | PtxSpecToken::LBrace
            )
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
                    _ => panic!("Optional modifier must contain a simple Atom, got: {:?}", inner),
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
        // - .collector::buffer::op where ::buffer, ::op are identifiers -> Sequence([::buffer, ::op])
        // - .dtype where .dtype is an identifier -> Atom(.dtype) (parameter reference)
        // - aa::item3 where neither are identifiers -> Atom(aa::item3) (literal)
        //
        // Rules:
        // 1. If the entire parsed string is in identifiers → Atom (parameter reference)
        // 2. If it splits into multiple parts that are ALL in identifiers → Sequence
        // 3. If only SOME parts are in identifiers → ERROR
        // 4. Otherwise → Atom (literal value)

        let saved_pos = self.pos;

        // First, parse the entire string to see what we have
        let full_string = self.parse_modifier_atom_with_concatenation()?;
        let full_string_text = match &full_string {
            Modifier::Atom(s) => s.clone(),
            Modifier::ImmediateNumber(n) => {
                // Immediate numbers are not split, just return them directly
                return Ok(Modifier::ImmediateNumber(n.clone()));
            }
            _ => unreachable!(),
        };
        let end_pos = self.pos; // Save the position after parsing the full string

        // Check if the entire string is a known identifier (parameter reference)
        if self.identifiers.contains(&full_string_text) {
            return Ok(Modifier::Atom(full_string_text));
        }

        // Reset and try to parse as a potential sequence
        self.pos = saved_pos;

        // Check if it starts with a directive (grouping prefix) followed by multiple :: parts
        if let Some(PtxSpecToken::Directive(directive_name)) = self.peek() {
            let directive_str = format!(".{}", directive_name);
            self.advance(); // consume the directive

            // Check if followed by ::
            if self.peek_is(&PtxSpecToken::DoubleColon) {
                // Collect all parts including the directive
                let mut parts_strings = Vec::new();
                parts_strings.push(directive_str); // Include the directive prefix

                while self.peek_is(&PtxSpecToken::DoubleColon) {
                    self.advance(); // consume ::

                    let mut part = String::from("::");
                    if let Some((token, _)) = self.advance() {
                        match token {
                            PtxSpecToken::Identifier(s) => part.push_str(&s),
                            PtxSpecToken::DecimalInteger(s) => part.push_str(&s),
                            PtxSpecToken::OctalInteger(s) => part.push_str(&s),
                            PtxSpecToken::HexInteger(s) => part.push_str(&s),
                            _ => {
                                return Err(SpecParseError {
                                    message: format!("Expected identifier or number after '::'"),
                                    span: self.current_span(),
                                });
                            }
                        }
                    }

                    // Check for trailing * characters
                    while self.peek_is(&PtxSpecToken::Star) {
                        self.advance();
                        part.push('*');
                    }

                    parts_strings.push(part);
                }

                // Now check: are the :: parts (excluding the first directive) identifiers?
                // The first part is the directive prefix (e.g., ".collector"), which may or may not be a parameter
                // The rest are :: parts (e.g., "::buffer", "::op")
                if parts_strings.len() > 1 {
                    // Check the :: parts (skip the first directive part)
                    let double_colon_parts = &parts_strings[1..];
                    let parts_in_identifiers: Vec<bool> = double_colon_parts
                        .iter()
                        .map(|p| self.identifiers.contains(p))
                        .collect();

                    let all_in = parts_in_identifiers.iter().all(|&b| b);
                    let some_in = parts_in_identifiers.iter().any(|&b| b);

                    if all_in {
                        // All :: parts are identifiers → Sequence
                        return Ok(Modifier::Sequence(parts_strings));
                    } else if some_in {
                        // Only some :: parts are identifiers → ERROR
                        return Err(SpecParseError {
                            message: format!(
                                "Partial identifier match in '{}': only some parts are known parameters",
                                full_string_text
                            ),
                            span: self.current_span(),
                        });
                    }
                    // else: none are identifiers, fall through to return as literal Atom
                }
            }
        }

        // If we get here, it's a literal atom (not a parameter reference, not a sequence)
        // Restore to the end position so tokens are properly consumed
        self.pos = end_pos;
        Ok(Modifier::Atom(full_string_text))
    }

    fn parse_modifier_atom_with_concatenation(&mut self) -> Result<Modifier, SpecParseError> {
        // Parse a modifier like .b8x16.b6x16_p32 (for parameter RHS)
        let mut result = String::new();

        // Start with a directive or ::identifier
        if let Some(token) = self.peek() {
            match token {
                PtxSpecToken::Directive(s) => {
                    result.push('.');
                    result.push_str(s);
                    self.advance();
                }
                PtxSpecToken::DoubleColon => {
                    result.push_str("::");
                    self.advance();
                    if let Some((token, _)) = self.advance() {
                        match token {
                            PtxSpecToken::Identifier(s) => result.push_str(&s),
                            PtxSpecToken::DecimalInteger(s) => result.push_str(&s),
                            _ => {
                                return Err(SpecParseError {
                                    message: format!("Expected identifier after '::'"),
                                    span: self.current_span(),
                                });
                            }
                        }
                    }
                }
                _ => {
                    return self.parse_modifier_atom();
                }
            }
        } else {
            return Err(SpecParseError {
                message: "Unexpected end of input".to_string(),
                span: None,
            });
        }

        // Check for continuation: :: or .
        loop {
            if self.peek_is(&PtxSpecToken::DoubleColon) {
                self.advance();
                result.push_str("::");

                if let Some((token, _)) = self.advance() {
                    match token {
                        PtxSpecToken::Identifier(s) => result.push_str(&s),
                        PtxSpecToken::DecimalInteger(s) => result.push_str(&s),
                        PtxSpecToken::OctalInteger(s) => result.push_str(&s),
                        PtxSpecToken::HexInteger(s) => result.push_str(&s),
                        _ => {
                            return Err(SpecParseError {
                                message: format!(
                                    "Expected identifier or number after '::', got {:?}",
                                    token
                                ),
                                span: self.current_span(),
                            });
                        }
                    }
                }
            } else if matches!(self.peek(), Some(PtxSpecToken::Directive(_))) {
                // Concatenate another directive (e.g., .b8x16.b6x16_p32)
                if let Some((PtxSpecToken::Directive(s), _)) = self.advance() {
                    result.push('.');
                    result.push_str(&s);
                }
            } else if matches!(self.peek(), Some(PtxSpecToken::Identifier(_))) {
                // Handle patterns like 02_13 where _13 is an identifier following a number
                if let Some((PtxSpecToken::Identifier(s), _)) = self.advance() {
                    result.push_str(&s);
                }
            } else {
                break;
            }
        }

        // Check for trailing * characters
        while self.peek_is(&PtxSpecToken::Star) {
            self.advance();
            result.push('*');
        }

        Ok(Modifier::Atom(result))
    }

    fn parse_modifier_atom(&mut self) -> Result<Modifier, SpecParseError> {
        // Parse a modifier like .dtype or ::buffer or .b8x16.b6x16_p32 or immediate number like 4, 8, 16
        let mut result = String::new();

        // Check if this is an immediate number (no prefix)
        if let Some(token) = self.peek() {
            match token {
                PtxSpecToken::DecimalInteger(_) | PtxSpecToken::HexInteger(_) => {
                    if let Some((tok, _)) = self.advance() {
                        let num_str = match tok {
                            PtxSpecToken::DecimalInteger(s) => s,
                            PtxSpecToken::HexInteger(s) => s,
                            _ => unreachable!(),
                        };
                        return Ok(Modifier::ImmediateNumber(num_str));
                    }
                }
                _ => {}
            }
        }

        // Check if it starts with dot or double colon
        if self.peek_is(&PtxSpecToken::Dot) {
            self.advance();
            result.push('.');
        } else if self.peek_is(&PtxSpecToken::DoubleColon) {
            self.advance();
            result.push_str("::");
        }

        // Now expect identifier or directive
        if let Some((token, _)) = self.advance() {
            match token {
                PtxSpecToken::Identifier(s) => result.push_str(&s),
                PtxSpecToken::Directive(s) => {
                    result.clear();
                    result.push('.');
                    result.push_str(&s);
                }
                _ => {
                    return Err(SpecParseError {
                        message: format!("Expected identifier in modifier, got {:?}", token),
                        span: self.current_span(),
                    });
                }
            }

            // Check for :: continuation
            while self.peek_is(&PtxSpecToken::DoubleColon) {
                self.advance();
                result.push_str("::");

                if let Some((token, _)) = self.advance() {
                    match token {
                        PtxSpecToken::Identifier(s) => result.push_str(&s),
                        PtxSpecToken::DecimalInteger(s) => result.push_str(&s),
                        PtxSpecToken::HexInteger(s) => result.push_str(&s),
                        _ => {
                            return Err(SpecParseError {
                                message: format!(
                                    "Expected identifier or number after '::', got {:?}",
                                    token
                                ),
                                span: self.current_span(),
                            });
                        }
                    }
                } else {
                    return Err(SpecParseError {
                        message: "Unexpected end after '::'".to_string(),
                        span: None,
                    });
                }
            }

            // Check for trailing * characters (like .discard*)
            while self.peek_is(&PtxSpecToken::Star) {
                self.advance();
                result.push('*');
            }

            Ok(Modifier::Atom(result))
        } else {
            Err(SpecParseError {
                message: "Unexpected end of input in modifier".to_string(),
                span: None,
            })
        }
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
                    _ => panic!("Optional operand {{, ...}} must contain a simple identifier, got: {:?}", inner_operand),
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
                _ => panic!("PipeChoice must have simple identifiers, got: {:?}", operand),
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
                    _ => panic!("PipeOptionalChoice must have simple identifiers, got: {:?}", operand),
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
                        _ => panic!("Optional operand {{...,}} must contain a simple identifier, got: {:?}", inner_operand),
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
                _ => panic!("Address [a] must contain a simple identifier, got: {:?}", first_operand),
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
            _ => panic!("ParenthesizedOperand (a) must contain a simple identifier, got: {:?}", operand),
        };

        Ok(OperandElement::ParenthesizedOperand(id))
    }
}
