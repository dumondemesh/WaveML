#!/usr/bin/env python3
import sys, os, re
from pathlib import Path

def find_workspace_members(toml_text):
    # naive parse of workspace.members = [ ... ]
    m = re.search(r'workspace\s*=\s*\{[^}]*\}', toml_text, re.DOTALL)
    if not m:
        # try cargo's usual [workspace] table
        m2 = re.search(r'\[workspace\](.*?)\n\[', toml_text, re.DOTALL)
        block = m2.group(1) if m2 else ""
        mm = re.search(r'members\s*=\s*\[(.*?)\]', block, re.DOTALL)
        if not mm:
            return None, None
        arr = mm.group(1)
        start = mm.start(1); end = mm.end(1)
        return (start, end), arr
    else:
        # not typical; fallback
        return None, None

def parse_members(arr_text):
    items = []
    for line in arr_text.splitlines():
        line = line.strip()
        if not line: continue
        # strip comments
        if "#" in line: line = line.split("#",1)[0].strip()
        # trim quotes and commas
        line = line.strip(",").strip().strip('"').strip("'")
        if line:
            items.append(line)
    return items

def main():
    if len(sys.argv) < 2:
        print("Usage: trim_workspace.py Cargo.toml", file=sys.stderr)
        sys.exit(2)
    cargo = Path(sys.argv[1])
    txt = cargo.read_text(encoding="utf-8")
    # robust parse for [workspace] members
    m = re.search(r'\[workspace\](.*?)(\n\[|$)', txt, re.DOTALL)
    if not m:
        print("[WARN] No [workspace] section found; nothing to trim")
        sys.exit(0)
    block = m.group(1)
    members_m = re.search(r'members\s*=\s*\[(.*?)\]', block, re.DOTALL)
    if not members_m:
        print("[WARN] No workspace.members found; nothing to trim")
        sys.exit(0)
    arr_text = members_m.group(1)
    items = parse_members(arr_text)
    keep = []
    removed = []
    for it in items:
        p = cargo.parent / it
        if any(ch in it for ch in ["*", "?", "["]):  # globs left as-is
            keep.append(it)
            continue
        if p.exists():
            keep.append(it)
        else:
            removed.append(it)
    new_arr = ",\n  ".join(f'"{k}"' for k in keep)
    new_block = block[:members_m.start(1)] + new_arr + block[members_m.end(1):]
    new_txt = txt[:m.start(1)] + new_block + txt[m.end(1):]
    cargo.write_text(new_txt, encoding="utf-8")
    if removed:
        print("[OK] Trimmed missing workspace members:", ", ".join(removed))
    else:
        print("[OK] Workspace members unchanged")
if __name__ == "__main__":
    main()
