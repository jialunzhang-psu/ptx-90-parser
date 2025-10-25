use std::fmt::Write as _;

use crate::r#type::{
    ExternCallBlock, ExternCallSetup, FunctionBody, FunctionEntryDirective,
    FunctionHeaderDirective, FunctionKernelDirective, FunctionStatement,
    GenericFunctionDeclaration, Module, ModuleDebugDirective, ModuleDirective, ModuleDirectiveKind,
    ModuleVariableDirective, Parameter, StatementDirective,
};

/// Convert a parsed PTX [`Module`] back into PTX source text.
pub fn unparse(module: &Module) -> String {
    let mut writer = ModuleWriter::default();
    writer.write_module(module);
    writer.finish()
}

#[derive(Default)]
struct ModuleWriter {
    lines: Vec<String>,
}

impl ModuleWriter {
    fn write_module(&mut self, module: &Module) {
        let mut first_function = true;
        for directive in &module.directives {
            if matches!(
                directive,
                ModuleDirective::FunctionKernel(FunctionKernelDirective::Entry(_))
                    | ModuleDirective::FunctionKernel(FunctionKernelDirective::Func(_))
            ) {
                if !first_function
                    && !self.lines.is_empty()
                    && !self.lines.last().unwrap().is_empty()
                {
                    self.lines.push(String::new());
                }
                first_function = false;
            }

            self.write_module_directive(directive);
        }
    }

    fn write_module_directive(&mut self, directive: &ModuleDirective) {
        match directive {
            ModuleDirective::Module(module) => self.write_module_header(module),
            ModuleDirective::Debug(debug) => self.write_debug_directive(debug),
            ModuleDirective::Linking(linking) => {
                self.lines.push(linking.raw.clone());
            }
            ModuleDirective::ModuleVariable(variable) => {
                let raw = match variable {
                    ModuleVariableDirective::Const(var) => &var.raw,
                    ModuleVariableDirective::Global(var) => &var.raw,
                    ModuleVariableDirective::Shared(var) => &var.raw,
                    ModuleVariableDirective::Tex(var) => &var.raw,
                };
                self.lines.push(raw.clone());
            }
            ModuleDirective::FunctionKernel(kernel) => {
                self.write_function_kernel(kernel);
            }
        }
    }

    fn write_module_header(&mut self, directive: &ModuleDirectiveKind) {
        match directive {
            ModuleDirectiveKind::Version(version) => {
                self.lines
                    .push(format!(".version {}.{}", version.major, version.minor));
            }
            ModuleDirectiveKind::Target(target) => {
                if target.raw.trim().is_empty() {
                    let joined = target.entries.join(", ");
                    self.lines.push(format!(".target {joined}"));
                } else {
                    self.lines.push(format!(".target {}", target.raw.trim()));
                }
            }
            ModuleDirectiveKind::AddressSize(size) => {
                self.lines.push(format!(".address_size {}", size.size));
            }
        }
    }

    fn write_debug_directive(&mut self, directive: &ModuleDebugDirective) {
        match directive {
            ModuleDebugDirective::File(file) => {
                let mut escaped = String::with_capacity(file.path.len());
                for ch in file.path.chars() {
                    if ch == '"' {
                        escaped.push('\\');
                    }
                    escaped.push(ch);
                }
                self.lines
                    .push(format!(".file {} \"{}\"", file.index, escaped));
            }
            ModuleDebugDirective::Section(section) => {
                if section.attributes.is_empty() {
                    self.lines.push(format!(".section {}", section.name));
                } else {
                    let attrs = section.attributes.join(" ");
                    self.lines
                        .push(format!(".section {} {}", section.name, attrs));
                }
            }
            ModuleDebugDirective::Dwarf(dwarf) => {
                self.lines
                    .push(line_with_comment(&dwarf.raw, dwarf.comment.as_deref()));
            }
        }
    }

    fn write_function_kernel(&mut self, kernel: &FunctionKernelDirective) {
        match kernel {
            FunctionKernelDirective::Entry(function) => {
                self.write_function_header(
                    ".entry",
                    &function.directives,
                    &function.name,
                    None,
                    &function.params,
                );
                self.write_function_body(&function.body);
            }
            FunctionKernelDirective::Func(function) => {
                self.write_function_header(
                    ".func",
                    &function.directives,
                    &function.name,
                    function.return_param.as_ref(),
                    &function.params,
                );
                self.write_function_body(&function.body);
            }
            FunctionKernelDirective::Alias(alias) => {
                self.lines.push(alias.raw.clone());
            }
        }
    }

