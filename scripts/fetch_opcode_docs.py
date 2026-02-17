#!/usr/bin/env python3
"""
Fetch SQLite opcode documentation and update Rust source with doc comments.

This script:
1. Fetches opcode documentation from https://sqlite.org/opcode.html
2. Parses the HTML to extract opcode names and descriptions
3. Updates src/insn.rs with proper documentation for each Insn enum variant
"""

import re
import urllib.request
from html.parser import HTMLParser
from pathlib import Path


class OpcodeParser(HTMLParser):
    """Parse SQLite opcode documentation HTML."""

    def __init__(self):
        super().__init__()
        self.opcodes = {}
        self.current_opcode = None
        self.current_desc = []
        self.in_dt = False
        self.in_dd = False
        self.in_p = False
        self.capture_text = False

    def handle_starttag(self, tag, attrs):
        if tag == "dt":
            self.in_dt = True
            self.capture_text = True
        elif tag == "dd":
            self.in_dd = True
        elif tag == "p" and self.in_dd:
            self.in_p = True
            self.capture_text = True

    def handle_endtag(self, tag):
        if tag == "dt":
            self.in_dt = False
            self.capture_text = False
        elif tag == "dd":
            if self.current_opcode and self.current_desc:
                # Join paragraphs and clean up
                desc = "\n\n".join(self.current_desc)
                self.opcodes[self.current_opcode] = desc
            self.in_dd = False
            self.current_opcode = None
            self.current_desc = []
        elif tag == "p" and self.in_dd:
            self.in_p = False
            self.capture_text = False

    def handle_data(self, data):
        data = data.strip()
        if not data:
            return

        if self.in_dt:
            # Extract opcode name (e.g., "Add" from "Add")
            self.current_opcode = data.strip()
        elif self.in_p and self.in_dd:
            # Accumulate description text
            if self.current_desc and not self.current_desc[-1].endswith(
                (".", ":", '"')
            ):
                self.current_desc[-1] += " " + data
            else:
                if len(self.current_desc) > 0 and self.current_desc[-1]:
                    self.current_desc[-1] += " " + data
                else:
                    self.current_desc.append(data)

    def handle_entityref(self, name):
        if self.capture_text:
            entities = {"lt": "<", "gt": ">", "amp": "&", "quot": '"', "nbsp": " "}
            char = entities.get(name, f"&{name};")
            if self.in_p and self.in_dd and self.current_desc:
                self.current_desc[-1] += char


def fetch_opcode_docs(local_file=None):
    """Fetch and parse opcode documentation from SQLite website or local file."""
    if local_file and Path(local_file).exists():
        print(f"Reading from local file: {local_file}")
        html = Path(local_file).read_text()
    else:
        url = "https://sqlite.org/opcode.html"
        print(f"Fetching {url}...")
        req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
        with urllib.request.urlopen(req, timeout=30) as response:
            html = response.read().decode("utf-8")

    # The opcodes are in a table format:
    # <a name="OpcodeName"></a>OpcodeName in one cell, description in next <td>
    opcodes = {}

    # Pattern to match opcode entries in the table
    # Format: <a name="OpcodeName"></a>OpcodeName\n<td>description</td>
    pattern = r'<a name="([A-Za-z0-9_]+)"></a>\1\s*\n<td>(.*?)</td></tr>'

    for match in re.finditer(pattern, html, re.DOTALL):
        name = match.group(1).strip()
        desc_html = match.group(2).strip()

        # Split into paragraphs (by <p> tags or by the first content before <p>)
        desc_parts = []

        # Handle content before the first <p> tag
        first_p_match = re.search(r"<p>", desc_html)
        if first_p_match:
            before_p = desc_html[: first_p_match.start()]
            before_p = re.sub(r"<[^>]+>", "", before_p)
            before_p = " ".join(before_p.split())
            if before_p:
                desc_parts.append(before_p)

            # Handle <p>...</p> content and content after
            remaining = desc_html[first_p_match.start() :]
            # Split on </p> to get paragraphs
            parts = re.split(r"</p>\s*(?:<p>)?", remaining)
            for part in parts:
                part = re.sub(r"<[^>]+>", "", part)
                part = " ".join(part.split())
                if part:
                    desc_parts.append(part)
        else:
            # No <p> tags, just clean the HTML
            desc_html = re.sub(r"<[^>]+>", "", desc_html)
            desc_html = " ".join(desc_html.split())
            if desc_html:
                desc_parts.append(desc_html)

        # Decode HTML entities
        full_desc = "\n\n".join(desc_parts)
        full_desc = full_desc.replace("&lt;", "<").replace("&gt;", ">")
        full_desc = full_desc.replace("&amp;", "&").replace("&quot;", '"')
        full_desc = full_desc.replace("&nbsp;", " ")
        full_desc = full_desc.replace("&#91;", "[").replace("&#93;", "]")

        if full_desc:
            opcodes[name] = full_desc

    print(f"Found {len(opcodes)} opcodes")
    return opcodes


def format_doc_comment(desc: str, indent: str = "    ") -> str:
    """Format description as Rust doc comment."""
    lines = []

    # Split into paragraphs
    paragraphs = desc.split("\n\n")

    for i, para in enumerate(paragraphs):
        # Wrap long lines at ~76 chars (accounting for indent + "/// ")
        words = para.split()
        current_line = ""

        for word in words:
            if len(current_line) + len(word) + 1 > 72:
                lines.append(f"{indent}/// {current_line}")
                current_line = word
            else:
                current_line = f"{current_line} {word}".strip()

        if current_line:
            lines.append(f"{indent}/// {current_line}")

        # Add blank doc line between paragraphs
        if i < len(paragraphs) - 1:
            lines.append(f"{indent}///")

    return "\n".join(lines)


