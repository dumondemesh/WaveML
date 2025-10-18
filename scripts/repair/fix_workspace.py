#!/usr/bin/env python3
import sys, re, os, json
from pathlib import Path

def find_table_ranges(s, header):
    pat = re.compile(r'^\[' + re.escape(header) + r'\]\s*$', re.MULTILINE)
    ranges = []
    for m in pat.finditer(s):
        start = m.start()
        next_m = re.search(r'^\[.*?\]\s*$', s[m.end():], re.MULTILINE)
        end = len(s) if not next_m else m.end() + next_m.start()
        ranges.append((start, end))
    return ranges

def build_members_list(root):
    root = Path(root)
    members = []
    crates_dir = root / "crates"
    if crates_dir.exists():
        for child in sorted(crates_dir.iterdir()):
            if child.is_dir() and (child/"Cargo.toml").exists():
                members.append(f"crates/{child.name}")
    return members

def render_workspace(members, resolver="2"):
    body = "[workspace]\n"
    if resolver:
        body += f'resolver = "{resolver}"\n'
    body += "members = [\n" + "".join(f'  "{m}",\n' for m in members) + "]\n"
    return body

def main():
    cargo_path = Path("Cargo.toml")
    if not cargo_path.exists():
        print("[ERR] Cargo.toml not found", file=sys.stderr); sys.exit(2)
    txt = cargo_path.read_text(encoding="utf-8")
    ranges = find_table_ranges(txt, "workspace")
    pre = txt[:ranges[0][0]] if ranges else txt
    post = txt[ranges[-1][1]:] if ranges else ""
    members = build_members_list(".")
    ws = render_workspace(members, resolver="2")
    new_txt = pre + ws + post
    backup = Path(str(cargo_path)+".bak_rc1g")
    backup.write_text(txt, encoding="utf-8")
    cargo_path.write_text(new_txt, encoding="utf-8")
    print(f"[OK] Rewrote workspace (resolver=2) with {len(members)} members")
    for m in members: print(" -", m)
    print(f"[OK] Backup saved to {backup}")
if __name__ == "__main__":
    main()
