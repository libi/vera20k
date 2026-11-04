#!/usr/bin/env python3
"""Regenerate docs/ares-support/mechanisms/<id>.md: per-mechanism reading index.

Each page lists the mechanism's keys, the Ares 3.0 hooks and helper functions to read
in ares-src (decompiled 3.0 first, recovered/0.A source second), the gamemd function
each hook sits in, the hotspots it shares, and VERA20K files that already cite those
native functions. Text between the notes markers is kept across regenerations.

Run after build_catalog.py:
  python3 -I tools/ares_support/build_index.py <ares-src> [vera20k root]
"""
import csv, os, re, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from mechanisms import MECHANISMS, HOOK_FILES  # noqa: E402

ares = sys.argv[1]
vera = sys.argv[2] if len(sys.argv) > 2 else os.path.dirname(os.path.dirname(HERE))
docs = os.path.join(vera, 'docs', 'ares-support')
out = os.path.join(docs, 'mechanisms')
NOTES_BEGIN, NOTES_END = '<!-- notes -->', '<!-- /notes -->'


def tsv(path):
    with open(path, newline='', encoding='utf-8', errors='replace') as f:
        return list(csv.DictReader(f, delimiter='\t'))


def ares_tsv(rel, header=None):
    with open(os.path.join(ares, rel), newline='', encoding='utf-8', errors='replace') as f:
        return list(csv.DictReader(f, delimiter='\t', fieldnames=header))


catalog = tsv(os.path.join(docs, 'catalog.tsv'))
hotspots = tsv(os.path.join(docs, 'hotspots.tsv'))

# Ares function address <-> name (decompiled index), hook table, hook source files
dec = ares_tsv('decompiled/index.tsv')
name_of = {r['address'].upper(): r['name'] for r in dec}
addr_of = defaultdict(list)
for r in dec:
    addr_of[r['name']].append(r['address'].upper())
hooks30 = ares_tsv('symbols/hooks_30.tsv', ['gamemd', 'name', 'size', 'ares'])
hook_by_ares = defaultdict(list)
for h in hooks30:
    hook_by_ares[h['ares'].upper()].append(h)
hook_info = {r['name']: r for r in ares_tsv('report/hooks.tsv')}
ctx = {r['hook']: r for r in ares_tsv('reference/gamemd_hooks/index.tsv')}
changed = {r['name'] for r in ares_tsv('report/changed_kept.tsv')}
verified = {r['address'].upper() for r in ares_tsv('symbols/verified_30.tsv')}
# Uses that only copy, construct or (de)serialise a field say nothing about behaviour.
INFRA = re.compile(r'Serialize|::ExtData::ExtData|~ExtData|LoadFromStream|SaveToStream|Swizzle|InheritSettings'
                   r'|LoadFromINI|::Initialize\b|CompleteInitialization|PrepareStream')


def mech_of_file(path):
    for frag, mid in HOOK_FILES:
        if frag in path:
            return mid
    return None


_src = {}
def src_line(path, needle):
    if path not in _src:
        try:
            _src[path] = open(os.path.join(ares, path), encoding='utf-8', errors='replace').read().splitlines()
        except OSError:
            _src[path] = []
    for i, l in enumerate(_src[path], 1):
        if needle in l:
            return '%s:%d' % (path, i)
    return path


def dec_path(addr):
    p = 'decompiled/functions/%s.c' % addr.upper()
    return p if os.path.exists(os.path.join(ares, p)) else ''


CALL = re.compile(r'\b([A-Za-z_][\w]*(?:::[\w~]+)+)\s*\(')
def callees(addr):
    """Named Ares functions called from a decompiled function (one level)."""
    p = dec_path(addr)
    if not p:
        return []
    text = open(os.path.join(ares, p), encoding='utf-8', errors='replace').read()
    own = name_of.get(addr.upper())
    found = []
    for n in CALL.findall(text):
        if n != own and n in addr_of and n not in found:
            found.append(n)
    return found


# VERA20K: which non-test .rs files cite a given native address
needed = set()
for h in hooks30:
    c = ctx.get(h['name'])
    if c:
        needed.add(int(c['gamemd_function_address'], 16))
    needed.add(int(h['gamemd'], 16))
vera_cites = defaultdict(set)
HEX = re.compile(r'(?<![0-9A-Za-z_])(?:0x)?([0-9A-Fa-f]{6,8})(?![0-9A-Za-z_])')
for root, _, files in os.walk(os.path.join(vera, 'src')):
    for fn in files:
        if fn.endswith('.rs') and not fn.endswith('_tests.rs') and fn != 'tests.rs':
            p = os.path.join(root, fn)
            rel = os.path.relpath(p, vera)
            for m in HEX.findall(open(p, encoding='utf-8', errors='replace').read()):
                try:
                    v = int(m, 16)
                except ValueError:
                    continue
                if v in needed:
                    vera_cites[v].add(rel)

keys_by = defaultdict(list)
for r in catalog:
    keys_by[r['mechanism']].append(r)
hot_by = defaultdict(list)
for h in hotspots:
    for m in h['mechanism_list'].split(', '):
        hot_by[m].append(h)