def update_insn_rs(opcodes: dict):
    """Update src/insn.rs with opcode documentation."""
    insn_path = Path(__file__).parent.parent / "src" / "insn.rs"

    print(f"Reading {insn_path}...")
    content = insn_path.read_text()

    # Map of Insn variant names to their SQLite opcode names
    # Some names differ between our API and SQLite
    variant_to_opcode = {
        "Integer": "Integer",
        "Int64": "Int64",
        "Real": "Real",
        "String8": "String8",
        "Null": "Null",
        "Add": "Add",
        "Subtract": "Subtract",
        "Multiply": "Multiply",
        "Divide": "Divide",
        "Remainder": "Remainder",
        "Concat": "Concat",
        "BitAnd": "BitAnd",
        "BitOr": "BitOr",
        "ShiftLeft": "ShiftLeft",
        "ShiftRight": "ShiftRight",
        "BitNot": "BitNot",
        "Not": "Not",
        "AddImm": "AddImm",
        "Copy": "Copy",
        "SCopy": "SCopy",
        "Move": "Move",
        "IntCopy": "IntCopy",
        "Halt": "Halt",
        "HaltWithError": "Halt",
        "HaltIfNull": "HaltIfNull",
        "Goto": "Goto",
        "Gosub": "Gosub",
        "Return": "Return",
        "If": "If",
        "IfNot": "IfNot",
        "IsNull": "IsNull",
        "NotNull": "NotNull",
        "Once": "Once",
        "Jump": "Jump",
        "Eq": "Eq",
        "Ne": "Ne",
        "Lt": "Lt",
        "Le": "Le",
        "Gt": "Gt",
        "Ge": "Ge",
        "IfPos": "IfPos",
        "IfNotZero": "IfNotZero",
        "DecrJumpZero": "DecrJumpZero",
        "MustBeInt": "MustBeInt",
        "ResultRow": "ResultRow",
        "OpenRead": "OpenRead",
        "OpenWrite": "OpenWrite",
        "OpenEphemeral": "OpenEphemeral",
        "Close": "Close",
        "Rewind": "Rewind",
        "Next": "Next",
        "Prev": "Prev",
        "Last": "Last",
        "SeekGE": "SeekGE",
        "SeekGT": "SeekGT",
        "SeekLE": "SeekLE",
        "SeekLT": "SeekLT",
        "SeekRowid": "SeekRowid",
        "Column": "Column",
        "Rowid": "Rowid",
        "NewRowid": "NewRowid",
        "Insert": "Insert",
        "Delete": "Delete",
        "MakeRecord": "MakeRecord",
        "IdxInsert": "IdxInsert",
        "IdxDelete": "IdxDelete",
        "IdxRowid": "IdxRowid",
        "Init": "Init",
        "InitCoroutine": "InitCoroutine",
        "Yield": "Yield",
        "EndCoroutine": "EndCoroutine",
        "AggStep": "AggStep",
        "AggFinal": "AggFinal",
        "Noop": "Noop",
        "Explain": "Explain",
    }

    updates_made = 0

    for variant, opcode in variant_to_opcode.items():
        if opcode not in opcodes:
            print(f"  Warning: No documentation found for {opcode}")
            continue

        doc = opcodes[opcode]

        # Create the new doc comment
        doc_comment = format_doc_comment(doc)

        # Find the variant in the enum and update its documentation
        # Pattern matches: existing doc comment (if any) + variant definition
        # We look for patterns like:
        #   /// existing doc
        #   VariantName {
        # or just:
        #   VariantName {

        # First, try to find variant with existing docs
        pattern = rf"(    /// [^\n]*\n(?:    ///[^\n]*\n)*)?    {variant}\s*\{{"

        def replacer(m):
            nonlocal updates_made
            updates_made += 1
            return f"{doc_comment}\n    {variant} {{"

        new_content = re.sub(pattern, replacer, content, count=1)

        if new_content != content:
            content = new_content
        else:
            # Try pattern for unit variants (no braces)
            pattern = rf"(    /// [^\n]*\n(?:    ///[^\n]*\n)*)?    {variant},"

            def replacer_unit(m):
                nonlocal updates_made
                updates_made += 1
                return f"{doc_comment}\n    {variant},"

            new_content = re.sub(pattern, replacer_unit, content, count=1)
            if new_content != content:
                content = new_content

    print(f"Writing {insn_path} with {updates_made} documentation updates...")
    insn_path.write_text(content)
    print("Done!")


def main():
    import sys

    # Check for local file argument or default
    local_file = sys.argv[1] if len(sys.argv) > 1 else "/tmp/opcode.html"

    opcodes = fetch_opcode_docs(local_file)

    # Print a few examples
    print("\nExample opcodes found:")
    for name in ["Add", "Integer", "Goto", "Halt"][:4]:
        if name in opcodes:
            print(f"\n{name}:")
            print(f"  {opcodes[name][:200]}...")

    print("\n")
    update_insn_rs(opcodes)


if __name__ == "__main__":
    main()
