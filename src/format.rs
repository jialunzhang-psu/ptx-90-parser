use ptx_parser::{
    AddressBase, AddressDisplacement, AddressDisplacementKind, AddressSign, DwarfDirective,
    EntryFunction, FuncFunction, FunctionAlias, FunctionBody, FunctionDim3, FunctionEntryDirective,
    FunctionHeaderDirective, FunctionKernelDirective, FunctionStatement,
    GenericFunctionDeclaration, GlobalInitializer, InitializerValue, Instruction, LinkingDirective,
    LinkingDirectiveKind, LocationDirective, MemoryOperand, ModifierKind, Module,
    ModuleDebugDirective, ModuleDirective, ModuleDirectiveKind, ModuleVariableDirective,
    NumericLiteral, Operand, Parameter, PragmaDirective, RegisterDeclaration, RegisterSpecifier,
    StatementDirective, StatementSectionDirective, VariableDirective, VariableQualifier,
};

pub fn module_to_tree(module: &Module) -> String {
    let mut writer = TreeWriter::new();
    write_module(&mut writer, module);
    writer.finish()
}

fn write_module(writer: &mut TreeWriter, module: &Module) {
    writer.open("module");

    for directive in &module.directives {
        match directive {
            ModuleDirective::FunctionKernel(kernel) => match kernel {
                FunctionKernelDirective::Entry(function) => write_entry_function(writer, function),
                FunctionKernelDirective::Func(function) => write_func_function(writer, function),
                FunctionKernelDirective::Alias(alias) => write_alias(writer, alias),
            },
            ModuleDirective::ModuleVariable(variable) => write_variable_directive(writer, variable),
            other => write_directive(writer, other),
        }
    }

    writer.close();
}

fn write_directive(writer: &mut TreeWriter, directive: &ModuleDirective) {
    match directive {
        ModuleDirective::Module(module) => write_module_header(writer, module),
        ModuleDirective::Debug(debug) => write_debug_directive(writer, debug),
        ModuleDirective::Linking(linking) => write_linking_directive(writer, linking),
        ModuleDirective::ModuleVariable(_) | ModuleDirective::FunctionKernel(_) => {}
    }
}

fn write_module_header(writer: &mut TreeWriter, directive: &ModuleDirectiveKind) {
    match directive {
        ModuleDirectiveKind::Version(version) => {
            writer.open(".version");
            writer.line(&format!("(major {})", version.major));
            writer.line(&format!("(minor {})", version.minor));
            writer.close();
        }
        ModuleDirectiveKind::Target(target) => {
            writer.open(".target");
            for entry in &target.entries {
                writer.line(&format!("(entry {})", quote(entry)));
            }
            writer.line(&format!("(raw {})", quote(&target.raw)));
            writer.close();
        }
        ModuleDirectiveKind::AddressSize(size) => {
            writer.open(".address_size");
            writer.line(&format!("(bits {})", size.size));
            writer.close();
        }
    }
}

fn write_debug_directive(writer: &mut TreeWriter, directive: &ModuleDebugDirective) {
    match directive {
        ModuleDebugDirective::File(file) => {
            writer.open(".file");
            writer.line(&format!("(index {})", file.index));
            writer.line(&format!("(path {})", quote(&file.path)));
            writer.close();
        }
        ModuleDebugDirective::Section(section) => {
            writer.open(".section");
            writer.line(&format!("(name {})", quote(&section.name)));
            for attribute in &section.attributes {
                writer.line(&format!("(attribute {})", quote(attribute)));
            }
            writer.close();
        }
        ModuleDebugDirective::Dwarf(dwarf) => {
            writer.open(&dwarf.keyword);
            for argument in &dwarf.arguments {
                writer.line(&format!("(argument {})", quote(argument)));
            }
            if let Some(comment) = &dwarf.comment {
                writer.line(&format!("(comment {})", quote(comment)));
            }
            writer.line(&format!("(raw {})", quote(&dwarf.raw)));
            writer.close();
        }
    }
}

fn write_linking_directive(writer: &mut TreeWriter, directive: &LinkingDirective) {
    let label = match directive.kind {
        LinkingDirectiveKind::Extern => ".extern",
        LinkingDirectiveKind::Visible => ".visible",
        LinkingDirectiveKind::Weak => ".weak",
        LinkingDirectiveKind::Common => ".common",
    };
    writer.open(label);
    if !directive.prototype.is_empty() {
        writer.line(&format!("(prototype {})", quote(&directive.prototype)));
    }
    writer.line(&format!("(raw {})", quote(&directive.raw)));
    writer.close();
}