    fn write_function_header(
        &mut self,
        keyword: &str,
        directives: &[FunctionHeaderDirective],
        name: &str,
        return_param: Option<&Parameter>,
        params: &[Parameter],
    ) {
        let (prefix_tokens, suffix_tokens) = split_header_directives(directives);
        let mut header_line = String::new();

        if !prefix_tokens.is_empty() {
            header_line.push_str(&prefix_tokens.join(" "));
            header_line.push(' ');
        }

        header_line.push_str(keyword);

        if let Some(ret) = return_param {
            header_line.push(' ');
            header_line.push('(');
            header_line.push_str(ret.raw.trim());
            header_line.push(')');
        }

        header_line.push(' ');
        header_line.push_str(name);

        if params.is_empty() {
            header_line.push_str("()");
            if !suffix_tokens.is_empty() {
                header_line.push(' ');
                header_line.push_str(&suffix_tokens.join(" "));
            }
            self.lines.push(header_line);
        } else {
            header_line.push('(');
            self.lines.push(header_line);
            for (index, param) in params.iter().enumerate() {
                let mut line = param.raw.clone();
                if index + 1 != params.len() {
                    line.push(',');
                }
                self.lines.push(format!("    {}", line));
            }
            let mut closing = String::from(")");
            if !suffix_tokens.is_empty() {
                closing.push(' ');
                closing.push_str(&suffix_tokens.join(" "));
            }
            self.lines.push(closing);
        }

        self.lines.push("{".into());
    }

    fn write_function_body(&mut self, body: &FunctionBody) {
        for entry in &body.entry_directives {
            self.write_entry_directive(entry, 1);
        }

        if !body.entry_directives.is_empty() && !body.statements.is_empty() {
            self.lines.push(String::new());
        }

        for statement in &body.statements {
            self.write_statement_with_indent(statement, 1);
        }

        self.lines.push("}".into());
    }

    fn finish(mut self) -> String {
        while matches!(self.lines.last(), Some(line) if line.is_empty()) {
            self.lines.pop();
        }

        let mut output = self.lines.join("\n");
        if !output.ends_with('\n') {
            output.push('\n');
        }
        output
    }
}

fn split_header_directives(directives: &[FunctionHeaderDirective]) -> (Vec<String>, Vec<String>) {
    let mut prefix = Vec::new();
    let mut suffix = Vec::new();

    for directive in directives {
        let tokens = header_directive_tokens(directive);
        if is_prefix_directive(directive) {
            append_tokens(&mut prefix, tokens);
        } else {
            append_tokens(&mut suffix, tokens);
        }
    }

    (prefix, suffix)
}

fn append_tokens(target: &mut Vec<String>, tokens: Vec<String>) {
    target.extend(tokens);
}

fn header_directive_tokens(directive: &FunctionHeaderDirective) -> Vec<String> {
    match directive {
        FunctionHeaderDirective::Visibility(vis) => match vis {
            crate::r#type::FunctionVisibility::Visible => vec![".visible".into()],
            crate::r#type::FunctionVisibility::Hidden => vec![".hidden".into()],
        },
        FunctionHeaderDirective::Linkage(linkage) => match linkage {
            crate::r#type::FunctionLinkage::Extern => vec![".extern".into()],
            crate::r#type::FunctionLinkage::Weak => vec![".weak".into()],
            crate::r#type::FunctionLinkage::WeakExtern => {
                vec![".weak".into(), ".extern".into()]
            }
        },
        FunctionHeaderDirective::NoReturn => vec![".noreturn".into()],
        FunctionHeaderDirective::AbiPreserve(value) => {
            vec![format!(".abi_preserve {}", value)]
        }
        FunctionHeaderDirective::AbiPreserveControl(value) => {
            vec![format!(".abi_preserve_control {}", value)]
        }
        FunctionHeaderDirective::MaxClusterRank(value) => {
            vec![format!(".maxclusterrank {}", value)]
        }
        FunctionHeaderDirective::BlocksAreClusters => vec![".blocksareclusters".into()],
        FunctionHeaderDirective::ExplicitCluster(dim) => {
            vec![format!(".explicitcluster {}", format_dim3(dim))]
        }
        FunctionHeaderDirective::ReqNctaPerCluster(dim) => {
            vec![format!(".reqnctapercluster {}", format_dim3(dim))]
        }
        FunctionHeaderDirective::MaxNReg(value) => {
            vec![format!(".maxnreg {}", value)]
        }
        FunctionHeaderDirective::MaxNTid(dim) => {
            vec![format!(".maxntid {}", format_dim3(dim))]
        }
        FunctionHeaderDirective::MinNCtaPerSm(value) => {
            vec![format!(".minnctapersm {}", value)]
        }
        FunctionHeaderDirective::ReqNTid(dim) => {
            vec![format!(".reqntid {}", format_dim3(dim))]
        }
        FunctionHeaderDirective::MaxNCtaPerSm(value) => {
            vec![format!(".maxnctapersm {}", value)]
        }
        FunctionHeaderDirective::Pragma(args) => {
            let joined = args.join(", ");
            vec![format!(".pragma {}", joined)]
        }
    }
}

