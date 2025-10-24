use crate::{
    r#type::{
        AddressBase, AddressDisplacement, AddressDisplacementKind, AddressSign,
        AddressSizeDirective, ArraySpecifier, AsyncGroupModifier, AtomicOperationModifier,
        CacheModifier, CallModifier, ConditionModifier, DwarfDirective, EntryFunction,
        FileDirective, FuncFunction, FunctionAlias, FunctionBody, FunctionDeclarationKind,
        FunctionDim3, FunctionEntryDirective, FunctionHeaderDirective, FunctionKernelDirective,
        FunctionLinkage, FunctionStatement, FunctionVisibility, GenericFunctionDeclaration,
        GlobalAddressSpace, GlobalInitializer, GlobalLinkage, GlobalMutability, GlobalVisibility,
        InitializerValue, Instruction, LinkingDirective, LinkingDirectiveKind, LocationDirective,
        MathModeModifier, MemoryOperand, MemoryOrderModifier, MemoryScopeModifier, ModifierKind,
        Module, ModuleDebugDirective, ModuleDirective, ModuleDirectiveKind,
        ModuleVariableDirective, NumericLiteral, OpcodeKind, Operand, Parameter,
        ParameterQualifiers, ParameterSpecifier, ParameterStorage, PointerAddressSpace,
        PointerQualifier, PragmaDirective, PtxParseError, RegisterDeclaration, RegisterSpecifier,
        RegisterType, RoundingModifier, ScalarType, SectionDirective, ShuffleModifier,
        StateSpaceModifier, StatementDirective, StatementSectionDirective, SynchronizationModifier,
        TargetDirective, TypeModifier, VariableDirective, VariableQualifier, VersionDirective,
    },
    InstructionOpcode,
};

use std::collections::VecDeque;

/// Parse a PTX source string into a lightweight abstract syntax tree (AST).
///
/// The parser performs a tolerant, line-oriented pass that recognises
/// module-level directives and function definitions. Inside functions it
/// categorises labels, directives, and instructions, providing a structured
/// representation without attempting full semantic validation of PTX.
pub fn parse(source: &str) -> Result<Module, PtxParseError> {
    Parser::new(source).parse()
}

/// Parse a single directive line without building a full module AST.
pub fn parse_module_directive(
    line: &str,
    line_number: usize,
) -> Result<ModuleDirective, PtxParseError> {
    let stripped = strip_comments(line);
    let trimmed = stripped.trim();
    if trimmed.is_empty() {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: "empty directive".into(),
        });
    }
    if trimmed.starts_with(".tex") {
        let tex = parse_tex_directive(trimmed, line_number)?;
        return Ok(ModuleDirective::ModuleVariable(
            ModuleVariableDirective::Tex(tex),
        ));
    }

    if likely_state_space(trimmed) {
        if let Some(variable) = parse_state_space_variable(trimmed, line_number)? {
            return Ok(ModuleDirective::ModuleVariable(variable));
        }
    }
    parse_module_directive_internal(trimmed, line_number)
}

pub fn parse_entry_directive(line: &str) -> Result<FunctionEntryDirective, PtxParseError> {
    let mut in_declaration = true;
    match parse_function_directive_stmt(line.trim(), None, 1, &mut in_declaration)? {
        FunctionBodyItem::Entry(entry) => Ok(entry),
        _ => Err(PtxParseError::InvalidDirective {
            line: 1,
            message: "expected entry directive".into(),
        }),
    }
}

pub fn parse_stmt_directive(line: &str) -> Result<FunctionStatement, PtxParseError> {
    let mut in_declaration = false;
    match parse_function_directive_stmt(line.trim(), None, 1, &mut in_declaration)? {
        FunctionBodyItem::Statement(stmt) => Ok(stmt),
        _ => Err(PtxParseError::InvalidDirective {
            line: 1,
            message: "expected statement directive".into(),
        }),
    }
}

pub fn parse_instruction_line(line: &str) -> Result<Instruction, PtxParseError> {
    parse_instruction(line.trim(), None, 1)
}

struct Parser<'a> {
    lines: Vec<&'a str>,
    index: usize,
}

enum FunctionBodyItem {
    Entry(FunctionEntryDirective),
    Statement(FunctionStatement),
}

enum ParsedFunction {
    Entry(EntryFunction),
    Func(FuncFunction),
    Linking(LinkingDirective),
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            lines: source.lines().collect(),
            index: 0,
        }
    }

    fn parse(mut self) -> Result<Module, PtxParseError> {
        let mut module = Module {
            directives: Vec::new(),
        };

        while self.index < self.lines.len() {
            let raw_line = self.lines[self.index];
            let trimmed = strip_comments(raw_line);
            self.index += 1;

            if trimmed.is_empty() {
                continue;
            }

            if trimmed == "{" || trimmed == "}" {
                continue;
            }

            if trimmed.starts_with(".alias") {
                let alias = parse_alias(&trimmed, self.index)?;
                module.directives.push(ModuleDirective::FunctionKernel(
                    FunctionKernelDirective::Alias(alias),
                ));
                continue;
            }

            if contains_keyword(&trimmed, ".entry") || contains_keyword(&trimmed, ".func") {
                match self.parse_function(trimmed)? {
                    ParsedFunction::Entry(function) => {
                        module.directives.push(ModuleDirective::FunctionKernel(
                            FunctionKernelDirective::Entry(function),
                        ));
                    }
                    ParsedFunction::Func(function) => {
                        module.directives.push(ModuleDirective::FunctionKernel(
                            FunctionKernelDirective::Func(function),
                        ));
                    }
                    ParsedFunction::Linking(linking) => {
                        module.directives.push(ModuleDirective::Linking(linking));
                    }
                }
                continue;
            }

            let line_number = self.index;
            if let Some(state_space) = self.try_parse_state_space(trimmed.clone(), line_number)? {
                module
                    .directives
                    .push(ModuleDirective::ModuleVariable(state_space));
                continue;
            }

            let directive = parse_module_directive_internal(&trimmed, self.index)?;
            module.directives.push(directive);
        }

        Ok(module)
    }

    fn parse_function(&mut self, header_line: String) -> Result<ParsedFunction, PtxParseError> {
        let header_start_line = self.index; // 1-based line number after initial increment
        let mut header = header_line;
        let mut is_declaration = header.contains(';');

        while !header.contains('{') && !is_declaration {
            let next_line = self
                .lines
                .get(self.index)
                .ok_or(PtxParseError::UnexpectedEof {
                    context: "function header",
                    line: header_start_line,
                })?;
            self.index += 1;
            let trimmed = strip_comments(next_line);
            if trimmed.is_empty() {
                continue;
            }
            header.push(' ');
            header.push_str(&trimmed);
            if trimmed.contains(';') {
                is_declaration = true;
            }
        }

        if is_declaration && !header.contains('{') {
            if let Some(linking) = parse_function_prototype_linking(&header) {
                return Ok(ParsedFunction::Linking(linking));
            }

            panic!(
                "Unsupported function declaration without body at line {}",
                header_start_line
            );
        }

        let mut header_split = header.splitn(2, '{');
        let header_part = header_split.next().unwrap_or_default().trim();
        let after_brace = header_split.next().map(str::trim).unwrap_or("");

        let keyword = if header_part.contains(".entry") {
            ".entry"
        } else if header_part.contains(".func") {
            ".func"
        } else {
            return Err(PtxParseError::InvalidFunctionHeader {
                line: header_start_line,
                message: "missing .entry or .func keyword".into(),
            });
        };

        let (directives, name, params, return_param) =
            parse_function_header(header_part, keyword, header_start_line)?;

        let mut entry_directives = Vec::new();
        let mut body_statements = Vec::new();
        let mut brace_depth: i32 = 1; // account for the opening brace consumed with the header
        let mut stmt_buffer = String::new();
        let mut pending_comment: Option<String> = None;
        let mut in_declaration = true;

        if !after_brace.is_empty() {
            if process_body_segment(
                after_brace,
                None,
                header_start_line,
                &mut brace_depth,
                &mut stmt_buffer,
                &mut pending_comment,
                &mut in_declaration,
                &mut entry_directives,
                &mut body_statements,
            )? {
                if !stmt_buffer.trim().is_empty() {
                    return Err(PtxParseError::InvalidInstruction {
                        line: header_start_line,
                        message: "unterminated instruction in function body".into(),
                    });
                }

                let body = FunctionBody {
                    entry_directives,
                    statements: body_statements,
                };
                return Ok(match keyword {
                    ".entry" => ParsedFunction::Entry(EntryFunction {
                        name,
                        directives,
                        params,
                        body,
                    }),
                    ".func" => ParsedFunction::Func(FuncFunction {
                        name,
                        directives,
                        return_param,
                        params,
                        body,
                    }),
                    _ => unreachable!("validated keyword"),
                });
            }
        }

        while self.index < self.lines.len() {
            let raw_line = self.lines[self.index];
            self.index += 1;

            let trimmed_line = raw_line.trim();
            if trimmed_line.is_empty() {
                continue;
            }
            if trimmed_line.starts_with("//") || trimmed_line.starts_with('#') {
                continue;
            }

            let (content, comment) = split_comment(trimmed_line);
            if content.trim().is_empty() {
                if let Some(comment) = comment {
                    pending_comment = Some(comment);
                }
                continue;
            }

            if process_body_segment(
                &content,
                comment,
                self.index,
                &mut brace_depth,
                &mut stmt_buffer,
                &mut pending_comment,
                &mut in_declaration,
                &mut entry_directives,
                &mut body_statements,
            )? {
                break;
            }
        }

        if brace_depth != 0 {
            return Err(PtxParseError::UnexpectedEof {
                context: "function body",
                line: header_start_line,
            });
        }

        if !stmt_buffer.trim().is_empty() {
            return Err(PtxParseError::InvalidInstruction {
                line: header_start_line,
                message: "unterminated instruction in function body".into(),
            });
        }

        let body = FunctionBody {
            entry_directives,
            statements: body_statements,
        };
        Ok(match keyword {
            ".entry" => ParsedFunction::Entry(EntryFunction {
                name,
                directives,
                params,
                body,
            }),
            ".func" => ParsedFunction::Func(FuncFunction {
                name,
                directives,
                return_param,
                params,
                body,
            }),
            _ => unreachable!("validated keyword"),
        })
    }

    fn try_parse_state_space(
        &mut self,
        mut current: String,
        line_number: usize,
    ) -> Result<Option<ModuleVariableDirective>, PtxParseError> {
        if !likely_state_space(&current) {
            return Ok(None);
        }

        let trimmed = current.trim_start();
        if trimmed.starts_with(".tex") {
            while !current.trim_end().ends_with(';') {
                let next_line = self
                    .lines
                    .get(self.index)
                    .ok_or(PtxParseError::UnexpectedEof {
                        context: "texture declaration",
                        line: line_number,
                    })?;
                self.index += 1;
                let trimmed = strip_comments(next_line);
                if trimmed.is_empty() {
                    continue;
                }
                if !current
                    .chars()
                    .last()
                    .map(|ch| ch.is_whitespace())
                    .unwrap_or(true)
                {
                    current.push(' ');
                }
                current.push_str(&trimmed);
            }

            let tex = parse_tex_directive(&current, line_number)?;
            return Ok(Some(ModuleVariableDirective::Tex(tex)));
        }

        while !current.trim_end().ends_with(';') {
            let next_line = self
                .lines
                .get(self.index)
                .ok_or(PtxParseError::UnexpectedEof {
                    context: "state space declaration",
                    line: line_number,
                })?;
            self.index += 1;
            let trimmed = strip_comments(next_line);
            if trimmed.is_empty() {
                continue;
            }
            if !current
                .chars()
                .last()
                .map(|ch| ch.is_whitespace())
                .unwrap_or(true)
            {
                current.push(' ');
            }
            current.push_str(&trimmed);
        }

        parse_state_space_variable(&current, line_number)
    }
}