fn write_variable_directive(writer: &mut TreeWriter, directive: &ModuleVariableDirective) {
    match directive {
        ModuleVariableDirective::Const(var) => write_variable(writer, "const", var),
        ModuleVariableDirective::Global(var) => write_variable(writer, "global", var),
        ModuleVariableDirective::Shared(var) => write_variable(writer, "shared", var),
        ModuleVariableDirective::Tex(var) => write_tex(writer, var),
    }
}

fn write_tex(writer: &mut TreeWriter, tex: &VariableDirective) {
    writer.open(".tex");
    writer.line(&format!("(name {})", quote(&tex.name)));
    if !tex.qualifiers.is_empty() {
        writer.open("qualifiers");
        for qualifier in &tex.qualifiers {
            match qualifier {
                VariableQualifier::Vector(width) => {
                    writer.line(&format!("(vector {})", width));
                }
                VariableQualifier::Sampler => writer.line("(sampler)"),
            }
        }
        writer.close();
    }
    writer.line(&format!("(raw {})", quote(&tex.raw)));
    writer.close();
}

fn write_variable(writer: &mut TreeWriter, label: &str, var: &VariableDirective) {
    writer.open(label);
    writer.line(&format!("(name {})", quote(&var.name)));

    if let Some(visibility) = var.visibility {
        writer.line(&format!("(visibility {})", visibility));
    }

    if !var.linkages.is_empty() {
        writer.open("linkage");
        for linkage in &var.linkages {
            writer.line(&format!("({})", linkage));
        }
        writer.close();
    }

    if let Some(space) = var.address_space {
        writer.line(&format!("(address_space {})", space));
    }

    if let Some(mutable) = var.mutability {
        writer.line(&format!("(mutability {})", mutable));
    }

    if let Some(alignment) = var.alignment {
        writer.line(&format!("(alignment {})", alignment));
    }

    if let Some(ty) = var.ty {
        writer.line(&format!("(type {})", ty));
    }

    if !var.qualifiers.is_empty() {
        writer.open("qualifiers");
        for qualifier in &var.qualifiers {
            match qualifier {
                VariableQualifier::Vector(width) => {
                    writer.line(&format!("(vector {})", width));
                }
                VariableQualifier::Sampler => writer.line("(sampler)"),
            }
        }
        writer.close();
    }

    if let Some(array) = &var.array {
        writer.open("array");
        for (index, bound) in array.dimensions.iter().enumerate() {
            match bound {
                Some(value) => writer.line(&format!("(dimension {} {})", index, value)),
                None => writer.line(&format!("(dimension {} unspecified)", index)),
            }
        }
        writer.close();
    }

    if let Some(initializer) = &var.initializer {
        writer.open("initializer");
        write_global_initializer(writer, initializer);
        writer.close();
    }

    writer.line(&format!("(raw {})", quote(&var.raw)));
    writer.close();
}

fn write_initializer_value(writer: &mut TreeWriter, value: &InitializerValue) {
    match value {
        InitializerValue::Numeric(numeric) => match numeric {
            NumericLiteral::Signed(value) => writer.line(&format!("(signed {})", value)),
            NumericLiteral::Unsigned(value) => writer.line(&format!("(unsigned {})", value)),
            NumericLiteral::Float64(bits) => {
                let value = f64::from_bits(*bits);
                writer.line(&format!("(float64 {})", value));
            }
            NumericLiteral::Float32(bits) => {
                let value = f32::from_bits(*bits);
                writer.line(&format!("(float32 {})", value));
            }
        },
        InitializerValue::Symbol(symbol) => writer.line(&format!("(symbol {})", quote(symbol))),
        InitializerValue::StringLiteral(literal) => {
            writer.line(&format!("(string {})", quote(literal)))
        }
    }
}

fn write_global_initializer(writer: &mut TreeWriter, init: &GlobalInitializer) {
    match init {
        GlobalInitializer::Scalar(value) => {
            writer.open("scalar");
            write_initializer_value(writer, value);
            writer.close();
        }
        GlobalInitializer::Aggregate(values) => {
            writer.open("aggregate");
            for value in values {
                writer.open("element");
                write_global_initializer(writer, value);
                writer.close();
            }
            writer.close();
        }
    }
}

