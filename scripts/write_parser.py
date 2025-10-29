
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
        "abs", "cvta", "membar", "setp", 
        "activemask", "discard", "min", "shf", 
        "add", "div", "mma", "shfl", 
        "addc", "dp2a", "mov", "shl", "vop",
        "alloca", "dp4a", "movmatrix", "shr", "vop2",
        "and", "elect", "mul", "sin", "vop4",
        "applypriority", "ex2", "mul24", "slct", 
        "atom", "exit", "multimem", "sqrt", 
        "bar", "fence", "nanosleep", "st", "vmad",
        "barrier", "fma", "neg", "stackrestore", 
        "bfe", "fns", "not", "stacksave", 
        "bfi", "getctarank", "or", "stmatrix", 
        "bfind", "griddepcontrol", "pmevent", "sub",
        "bmsk", "isspacep", "popc", "subc", 
        "bra", "istypep", "prefetch", "suld", 
        "brev", "ld", "prefetchu", "suq", "vote",
        "brkpt", "ldmatrix", "prmt", "sured", "vset",
        "brx", "ldu", "rcp", "sust", "vset2",
        "call", "lg2", "red", "szext", "vset4",
        "clz", "lop3", "redux", "tanh", "vsh",
        "cnot", "mad", "rem", "tcgen05", 
        "copysign", "mad24", "ret", "tensormap",
        "cos", "madc", "rsqrt", "testp", 
        "clusterlaunchcontrol", "mapa", "sad", "tex",
        "cp", "match", "selp", "tld4", "wgmma",
        "createpolicy", "max", "set", "trap", "wmma",
        "cvt", "mbarrier", "setmaxnreg", "txq", "xor"
    ]
    docs_dir = 'docs/instructions'
    impl_dir = 'src/type/instruction'

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

        impl_path = os.path.join(impl_dir, f'{instruction_name}.rs')

        if not os.path.exists(impl_path):
            print(f"Warning: Implementation file not found for {instruction_name} at {impl_path}")
            continue

        with open(impl_path, 'r') as f:
            impl_content = f.read()

        prompt = f"""Now you are writing a part of a PTX code parser. Read `src/type/instruction/{instruction_name}.rs` and `src/parser/mod.rs` and `src/type/common.rs`. In `src/parser/instruction/{instruction_name}.rs`, implement the `PtxParser` trait (in `src/parser/mod.rs`) for the instruction. 
        The implementation start with 
        ```rust
        use crate::{{
            lexer::PtxToken,
            parser::*,
            r#type::{{common::*, instruction::{instruction_name}::*}}
        }};
        ```.
        Then add both positive and negative tests in `tests/parser/instruction/{instruction_name}.rs`. Run the tests to make sure they pass. Don't make type alias for instruction types. """
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