fn process_body_segment(
    segment: &str,
    comment: Option<String>,
    line_number: usize,
    brace_depth: &mut i32,
    stmt_buffer: &mut String,
    pending_comment: &mut Option<String>,
    in_declaration: &mut bool,
    entry_directives: &mut Vec<FunctionEntryDirective>,
    statements: &mut Vec<FunctionStatement>,
) -> Result<bool, PtxParseError> {
    let open = count_occurrences(segment, '{') as i32;
    let close = count_occurrences(segment, '}') as i32;
    *brace_depth += open;
    *brace_depth -= close;

    if *brace_depth < 0 {
        return Err(PtxParseError::InvalidFunctionHeader {
            line: line_number,
            message: "mismatched braces in function body".into(),
        });
    }

    let sanitized = segment
        .replace('{', " ")
        .replace('}', " ")
        .trim()
        .to_string();

    if sanitized.is_empty() {
        if let Some(comment) = comment {
            *pending_comment = Some(comment);
        }
        return Ok(*brace_depth == 0);
    }

    if sanitized.ends_with(':') || sanitized.starts_with('.') || sanitized.starts_with("@@") {
        match parse_function_stmt(&sanitized, comment, line_number, in_declaration)? {
            FunctionBodyItem::Entry(entry) => entry_directives.push(entry),
            FunctionBodyItem::Statement(stmt) => statements.push(stmt),
        }
        stmt_buffer.clear();
        *pending_comment = None;
        return Ok(*brace_depth == 0);
    }

    if !stmt_buffer.is_empty() {
        stmt_buffer.push(' ');
    }
    stmt_buffer.push_str(&sanitized);

    if let Some(comment) = comment {
        *pending_comment = Some(comment);
    }

    while let Some(idx) = stmt_buffer.find(';') {
        let (statement, rest) = stmt_buffer.split_at(idx + 1);
        let statement = statement.trim();
        if !statement.is_empty() {
            match parse_function_stmt(
                statement,
                pending_comment.take(),
                line_number,
                in_declaration,
            )? {
                FunctionBodyItem::Entry(entry) => entry_directives.push(entry),
                FunctionBodyItem::Statement(stmt) => statements.push(stmt),
            };
        } else {
            pending_comment.take();
        }
        *stmt_buffer = rest.trim_start().to_string();
    }

    Ok(*brace_depth == 0)
}

fn parse_function_prototype_linking(header: &str) -> Option<LinkingDirective> {
    let trimmed = header.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut tokens = trimmed.split_whitespace();
    let first = tokens.next()?;

    let kind = match first {
        ".extern" => LinkingDirectiveKind::Extern,
        ".visible" => LinkingDirectiveKind::Visible,
        ".weak" => LinkingDirectiveKind::Weak,
        ".common" => LinkingDirectiveKind::Common,
        _ => return None,
    };

    let prototype = trimmed[first.len()..]
        .trim()
        .trim_end_matches(';')
        .trim()
        .to_string();
    let raw = trimmed.to_string();

    Some(LinkingDirective {
        kind,
        prototype,
        raw,
    })
}

fn parse_module_directive_internal(
    line: &str,
    line_number: usize,
) -> Result<ModuleDirective, PtxParseError> {
    let trimmed = line.trim();

    if trimmed.starts_with(".version") {
        let parts: Vec<_> = trimmed.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(PtxParseError::InvalidDirective {
                line: line_number,
                message: "missing version number".into(),
            });
        }
        let mut version_iter = parts[1].split('.');
        let major = version_iter
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or_else(|| PtxParseError::InvalidDirective {
                line: line_number,
                message: "invalid major version".into(),
            })?;
        let minor = version_iter
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or_else(|| PtxParseError::InvalidDirective {
                line: line_number,
                message: "invalid minor version".into(),
            })?;
        return Ok(ModuleDirective::Module(ModuleDirectiveKind::Version(
            VersionDirective { major, minor },
        )));
    }

    if trimmed.starts_with(".target") {
        let rest = trimmed.trim_start_matches(".target").trim();
        let directive = parse_target_directive(rest, line_number)?;
        return Ok(ModuleDirective::Module(ModuleDirectiveKind::Target(
            directive,
        )));
    }

    if trimmed.starts_with(".address_size") {
        let rest = trimmed.trim_start_matches(".address_size").trim();
        let size = rest
            .split_whitespace()
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or_else(|| PtxParseError::InvalidDirective {
                line: line_number,
                message: "invalid address size".into(),
            })?;
        return Ok(ModuleDirective::Module(ModuleDirectiveKind::AddressSize(
            AddressSizeDirective { size },
        )));
    }

    if trimmed.starts_with(".file") {
        let rest = trimmed.trim_start_matches(".file").trim();
        if rest.is_empty() {
            return Err(PtxParseError::InvalidDirective {
                line: line_number,
                message: "missing index for .file directive".into(),
            });
        }

        let index_len = rest.find(char::is_whitespace).unwrap_or(rest.len());
        let (index_part, remainder) = rest.split_at(index_len);
        let index = index_part
            .parse::<u32>()
            .map_err(|_| PtxParseError::InvalidDirective {
                line: line_number,
                message: format!("invalid .file index '{index_part}'"),
            })?;

        let path_part = remainder.trim();
        if path_part.is_empty() {
            return Err(PtxParseError::InvalidDirective {
                line: line_number,
                message: "missing path in .file directive".into(),
            });
        }

        let path = if let Some(stripped) = path_part.strip_prefix('"') {
            if let Some(end_idx) = stripped.find('"') {
                let path = stripped[..end_idx].to_string();
                if !stripped[end_idx + 1..].trim().is_empty() {
                    return Err(PtxParseError::InvalidDirective {
                        line: line_number,
                        message: "unexpected tokens after .file path".into(),
                    });
                }
                path
            } else {
                return Err(PtxParseError::InvalidDirective {
                    line: line_number,
                    message: "unterminated quoted path in .file directive".into(),
                });
            }
        } else {
            path_part.to_string()
        };

        return Ok(ModuleDirective::Debug(ModuleDebugDirective::File(
            FileDirective { index, path },
        )));
    }

    if trimmed.starts_with(".section") {
        let rest = trimmed.trim_start_matches(".section").trim();
        if rest.is_empty() {
            return Err(PtxParseError::InvalidDirective {
                line: line_number,
                message: "missing section name".into(),
            });
        }

        let mut parts = rest.split_whitespace();
        let name = parts
            .next()
            .ok_or_else(|| PtxParseError::InvalidDirective {
                line: line_number,
                message: "missing section name".into(),
            })?
            .to_string();
        let attributes = parts.map(|part| part.trim().to_string()).collect();
        return Ok(ModuleDirective::Debug(ModuleDebugDirective::Section(
            SectionDirective { name, attributes },
        )));
    }

    if trimmed.starts_with("@@") {
        let mut parts = trimmed.split_whitespace();
        let keyword = parts.next().unwrap_or_default();
        let arguments = parts.map(|token| token.to_string()).collect();
        let directive = DwarfDirective {
            keyword: keyword.to_string(),
            arguments,
            comment: None,
            raw: trimmed.to_string(),
        };
        return Ok(ModuleDirective::Debug(ModuleDebugDirective::Dwarf(
            directive,
        )));
    }

    if let Some(linking) = parse_module_linking(trimmed, line_number)? {
        return Ok(ModuleDirective::Linking(linking));
    }

    Err(PtxParseError::InvalidDirective {
        line: line_number,
        message: format!("unsupported directive '{line}'"),
    })
}

fn parse_module_linking(
    trimmed: &str,
    line_number: usize,
) -> Result<Option<LinkingDirective>, PtxParseError> {
    for (keyword, kind) in [
        (".extern", LinkingDirectiveKind::Extern),
        (".visible", LinkingDirectiveKind::Visible),
        (".weak", LinkingDirectiveKind::Weak),
        (".common", LinkingDirectiveKind::Common),
    ] {
        if trimmed.starts_with(keyword) {
            let remainder = trimmed[keyword.len()..].trim();
            if remainder.is_empty() {
                return Err(PtxParseError::InvalidDirective {
                    line: line_number,
                    message: format!("missing prototype for {keyword} directive"),
                });
            }
            let prototype = remainder.trim_end_matches(';').trim().to_string();
            return Ok(Some(LinkingDirective {
                kind,
                prototype,
                raw: trimmed.to_string(),
            }));
        }
    }

    Ok(None)
}

fn parse_tex_directive(line: &str, line_number: usize) -> Result<VariableDirective, PtxParseError> {
    let trimmed = line.trim();
    if !trimmed.starts_with(".tex") {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: "expected .tex directive".into(),
        });
    }

    let without_semicolon = trimmed.trim_end_matches(';').trim();
    let mut tokens: Vec<String> = without_semicolon
        .split_whitespace()
        .map(|tok| tok.trim().to_string())
        .filter(|tok| !tok.is_empty())
        .collect();

    if tokens.is_empty() || tokens[0] != ".tex" {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: "malformed .tex directive".into(),
        });
    }

    tokens.remove(0);
    if tokens.is_empty() {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: "missing identifier in .tex directive".into(),
        });
    }

    let name_token = tokens.pop().unwrap();
    if name_token.starts_with('.') {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: "invalid texture identifier".into(),
        });
    }

    let name = name_token.trim_end_matches(';').to_string();
    let mut qualifiers = Vec::new();
    let mut ty = None;
    for token in tokens {
        if let Some(scalar) = parse_scalar_type(&token) {
            if ty.is_some() {
                return Err(PtxParseError::InvalidDirective {
                    line: line_number,
                    message: "multiple type specifiers in .tex directive".into(),
                });
            }
            ty = Some(scalar);
            continue;
        }

        qualifiers.push(variable_qualifier_from_token(
            &token,
            line_number,
            VariableQualifierContext::Tex,
        )?);
    }

    Ok(VariableDirective {
        visibility: None,
        linkages: Vec::new(),
        address_space: None,
        mutability: None,
        alignment: None,
        ty,
        qualifiers,
        name,
        array: None,
        initializer: None,
        raw: line.trim().to_string(),
    })
}

fn parse_state_space_variable(
    line: &str,
    line_number: usize,
) -> Result<Option<ModuleVariableDirective>, PtxParseError> {
    if !line.trim_end().ends_with(';') {
        return Ok(None);
    }

    let without_semicolon = line.trim_end().trim_end_matches(';').trim_end();
    if without_semicolon.is_empty() {
        return Ok(None);
    }

    let (decl_part, initializer_part) = split_initializer(without_semicolon);
    let initializer = initializer_part.and_then(|init| {
        let trimmed = init.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    });

    let mut tokens: Vec<String> = decl_part
        .split_whitespace()
        .map(|tok| tok.trim().trim_matches(',').to_string())
        .filter(|tok| !tok.is_empty())
        .collect();

    if tokens.len() < 2 {
        return Ok(None);
    }

    let name_token = tokens.pop().unwrap();
    if name_token.starts_with('.') {
        return Ok(None);
    }

    let (name, array) = split_name_token(&name_token, line_number)?;

    let mut visibility = None;
    let mut linkages = Vec::new();
    let mut address_space = None;
    let mut mutability = None;
    let mut ty = None;
    let mut qualifiers: Vec<VariableQualifier> = Vec::new();
    let mut alignment = None;
    let mut idx = 0;
    while idx < tokens.len() {
        let token = tokens[idx].clone();
        if token == ".align" {
            if idx + 1 < tokens.len() {
                if let Ok(value) = tokens[idx + 1].parse::<u32>() {
                    alignment = Some(value);
                    idx += 2;
                    continue;
                }
            }
            return Err(PtxParseError::InvalidGlobal {
                line: line_number,
                message: "malformed .align qualifier on global variable".into(),
            });
        }

        if let Some(parsed) = parse_global_visibility(&token) {
            visibility = Some(parsed);
        } else if let Some(linkage) = parse_global_linkage(&token) {
            if !linkages.contains(&linkage) {
                linkages.push(linkage);
            }
        } else if let Some(space) = parse_global_address_space(&token) {
            address_space = Some(space);
        } else if token == ".volatile" {
            return Err(PtxParseError::InvalidGlobal {
                line: line_number,
                message: "module-level state space declarations cannot use '.volatile'".into(),
            });
        } else if let Some(mutable) = parse_global_mutability(&token) {
            mutability = Some(mutable);
        } else if let Some(scalar) = parse_scalar_type(&token) {
            ty = Some(scalar);
        } else {
            qualifiers.push(variable_qualifier_from_token(
                &token,
                line_number,
                VariableQualifierContext::StateSpace,
            )?);
        }
        idx += 1;
    }

    let parsed_initializer = if let Some(raw_init) = initializer {
        Some(parse_global_initializer(&raw_init, line_number)?)
    } else {
        None
    };

    let data = ParsedStateVariable {
        visibility,
        linkages,
        address_space,
        mutability,
        alignment,
        ty,
        qualifiers,
        name,
        array,
        initializer: parsed_initializer,
        raw: line.trim().to_string(),
    };

    let directive = match data.address_space {
        Some(GlobalAddressSpace::Const) => {
            ModuleVariableDirective::Const(data.into_variable(GlobalAddressSpace::Const))
        }
        Some(GlobalAddressSpace::Shared) => {
            ModuleVariableDirective::Shared(data.into_variable(GlobalAddressSpace::Shared))
        }
        Some(GlobalAddressSpace::Global) => {
            ModuleVariableDirective::Global(data.into_variable(GlobalAddressSpace::Global))
        }
        _ => return Ok(None),
    };

    Ok(Some(directive))
}