fn write_entry_function(writer: &mut TreeWriter, function: &EntryFunction) {
    write_function(
        writer,
        "entry",
        &function.name,
        &function.directives,
        None,
        &function.params,
        &function.body,
    );
}

fn write_func_function(writer: &mut TreeWriter, function: &FuncFunction) {
    write_function(
        writer,
        "func",
        &function.name,
        &function.directives,
        function.return_param.as_ref(),
        &function.params,
        &function.body,
    );
}

fn write_function(
    writer: &mut TreeWriter,
    label: &str,
    name: &str,
    directives: &[FunctionHeaderDirective],
    return_param: Option<&Parameter>,
    params: &[Parameter],
    body: &FunctionBody,
) {
    writer.open(label);
    writer.line(&format!("(name {})", quote(name)));

    if !directives.is_empty() {
        writer.open("directives");
        for directive in directives {
            write_function_directive(writer, directive);
        }
        writer.close();
    }

    if let Some(param) = return_param {
        writer.open("return_params");
        write_param(writer, param);
        writer.close();
    }

    if !params.is_empty() {
        writer.open("params");
        for param in params {
            write_param(writer, param);
        }
        writer.close();
    }

    if !body.entry_directives.is_empty() {
        writer.open("entry_directives");
        for directive in &body.entry_directives {
            write_entry_directive(writer, directive);
        }
        writer.close();
    }

    if !body.statements.is_empty() {
        writer.open("body");
        for stmt in &body.statements {
            write_statement(writer, stmt);
        }
        writer.close();
    }

    writer.close();
}

fn write_function_directive(writer: &mut TreeWriter, directive: &FunctionHeaderDirective) {
    match directive {
        FunctionHeaderDirective::Visibility(visibility) => {
            writer.line(&format!("(visibility {})", visibility));
        }
        FunctionHeaderDirective::Linkage(linkage) => {
            writer.line(&format!("(linkage {})", linkage));
        }
        FunctionHeaderDirective::NoReturn => writer.line("(noreturn)"),
        FunctionHeaderDirective::AbiPreserve(value) => {
            writer.line(&format!("(abi_preserve {})", value));
        }
        FunctionHeaderDirective::AbiPreserveControl(value) => {
            writer.line(&format!("(abi_preserve_control {})", value));
        }
        FunctionHeaderDirective::MaxClusterRank(value) => {
            writer.line(&format!("(maxclusterrank {})", value));
        }
        FunctionHeaderDirective::BlocksAreClusters => writer.line("(blocksareclusters)"),
        FunctionHeaderDirective::ExplicitCluster(dim) => {
            writer.open("explicitcluster");
            write_dim3(writer, dim);
            writer.close();
        }
        FunctionHeaderDirective::ReqNctaPerCluster(dim) => {
            writer.open("reqnctapercluster");
            write_dim3(writer, dim);
            writer.close();
        }
        FunctionHeaderDirective::MaxNReg(value) => {
            writer.line(&format!("(maxnreg {})", value));
        }
        FunctionHeaderDirective::MaxNTid(dim) => {
            writer.open("maxntid");
            write_dim3(writer, dim);
            writer.close();
        }
        FunctionHeaderDirective::MinNCtaPerSm(value) => {
            writer.line(&format!("(minnctapersm {})", value));
        }
        FunctionHeaderDirective::ReqNTid(dim) => {
            writer.open("reqntid");
            write_dim3(writer, dim);
            writer.close();
        }
        FunctionHeaderDirective::MaxNCtaPerSm(value) => {
            writer.line(&format!("(maxnctapersm {})", value));
        }
        FunctionHeaderDirective::Pragma(args) => {
            writer.open("pragma");
            for arg in args {
                writer.line(&format!("(arg {})", quote(arg)));
            }
            writer.close();
        }
    }
}

fn write_dim3(writer: &mut TreeWriter, dim: &FunctionDim3) {
    writer.line(&format!("(x {})", dim.x));
    if let Some(y) = dim.y {
        writer.line(&format!("(y {})", y));
    }
    if let Some(z) = dim.z {
        writer.line(&format!("(z {})", z));
    }
}