os.makedirs(out, exist_ok=True)
for mid, ms, name, dep, hint, doc in MECHANISMS:
    keys = keys_by.get(mid, [])
    # Ares functions that touch the keys
    use_funcs = {}
    for k in keys:
        for u in filter(None, (x.strip() for x in k['uses'].split(';'))):
            addr, rest = u.split(' ', 1)
            if not INFRA.search(rest):
                use_funcs.setdefault(addr.upper(), rest)
    # hooks: those whose Ares body is a use function, plus hooks in files mapped to this mechanism
    hooks = {}
    for addr in use_funcs:
        for h in hook_by_ares.get(addr, []):
            hooks[h['name']] = h
    for h in hooks30:
        info = hook_info.get(h['name'])
        if info and mech_of_file(info['file']) == mid:
            hooks[h['name']] = h
    hook_ares = {h['ares'].upper() for h in hooks.values()}
    helpers = []
    for a in sorted(hook_ares | set(use_funcs)):
        for n in callees(a):
            if INFRA.search(n) or re.match(r'(Ares(ByteStream|Swizzle|Stream)|CSFText)::', n):
                continue
            for ha in addr_of[n]:
                if ha not in hook_ares and ha not in use_funcs and (n, ha) not in helpers:
                    helpers.append((n, ha))

    path = os.path.join(out, mid + '.md')
    notes = '\n（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）\n'
    if os.path.exists(path):
        old = open(path, encoding='utf-8').read()
        if NOTES_BEGIN in old and NOTES_END in old:
            notes = old.split(NOTES_BEGIN, 1)[1].split(NOTES_END, 1)[0]

    L = ['# `%s` %s' % (mid, name), '',
         '> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。', '',
         '| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |', '|---|---|---|---|',
         '| %s | %s | %s | %s |' % (ms, dep or '—', hint, doc or '—'), '',
         '## 备注', NOTES_BEGIN + notes + NOTES_END, '']
    L += ['## 配置键（%d）' % len(keys), '']
    if keys:
        L += ['| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |', '|---|---|---|---|---|---|---|']
        for k in keys:
            L.append('| `%s` | %s | %s | %s | %s | %s | %s |' % (
                k['key'], k['cls'], k['type'].replace('|', '\\|') or '—', k['default'].replace('|', '\\|') or '—',
                k['read_at'] or k['loader'] or '—', k['use_status'] or '—', k['vera_literal'] or '—'))
    else:
        L.append('无键（基础设施或无条件行为）。')
    L += ['', '## 钩子（%d）' % len(hooks), '',
          '"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。'
          '`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。', '']
    if hooks:
        L += ['| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |',
              '|---|---|---|---|---|---|---|---|']
        for h in sorted(hooks.values(), key=lambda x: int(x['gamemd'], 16)):
            info = hook_info.get(h['name'], {})
            c = ctx.get(h['name'])
            fn = c['gamemd_function_address'] if c else ''
            cfile = 'reference/gamemd_hooks/%s.c' % fn if fn else '—'
            status = info.get('status', '?') + (' ⚠改过' if h['name'] in changed else '')
            src = src_line(info['file'], h['name']) if info.get('file') else '—'
            cites = sorted(vera_cites.get(int(fn, 16), ())) if fn else []
            ok = '✓' if (h['ares'].upper() in verified or info.get('status', '').startswith(('new', 'replaces'))) else '—'
            L.append('| %s | `%s` | %s | %s | %s `%s` | %s | %s | %s |' % (
                h['gamemd'], h['name'], status, ok, fn, c['gamemd_function'] if c else '?', dec_path(h['ares']) or '—',
                src, ', '.join(cites[:3]) + (' …' if len(cites) > 3 else '') or '—'))
    others = [(a, n) for a, n in use_funcs.items() if a not in hook_ares]
    if others or helpers:
        L += ['', '## 非钩子函数', '', '| Ares 地址 | 函数 | 来源 | 3.0 反编译 |', '|---|---|---|---|']
        for a, n in sorted(others):
            L.append('| %s | %s | 使用键 | %s |' % (a, n.replace('|', '\\|'), dec_path(a) or '—'))
        for n, a in helpers:
            L.append('| %s | `%s` | 被钩子调用 | %s |' % (a, n, dec_path(a) or '—'))
    hs = hot_by.get(mid, [])
    if hs:
        L += ['', '## 涉及的热点函数（%d）' % len(hs), '',
              '与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。', '',
              '| gamemd 函数 | 钩子数 | 共用机制 |', '|---|---|---|']
        for h in hs:
            L.append('| %s `%s` | %s | %s |' % (h['gamemd_address'], h['gamemd_function'], h['hooks'], h['mechanism_list']))
    open(path, 'w', encoding='utf-8').write('\n'.join(L) + '\n')

print('%d mechanism pages in %s' % (len(MECHANISMS), os.path.relpath(out, vera)))

# README milestone tables between <!-- table:Mx --> markers
readme = os.path.join(docs, 'README.md')
text = open(readme, encoding='utf-8').read()
for ms in sorted({m[1] for m in MECHANISMS}):
    begin, end = '<!-- table:%s -->' % ms, '<!-- /table:%s -->' % ms
    if begin not in text:
        continue
    rows = ['| 机制 | 名称 | 键 | 依赖 | 现状 |', '|---|---|---|---|---|']
    for mid, m, name, dep, hint, _ in MECHANISMS:
        if m == ms:
            rows.append('| [`%s`](mechanisms/%s.md) | %s | %d | %s | %s |' % (
                mid, mid, name, len(keys_by.get(mid, [])), dep or '—', hint))
    head, rest = text.split(begin, 1)
    _, tail = rest.split(end, 1)
    text = head + begin + '\n' + '\n'.join(rows) + '\n' + end + tail
open(readme, 'w', encoding='utf-8').write(text)