fn parse_function_header(
    header: &str,
    keyword: &str,
    line_number: usize,
) -> Result<
    (
        Vec<FunctionHeaderDirective>,
        String,
        Vec<Parameter>,
        Option<Parameter>,
    ),
    PtxParseError,
> {
    let kind_pos = header
        .find(keyword)
        .ok_or_else(|| PtxParseError::InvalidFunctionHeader {
            line: line_number,
            message: format!("missing {keyword} keyword"),
        })?;

    let qualifiers_part = header[..kind_pos].trim();
    let mut directives = Vec::new();
    parse_function_header_directives(qualifiers_part, &mut directives, line_number)?;

    let mut after_kind = header[kind_pos + keyword.len()..].trim();
    let mut return_param = None;
    if keyword == ".func" {
        loop {
            let trimmed = after_kind.trim_start();
            after_kind = trimmed;
            if !after_kind.starts_with('(') {
                break;
            }

            let (section, remainder) = extract_parenthesized_section(after_kind, line_number)?;

            let looks_like_return_param = section
                .split(|ch: char| ch.is_whitespace() || ch == ',')
                .filter(|tok| !tok.is_empty())
                .any(|tok| {
                    matches!(
                        tok.trim_start_matches('.').to_ascii_lowercase().as_str(),
                        "param" | "reg"
                    )
                });

            if looks_like_return_param && return_param.is_none() {
                let params = parse_parameters(&section, line_number)?;
                if params.len() > 1 {
                    return Err(PtxParseError::InvalidFunctionHeader {
                        line: line_number,
                        message: "multiple return values are not supported".into(),
                    });
                }
                if let Some(param) = params.into_iter().next() {
                    return_param = Some(param);
                    after_kind = remainder;
                    continue;
                }
            }

            after_kind = remainder;
        }
    }

    let after_kind = after_kind.trim();
    let (name, params, trailing_after_params) = if let Some(open_paren) = after_kind.find('(') {
        let signature_part = after_kind[..open_paren].trim();
        let name = signature_part
            .split_whitespace()
            .last()
            .ok_or_else(|| PtxParseError::InvalidFunctionHeader {
                line: line_number,
                message: "missing function name".into(),
            })?
            .to_string();

        let close_paren =
            after_kind
                .rfind(')')
                .ok_or_else(|| PtxParseError::InvalidFunctionHeader {
                    line: line_number,
                    message: "missing ')' in function header".into(),
                })?;

        if close_paren <= open_paren {
            return Err(PtxParseError::InvalidFunctionHeader {
                line: line_number,
                message: "malformed parameter list".into(),
            });
        }

        let params_raw = &after_kind[open_paren + 1..close_paren];
        let params = parse_parameters(params_raw, line_number)?;
        let trailing_slice = after_kind[close_paren + 1..].trim();
        let trailing = if trailing_slice.is_empty() {
            None
        } else {
            Some(trailing_slice.to_string())
        };
        (name, params, trailing)
    } else {
        if keyword == ".func" {
            return Err(PtxParseError::InvalidFunctionHeader {
                line: line_number,
                message: ".func directive requires parameter list".into(),
            });
        }
        let mut parts = after_kind.split_whitespace();
        let name_token = parts
            .next()
            .ok_or_else(|| PtxParseError::InvalidFunctionHeader {
                line: line_number,
                message: "missing kernel name".into(),
            })?;
        let remainder: Vec<&str> = parts.collect();
        let trailing = if remainder.is_empty() {
            None
        } else {
            Some(remainder.join(" "))
        };
        (name_token.to_string(), Vec::new(), trailing)
    };

    if let Some(extra_tokens) = trailing_after_params {
        parse_function_header_directives(&extra_tokens, &mut directives, line_number)?;
    }

    Ok((directives, name, params, return_param))
}

fn extract_parenthesized_section<'a>(
    input: &'a str,
    line_number: usize,
) -> Result<(String, &'a str), PtxParseError> {
    debug_assert!(input.starts_with('('));

    let mut depth = 0i32;
    let mut end_idx = None;
    for (idx, ch) in input.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    end_idx = Some(idx);
                    break;
                }
            }
            _ => {}
        }
    }

    let end_idx = end_idx.ok_or_else(|| PtxParseError::InvalidFunctionHeader {
        line: line_number,
        message: "unbalanced parentheses in function header".into(),
    })?;

    let section = input[1..end_idx].to_string();
    let remainder = &input[end_idx + 1..];
    Ok((section, remainder))
}

#[derive(Default)]
struct LinkageAccumulator {
    seen_extern: bool,
    seen_weak: bool,
}

fn parse_function_header_directives(
    segment: &str,
    directives: &mut Vec<FunctionHeaderDirective>,
    line_number: usize,
) -> Result<(), PtxParseError> {
    if segment.trim().is_empty() {
        return Ok(());
    }

    let mut tokens: VecDeque<String> = segment
        .split_whitespace()
        .map(|tok| tok.to_string())
        .collect();
    let mut linkage = LinkageAccumulator::default();

    while let Some(token) = tokens.pop_front() {
        let trimmed = token.trim();
        if trimmed.is_empty() {
            continue;
        }

        if !trimmed.starts_with('.') {
            let cleaned = strip_trailing_delimiters(trimmed);
            return Err(PtxParseError::InvalidFunctionHeader {
                line: line_number,
                message: format!("unexpected token '{}' in function header", cleaned),
            });
        }

        let mut directive_token = trimmed.trim_start_matches('.').to_string();
        let ends_with_semicolon = directive_token.contains(';');
        directive_token = strip_trailing_delimiters(&directive_token);
        let lower = directive_token.to_ascii_lowercase();

        match lower.as_str() {
            "visible" => {
                flush_linkage(&mut linkage, directives);
                directives.push(FunctionHeaderDirective::Visibility(
                    FunctionVisibility::Visible,
                ));
            }
            "hidden" => {
                flush_linkage(&mut linkage, directives);
                directives.push(FunctionHeaderDirective::Visibility(
                    FunctionVisibility::Hidden,
                ));
            }
            "extern" => {
                linkage.seen_extern = true;
            }
            "weak" => {
                linkage.seen_weak = true;
            }
            "weakextern" => {
                flush_linkage(&mut linkage, directives);
                directives.push(FunctionHeaderDirective::Linkage(
                    FunctionLinkage::WeakExtern,
                ));
            }
            "noreturn" => {
                flush_linkage(&mut linkage, directives);
                directives.push(FunctionHeaderDirective::NoReturn);
            }
            "abi_preserve" => {
                flush_linkage(&mut linkage, directives);
                let value = parse_numeric_argument(&mut tokens, line_number, ".abi_preserve")?;
                directives.push(FunctionHeaderDirective::AbiPreserve(value));
            }
            "abi_preserve_control" => {
                flush_linkage(&mut linkage, directives);
                let value =
                    parse_numeric_argument(&mut tokens, line_number, ".abi_preserve_control")?;
                directives.push(FunctionHeaderDirective::AbiPreserveControl(value));
            }
            "maxclusterrank" => {
                flush_linkage(&mut linkage, directives);
                let value = parse_numeric_argument(&mut tokens, line_number, ".maxclusterrank")?;
                directives.push(FunctionHeaderDirective::MaxClusterRank(value));
            }
            "blocksareclusters" => {
                flush_linkage(&mut linkage, directives);
                directives.push(FunctionHeaderDirective::BlocksAreClusters);
            }
            "explicitcluster" => {
                flush_linkage(&mut linkage, directives);
                let dims = parse_dim3_arguments(&mut tokens, line_number, ".explicitcluster")?;
                directives.push(FunctionHeaderDirective::ExplicitCluster(dims));
            }
            "reqnctapercluster" => {
                flush_linkage(&mut linkage, directives);
                let dims = parse_dim3_arguments(&mut tokens, line_number, ".reqnctapercluster")?;
                directives.push(FunctionHeaderDirective::ReqNctaPerCluster(dims));
            }
            "maxnreg" => {
                flush_linkage(&mut linkage, directives);
                let value = parse_numeric_argument(&mut tokens, line_number, ".maxnreg")?;
                directives.push(FunctionHeaderDirective::MaxNReg(value));
            }
            "maxntid" => {
                flush_linkage(&mut linkage, directives);
                let dims = parse_dim3_arguments(&mut tokens, line_number, ".maxntid")?;
                directives.push(FunctionHeaderDirective::MaxNTid(dims));
            }
            "minnctapersm" => {
                flush_linkage(&mut linkage, directives);
                let value = parse_numeric_argument(&mut tokens, line_number, ".minnctapersm")?;
                directives.push(FunctionHeaderDirective::MinNCtaPerSm(value));
            }
            "reqntid" => {
                flush_linkage(&mut linkage, directives);
                let dims = parse_dim3_arguments(&mut tokens, line_number, ".reqntid")?;
                directives.push(FunctionHeaderDirective::ReqNTid(dims));
            }
            "maxnctapersm" => {
                flush_linkage(&mut linkage, directives);
                let value = parse_numeric_argument(&mut tokens, line_number, ".maxnctapersm")?;
                directives.push(FunctionHeaderDirective::MaxNCtaPerSm(value));
            }
            "pragma" => {
                flush_linkage(&mut linkage, directives);
                let args = parse_pragma_arguments(&mut tokens, line_number)?;
                directives.push(FunctionHeaderDirective::Pragma(args));
            }
            _ => {
                let mut original = String::from(".");
                original.push_str(&directive_token);
                return Err(PtxParseError::InvalidFunctionHeader {
                    line: line_number,
                    message: format!("unrecognised function header directive '{}'", original),
                });
            }
        }

        if ends_with_semicolon {
            flush_linkage(&mut linkage, directives);
        }
    }

    flush_linkage(&mut linkage, directives);
    Ok(())
}

fn flush_linkage(linkage: &mut LinkageAccumulator, directives: &mut Vec<FunctionHeaderDirective>) {
    if linkage.seen_extern || linkage.seen_weak {
        let directive = match (linkage.seen_extern, linkage.seen_weak) {
            (true, true) => FunctionLinkage::WeakExtern,
            (true, false) => FunctionLinkage::Extern,
            (false, true) => FunctionLinkage::Weak,
            (false, false) => return,
        };
        directives.push(FunctionHeaderDirective::Linkage(directive));
        *linkage = LinkageAccumulator::default();
    }
}

fn strip_trailing_delimiters(value: &str) -> String {
    value
        .trim_end_matches(|ch| ch == ',' || ch == ';')
        .to_string()
}

fn parse_numeric_argument(
    tokens: &mut VecDeque<String>,
    line_number: usize,
    directive: &str,
) -> Result<u32, PtxParseError> {
    let token = tokens
        .pop_front()
        .ok_or_else(|| PtxParseError::InvalidFunctionHeader {
            line: line_number,
            message: format!("missing numeric argument for {directive}"),
        })?;
    let (value, has_comma) = parse_numeric_token(&token, line_number, directive)?;
    if has_comma {
        return Err(PtxParseError::InvalidFunctionHeader {
            line: line_number,
            message: format!("unexpected ',' after argument for {directive}"),
        });
    }
    Ok(value)
}

