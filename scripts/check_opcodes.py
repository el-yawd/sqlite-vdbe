#!/usr/bin/env python3
"""
Check which SQLite VDBE opcodes are implemented in the Rust Insn enum.

This script:
1. Fetches opcode list from https://sqlite.org/opcode.html (or local file)
2. Parses src/insn.rs to find implemented opcodes
3. Reports missing and extra opcodes

Usage:
    python3 scripts/check_opcodes.py [/path/to/opcode.html]
"""

import re
import sys
from pathlib import Path


def fetch_sqlite_opcodes(local_file=None):
    """Get the list of all SQLite opcodes from documentation."""
    if local_file and Path(local_file).exists():
        print(f"Reading opcodes from: {local_file}")
        html = Path(local_file).read_text()
    else:
        import urllib.request
        url = 'https://sqlite.org/opcode.html'
        print(f"Fetching opcodes from: {url}")
        req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})
        with urllib.request.urlopen(req, timeout=30) as response:
            html = response.read().decode('utf-8')

    # Extract opcode names from <a name="OpcodeName"></a>
    pattern = r'<a name="([A-Za-z0-9_]+)"></a>\1'
    opcodes = set(re.findall(pattern, html))

    print(f"Found {len(opcodes)} SQLite opcodes")
    return opcodes


def parse_raw_opcode_enum(insn_path: Path):
    """Parse the RawOpcode enum to get implemented opcode values."""
    content = insn_path.read_text()

    # Find the RawOpcode enum
    enum_match = re.search(
        r'pub enum RawOpcode \{(.*?)\n\}',
        content,
        re.DOTALL
    )

    if not enum_match:
        print("ERROR: Could not find RawOpcode enum")
        return set()

    enum_body = enum_match.group(1)

    # Extract variant names (e.g., "Add = 106" -> "Add")
    variants = re.findall(r'(\w+)\s*=\s*\d+', enum_body)

    return set(variants)


def parse_insn_enum(insn_path: Path):
    """Parse the Insn enum to get high-level instruction variants."""
    content = insn_path.read_text()

    # Find the Insn enum
    enum_match = re.search(
        r'pub enum Insn \{(.*?)\n\}',
        content,
        re.DOTALL
    )

    if not enum_match:
        print("ERROR: Could not find Insn enum")
        return {}, set()

    enum_body = enum_match.group(1)

    # Extract variant names and their struct/unit form
    # Matches: "VariantName {" or "VariantName,"
    variants = re.findall(r'^\s+(\w+)\s*[{,]', enum_body, re.MULTILINE)

    # Also parse the raw_opcode() method to get the mapping
    opcode_match = re.search(
        r'pub fn raw_opcode\(&self\) -> u8 \{.*?match self \{(.*?)\n\s+\}\s*\}',
        content,
        re.DOTALL
    )

    variant_to_opcode = {}
    if opcode_match:
        match_body = opcode_match.group(1)
        # Parse lines like: Insn::Add { .. } => RawOpcode::Add as u8,
        mappings = re.findall(
            r'Insn::(\w+)\s*(?:\{[^}]*\}|)?\s*=>\s*RawOpcode::(\w+)',
            match_body
        )
        for variant, opcode in mappings:
            variant_to_opcode[variant] = opcode

    return variant_to_opcode, set(variants)


