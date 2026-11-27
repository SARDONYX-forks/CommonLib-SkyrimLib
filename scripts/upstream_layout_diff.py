"""Report semantic layout changes between two CommonLib revisions for headers
that have a Rust translation: field offset moves, size asserts, vfunc index moves."""
import os, re, subprocess, sys, collections

REPO = os.environ.get('COMMONLIB_DIR', r'D:/Programming/cpp/CommonLibSSE-NG')
RS = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'libskyrim', 'src', 're')
BASE = sys.argv[1] if len(sys.argv) > 1 else 'cf14a90c4'  # CommonLib commit the Rust translation was synced to
HEAD = sys.argv[2] if len(sys.argv) > 2 else 'HEAD'
os.chdir(REPO)

def snake(n):
    n = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", n)
    n = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", n)
    return n.replace("__", "_").lower() + ".rs"

rs_files = set(os.listdir(RS))

def show(rev, path):
    r = subprocess.run(['git', 'show', f'{rev}:{path}'], capture_output=True, text=True, encoding='utf-8', errors='replace')
    return r.stdout if r.returncode == 0 else ''

FIELD = re.compile(r'^\s*(?:mutable\s+)?([\w:<>,\s\*&\[\]]+?)\s+(\w+)(\s*\[[^\]]*\])?\s*(?::\s*\d+)?\s*(?:\{[^}]*\})?\s*;\s*/[/*]\s*([0-9A-Fa-f]{2,5})\b')
FIELD_MACRO = re.compile(r'^\s*(?:mutable\s+)?([\w:<>,\s\*&\[\]]+?)\s+(\w+)(\s*\[[^\]]*\])?\s*;\s*/\*\s*([0-9A-Fa-f]{2,5})\b')
VIRT = re.compile(r'virtual\s+[^;]*?\b(\w+)\s*\([^;]*;\s*//\s*([0-9A-Fa-f]{2,3})\b')
SIZE = re.compile(r'(static_assert\(sizeof\((\w+)\)\s*==\s*(0x[0-9A-Fa-f]+)\))|(STATIC_ASSERT_SIZE\((\w+)\s*,([^)]*)\))')

def parse(txt):
    fields, virts, sizes = {}, {}, {}
    scope = []
    for line in txt.splitlines():
        m = re.match(r'^\s*(?:class|struct|union)\s+(?:alignas\(\w+\)\s+)?(\w+)', line)
        if m and not line.rstrip().endswith(';'):
            scope.append(m.group(1))
        m = FIELD.match(line) or FIELD_MACRO.match(line)
        if m and 'return' not in line and '(' not in m.group(1):
            key = (scope[-1] if scope else '?', m.group(2))
            fields.setdefault(key, set()).add(m.group(4).upper().lstrip('0') or '0')
        m = VIRT.search(line)
        if m:
            virts.setdefault((scope[-1] if scope else '?', m.group(1)), set()).add(m.group(2).upper().lstrip('0') or '0')
        for m in SIZE.finditer(line):
            if m.group(2):
                sizes.setdefault(m.group(2), set()).add(m.group(3))
            else:
                sizes.setdefault(m.group(5), set()).add(m.group(6).replace(' ', ''))
    return fields, virts, sizes

out = subprocess.run(['git', 'diff', '--name-only', BASE, HEAD, '--', 'include/RE'], capture_output=True, text=True).stdout.split()
report = []
for path in out:
    name = os.path.basename(path)[:-2]
    rs = snake(name)
    if rs not in rs_files:
        continue
    b, h = parse(show(BASE, path)), parse(show(HEAD, path))
    lines = []
    for k, offs in b[0].items():
        if k in h[0] and h[0][k] != offs:
            lines.append(f'  FIELD {k[0]}::{k[1]} {sorted(offs)} -> {sorted(h[0][k])}')
    removed = [k for k in b[0] if k not in h[0]]
    for k, offs in b[1].items():
        if k in h[1] and h[1][k] != offs:
            lines.append(f'  VFUNC {k[0]}::{k[1]} {sorted(offs)} -> {sorted(h[1][k])}')
    for k, v in b[2].items():
        if k in h[2] and h[2][k] != v:
            lines.append(f'  SIZE {k} {sorted(v)} -> {sorted(h[2][k])}')
    if lines:
        report.append(f'{rs} ({path})')
        report.extend(lines)
print('\n'.join(report))