fn parse_dim3_arguments(
    tokens: &mut VecDeque<String>,
    line_number: usize,
    directive: &str,
) -> Result<FunctionDim3, PtxParseError> {
    let first = tokens
        .pop_front()
        .ok_or_else(|| PtxParseError::InvalidFunctionHeader {
            line: line_number,
            message: format!("missing arguments for {directive}"),
        })?;
    let (x, mut expect_more) = parse_numeric_token(&first, line_number, directive)?;
    let mut y = None;
    let mut z = None;

    if expect_more {
        let second = tokens
            .pop_front()
            .ok_or_else(|| PtxParseError::InvalidFunctionHeader {
                line: line_number,
                message: format!("missing 'y' argument for {directive}"),
            })?;
        let (value, more) = parse_numeric_token(&second, line_number, directive)?;
        y = Some(value);
        expect_more = more;
    } else {
        expect_more = false;
    }

    if expect_more {
        let third = tokens
            .pop_front()
            .ok_or_else(|| PtxParseError::InvalidFunctionHeader {
                line: line_number,
                message: format!("missing 'z' argument for {directive}"),
            })?;
        let (value, more) = parse_numeric_token(&third, line_number, directive)?;
        if more {
            return Err(PtxParseError::InvalidFunctionHeader {
                line: line_number,
                message: format!("unexpected extra arguments for {directive}"),
            });
        }
        z = Some(value);
    }

    Ok(FunctionDim3 { x, y, z })
}

fn parse_numeric_token(
    token: &str,
    line_number: usize,
    directive: &str,
) -> Result<(u32, bool), PtxParseError> {
    let mut trimmed = token.trim();
    let mut has_semicolon = false;
    if trimmed.ends_with(';') {
        has_semicolon = true;
        trimmed = trimmed.trim_end_matches(';').trim_end();
    }

    let mut has_comma = false;
    if trimmed.ends_with(',') {
        has_comma = true;
        trimmed = trimmed.trim_end_matches(',').trim_end();
    }

    if trimmed.is_empty() {
        return Err(PtxParseError::InvalidFunctionHeader {
            line: line_number,
            message: format!("expected numeric value for {directive}"),
        });
    }

    let value = trimmed
        .parse::<u32>()
        .map_err(|_| PtxParseError::InvalidFunctionHeader {
            line: line_number,
            message: format!("invalid numeric value '{}' in {}", trimmed, directive),
        })?;

    if has_semicolon && has_comma {
        return Err(PtxParseError::InvalidFunctionHeader {
            line: line_number,
            message: format!(
                "unexpected ';' after comma-separated value in {}",
                directive
            ),
        });
    }

    Ok((value, has_comma))
}

fn parse_pragma_arguments(
    tokens: &mut VecDeque<String>,
    line_number: usize,
) -> Result<Vec<String>, PtxParseError> {
    let mut args = Vec::new();
    let mut found_semicolon = false;

    while let Some(token) = tokens.pop_front() {
        let trimmed = token.trim();
        if trimmed.is_empty() {
            continue;
        }
        let has_semicolon = trimmed.contains(';');
        let cleaned = trimmed.trim_end_matches(';').to_string();
        if !cleaned.is_empty() {
            args.push(cleaned);
        }
        if has_semicolon {
            found_semicolon = true;
            break;
        }
    }

    if !found_semicolon {
        return Err(PtxParseError::InvalidFunctionHeader {
            line: line_number,
            message: ".pragma directive missing terminating ';'".into(),
        });
    }

    if args.is_empty() {
        return Err(PtxParseError::InvalidFunctionHeader {
            line: line_number,
            message: "expected arguments after .pragma".into(),
        });
    }

    Ok(args)
}

fn parse_parameters(raw: &str, line_number: usize) -> Result<Vec<Parameter>, PtxParseError> {
    let mut params = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;

    for ch in raw.chars() {
        match ch {
            '(' => {
                depth += 1;
                current.push(ch);
            }
            ')' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                if let Some(param) = finalize_param(&current, line_number)? {
                    params.push(param);
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }

    if !current.trim().is_empty() {
        if let Some(param) = finalize_param(&current, line_number)? {
            params.push(param);
        }
    }

    if depth != 0 {
        return Err(PtxParseError::InvalidFunctionHeader {
            line: line_number,
            message: "unbalanced parentheses in parameter list".into(),
        });
    }

    Ok(params)
}

fn finalize_param(raw: &str, line_number: usize) -> Result<Option<Parameter>, PtxParseError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let mut tokens: Vec<String> = trimmed
        .split_whitespace()
        .map(|tok| tok.trim_matches(',').trim_end_matches(';').to_string())
        .filter(|tok| !tok.is_empty())
        .collect();

    if tokens.is_empty() {
        return Err(PtxParseError::InvalidFunctionHeader {
            line: line_number,
            message: format!("unable to parse parameter '{}': missing name", raw.trim()),
        });
    }

    let name_token = tokens.pop().unwrap();
    let specifiers_list = tokens.clone();
    let (base_name, array_from_name) = split_name_token(&name_token, line_number)?;

    let mut storage = None;
    let mut alignment = None;
    let mut ty = None;
    let mut qualifiers = ParameterQualifiers::default();
    let mut pointer: Option<PointerQualifier> = None;
    let mut idx = 0;

    while idx < tokens.len() {
        let token = tokens[idx].clone();
        let trimmed = token.trim();
        let lower = trimmed.trim_start_matches('.').to_ascii_lowercase();

        match lower.as_str() {
            "param" => {
                storage = Some(ParameterStorage::Param);
                idx += 1;
                continue;
            }
            "align" => {
                let next =
                    tokens
                        .get(idx + 1)
                        .ok_or_else(|| PtxParseError::InvalidFunctionHeader {
                            line: line_number,
                            message: "expected alignment value after .align".into(),
                        })?;
                let value = next.trim_matches(',').parse::<u32>().map_err(|_| {
                    PtxParseError::InvalidFunctionHeader {
                        line: line_number,
                        message: format!("invalid alignment value '{}'", next),
                    }
                })?;
                alignment = Some(value);
                idx += 2;
                continue;
            }
            "const" => {
                if let Some(ptr) = pointer.as_mut() {
                    if ptr.address_space.is_none() {
                        ptr.address_space = Some(PointerAddressSpace::Const);
                        idx += 1;
                        continue;
                    }
                }
                qualifiers.is_const = true;
                idx += 1;
                continue;
            }
            "volatile" => {
                qualifiers.is_volatile = true;
                idx += 1;
                continue;
            }
            "restrict" => {
                qualifiers.is_restrict = true;
                idx += 1;
                continue;
            }
            "noalias" => {
                qualifiers.is_noalias = true;
                idx += 1;
                continue;
            }
            "ptr" => {
                pointer.get_or_insert_with(PointerQualifier::default);
                idx += 1;
                continue;
            }
            "global" => {
                if let Some(ptr) = pointer.as_mut() {
                    ptr.address_space = Some(PointerAddressSpace::Global);
                    idx += 1;
                    continue;
                }
            }
            "shared" => {
                if let Some(ptr) = pointer.as_mut() {
                    ptr.address_space = Some(PointerAddressSpace::Shared);
                    idx += 1;
                    continue;
                }
            }
            "local" => {
                if let Some(ptr) = pointer.as_mut() {
                    ptr.address_space = Some(PointerAddressSpace::Local);
                    idx += 1;
                    continue;
                }
            }
            "reg" => {
                idx += 1;
                continue;
            }
            _ => {}
        }

        if let Some(scalar) = parse_scalar_type(&token) {
            ty = Some(scalar);
        } else {
            return Err(PtxParseError::InvalidFunctionHeader {
                line: line_number,
                message: format!("unrecognised parameter qualifier '{}'", token),
            });
        }
        idx += 1;
    }

    if let Some(ptr) = pointer {
        qualifiers.pointer = Some(ptr);
    }

    Ok(Some(Parameter {
        name: base_name,
        storage,
        alignment,
        ty,
        qualifiers,
        array: array_from_name,
        specifiers: specifiers_list
            .into_iter()
            .map(ParameterSpecifier::new)
            .collect(),
        raw: trimmed.to_string(),
    }))
}

fn parse_function_stmt(
    content: &str,
    comment: Option<String>,
    line_number: usize,
    in_declaration: &mut bool,
) -> Result<FunctionBodyItem, PtxParseError> {
    let trimmed = content.trim();

    if trimmed.ends_with(':') {
        *in_declaration = false;
        return Ok(FunctionBodyItem::Statement(FunctionStatement::Label(
            trimmed.trim_end_matches(':').to_string(),
        )));
    }

    if trimmed.starts_with('.') || trimmed.starts_with("@@") {
        return parse_function_directive_stmt(trimmed, comment, line_number, in_declaration);
    }

    let instruction = parse_instruction(trimmed, comment, line_number)?;
    *in_declaration = false;
    Ok(FunctionBodyItem::Statement(FunctionStatement::Instruction(
        instruction,
    )))
}

fn parse_function_directive_stmt(
    line: &str,
    comment: Option<String>,
    line_number: usize,
    in_declaration: &mut bool,
) -> Result<FunctionBodyItem, PtxParseError> {
    let trimmed = line.trim();
    let without_semicolon = trimmed
        .strip_suffix(';')
        .map(|s| s.trim_end())
        .unwrap_or(trimmed);

    if without_semicolon.is_empty() {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: "empty directive".into(),
        });
    }

    let mut parts = without_semicolon.split_whitespace();
    let keyword = parts.next().unwrap();
    let normalized = keyword.trim_start_matches('.').to_ascii_lowercase();
    let arguments: Vec<String> = parts.map(|tok| tok.to_string()).collect();

    if keyword.starts_with("@@") {
        let directive = DwarfDirective {
            keyword: keyword.to_string(),
            arguments,
            comment,
            raw: trimmed.to_string(),
        };
        if *in_declaration {
            return Ok(FunctionBodyItem::Entry(FunctionEntryDirective::Dwarf(
                directive,
            )));
        }
        *in_declaration = false;
        return Ok(FunctionBodyItem::Statement(FunctionStatement::Directive(
            StatementDirective::Dwarf(directive),
        )));
    }

    match normalized.as_str() {
        "reg" => {
            let directive = parse_register_declaration(
                keyword,
                without_semicolon,
                comment,
                trimmed.to_string(),
                line_number,
            )?;
            Ok(FunctionBodyItem::Entry(FunctionEntryDirective::Reg(
                directive,
            )))
        }
        "local" | "param" | "shared" => {
            let kind = match normalized.as_str() {
                "local" => FunctionDeclarationKind::Local,
                "param" => FunctionDeclarationKind::Param,
                "shared" => FunctionDeclarationKind::Shared,
                _ => unreachable!(),
            };

            let directive = GenericFunctionDeclaration {
                kind,
                keyword: keyword.to_string(),
                arguments,
                comment,
                raw: trimmed.to_string(),
            };

            let entry = match kind {
                FunctionDeclarationKind::Local => FunctionEntryDirective::Local(directive),
                FunctionDeclarationKind::Param => FunctionEntryDirective::Param(directive),
                FunctionDeclarationKind::Shared => FunctionEntryDirective::Shared(directive),
                _ => unreachable!(),
            };

            Ok(FunctionBodyItem::Entry(entry))
        }
        "pragma" => {
            let mut pragma_arguments = Vec::new();
            for token in &arguments {
                pragma_arguments.push(unquote_string(token, line_number)?);
            }

            let directive = PragmaDirective {
                arguments: pragma_arguments,
                comment,
                raw: trimmed.to_string(),
            };

            if *in_declaration {
                Ok(FunctionBodyItem::Entry(FunctionEntryDirective::Pragma(
                    directive,
                )))
            } else {
                *in_declaration = false;
                Ok(FunctionBodyItem::Statement(FunctionStatement::Directive(
                    StatementDirective::Pragma(directive),
                )))
            }
        }
        "loc" => {
            if arguments.len() < 3 {
                return Err(PtxParseError::InvalidDirective {
                    line: line_number,
                    message: ".loc directive expects file, line, and column arguments".into(),
                });
            }

            let file_index =
                arguments[0]
                    .parse::<u32>()
                    .map_err(|_| PtxParseError::InvalidDirective {
                        line: line_number,
                        message: format!("invalid .loc file index '{}'", arguments[0]),
                    })?;
            let line_value =
                arguments[1]
                    .parse::<u32>()
                    .map_err(|_| PtxParseError::InvalidDirective {
                        line: line_number,
                        message: format!("invalid .loc line '{}'", arguments[1]),
                    })?;
            let column_value =
                arguments[2]
                    .parse::<u32>()
                    .map_err(|_| PtxParseError::InvalidDirective {
                        line: line_number,
                        message: format!("invalid .loc column '{}'", arguments[2]),
                    })?;

            let options = if arguments.len() > 3 {
                arguments[3..].to_vec()
            } else {
                Vec::new()
            };

            let directive = LocationDirective {
                file_index,
                line: line_value,
                column: column_value,
                options,
                comment,
                raw: trimmed.to_string(),
            };

            if *in_declaration {
                Ok(FunctionBodyItem::Entry(FunctionEntryDirective::Loc(
                    directive,
                )))
            } else {
                *in_declaration = false;
                Ok(FunctionBodyItem::Statement(FunctionStatement::Directive(
                    StatementDirective::Loc(directive),
                )))
            }
        }
        "section" => {
            if arguments.is_empty() {
                return Err(PtxParseError::InvalidDirective {
                    line: line_number,
                    message: ".section directive requires a name".into(),
                });
            }
            let name = arguments[0].clone();
            let rest = if arguments.len() > 1 {
                arguments[1..].to_vec()
            } else {
                Vec::new()
            };
            let directive = StatementSectionDirective {
                name,
                arguments: rest,
                comment,
                raw: trimmed.to_string(),
            };
            *in_declaration = false;
            Ok(FunctionBodyItem::Statement(FunctionStatement::Directive(
                StatementDirective::Section(directive),
            )))
        }
        _ => Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: format!("unsupported directive '{keyword}'"),
        }),
    }
}

