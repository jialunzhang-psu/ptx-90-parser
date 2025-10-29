import os
import re
from bs4 import BeautifulSoup

def get_instruction_name_from_heading(heading):
    code_tag = heading.find('code')
    if code_tag:
        return code_tag.text.strip()
    return None

def get_section_content(start_element, section_name):
    section_heading_p = start_element.find_next_sibling('p', class_='rubric', string=section_name)
    if not section_heading_p:
        return None

    content_elements = []
    next_element = section_heading_p.find_next_sibling()
    while next_element and (next_element.name != 'p' or 'rubric' not in next_element.get('class', [])):
        if next_element.name == 'div' and 'highlight-text' in next_element.get('class', []):
            div_content = next_element.find('div', class_='highlight')
            if div_content:
                for button_tag in div_content.find_all('button', class_='copybtn'):
                    button_tag.decompose()
                content_elements.append(f"```\n{div_content.text.strip()}\n```")
            else:
                content_elements.append(f"```\n{next_element.text.strip()}\n```")
        elif next_element.name == 'p':
            content_elements.append(next_element.text.strip())
        next_element = next_element.find_next_sibling()

    return "\n".join(content_elements)

def main():
    doc_path = 'doc.html'
    if not os.path.exists(doc_path):
        print(f"Error: {doc_path} not found.")
        return

    with open(doc_path, 'r') as f:
        soup = BeautifulSoup(f, 'html.parser')

    output_dir = 'docs/instructions'
    os.makedirs(output_dir, exist_ok=True)

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
        "bfe", "fns", "not", "stacksave", "vmax2", # Corrected "stacsave" to "stacksave"
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

    processed_instruction_names = set()

    for target_instruction_name in user_instructions:
        if target_instruction_name in processed_instruction_names:
            continue

        found_documentation = False
        
        # Search for an exact match for the instruction name in doc.html
        # We will search within all <p class="rubric"> tags
        for rubric_code_tag in soup.find_all('p', class_='rubric'):
            code_tags = rubric_code_tag.find_all('code')
            
            for code_tag in code_tags:
                instruction_name_in_doc = code_tag.text.strip()

                if instruction_name_in_doc == target_instruction_name:
                    # Found an exact match
                    description = get_section_content(rubric_code_tag, 'Description')
                    syntax = get_section_content(rubric_code_tag, 'Syntax')
                    semantics = get_section_content(rubric_code_tag, 'Semantics')
                    examples = get_section_content(rubric_code_tag, 'Examples')

                    if any([description, syntax, semantics, examples]):
                        file_path = os.path.join(output_dir, f'{target_instruction_name}.md')
                        with open(file_path, 'w') as f:
                            if description:
                                f.write(f"### Description\n\n{description}\n\n")
                            if syntax:
                                f.write(f"### Syntax\n\n{syntax}\n\n")
                            if semantics:
                                f.write(f"### Semantics\n\n{semantics}\n\n")
                            if examples:
                                f.write(f"### Examples\n\n{examples}\n\n")
                        print(f"Wrote documentation for {target_instruction_name} to {file_path}.")
                        processed_instruction_names.add(target_instruction_name)
                        found_documentation = True
                        break # Break from inner loop (code_tags)
            if found_documentation:
                break # Break from outer loop (rubric_code_tag)
        
        if not found_documentation:
            # If no exact match, try to find a partial match (e.g., user_instruction "elect" matches "elect.sync")
            # This part is crucial for instructions like "elect" that are documented as "elect.sync" or "elect.sync.aligned" etc.
            for rubric_code_tag in soup.find_all('p', class_='rubric'):
                code_tags = rubric_code_tag.find_all('code')
                for code_tag in code_tags:
                    instruction_name_in_doc = code_tag.text.strip()
                    if instruction_name_in_doc.startswith(target_instruction_name + '.'):
                        description = get_section_content(rubric_code_tag, 'Description')
                        syntax = get_section_content(rubric_code_tag, 'Syntax')
                        semantics = get_section_content(rubric_code_tag, 'Semantics')
                        examples = get_section_content(rubric_code_tag, 'Examples')

                        if any([description, syntax, semantics, examples]):
                            file_path = os.path.join(output_dir, f'{target_instruction_name}.md') # Use user's name for file
                            with open(file_path, 'w') as f:
                                if description:
                                    f.write(f"### Description\n\n{description}\n\n")
                                if syntax:
                                    f.write(f"### Syntax\n\n{syntax}\n\n")
                                if semantics:
                                    f.write(f"### Semantics\n\n{semantics}\n\n")
                                if examples:
                                    f.write(f"### Examples\n\n{examples}\n\n")
                            print(f"Wrote documentation for {target_instruction_name} (from {instruction_name_in_doc}) to {file_path}.")
                            processed_instruction_names.add(target_instruction_name)
                            found_documentation = True
                            break # Break from inner loop (code_tags)
                if found_documentation:
                    break # Break from outer loop (rubric_code_tag)

if __name__ == "__main__":
    main()