fn is_prefix_directive(directive: &FunctionHeaderDirective) -> bool {
    matches!(
        directive,
        FunctionHeaderDirective::Visibility(_) | FunctionHeaderDirective::Linkage(_)
    )
}

fn format_dim3(dim: &crate::r#type::FunctionDim3) -> String {
    let mut result = dim.x.to_string();
    if let Some(y) = dim.y {
        write!(result, ", {}", y).unwrap();
    }
    if let Some(z) = dim.z {
        write!(result, ", {}", z).unwrap();
    }
    result
}

fn write_generic_declaration(decl: &GenericFunctionDeclaration) -> String {
    line_with_comment(&decl.raw, decl.comment.as_deref())
}

fn line_with_comment(raw: &str, comment: Option<&str>) -> String {
    match comment {
        Some(comment) if !comment.is_empty() => {
            format!("{raw} // {comment}")
        }
        _ => raw.to_string(),
    }
}

fn indent(level: usize) -> String {
    "    ".repeat(level)
}

impl ModuleWriter {
    fn write_entry_directive(&mut self, entry: &FunctionEntryDirective, indent_level: usize) {
        let line = match entry {
            FunctionEntryDirective::Reg(reg) => line_with_comment(&reg.raw, reg.comment.as_deref()),
            FunctionEntryDirective::Local(decl)
            | FunctionEntryDirective::Param(decl)
            | FunctionEntryDirective::Shared(decl) => write_generic_declaration(decl),
            FunctionEntryDirective::Pragma(pragma) => {
                line_with_comment(&pragma.raw, pragma.comment.as_deref())
            }
            FunctionEntryDirective::Loc(loc) => line_with_comment(&loc.raw, loc.comment.as_deref()),
            FunctionEntryDirective::Dwarf(dwarf) => {
                line_with_comment(&dwarf.raw, dwarf.comment.as_deref())
            }
        };

        self.lines.push(format!("{}{}", indent(indent_level), line));
    }

    fn write_statement_with_indent(&mut self, statement: &FunctionStatement, indent_level: usize) {
        match statement {
            FunctionStatement::Label(label) => {
                self.lines
                    .push(format!("{}{}:", indent(indent_level), label));
            }
            FunctionStatement::Directive(directive) => {
                let line = match directive {
                    StatementDirective::Dwarf(dwarf) => {
                        line_with_comment(&dwarf.raw, dwarf.comment.as_deref())
                    }
                    StatementDirective::Loc(loc) => {
                        line_with_comment(&loc.raw, loc.comment.as_deref())
                    }
                    StatementDirective::Pragma(pragma) => {
                        line_with_comment(&pragma.raw, pragma.comment.as_deref())
                    }
                    StatementDirective::Section(section) => {
                        line_with_comment(&section.raw, section.comment.as_deref())
                    }
                };
                self.lines.push(format!("{}{}", indent(indent_level), line));
            }
            FunctionStatement::Instruction(inst) => {
                self.lines.push(format!(
                    "{}{}",
                    indent(indent_level),
                    line_with_comment(&inst.raw, inst.comment.as_deref())
                ));
            }
            FunctionStatement::ExternCallBlock(block) => {
                self.write_extern_call_block(block, indent_level);
            }
        }
    }

    fn write_extern_call_block(&mut self, block: &ExternCallBlock, indent_level: usize) {
        self.lines.push(format!("{}{{", indent(indent_level)));

        if !block.declarations.is_empty() {
            for decl in &block.declarations {
                self.write_entry_directive(decl, indent_level + 1);
            }
        }

        if !block.declarations.is_empty()
            && (!block.setup.is_empty()
                || !block.post_call.is_empty()
                || !block.call.raw.is_empty())
        {
            self.lines.push(String::new());
        }

        for step in &block.setup {
            match step {
                ExternCallSetup::Param(param) => {
                    self.lines.push(format!(
                        "{}{}",
                        indent(indent_level + 1),
                        write_generic_declaration(param)
                    ));
                }
                ExternCallSetup::Store(inst) => {
                    self.lines.push(format!(
                        "{}{}",
                        indent(indent_level + 1),
                        line_with_comment(&inst.raw, inst.comment.as_deref())
                    ));
                }
            }
        }

        self.lines.push(format!(
            "{}{}",
            indent(indent_level + 1),
            line_with_comment(&block.call.raw, block.call.comment.as_deref())
        ));

        for inst in &block.post_call {
            self.lines.push(format!(
                "{}{}",
                indent(indent_level + 1),
                line_with_comment(&inst.raw, inst.comment.as_deref())
            ));
        }

        self.lines.push(format!("{}}}", indent(indent_level)));
    }
}