fn parse_instruction(
    line: &str,
    comment: Option<String>,
    line_number: usize,
) -> Result<Instruction, PtxParseError> {
    let raw_line = line.trim().to_string();
    let mut stmt = line.trim_end_matches(';').trim();
    if stmt.is_empty() {
        return Err(PtxParseError::InvalidInstruction {
            line: line_number,
            message: "empty instruction".into(),
        });
    }

    let mut predicate = None;
    if stmt.starts_with('@') {
        if let Some((pred, rest)) = stmt.split_once(' ') {
            predicate = Some(pred.trim_start_matches('@').to_string());
            stmt = rest.trim();
        } else {
            return Err(PtxParseError::InvalidInstruction {
                line: line_number,
                message: "predicate missing instruction".into(),
            });
        }
    }

    let mut parts = stmt.split_whitespace();
    let opcode_raw = parts
        .next()
        .ok_or_else(|| PtxParseError::InvalidInstruction {
            line: line_number,
            message: "missing opcode".into(),
        })?;

    let opcode_text = opcode_raw
        .split('.')
        .next()
        .unwrap_or(opcode_raw)
        .to_string();

    let modifier_texts: Vec<String> = opcode_raw
        .split('.')
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let opcode_kind =
        classify_opcode(&opcode_text).ok_or_else(|| PtxParseError::InvalidInstruction {
            line: line_number,
            message: format!("unknown opcode '{opcode_text}'"),
        })?;
    let modifiers = parse_instruction_modifiers(&modifier_texts, line_number)?;

    let operands_raw = parts.collect::<Vec<_>>().join(" ");
    let operands = parse_operands(&operands_raw, line_number)?;

    Ok(Instruction {
        predicate,
        opcode: InstructionOpcode {
            kind: opcode_kind,
            modifiers,
        },
        operands,
        comment,
        raw: raw_line,
    })
}

fn parse_instruction_modifiers(
    modifier_texts: &[String],
    line_number: usize,
) -> Result<Vec<ModifierKind>, PtxParseError> {
    let mut modifiers = Vec::new();
    let mut index = 0;

    while index < modifier_texts.len() {
        let current = &modifier_texts[index];
        let trimmed = current.trim_start_matches('.');
        let lower = trimmed.to_ascii_lowercase();
        let primary = match lower.split("::").next() {
            Some(token) if !token.is_empty() => token,
            _ => lower.as_str(),
        };

        if trimmed.eq_ignore_ascii_case("to") {
            if let Some(next) = modifier_texts.get(index + 1) {
                let next_trimmed = next.trim_start_matches('.');
                if let Some(space) =
                    parse_address_space_modifier(&next_trimmed.to_ascii_lowercase())
                {
                    modifiers.push(ModifierKind::Conversion(space));
                    index += 2;
                    continue;
                }
            }

            return Err(PtxParseError::InvalidInstruction {
                line: line_number,
                message: "conversion modifier missing address space".into(),
            });
        }

        if let Some(modifier) = classify_modifier(primary) {
            modifiers.push(modifier);
            index += 1;
            continue;
        }

        if let Some(width) = parse_vector_width_modifier(primary) {
            modifiers.push(ModifierKind::VectorWidth(width));
            index += 1;
            continue;
        }

        if primary == "wide" {
            modifiers.push(ModifierKind::Wide);
            index += 1;
            continue;
        }

        return Err(PtxParseError::InvalidInstruction {
            line: line_number,
            message: format!("unrecognised modifier '{}'", current),
        });
    }

    Ok(modifiers)
}

fn parse_operands(raw: &str, line_number: usize) -> Result<Vec<Operand>, PtxParseError> {
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }

    let mut operands = Vec::new();
    let mut current = String::new();
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;

    let push_current =
        |store: &mut Vec<Operand>, token: &mut String| -> Result<(), PtxParseError> {
            let trimmed = token.trim().trim_end_matches(';').trim();
            if !trimmed.is_empty() {
                store.push(parse_operand(trimmed, line_number)?);
            }
            token.clear();
            Ok(())
        };

    for ch in raw.chars() {
        match ch {
            ',' if paren_depth == 0 && bracket_depth == 0 => {
                push_current(&mut operands, &mut current)?;
            }
            '(' => {
                paren_depth += 1;
                current.push(ch);
            }
            ')' => {
                if paren_depth == 0 {
                    return Err(PtxParseError::InvalidInstruction {
                        line: line_number,
                        message: "unmatched ')' in operand list".into(),
                    });
                }
                paren_depth -= 1;
                current.push(ch);
            }
            '[' => {
                bracket_depth += 1;
                current.push(ch);
            }
            ']' => {
                if bracket_depth == 0 {
                    return Err(PtxParseError::InvalidInstruction {
                        line: line_number,
                        message: "unmatched ']' in operand list".into(),
                    });
                }
                bracket_depth -= 1;
                current.push(ch);
            }
            _ => current.push(ch),
        }
    }

    if paren_depth != 0 {
        return Err(PtxParseError::InvalidInstruction {
            line: line_number,
            message: "unterminated '(' in operand list".into(),
        });
    }

    if bracket_depth != 0 {
        return Err(PtxParseError::InvalidInstruction {
            line: line_number,
            message: "unterminated '[' in operand list".into(),
        });
    }

    push_current(&mut operands, &mut current)?;

    Ok(operands)
}

fn parse_operand(token: &str, line_number: usize) -> Result<Operand, PtxParseError> {
    if token.is_empty() {
        return Err(PtxParseError::InvalidInstruction {
            line: line_number,
            message: "empty operand".into(),
        });
    }

    if token.starts_with('[') && token.ends_with(']') {
        return parse_memory_operand(token, line_number);
    }

    if token.starts_with('(') && token.ends_with(')') {
        let inner = token[1..token.len() - 1].trim();
        let items = if inner.is_empty() {
            Vec::new()
        } else {
            inner
                .split(',')
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .map(|part| part.to_string())
                .collect()
        };
        return Ok(Operand::Parenthesized(items));
    }

    if let Some(paren_index) = token.find('(') {
        if paren_index > 0 && token.ends_with(')') {
            let name_part = token[..paren_index].trim();
            if is_symbol_token(name_part) {
                let args_inner = token[paren_index + 1..token.len() - 1].trim();
                let arguments = if args_inner.is_empty() {
                    Vec::new()
                } else {
                    args_inner
                        .split(',')
                        .map(|arg| arg.trim())
                        .filter(|arg| !arg.is_empty())
                        .map(|arg| arg.to_string())
                        .collect()
                };
                return Ok(Operand::CallTarget {
                    name: name_part.to_string(),
                    arguments,
                });
            }
        }
    }

    if token.starts_with('%') {
        return Ok(Operand::Register(token.to_string()));
    }

    if is_numeric_literal(token) {
        return Ok(Operand::Immediate(token.to_string()));
    }

    if is_symbol_token(token) {
        return Ok(Operand::Symbol(token.to_string()));
    }

    Err(PtxParseError::InvalidInstruction {
        line: line_number,
        message: format!("unknown operand '{token}'"),
    })
}

fn parse_memory_operand(token: &str, line_number: usize) -> Result<Operand, PtxParseError> {
    if token.len() < 2 {
        return Err(PtxParseError::InvalidInstruction {
            line: line_number,
            message: format!("invalid memory operand: {token}"),
        });
    }

    let inner = token[1..token.len() - 1].trim();
    if inner.is_empty() {
        return Err(PtxParseError::InvalidInstruction {
            line: line_number,
            message: "memory operand missing address expression".into(),
        });
    }

    let tokens = tokenize_address(inner, line_number)?;
    let memory = build_memory_operand(&tokens, line_number)?;

    Ok(Operand::Memory(memory))
}

#[derive(Debug, Clone)]
enum AddressToken {
    Register(String),
    Symbol(String),
    Immediate(String),
    Plus,
    Minus,
    Star,
}

fn tokenize_address(expr: &str, line_number: usize) -> Result<Vec<AddressToken>, PtxParseError> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    let flush_current =
        |current: &mut String, tokens: &mut Vec<AddressToken>| -> Result<(), PtxParseError> {
            if current.is_empty() {
                return Ok(());
            }
            let token = classify_address_token(current.as_str(), line_number)?;
            tokens.push(token);
            current.clear();
            Ok(())
        };

    for ch in expr.chars() {
        match ch {
            '+' => {
                flush_current(&mut current, &mut tokens)?;
                tokens.push(AddressToken::Plus);
            }
            '-' => {
                flush_current(&mut current, &mut tokens)?;
                tokens.push(AddressToken::Minus);
            }
            '*' => {
                flush_current(&mut current, &mut tokens)?;
                tokens.push(AddressToken::Star);
            }
            ch if ch.is_whitespace() => {
                flush_current(&mut current, &mut tokens)?;
            }
            _ => current.push(ch),
        }
    }

    flush_current(&mut current, &mut tokens)?;

    if tokens.is_empty() {
        return Err(PtxParseError::InvalidInstruction {
            line: line_number,
            message: "empty memory address expression".into(),
        });
    }

    Ok(tokens)
}

fn classify_address_token(token: &str, line_number: usize) -> Result<AddressToken, PtxParseError> {
    if token.starts_with('%') && token.len() > 1 {
        return Ok(AddressToken::Register(token.to_string()));
    }
    if is_numeric_literal(token) {
        return Ok(AddressToken::Immediate(token.to_string()));
    }
    if is_symbol_token(token) {
        return Ok(AddressToken::Symbol(token.to_string()));
    }

    Err(PtxParseError::InvalidInstruction {
        line: line_number,
        message: format!("invalid memory address token '{token}'"),
    })
}