fn write_alias(writer: &mut TreeWriter, alias: &FunctionAlias) {
    writer.open(".alias");
    writer.line(&format!("(alias {})", quote(&alias.alias)));
    writer.line(&format!("(target {})", quote(&alias.target)));
    writer.line(&format!("(raw {})", quote(&alias.raw)));
    writer.close();
}

fn write_param(writer: &mut TreeWriter, param: &Parameter) {
    writer.open("param");
    writer.line(&format!("(name {})", quote(&param.name)));
    if let Some(alignment) = param.alignment {
        writer.line(&format!("(alignment {})", alignment));
    }
    if let Some(ty) = param.ty {
        writer.line(&format!("(type {})", ty));
    }
    if !param.qualifiers.is_empty() {
        writer.open("qualifiers");
        if param.qualifiers.is_const {
            writer.line("(const)");
        }
        if param.qualifiers.is_volatile {
            writer.line("(volatile)");
        }
        if param.qualifiers.is_restrict {
            writer.line("(restrict)");
        }
        if param.qualifiers.is_noalias {
            writer.line("(noalias)");
        }
        if let Some(pointer) = &param.qualifiers.pointer {
            writer.open("pointer");
            match pointer.address_space {
                Some(space) => writer.line(&format!("(address_space {})", space)),
                None => writer.line("(address_space unspecified)"),
            }
            writer.close();
        }
        writer.close();
    }
    if let Some(array) = &param.array {
        writer.open("array");
        for (index, bound) in array.dimensions.iter().enumerate() {
            match bound {
                Some(value) => writer.line(&format!("(dimension {} {})", index, value)),
                None => writer.line(&format!("(dimension {} unspecified)", index)),
            }
        }
        writer.close();
    }
    writer.line(&format!("(raw {})", quote(&param.raw)));
    writer.close();
}

fn write_entry_directive(writer: &mut TreeWriter, directive: &FunctionEntryDirective) {
    match directive {
        FunctionEntryDirective::Reg(decl) => write_register_declaration(writer, decl),
        FunctionEntryDirective::Local(decl) => write_generic_entry(writer, decl),
        FunctionEntryDirective::Param(decl) => write_generic_entry(writer, decl),
        FunctionEntryDirective::Shared(decl) => write_generic_entry(writer, decl),
        FunctionEntryDirective::Pragma(pragma) => write_pragma_directive(writer, pragma),
        FunctionEntryDirective::Loc(loc) => write_loc_directive(writer, loc),
        FunctionEntryDirective::Dwarf(dwarf) => write_dwarf_directive(writer, dwarf),
    }
}

fn write_statement(writer: &mut TreeWriter, stmt: &FunctionStatement) {
    match stmt {
        FunctionStatement::Label(name) => writer.line(&format!("(label {})", quote(name.as_str()))),
        FunctionStatement::Directive(directive) => write_statement_directive(writer, directive),
        FunctionStatement::Instruction(instr) => write_instruction(writer, instr),
    }
}

fn write_register_declaration(writer: &mut TreeWriter, decl: &RegisterDeclaration) {
    writer.open(&decl.keyword);
    if let Some(scalar) = decl.ty.scalar {
        writer.line(&format!("(type {})", scalar));
    } else {
        writer.line(&format!("(type {})", quote(&decl.ty.raw)));
    }
    if !decl.registers.is_empty() {
        writer.open("registers");
        for register in &decl.registers {
            match register {
                RegisterSpecifier::Named(name) => {
                    writer.line(&format!("(name {})", quote(name.as_str())));
                }
                RegisterSpecifier::Range { prefix, count } => {
                    writer.open("range");
                    writer.line(&format!("(prefix {})", quote(prefix.as_str())));
                    writer.line(&format!("(count {})", count));
                    writer.close();
                }
            }
        }
        writer.close();
    }
    if let Some(comment) = &decl.comment {
        writer.line(&format!("(comment {})", quote(comment.as_str())));
    }
    writer.line(&format!("(raw {})", quote(&decl.raw)));
    writer.close();
}

fn write_generic_entry(writer: &mut TreeWriter, directive: &GenericFunctionDeclaration) {
    writer.open(&directive.keyword);
    writer.line(&format!("(kind {})", directive.kind));
    for argument in &directive.arguments {
        writer.line(&format!("(argument {})", quote(argument.as_str())));
    }
    if let Some(comment) = &directive.comment {
        writer.line(&format!("(comment {})", quote(comment.as_str())));
    }
    writer.line(&format!("(raw {})", quote(&directive.raw)));
    writer.close();
}