def main():
    # Check for local file argument
    local_file = sys.argv[1] if len(sys.argv) > 1 else '/tmp/opcode.html'

    # Get SQLite opcodes
    try:
        sqlite_opcodes = fetch_sqlite_opcodes(local_file)
    except Exception as e:
        print(f"ERROR: Could not fetch SQLite opcodes: {e}")
        print("Try downloading manually: curl -fsSL https://sqlite.org/opcode.html -o /tmp/opcode.html")
        sys.exit(1)

    # Get implemented opcodes from Rust source
    insn_path = Path(__file__).parent.parent / 'src' / 'insn.rs'
    print(f"Reading Rust source: {insn_path}")

    raw_opcodes = parse_raw_opcode_enum(insn_path)
    variant_to_opcode, insn_variants = parse_insn_enum(insn_path)

    print(f"Found {len(raw_opcodes)} opcodes in RawOpcode enum")
    print(f"Found {len(insn_variants)} variants in Insn enum")

    # Opcodes in RawOpcode but not in SQLite docs (might be deprecated or internal)
    extra_raw = raw_opcodes - sqlite_opcodes
    if extra_raw:
        print(f"\n[INFO] {len(extra_raw)} opcodes in RawOpcode but not in SQLite docs:")
        for op in sorted(extra_raw):
            print(f"  - {op}")

    # Opcodes in SQLite docs but not in RawOpcode
    missing_raw = sqlite_opcodes - raw_opcodes
    if missing_raw:
        print(f"\n[WARNING] {len(missing_raw)} SQLite opcodes missing from RawOpcode enum:")
        for op in sorted(missing_raw):
            print(f"  - {op}")

    # Check which RawOpcodes have high-level Insn wrappers
    opcodes_with_insn = set(variant_to_opcode.values())
    opcodes_without_insn = raw_opcodes - opcodes_with_insn

    # Exclude 'Raw' variant which is a catch-all
    insn_variants_real = insn_variants - {'Raw'}

    print(f"\n--- Coverage Summary ---")
    print(f"SQLite opcodes documented:     {len(sqlite_opcodes)}")
    print(f"RawOpcode enum variants:       {len(raw_opcodes)}")
    print(f"Insn enum variants:            {len(insn_variants_real)} (excluding Raw)")
    print(f"Opcodes with Insn wrapper:     {len(opcodes_with_insn)}")

    if opcodes_without_insn:
        print(f"\n[INFO] {len(opcodes_without_insn)} opcodes in RawOpcode without high-level Insn wrapper:")
        print("       (Use Insn::Raw { opcode: RawOpcode::X, ... } to access these)")
        # Group by category for readability
        categories = {
            'Virtual Tables': ['VFilter', 'VUpdate', 'VNext', 'VBegin', 'VCreate',
                              'VDestroy', 'VOpen', 'VCheck', 'VInitIn', 'VColumn', 'VRename'],
            'Savepoints/Transactions': ['Savepoint', 'AutoCommit', 'Transaction',
                                        'Checkpoint', 'JournalMode', 'Vacuum'],
            'Sorting': ['SorterSort', 'Sort', 'SorterNext', 'SorterOpen',
                       'SorterCompare', 'SorterData', 'SorterInsert', 'ResetSorter'],
            'Functions': ['PureFunc', 'Function', 'AggInverse', 'AggStep1', 'AggValue'],
            'Cursors (Advanced)': ['OpenDup', 'OpenAutoindex', 'OpenPseudo', 'ColumnsUsed',
                                   'SeekScan', 'SeekHit', 'Sequence', 'SequenceTest',
                                   'RowCell', 'NullRow', 'SeekEnd', 'DeferredSeek',
                                   'FinishSeek', 'RowData'],
            'Schema/DDL': ['CreateBtree', 'SqlExec', 'ParseSchema', 'LoadAnalysis',
                          'DropTable', 'DropIndex', 'DropTrigger', 'Destroy', 'Clear'],
            'Other': []
        }

        categorized = set()
        for cat, ops in categories.items():
            found = [op for op in ops if op in opcodes_without_insn]
            if found:
                print(f"\n  {cat}:")
                for op in sorted(found):
                    print(f"    - {op}")
                    categorized.add(op)

        uncategorized = opcodes_without_insn - categorized
        if uncategorized:
            print(f"\n  Other:")
            for op in sorted(uncategorized):
                print(f"    - {op}")

    # Final status
    coverage = len(opcodes_with_insn) / len(raw_opcodes) * 100 if raw_opcodes else 0
    print(f"\n--- Result ---")
    print(f"High-level Insn coverage: {coverage:.1f}%")

    if missing_raw:
        print(f"\n[WARNING] {len(missing_raw)} opcodes in SQLite docs but not in RawOpcode enum:")
        print("          (These may be from a newer SQLite version than bundled 3.45.0)")
        for op in sorted(missing_raw):
            print(f"  - {op}")
        # Don't fail - just warn, as docs may be for a newer version
        sys.exit(0)
    else:
        print("\nAll SQLite opcodes are present in RawOpcode enum!")
        sys.exit(0)


if __name__ == '__main__':
    main()