fn build_memory_operand(
    tokens: &[AddressToken],
    line_number: usize,
) -> Result<MemoryOperand, PtxParseError> {
    let mut base: Option<AddressBase> = None;
    let mut displacements = Vec::new();

    let mut pending_sign = AddressSign::Positive;
    let mut sign_set = false;

    let mut index = 0;
    while index < tokens.len() {
        match &tokens[index] {
            AddressToken::Plus => {
                if sign_set {
                    // multiple plus operators keep the current sign
                } else {
                    pending_sign = AddressSign::Positive;
                    sign_set = true;
                }
                index += 1;
            }
            AddressToken::Minus => {
                if sign_set {
                    pending_sign = pending_sign.negate();
                } else {
                    pending_sign = AddressSign::Negative;
                    sign_set = true;
                }
                index += 1;
            }
            AddressToken::Star => {
                return Err(PtxParseError::InvalidInstruction {
                    line: line_number,
                    message: "unexpected '*' in memory address expression".into(),
                });
            }
            AddressToken::Register(name) => {
                let sign = if sign_set {
                    pending_sign
                } else {
                    AddressSign::Positive
                };
                pending_sign = AddressSign::Positive;
                sign_set = false;

                let mut scale = None;
                if index + 1 < tokens.len() {
                    if matches!(tokens[index + 1], AddressToken::Star) {
                        if index + 2 >= tokens.len() {
                            return Err(PtxParseError::InvalidInstruction {
                                line: line_number,
                                message: "missing scale after '*' in memory address".into(),
                            });
                        }
                        match &tokens[index + 2] {
                            AddressToken::Immediate(value) => {
                                scale = Some(value.clone());
                                index += 2;
                            }
                            _ => {
                                return Err(PtxParseError::InvalidInstruction {
                                    line: line_number,
                                    message: "expected immediate scale after '*' in memory address"
                                        .into(),
                                });
                            }
                        }
                    }
                }

                let displacement = AddressDisplacement {
                    sign,
                    kind: AddressDisplacementKind::Register {
                        register: name.clone(),
                        scale: scale.clone(),
                    },
                };

                let can_be_base =
                    base.is_none() && matches!(sign, AddressSign::Positive) && scale.is_none();
                if can_be_base {
                    base = Some(AddressBase::Register(name.clone()));
                } else {
                    displacements.push(displacement);
                }

                index += 1;
            }
            AddressToken::Symbol(name) => {
                let sign = if sign_set {
                    pending_sign
                } else {
                    AddressSign::Positive
                };
                pending_sign = AddressSign::Positive;
                sign_set = false;

                let displacement = AddressDisplacement {
                    sign,
                    kind: AddressDisplacementKind::Symbol(name.clone()),
                };

                if base.is_none() && matches!(sign, AddressSign::Positive) {
                    base = Some(AddressBase::Symbol(name.clone()));
                } else {
                    displacements.push(displacement);
                }

                index += 1;
            }
            AddressToken::Immediate(value) => {
                let sign = if sign_set {
                    pending_sign
                } else {
                    AddressSign::Positive
                };
                pending_sign = AddressSign::Positive;
                sign_set = false;

                displacements.push(AddressDisplacement {
                    sign,
                    kind: AddressDisplacementKind::Immediate(value.clone()),
                });

                index += 1;
            }
        }
    }

    if sign_set {
        return Err(PtxParseError::InvalidInstruction {
            line: line_number,
            message: "dangling sign in memory address expression".into(),
        });
    }

    Ok(MemoryOperand {
        base,
        displacements,
    })
}

fn classify_opcode(opcode: &str) -> Option<OpcodeKind> {
    match opcode.to_ascii_lowercase().as_str() {
        "abs" => Some(OpcodeKind::Abs),
        "activemask" => Some(OpcodeKind::Activemask),
        "add" => Some(OpcodeKind::Add),
        "addc" => Some(OpcodeKind::Addc),
        "alloca" => Some(OpcodeKind::Alloca),
        "and" => Some(OpcodeKind::And),
        "applypriority" => Some(OpcodeKind::Applypriority),
        "atom" => Some(OpcodeKind::Atom),
        "bar" => Some(OpcodeKind::Bar),
        "barrier" => Some(OpcodeKind::Barrier),
        "bfe" => Some(OpcodeKind::Bfe),
        "bfi" => Some(OpcodeKind::Bfi),
        "bfind" => Some(OpcodeKind::Bfind),
        "bmsk" => Some(OpcodeKind::Bmsk),
        "brev" => Some(OpcodeKind::Brev),
        "bra" => Some(OpcodeKind::Bra),
        "brkpt" => Some(OpcodeKind::Brkpt),
        "brx" => Some(OpcodeKind::Brx),
        "call" => Some(OpcodeKind::Call),
        "clz" => Some(OpcodeKind::Clz),
        "clusterlaunchcontrol" => Some(OpcodeKind::Clusterlaunchcontrol),
        "cnot" => Some(OpcodeKind::Cnot),
        "copysign" => Some(OpcodeKind::Copysign),
        "cos" => Some(OpcodeKind::Cos),
        "cp" => Some(OpcodeKind::Cp),
        "createpolicy" => Some(OpcodeKind::Createpolicy),
        "cvt" => Some(OpcodeKind::Cvt),
        "cvta" => Some(OpcodeKind::Cvta),
        "div" => Some(OpcodeKind::Div),
        "discard" => Some(OpcodeKind::Discard),
        "dp2a" => Some(OpcodeKind::Dp2a),
        "dp4a" => Some(OpcodeKind::Dp4a),
        "elect" => Some(OpcodeKind::Elect),
        "ex2" => Some(OpcodeKind::Ex2),
        "exit" => Some(OpcodeKind::Exit),
        "fence" => Some(OpcodeKind::Fence),
        "fma" => Some(OpcodeKind::Fma),
        "fns" => Some(OpcodeKind::Fns),
        "getctarank" => Some(OpcodeKind::Getctarank),
        "griddepcontrol" => Some(OpcodeKind::Griddepcontrol),
        "isspacep" => Some(OpcodeKind::Isspacep),
        "istypep" => Some(OpcodeKind::Istypep),
        "ld" => Some(OpcodeKind::Ld),
        "ldmatrix" => Some(OpcodeKind::Ldmatrix),
        "ldu" => Some(OpcodeKind::Ldu),
        "lg2" => Some(OpcodeKind::Lg2),
        "lop3" => Some(OpcodeKind::Lop3),
        "mad" => Some(OpcodeKind::Mad),
        "mad24" => Some(OpcodeKind::Mad24),
        "madc" => Some(OpcodeKind::Madc),
        "mapa" => Some(OpcodeKind::Mapa),
        "match" => Some(OpcodeKind::Match),
        "max" => Some(OpcodeKind::Max),
        "mbarrier" => Some(OpcodeKind::Mbarrier),
        "membar" => Some(OpcodeKind::Membar),
        "min" => Some(OpcodeKind::Min),
        "mov" => Some(OpcodeKind::Mov),
        "movmatrix" => Some(OpcodeKind::Movmatrix),
        "mma" => Some(OpcodeKind::Mma),
        "mul" => Some(OpcodeKind::Mul),
        "mul24" => Some(OpcodeKind::Mul24),
        "multimem" => Some(OpcodeKind::Multimem),
        "nanosleep" => Some(OpcodeKind::Nanosleep),
        "neg" => Some(OpcodeKind::Neg),
        "not" => Some(OpcodeKind::Not),
        "or" => Some(OpcodeKind::Or),
        "pmevent" => Some(OpcodeKind::Pmevent),
        "popc" => Some(OpcodeKind::Popc),
        "prefetch" => Some(OpcodeKind::Prefetch),
        "prefetchu" => Some(OpcodeKind::Prefetchu),
        "prmt" => Some(OpcodeKind::Prmt),
        "rcp" => Some(OpcodeKind::Rcp),
        "red" => Some(OpcodeKind::Red),
        "redux" => Some(OpcodeKind::Redux),
        "rem" => Some(OpcodeKind::Rem),
        "rsqrt" => Some(OpcodeKind::Rsqrt),
        "sad" => Some(OpcodeKind::Sad),
        "selp" => Some(OpcodeKind::Selp),
        "set" => Some(OpcodeKind::Set),
        "setmaxnreg" => Some(OpcodeKind::Setmaxnreg),
        "setp" => Some(OpcodeKind::Setp),
        "shf" => Some(OpcodeKind::Shf),
        "shfl" => Some(OpcodeKind::Shfl),
        "shl" => Some(OpcodeKind::Shl),
        "shr" => Some(OpcodeKind::Shr),
        "sin" => Some(OpcodeKind::Sin),
        "slct" => Some(OpcodeKind::Slct),
        "sqrt" => Some(OpcodeKind::Sqrt),
        "stackrestore" => Some(OpcodeKind::Stackrestore),
        "stacksave" => Some(OpcodeKind::Stacksave),
        "st" => Some(OpcodeKind::St),
        "stmatrix" => Some(OpcodeKind::Stmatrix),
        "sub" => Some(OpcodeKind::Sub),
        "subc" => Some(OpcodeKind::Subc),
        "suq" => Some(OpcodeKind::Suq),
        "suld" => Some(OpcodeKind::Suld),
        "sured" => Some(OpcodeKind::Sured),
        "sust" => Some(OpcodeKind::Sust),
        "szext" => Some(OpcodeKind::Szext),
        "tanh" => Some(OpcodeKind::Tanh),
        "tcgen05" => Some(OpcodeKind::Tcgen05),
        "tensormap" => Some(OpcodeKind::Tensormap),
        "tex" => Some(OpcodeKind::Tex),
        "testp" => Some(OpcodeKind::Testp),
        "tld4" => Some(OpcodeKind::Tld4),
        "trap" => Some(OpcodeKind::Trap),
        "txq" => Some(OpcodeKind::Txq),
        "vabsdiff" => Some(OpcodeKind::Vabsdiff),
        "vabsdiff2" => Some(OpcodeKind::Vabsdiff2),
        "vabsdiff4" => Some(OpcodeKind::Vabsdiff4),
        "vadd" => Some(OpcodeKind::Vadd),
        "vadd2" => Some(OpcodeKind::Vadd2),
        "vadd4" => Some(OpcodeKind::Vadd4),
        "vavrg2" => Some(OpcodeKind::Vavrg2),
        "vavrg4" => Some(OpcodeKind::Vavrg4),
        "vmad" => Some(OpcodeKind::Vmad),
        "vmax" => Some(OpcodeKind::Vmax),
        "vmax2" => Some(OpcodeKind::Vmax2),
        "vmax4" => Some(OpcodeKind::Vmax4),
        "vmin" => Some(OpcodeKind::Vmin),
        "vmin2" => Some(OpcodeKind::Vmin2),
        "vmin4" => Some(OpcodeKind::Vmin4),
        "vset" => Some(OpcodeKind::Vset),
        "vset2" => Some(OpcodeKind::Vset2),
        "vset4" => Some(OpcodeKind::Vset4),
        "vshl" => Some(OpcodeKind::Vshl),
        "vshr" => Some(OpcodeKind::Vshr),
        "vsub" => Some(OpcodeKind::Vsub),
        "vsub2" => Some(OpcodeKind::Vsub2),
        "vsub4" => Some(OpcodeKind::Vsub4),
        "vote" => Some(OpcodeKind::Vote),
        "wgmma" => Some(OpcodeKind::Wgmma),
        "wmma" => Some(OpcodeKind::Wmma),
        "xor" => Some(OpcodeKind::Xor),
        "ret" => Some(OpcodeKind::Ret),
        _ => None,
    }
}

fn classify_modifier(lower: &str) -> Option<ModifierKind> {
    if let Some(space) = parse_address_space_modifier(lower) {
        return Some(ModifierKind::AddressSpace(space));
    }
    if let Some(ty) = parse_type_modifier(lower) {
        return Some(ModifierKind::Type(ty));
    }
    if let Some(cond) = parse_condition_modifier(lower) {
        return Some(ModifierKind::Condition(cond));
    }
    if let Some(rounding) = parse_rounding_modifier(lower) {
        return Some(ModifierKind::Rounding(rounding));
    }
    if let Some(mode) = parse_math_mode_modifier(lower) {
        return Some(ModifierKind::MathMode(mode));
    }
    if let Some(sync) = parse_synchronization_modifier(lower) {
        return Some(ModifierKind::Synchronization(sync));
    }
    if let Some(group) = parse_async_group_modifier(lower) {
        return Some(ModifierKind::AsyncGroup(group));
    }
    if let Some(shuffle) = parse_shuffle_modifier(lower) {
        return Some(ModifierKind::Shuffle(shuffle));
    }
    if let Some(cache) = parse_cache_modifier(lower) {
        return Some(ModifierKind::Cache(cache));
    }
    if let Some(scope) = parse_scope_modifier(lower) {
        return Some(ModifierKind::Scope(scope));
    }
    if let Some(atom) = parse_atomic_modifier(lower) {
        return Some(ModifierKind::Atomic(atom));
    }
    if let Some(call) = parse_call_modifier(lower) {
        return Some(ModifierKind::Call(call));
    }
    if let Some(order) = parse_memory_order_modifier(lower) {
        return Some(ModifierKind::MemoryOrder(order));
    }

    None
}

fn parse_type_modifier(lower: &str) -> Option<TypeModifier> {
    match lower {
        "f16" => Some(TypeModifier::F16),
        "f32" => Some(TypeModifier::F32),
        "f64" => Some(TypeModifier::F64),
        "f128" => Some(TypeModifier::F128),
        "b8" => Some(TypeModifier::B8),
        "b16" => Some(TypeModifier::B16),
        "b32" => Some(TypeModifier::B32),
        "b64" => Some(TypeModifier::B64),
        "s8" => Some(TypeModifier::S8),
        "s16" => Some(TypeModifier::S16),
        "s32" => Some(TypeModifier::S32),
        "s64" => Some(TypeModifier::S64),
        "u8" => Some(TypeModifier::U8),
        "u16" => Some(TypeModifier::U16),
        "u32" => Some(TypeModifier::U32),
        "u64" => Some(TypeModifier::U64),
        "pred" => Some(TypeModifier::Pred),
        _ => None,
    }
}