fn write_statement_directive(writer: &mut TreeWriter, directive: &StatementDirective) {
    match directive {
        StatementDirective::Dwarf(dwarf) => write_dwarf_directive(writer, dwarf),
        StatementDirective::Loc(loc) => write_loc_directive(writer, loc),
        StatementDirective::Pragma(pragma) => write_pragma_directive(writer, pragma),
        StatementDirective::Section(section) => write_section_directive(writer, section),
    }
}

fn write_dwarf_directive(writer: &mut TreeWriter, directive: &DwarfDirective) {
    writer.open(&directive.keyword);
    for argument in &directive.arguments {
        writer.line(&format!("(argument {})", quote(argument.as_str())));
    }
    if let Some(comment) = &directive.comment {
        writer.line(&format!("(comment {})", quote(comment.as_str())));
    }
    writer.line(&format!("(raw {})", quote(&directive.raw)));
    writer.close();
}

fn write_loc_directive(writer: &mut TreeWriter, loc: &LocationDirective) {
    writer.open(".loc");
    writer.line(&format!("(file {})", loc.file_index));
    writer.line(&format!("(line {})", loc.line));
    writer.line(&format!("(column {})", loc.column));
    for option in &loc.options {
        writer.line(&format!("(option {})", quote(option)));
    }
    if let Some(comment) = &loc.comment {
        writer.line(&format!("(comment {})", quote(comment)));
    }
    writer.line(&format!("(raw {})", quote(&loc.raw)));
    writer.close();
}

fn write_pragma_directive(writer: &mut TreeWriter, pragma: &PragmaDirective) {
    writer.open(".pragma");
    for argument in &pragma.arguments {
        writer.line(&format!("(argument {})", quote(argument)));
    }
    if let Some(comment) = &pragma.comment {
        writer.line(&format!("(comment {})", quote(comment)));
    }
    writer.line(&format!("(raw {})", quote(&pragma.raw)));
    writer.close();
}

fn write_section_directive(writer: &mut TreeWriter, directive: &StatementSectionDirective) {
    writer.open(".section");
    writer.line(&format!("(name {})", quote(&directive.name)));
    for argument in &directive.arguments {
        writer.line(&format!("(argument {})", quote(argument)));
    }
    if let Some(comment) = &directive.comment {
        writer.line(&format!("(comment {})", quote(comment)));
    }
    writer.line(&format!("(raw {})", quote(&directive.raw)));
    writer.close();
}

fn write_instruction(writer: &mut TreeWriter, instr: &Instruction) {
    writer.open("instruction");
    writer.line(&format!("(raw {})", quote(&instr.raw)));
    if let Some(predicate) = &instr.predicate {
        writer.line(&format!("(predicate {})", quote(predicate)));
    }
    if instr.opcode.modifiers.is_empty() && instr.operands.is_empty() {
        writer.line(&format!("({})", instr.opcode.kind));
    } else {
        let opcode_label = instr.opcode.kind.to_string();
        writer.open(&opcode_label);
        for modifier in &instr.opcode.modifiers {
            write_modifier(writer, modifier);
        }
        for operand in &instr.operands {
            write_operand(writer, operand);
        }
        writer.close();
    }
    if let Some(comment) = &instr.comment {
        writer.line(&format!("(comment {})", quote(comment)));
    }
    writer.close();
}

fn write_modifier(writer: &mut TreeWriter, modifier: &ModifierKind) {
    match modifier {
        ModifierKind::Type(ty) => writer.line(&format!("(type {})", ty)),
        ModifierKind::Condition(cond) => writer.line(&format!("(condition {})", cond)),
        ModifierKind::AddressSpace(space) => writer.line(&format!("(address_space {})", space)),
        ModifierKind::Conversion(space) => writer.line(&format!("(conversion_to {})", space)),
        ModifierKind::Rounding(mode) => writer.line(&format!("(rounding {})", mode)),
        ModifierKind::VectorWidth(width) => writer.line(&format!("(vector_width {})", width)),
        ModifierKind::MathMode(mode) => writer.line(&format!("(math_mode {})", mode)),
        ModifierKind::Synchronization(kind) => writer.line(&format!("(synchronization {})", kind)),
        ModifierKind::AsyncGroup(group) => writer.line(&format!("(async_group {})", group)),
        ModifierKind::Shuffle(mode) => writer.line(&format!("(shuffle {})", mode)),
        ModifierKind::Cache(cache) => writer.line(&format!("(cache {})", cache)),
        ModifierKind::Scope(scope) => writer.line(&format!("(scope {})", scope)),
        ModifierKind::Atomic(op) => writer.line(&format!("(atomic {})", op)),
        ModifierKind::Call(call) => writer.line(&format!("(call_modifier {})", call)),
        ModifierKind::MemoryOrder(order) => writer.line(&format!("(memory_order {})", order)),
        ModifierKind::Wide => writer.line("(wide)"),
    }
}

