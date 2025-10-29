
import os
import subprocess
import argparse # Import argparse

def main():
    parser = argparse.ArgumentParser(description="Compare documentation with Rust implementation for PTX instructions.")
    parser.add_argument("--start", type=int, default=1,
                        help="Start processing from the N-th instruction (1-indexed).")
    parser.add_argument("--end", type=int, default=0,
                        help="End processing at the N-th instruction (1-indexed). If 0, process until the end.")
    args = parser.parse_args()

    user_instructions = [
        "abs", "cvta", "membar", "setp", "vabsdiff",
        "activemask", "discard", "min", "shf", "vabsdiff2",
        "add", "div", "mma", "shfl", "vabsdiff4",
        "addc", "dp2a", "mov", "shl", "vadd",
        "alloca", "dp4a", "movmatrix", "shr", "vadd2",
        "and", "elect", "mul", "sin", "vadd4",
        "applypriority", "ex2", "mul24", "slct", "vavrg2",
        "atom", "exit", "multimem", "sqrt", "vavrg4",
        "bar", "fence", "nanosleep", "st", "vmad",
        "barrier", "fma", "neg", "stackrestore", "vmax",
        "bfe", "fns", "not", "stacksave", "vmax2",
        "bfi", "getctarank", "or", "stmatrix", "vmax4",
        "bfind", "griddepcontrol", "pmevent", "sub", "vmin",
        "bmsk", "isspacep", "popc", "subc", "vmin2",
        "bra", "istypep", "prefetch", "suld", "vmin4",
        "brev", "ld", "prefetchu", "suq", "vote",
        "brkpt", "ldmatrix", "prmt", "sured", "vset",
        "brx", "ldu", "rcp", "sust", "vset2",
        "call", "lg2", "red", "szext", "vset4",
        "clz", "lop3", "redux", "tanh", "vshl",
        "cnot", "mad", "rem", "tcgen05", "vshr",
        "copysign", "mad24", "ret", "tensormap", "vsub",
        "cos", "madc", "rsqrt", "testp", "vsub2",
        "clusterlaunchcontrol", "mapa", "sad", "tex", "vsub4",
        "cp", "match", "selp", "tld4", "wgmma",
        "createpolicy", "max", "set", "trap", "wmma",
        "cvt", "mbarrier", "setmaxnreg", "txq", "xor"
    ]

    docs_dir = 'docs/instructions'
    impl_dir = 'src/type/instructions'

    # Create /tmp/instructions directory if it doesn't exist
    output_tmp_dir = '/tmp/instructions'
    os.makedirs(output_tmp_dir, exist_ok=True)

    start_index = args.start - 1 # Convert to 0-indexed
    end_index = args.end if args.end == 0 else args.end - 1 # Convert to 0-indexed, 0 means until end

    if start_index < 0 or start_index >= len(user_instructions):
        print(f"Error: --start value {args.start} is out of range. Must be between 1 and {len(user_instructions)}.")
        return
    if args.end != 0 and (end_index < start_index or end_index >= len(user_instructions)):
        print(f"Error: --end value {args.end} is out of range or less than --start. Must be between {args.start} and {len(user_instructions)}.")
        return

    instructions_to_process = user_instructions[start_index : end_index + 1 if args.end != 0 else len(user_instructions)]

    for i, instruction_name in enumerate(instructions_to_process):
        current_instruction_number = start_index + i + 1 # 1-indexed instruction number
        print(f"- {current_instruction_number}/{len(user_instructions)}: {instruction_name}")

        doc_path = os.path.join(docs_dir, f'{instruction_name}.md')
        impl_path = os.path.join(impl_dir, f'{instruction_name}.rs')

        if not os.path.exists(doc_path):
            print(f"Warning: Documentation file not found for {instruction_name} at {doc_path}")
            continue
        if not os.path.exists(impl_path):
            print(f"Warning: Implementation file not found for {instruction_name} at {impl_path}")
            continue

        with open(doc_path, 'r') as f:
            doc_content = f.read()
        with open(impl_path, 'r') as f:
            impl_content = f.read()

        prompt = f"""Compare the following documentation with its Rust implementation at `src/type/instructions/{instruction_name}.rs`. Fix any discrepancies in the Rust code to ensure it accurately reflects the documented behavior. Refactor when it is away from the guidelines below.
        Guidelines:
         - Note that the Rust code only focuses on the syntax and structure of the instruction, not its full semantics. You should only modify the type definitions but not add any new functions. 
         - In general, you should map the syntax to the type definitions as follows: 
            1. For each parallel possibility (examples are 1. different forms of the instruction 2. different data types, like `.type = {{ .u32, .u64}}`), use an enum variant.
            2. For each sequence of elements, use a struct with named fields.
            3. Reuse primitives from `src/type/common.rs` (for example, `Predicate`, `AddressSpace`, etc.) and avoid using "String" in the definition. If you find something missing, you should create a new basic type for it in `common.rs`. But `DataType` in `common.rs` is an exception, you should define a new enum for data types specific to the instruction.
            4. Name the new types without prefixes when possible. For example, if the instruction is `add`, name the main type `Add` instead of `AddInstruction` or similar. For the datatype in `add`, name it just `DataType`, because the module path already indicates it's for `add`.
            5. Comment the corresponding lines with the syntax from the given documentation without any extra clichés.
         - The Rust type definition should as flat as possible, don't nest things unnecessarily (especially when the syntax does not have nested structures). In theory, there should not be a struct with a single field. For example of flattening:
              ```rust
                /// `p`
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct ScalarRegister(pub RegisterOperand);

                #[derive(Debug, Clone, PartialEq, Eq)]
                pub enum GenericSource {{
                    /// `a`
                    Register(ScalarRegister),
                    /// `var`
                    Variable(VariableSymbol),
                    /// `var+imm`
                    VariableWithImmediate(VariableWithImmediate),
                }}

                /// `var`
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct VariableSymbol(pub String);
                ```
                should be just
                ```rust
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub enum GenericSource {{
                    /// `a`
                    Register(RegisterOperand),
                    /// `var`
                    Variable(String),
                    /// `var+imm`
                    VariableWithImmediate(VariableWithImmediate),
                }}

               ```
               For another example,
               ```rust
               /// `movmatrix.sync.aligned.shape.trans.type d, a;`
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Movmatrix {{
                    pub shape: Shape,
                    pub data_type: DataType,
                    pub operands: Operands,
                }}

                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Operands {{
                    pub destination: RegisterOperand,
                    pub source: RegisterOperand,
                }}
                ```
                should be just
                ```rust
               /// `movmatrix.sync.aligned.shape.trans.type d, a;`
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Movmatrix {{
                    pub shape: Shape,
                    pub data_type: DataType,
                    pub destination: RegisterOperand,
                    pub source: RegisterOperand,
                }}
                ```
                For another example,
                ```rust
                /// `immAlign`
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct Alignment(pub String);
                ```
                should be just a `String` with a comment `/// immAlign`.
 
         - Don't worry about other files or any re-exports. So you don't need to modify add bullshit like 
            ```rust
            pub type CvtaOpcode = Cvta;
            pub type CvtaOperands = Cvta;
            pub type CvtaModifiers = ();
            ```


--- Documentation ({instruction_name}.md) ---
{doc_content}
"""
        # print(prompt)
        
        try:
            # Call codex using subprocess
            codex_result = subprocess.run(['codex', 'e', prompt], capture_output=True, text=True, check=True)
            codex_response = codex_result.stdout
            outage = "ERROR: You've hit your usage limit. To get more access now, send a request to your admin or try again later."
            if outage in codex_result.stderr:
                print(f"\n--- {outage} ---")
        except FileNotFoundError:
            codex_response = "Error: 'codex' command not found. Please ensure 'codex' is installed and accessible in your PATH."
        except subprocess.CalledProcessError as e:
            codex_response = f"Error calling 'codex': {e}\nStdout: {e.stdout}\nStderr: {e.stderr}"
        except Exception as e:
            codex_response = f"An unexpected error occurred while calling 'codex': {e}"

        # Record the codex response to a file
        output_file_path = os.path.join(output_tmp_dir, f'{instruction_name}.md')
        with open(output_file_path, 'w') as f:
            f.write(codex_response)
        print(f"  Codex response saved to {output_file_path}")

        # print(f"\n--- Codex (AI) Response for {instruction_name} ---")
        # print(codex_response)
        # print("\n")

        # # Wait for user input
        # input("Press Enter to continue to the next instruction...")
        # print("\n" + "="*80 + "\n") # Separator for readability

if __name__ == "__main__":
    main()