fn parse_condition_modifier(lower: &str) -> Option<ConditionModifier> {
    match lower {
        "eq" => Some(ConditionModifier::Eq),
        "ne" => Some(ConditionModifier::Ne),
        "lt" => Some(ConditionModifier::Lt),
        "le" => Some(ConditionModifier::Le),
        "gt" => Some(ConditionModifier::Gt),
        "ge" => Some(ConditionModifier::Ge),
        "lo" => Some(ConditionModifier::Lo),
        "hi" => Some(ConditionModifier::Hi),
        "ls" => Some(ConditionModifier::Ls),
        "hs" => Some(ConditionModifier::Hs),
        _ => None,
    }
}

fn parse_address_space_modifier(lower: &str) -> Option<StateSpaceModifier> {
    match lower {
        "param" => Some(StateSpaceModifier::Param),
        "global" => Some(StateSpaceModifier::Global),
        "local" => Some(StateSpaceModifier::Local),
        "shared" => Some(StateSpaceModifier::Shared),
        "const" => Some(StateSpaceModifier::Const),
        "generic" => Some(StateSpaceModifier::Generic),
        _ => None,
    }
}

fn parse_rounding_modifier(lower: &str) -> Option<RoundingModifier> {
    match lower {
        "rn" => Some(RoundingModifier::Rn),
        "rz" => Some(RoundingModifier::Rz),
        "rm" => Some(RoundingModifier::Rm),
        "rp" => Some(RoundingModifier::Rp),
        _ => None,
    }
}

fn parse_math_mode_modifier(lower: &str) -> Option<MathModeModifier> {
    match lower {
        "approx" => Some(MathModeModifier::Approx),
        "full" => Some(MathModeModifier::Full),
        _ => None,
    }
}

fn parse_synchronization_modifier(lower: &str) -> Option<SynchronizationModifier> {
    match lower {
        "sync" => Some(SynchronizationModifier::Sync),
        "async" => Some(SynchronizationModifier::Async),
        _ => None,
    }
}

fn parse_async_group_modifier(lower: &str) -> Option<AsyncGroupModifier> {
    match lower {
        "commit_group" => Some(AsyncGroupModifier::CommitGroup),
        "wait_group" => Some(AsyncGroupModifier::WaitGroup),
        _ => None,
    }
}

fn parse_shuffle_modifier(lower: &str) -> Option<ShuffleModifier> {
    match lower {
        "bfly" => Some(ShuffleModifier::Bfly),
        "down" => Some(ShuffleModifier::Down),
        "up" => Some(ShuffleModifier::Up),
        "idx" => Some(ShuffleModifier::Idx),
        _ => None,
    }
}

fn parse_cache_modifier(lower: &str) -> Option<CacheModifier> {
    match lower {
        "nc" => Some(CacheModifier::Nc),
        "ca" => Some(CacheModifier::Ca),
        "cg" => Some(CacheModifier::Cg),
        "cs" => Some(CacheModifier::Cs),
        "lu" => Some(CacheModifier::Lu),
        _ => None,
    }
}

fn parse_scope_modifier(lower: &str) -> Option<MemoryScopeModifier> {
    match lower {
        "cta" => Some(MemoryScopeModifier::Cta),
        "gl" => Some(MemoryScopeModifier::Gl),
        "gpu" => Some(MemoryScopeModifier::Gpu),
        "sys" => Some(MemoryScopeModifier::Sys),
        _ => None,
    }
}

fn parse_atomic_modifier(lower: &str) -> Option<AtomicOperationModifier> {
    match lower {
        "cas" => Some(AtomicOperationModifier::Cas),
        "add" => Some(AtomicOperationModifier::Add),
        "inc" => Some(AtomicOperationModifier::Inc),
        "dec" => Some(AtomicOperationModifier::Dec),
        "exch" => Some(AtomicOperationModifier::Exch),
        "min" => Some(AtomicOperationModifier::Min),
        "max" => Some(AtomicOperationModifier::Max),
        "and" => Some(AtomicOperationModifier::And),
        "or" => Some(AtomicOperationModifier::Or),
        "xor" => Some(AtomicOperationModifier::Xor),
        _ => None,
    }
}

fn parse_call_modifier(lower: &str) -> Option<CallModifier> {
    match lower {
        "uni" => Some(CallModifier::Uni),
        _ => None,
    }
}

fn parse_vector_width_modifier(lower: &str) -> Option<u32> {
    lower
        .strip_prefix('v')
        .and_then(|value| value.parse::<u32>().ok())
}

fn parse_memory_order_modifier(lower: &str) -> Option<MemoryOrderModifier> {
    match lower {
        "relaxed" => Some(MemoryOrderModifier::Relaxed),
        "acquire" => Some(MemoryOrderModifier::Acquire),
        "release" => Some(MemoryOrderModifier::Release),
        "acq_rel" | "acqrel" => Some(MemoryOrderModifier::AcqRel),
        "sc" => Some(MemoryOrderModifier::Sc),
        _ => None,
    }
}

fn is_numeric_literal(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }

    if token.starts_with("0x") || token.starts_with("0X") {
        return token.len() > 2 && token[2..].chars().all(|c| c.is_ascii_hexdigit());
    }

    if token.len() > 2 {
        let prefix = &token[..2];
        if matches!(prefix, "0d" | "0D" | "0f" | "0F") {
            return token[2..].chars().all(|c| c.is_ascii_hexdigit());
        }
    }

    if token.starts_with('-') || token.starts_with('+') {
        return is_numeric_literal(&token[1..]);
    }

    token.parse::<i64>().is_ok() || token.parse::<u64>().is_ok() || token.parse::<f64>().is_ok()
}

fn is_symbol_token(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }

    let mut chars = token.chars();
    match chars.next() {
        Some(ch) if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' || ch == '.' => {}
        _ => return false,
    }

    chars.all(|ch| {
        ch.is_ascii_alphanumeric() || matches!(ch, '_' | '$' | '.' | ':' | '<' | '>' | '?' | '@')
    })
}

fn split_comment(line: &str) -> (String, Option<String>) {
    if let Some(pos) = line.find("//") {
        let (content, comment) = line.split_at(pos);
        (
            content.trim_end().to_string(),
            Some(comment[2..].trim().to_string()),
        )
    } else if let Some(pos) = line.find('#') {
        let (content, comment) = line.split_at(pos);
        (
            content.trim_end().to_string(),
            Some(comment[1..].trim().to_string()),
        )
    } else {
        (line.to_string(), None)
    }
}

fn strip_comments(line: &str) -> String {
    if let Some(pos) = line.find("//") {
        line[..pos].trim().to_string()
    } else if let Some(pos) = line.find('#') {
        line[..pos].trim().to_string()
    } else {
        line.trim().to_string()
    }
}

fn count_occurrences(line: &str, ch: char) -> usize {
    line.chars().filter(|c| *c == ch).count()
}

fn contains_keyword(line: &str, keyword: &str) -> bool {
    line.split_whitespace().any(|token| token == keyword)
}

fn likely_state_space(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with(".tex")
        || trimmed.contains(".global")
        || trimmed.contains(".const")
        || trimmed.contains(".shared")
}

fn split_initializer(line: &str) -> (&str, Option<&str>) {
    if let Some(idx) = line.find('=') {
        let (left, right) = line.split_at(idx);
        (left.trim_end(), Some(right[1..].trim_start()))
    } else {
        (line, None)
    }
}

fn parse_target_directive(
    rest: &str,
    line_number: usize,
) -> Result<TargetDirective, PtxParseError> {
    if rest.is_empty() {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: ".target directive expects at least one argument".into(),
        });
    }

    let mut entries = Vec::new();
    for entry in rest.split(',') {
        let token = entry.trim();
        if token.is_empty() {
            continue;
        }
        if !is_valid_target_entry(token) {
            return Err(PtxParseError::InvalidDirective {
                line: line_number,
                message: format!("invalid .target entry '{token}'"),
            });
        }
        entries.push(token.to_string());
    }

    if entries.is_empty() {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: "unable to parse arguments for .target directive".into(),
        });
    }

    Ok(TargetDirective {
        entries,
        raw: rest.to_string(),
    })
}

fn parse_alias(line: &str, line_number: usize) -> Result<FunctionAlias, PtxParseError> {
    let trimmed = line.trim();
    if !trimmed.ends_with(';') {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: ".alias directive must end with ';'".into(),
        });
    }

    let body = trimmed
        .trim_end_matches(';')
        .trim_start_matches(".alias")
        .trim();

    let mut parts = body
        .split(',')
        .map(|part| part.trim())
        .filter(|part| !part.is_empty());

    let alias = parts
        .next()
        .ok_or_else(|| PtxParseError::InvalidDirective {
            line: line_number,
            message: "missing alias name in .alias directive".into(),
        })?;
    let target = parts
        .next()
        .ok_or_else(|| PtxParseError::InvalidDirective {
            line: line_number,
            message: "missing aliasee in .alias directive".into(),
        })?;

    if parts.next().is_some() {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: "too many operands in .alias directive".into(),
        });
    }

    if alias.is_empty() || target.is_empty() {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: "invalid operands in .alias directive".into(),
        });
    }

    Ok(FunctionAlias {
        alias: alias.to_string(),
        target: target.to_string(),
        raw: trimmed.to_string(),
    })
}

fn is_valid_target_entry(entry: &str) -> bool {
    const BASE_ENTRIES: &[&str] = &[
        "sm_120a",
        "sm_120f",
        "sm_120",
        "sm_121a",
        "sm_121f",
        "sm_121",
        "sm_110a",
        "sm_110f",
        "sm_110",
        "sm_100a",
        "sm_100f",
        "sm_100",
        "sm_101a",
        "sm_101f",
        "sm_101",
        "sm_103a",
        "sm_103f",
        "sm_103",
        "sm_90a",
        "sm_90",
        "sm_80",
        "sm_86",
        "sm_87",
        "sm_88",
        "sm_89",
        "sm_70",
        "sm_72",
        "sm_75",
        "sm_60",
        "sm_61",
        "sm_62",
        "sm_50",
        "sm_52",
        "sm_53",
        "sm_30",
        "sm_32",
        "sm_35",
        "sm_37",
        "sm_20",
        "sm_10",
        "sm_11",
        "sm_12",
        "sm_13",
        "texmode_unified",
        "texmode_independent",
        "debug",
        "map_f64_to_f32",
    ];

    BASE_ENTRIES.contains(&entry)
}

#[derive(Debug)]
struct ParsedStateVariable {
    visibility: Option<GlobalVisibility>,
    linkages: Vec<GlobalLinkage>,
    address_space: Option<GlobalAddressSpace>,
    mutability: Option<GlobalMutability>,
    alignment: Option<u32>,
    ty: Option<ScalarType>,
    qualifiers: Vec<VariableQualifier>,
    name: String,
    array: Option<ArraySpecifier>,
    initializer: Option<GlobalInitializer>,
    raw: String,
}

impl ParsedStateVariable {
    fn into_variable(mut self, space: GlobalAddressSpace) -> VariableDirective {
        self.address_space = Some(space);
        VariableDirective {
            visibility: self.visibility,
            linkages: self.linkages,
            address_space: self.address_space,
            mutability: self.mutability,
            alignment: self.alignment,
            ty: self.ty,
            qualifiers: self.qualifiers,
            name: self.name,
            array: self.array,
            initializer: self.initializer,
            raw: self.raw,
        }
    }
}