fn write_operand(writer: &mut TreeWriter, operand: &Operand) {
    match operand {
        Operand::Register(name) => writer.line(&format!("(register {})", quote(name))),
        Operand::Immediate(value) => writer.line(&format!("(immediate {})", quote(value))),
        Operand::Symbol(symbol) => writer.line(&format!("(symbol {})", quote(symbol))),
        Operand::CallTarget { name, arguments } => {
            writer.open("call_target");
            writer.line(&format!("(name {})", quote(name)));
            if !arguments.is_empty() {
                writer.line(&format!("(arguments {})", join_quoted(arguments)));
            }
            writer.close();
        }
        Operand::Memory(memory) => write_memory_operand(writer, memory),
        Operand::Parenthesized(items) => {
            writer.line(&format!("(parenthesized {})", join_quoted(items)));
        }
    }
}

fn write_memory_operand(writer: &mut TreeWriter, memory: &MemoryOperand) {
    writer.open("address");
    if let Some(base) = &memory.base {
        write_address_base(writer, base);
    }
    if !memory.displacements.is_empty() {
        writer.open("offset");
        for displacement in &memory.displacements {
            write_address_displacement(writer, displacement);
        }
        writer.close();
    }
    writer.close();
}

fn write_address_base(writer: &mut TreeWriter, base: &AddressBase) {
    writer.open("base");
    match base {
        AddressBase::Register(name) => writer.line(&format!("(register {})", quote(name))),
        AddressBase::Symbol(symbol) => writer.line(&format!("(symbol {})", quote(symbol))),
    }
    writer.close();
}

fn write_address_displacement(writer: &mut TreeWriter, displacement: &AddressDisplacement) {
    match &displacement.kind {
        AddressDisplacementKind::Register { register, scale } => {
            writer.open("register");
            match displacement.sign {
                AddressSign::Positive => writer.line("(sign +)"),
                AddressSign::Negative => writer.line("(sign -)"),
            }
            writer.line(&format!("(name {})", quote(register)));
            if let Some(scale) = scale {
                writer.line(&format!("(scale {})", quote(scale)));
            }
            writer.close();
        }
        AddressDisplacementKind::Symbol(symbol) => {
            writer.open("symbol");
            match displacement.sign {
                AddressSign::Positive => writer.line("(sign +)"),
                AddressSign::Negative => writer.line("(sign -)"),
            }
            writer.line(&format!("(name {})", quote(symbol)));
            writer.close();
        }
        AddressDisplacementKind::Immediate(value) => {
            writer.open("immediate");
            match displacement.sign {
                AddressSign::Positive => writer.line("(sign +)"),
                AddressSign::Negative => writer.line("(sign -)"),
            }
            writer.line(&format!("(value {})", quote(value)));
            writer.close();
        }
    }
}

fn join_quoted(items: &[String]) -> String {
    items
        .iter()
        .map(|item| quote(item))
        .collect::<Vec<_>>()
        .join(" ")
}

fn quote(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

struct TreeWriter {
    output: String,
    indent: usize,
}

impl TreeWriter {
    fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
        }
    }

    fn line(&mut self, text: &str) {
        for _ in 0..self.indent {
            self.output.push_str("  ");
        }
        self.output.push_str(text);
        self.output.push('\n');
    }

    fn open(&mut self, label: &str) {
        self.line(&format!("({label}"));
        self.indent += 1;
    }

    fn close(&mut self) {
        if self.indent == 0 {
            return;
        }
        self.indent -= 1;
        for _ in 0..self.indent {
            self.output.push_str("  ");
        }
        self.output.push_str(")\n");
    }

    fn finish(mut self) -> String {
        while self.indent > 0 {
            self.close();
        }
        self.output
    }
}