fn parse_register_declaration(
    keyword: &str,
    directive_body: &str,
    comment: Option<String>,
    raw: String,
    line_number: usize,
) -> Result<RegisterDeclaration, PtxParseError> {
    let rest = directive_body
        .strip_prefix(keyword)
        .unwrap_or(directive_body)
        .trim_start();

    if rest.is_empty() {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: format!("missing register type in '{keyword}' directive"),
        });
    }

    let (type_token, registers_part) =
        if let Some((idx, _)) = rest.char_indices().find(|(_, ch)| ch.is_whitespace()) {
            let (ty, remainder) = rest.split_at(idx);
            (ty, remainder.trim_start())
        } else {
            return Err(PtxParseError::InvalidDirective {
                line: line_number,
                message: format!("missing register list in '{keyword}' directive"),
            });
        };

    if type_token.is_empty() {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: format!("missing register type in '{keyword}' directive"),
        });
    }

    if registers_part.is_empty() {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: format!("missing registers in '{keyword}' directive"),
        });
    }

    let ty = RegisterType {
        scalar: parse_scalar_type(type_token),
        raw: type_token.to_string(),
    };

    let mut registers = Vec::new();
    for spec in registers_part.split(',') {
        let trimmed = spec.trim();
        if trimmed.is_empty() {
            continue;
        }
        registers.push(parse_register_specifier(trimmed, line_number)?);
    }

    if registers.is_empty() {
        return Err(PtxParseError::InvalidDirective {
            line: line_number,
            message: format!("no registers listed in '{keyword}' directive"),
        });
    }

    Ok(RegisterDeclaration {
        keyword: keyword.to_string(),
        ty,
        registers,
        comment,
        raw,
    })
}

fn parse_register_specifier(
    token: &str,
    line_number: usize,
) -> Result<RegisterSpecifier, PtxParseError> {
    if let Some(start) = token.find('<') {
        let end = token
            .rfind('>')
            .ok_or_else(|| PtxParseError::InvalidDirective {
                line: line_number,
                message: format!("unterminated register range '{token}'"),
            })?;

        if end <= start + 1 {
            return Err(PtxParseError::InvalidDirective {
                line: line_number,
                message: format!("empty register range '{token}'"),
            });
        }

        let prefix = token[..start].trim();
        if prefix.is_empty() {
            return Err(PtxParseError::InvalidDirective {
                line: line_number,
                message: format!("missing prefix in register range '{token}'"),
            });
        }

        let count_str = token[start + 1..end].trim();
        if count_str.is_empty() {
            return Err(PtxParseError::InvalidDirective {
                line: line_number,
                message: format!("missing count in register range '{token}'"),
            });
        }

        let count = count_str
            .parse::<u32>()
            .map_err(|_| PtxParseError::InvalidDirective {
                line: line_number,
                message: format!("invalid register count '{count_str}' in '{token}'"),
            })?;

        return Ok(RegisterSpecifier::Range {
            prefix: prefix.to_string(),
            count,
        });
    }

    Ok(RegisterSpecifier::Named(token.to_string()))
}

fn split_name_token(
    token: &str,
    line_number: usize,
) -> Result<(String, Option<ArraySpecifier>), PtxParseError> {
    if let Some(start) = token.find('[') {
        if !token.ends_with(']') {
            return Err(PtxParseError::InvalidGlobal {
                line: line_number,
                message: format!("unterminated array specifier in global name '{token}'"),
            });
        }

        let name = token[..start].to_string();
        let spec = parse_array_specifier(&token[start..], line_number)?;
        Ok((name, Some(spec)))
    } else {
        Ok((token.to_string(), None))
    }
}

fn parse_array_specifier(spec: &str, line_number: usize) -> Result<ArraySpecifier, PtxParseError> {
    if spec.is_empty() || !spec.starts_with('[') {
        return Err(PtxParseError::InvalidGlobal {
            line: line_number,
            message: format!("invalid array specifier '{spec}'"),
        });
    }

    let mut dimensions = Vec::new();
    let mut idx = 0;
    while idx < spec.len() {
        let remaining = &spec[idx..];
        if !remaining.starts_with('[') {
            return Err(PtxParseError::InvalidGlobal {
                line: line_number,
                message: format!("malformed array specifier '{spec}'"),
            });
        }

        let close_offset = remaining
            .find(']')
            .ok_or_else(|| PtxParseError::InvalidGlobal {
                line: line_number,
                message: format!("unterminated array specifier '{spec}'"),
            })?;
        let close_idx = idx + close_offset;
        let inner = spec[idx + 1..close_idx].trim();
        if inner.is_empty() {
            dimensions.push(None);
        } else {
            let bound = inner
                .parse::<u64>()
                .map_err(|_| PtxParseError::InvalidGlobal {
                    line: line_number,
                    message: format!("invalid array bound '{inner}'"),
                })?;
            dimensions.push(Some(bound));
        }
        idx = close_idx + 1;
    }

    if dimensions.is_empty() {
        return Err(PtxParseError::InvalidGlobal {
            line: line_number,
            message: format!("empty array specifier '{spec}'"),
        });
    }

    Ok(ArraySpecifier { dimensions })
}

fn parse_global_visibility(token: &str) -> Option<GlobalVisibility> {
    match token {
        ".visible" => Some(GlobalVisibility::Visible),
        ".hidden" => Some(GlobalVisibility::Hidden),
        _ => None,
    }
}

fn parse_global_linkage(token: &str) -> Option<GlobalLinkage> {
    match token {
        ".extern" => Some(GlobalLinkage::Extern),
        ".weak" => Some(GlobalLinkage::Weak),
        ".weakextern" => Some(GlobalLinkage::WeakExtern),
        _ => None,
    }
}

fn parse_global_address_space(token: &str) -> Option<GlobalAddressSpace> {
    match token {
        ".global" => Some(GlobalAddressSpace::Global),
        ".const" => Some(GlobalAddressSpace::Const),
        ".shared" => Some(GlobalAddressSpace::Shared),
        ".local" => Some(GlobalAddressSpace::Local),
        _ => None,
    }
}

fn parse_global_mutability(token: &str) -> Option<GlobalMutability> {
    match token {
        ".const" => Some(GlobalMutability::Const),
        _ => None,
    }
}

fn parse_scalar_type(token: &str) -> Option<ScalarType> {
    if !token.starts_with('.') {
        return None;
    }

    match token[1..].to_ascii_lowercase().as_str() {
        "b8" => Some(ScalarType::B8),
        "b16" => Some(ScalarType::B16),
        "b32" => Some(ScalarType::B32),
        "b64" => Some(ScalarType::B64),
        "s8" => Some(ScalarType::S8),
        "s16" => Some(ScalarType::S16),
        "s32" => Some(ScalarType::S32),
        "s64" => Some(ScalarType::S64),
        "u8" => Some(ScalarType::U8),
        "u16" => Some(ScalarType::U16),
        "u32" => Some(ScalarType::U32),
        "u64" => Some(ScalarType::U64),
        "f16" => Some(ScalarType::F16),
        "f32" => Some(ScalarType::F32),
        "f64" => Some(ScalarType::F64),
        "pred" => Some(ScalarType::Pred),
        "texref" => Some(ScalarType::TexRef),
        "samplerref" => Some(ScalarType::SamplerRef),
        "surfref" => Some(ScalarType::SurfRef),
        _ => None,
    }
}

enum VariableQualifierContext {
    Tex,
    StateSpace,
}

fn variable_qualifier_from_token(
    token: &str,
    line_number: usize,
    context: VariableQualifierContext,
) -> Result<VariableQualifier, PtxParseError> {
    if let Some(rest) = token.strip_prefix(".v") {
        if let Ok(width) = rest.parse::<u32>() {
            return Ok(VariableQualifier::Vector(width));
        }
    }

    if matches!(context, VariableQualifierContext::Tex) && token.eq_ignore_ascii_case(".sampler") {
        return Ok(VariableQualifier::Sampler);
    }

    let message = format!("unrecognised qualifier '{}'", token);
    match context {
        VariableQualifierContext::Tex => Err(PtxParseError::InvalidDirective {
            line: line_number,
            message,
        }),
        VariableQualifierContext::StateSpace => Err(PtxParseError::InvalidGlobal {
            line: line_number,
            message,
        }),
    }
}

fn parse_global_initializer(
    raw: &str,
    line_number: usize,
) -> Result<GlobalInitializer, PtxParseError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(PtxParseError::InvalidGlobal {
            line: line_number,
            message: "empty global initializer".into(),
        });
    }

    if trimmed.starts_with('{') {
        if !trimmed.ends_with('}') {
            return Err(PtxParseError::InvalidGlobal {
                line: line_number,
                message: "unterminated aggregate initializer".into(),
            });
        }
        let inner = &trimmed[1..trimmed.len() - 1];
        let mut values = Vec::new();
        for segment in split_top_level_commas(inner) {
            let token = segment.trim();
            if token.is_empty() {
                continue;
            }
            values.push(parse_global_initializer(token, line_number)?);
        }
        return Ok(GlobalInitializer::Aggregate(values));
    }

    let value = parse_initializer_value(trimmed, line_number)?;
    Ok(GlobalInitializer::Scalar(value))
}

fn split_top_level_commas(input: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (idx, ch) in input.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            ',' if depth == 0 => {
                parts.push(&input[start..idx]);
                start = idx + 1;
            }
            _ => {}
        }
    }
    if start <= input.len() {
        parts.push(&input[start..]);
    }
    parts
}

fn parse_initializer_value(
    token: &str,
    line_number: usize,
) -> Result<InitializerValue, PtxParseError> {
    if token.is_empty() {
        return Err(PtxParseError::InvalidGlobal {
            line: line_number,
            message: "empty initializer token".into(),
        });
    }

    if token.starts_with('"') {
        if !token.ends_with('"') || token.len() < 2 {
            return Err(PtxParseError::InvalidGlobal {
                line: line_number,
                message: "unterminated string literal in initializer".into(),
            });
        }
        let literal = parse_string_literal(token, line_number)?;
        return Ok(InitializerValue::StringLiteral(literal));
    }

    if let Some(numeric) = try_parse_numeric_literal(token) {
        return Ok(InitializerValue::Numeric(numeric));
    }

    if is_symbol_token(token) {
        return Ok(InitializerValue::Symbol(token.to_string()));
    }

    Ok(InitializerValue::Symbol(token.to_string()))
}

fn parse_string_literal(token: &str, line_number: usize) -> Result<String, PtxParseError> {
    let inner = &token[1..token.len() - 1];
    let mut result = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            let Some(escaped) = chars.next() else {
                return Err(PtxParseError::InvalidGlobal {
                    line: line_number,
                    message: "incomplete escape sequence in string literal".into(),
                });
            };
            match escaped {
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                't' => result.push('\t'),
                '\\' => result.push('\\'),
                '"' => result.push('"'),
                other => {
                    result.push(other);
                }
            }
        } else {
            result.push(ch);
        }
    }
    Ok(result)
}

fn unquote_string(token: &str, line_number: usize) -> Result<String, PtxParseError> {
    if token.starts_with('"') && token.ends_with('"') && token.len() >= 2 {
        parse_string_literal(token, line_number)
    } else {
        Ok(token.to_string())
    }
}

fn try_parse_numeric_literal(token: &str) -> Option<NumericLiteral> {
    if let Some(hex) = token
        .strip_prefix("0d")
        .or_else(|| token.strip_prefix("0D"))
    {
        if let Ok(bits) = u64::from_str_radix(hex, 16) {
            return Some(NumericLiteral::Float64(bits));
        }
    }

    if let Some(hex) = token
        .strip_prefix("0f")
        .or_else(|| token.strip_prefix("0F"))
    {
        if let Ok(bits) = u32::from_str_radix(hex, 16) {
            return Some(NumericLiteral::Float32(bits));
        }
    }

    if let Some(hex) = token
        .strip_prefix("0x")
        .or_else(|| token.strip_prefix("0X"))
    {
        if let Ok(value) = u64::from_str_radix(hex, 16) {
            return Some(NumericLiteral::Unsigned(value));
        }
    }

    if let Some(hex) = token
        .strip_prefix("-0x")
        .or_else(|| token.strip_prefix("-0X"))
    {
        if let Ok(value) = i64::from_str_radix(hex, 16) {
            return Some(NumericLiteral::Signed(-value));
        }
    }

    if let Ok(value) = token.parse::<i64>() {
        return Some(NumericLiteral::Signed(value));
    }

    if let Ok(value) = token.parse::<u64>() {
        return Some(NumericLiteral::Unsigned(value));
    }

    if let Ok(value) = token.parse::<f64>() {
        return Some(NumericLiteral::Float64(value.to_bits()));
    }

    if let Ok(value) = token.parse::<f32>() {
        return Some(NumericLiteral::Float32(value.to_bits()));
    }

    None
}
