#!/usr/bin/env python3
"""Regenerate docs/ares-support/{catalog,mechanisms,hotspots}.tsv.

Needs a checkout of the Ares 3.0 recovery workspace (ares-src: recovered source,
symbol tables and reports); this repository holds none of it.

usage: python3 -I tools/ares_support/build_catalog.py <ares-src> [vera20k root]
"""
import csv, os, re, sys
from collections import defaultdict, Counter

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from mechanisms import MECHANISMS, RULES, IGNORE, HOOK_FILES  # noqa: E402

ares = sys.argv[1]
vera = sys.argv[2] if len(sys.argv) > 2 else os.path.dirname(os.path.dirname(HERE))
out = os.path.join(vera, 'docs', 'ares-support')

EXT_DIR = {'AnimTypeExt': 'AnimType', 'BuildingTypeExt': 'BuildingType', 'BulletTypeExt': 'BulletType',
           'HouseTypeExt': 'HouseType', 'RulesExt': 'Rules', 'SideExt': 'Side', 'SWTypeExt': 'SWType',
           'TechnoTypeExt': 'TechnoType', 'TiberiumExt': 'Tiberium', 'WarheadTypeExt': 'WarheadType',
           'WeaponTypeExt': 'WeaponType', '?': 'AttachEffect'}
SECTION = {'AnimType': 'art [Anim]', 'BuildingType': 'rules [BuildingType]', 'BulletType': 'rules [Projectile]',
           'HouseType': 'rules [Country]', 'Rules': 'rules [General]/[CombatDamage]/[AudioVisual]/...',
           'Side': 'rules [Side]', 'SWType': 'rules [SuperWeapon]', 'TechnoType': 'rules/art [TechnoType]',
           'Tiberium': 'rules [Tiberium]', 'WarheadType': 'rules [Warhead]', 'WeaponType': 'rules [Weapon]',
           'AttachEffect': 'rules [TechnoType]/[Warhead]', 'RadTypes': 'rules [RadiationType]'}


def tsv(p):
    with open(os.path.join(ares, p), newline='', encoding='utf-8', errors='replace') as f:
        return list(csv.DictReader(f, delimiter='\t'))


_lines = {}
def lines(p):
    if p not in _lines:
        try:
            _lines[p] = open(os.path.join(ares, p), encoding='utf-8', errors='replace').read().splitlines()
        except OSError:
            _lines[p] = []
    return _lines[p]


def body_paths(cls):
    if cls == 'AttachEffect':
        return 'src/Misc/AttachEffect.h', 'src/Misc/AttachEffect.cpp'
    return 'src/Ext/%s/Body.h' % cls, 'src/Ext/%s/Body.cpp' % cls


def decl_default(cls, member):
    h, _ = body_paths(cls)
    typ = dflt = ''
    for line in lines(h):
        s = line.strip()
        m = re.match(r'^([A-Za-z_][\w:<>,\s\*]*?)\s+' + re.escape(member) + r'\s*;', s)
        if m and not typ:
            typ = m.group(1).strip()
        m = re.match(r'^' + re.escape(member) + r'\s*\((.*)\)\s*,?\s*(//.*)?$', s)
        if m and not dflt:
            dflt = m.group(1).strip() or '(empty)'
    return typ, dflt


_all_src = None
def find_read(key, first):
    """file:line of the first line quoting the key: the class body first, then all of src/."""
    global _all_src
    needle = '"%s"' % key
    for i, l in enumerate(lines(first), 1):
        if needle in l:
            return '%s:%d' % (first, i)
    if _all_src is None:
        _all_src = []
        for root, _, files in os.walk(os.path.join(ares, 'src')):
            for fn in sorted(files):
                if fn.endswith(('.cpp', '.h')):
                    _all_src.append(os.path.relpath(os.path.join(root, fn), ares))
    for p in _all_src:
        for i, l in enumerate(lines(p), 1):
            if needle in l:
                return '%s:%d' % (p, i)
    return ''


def mechanism(cls, key):
    for mid, ext, pat in RULES:
        if ext and not re.search(ext, cls):
            continue
        if re.search(pat, key):
            return mid
    return '?'


# 1. ExtData keys
# A use counts as 3.0 source when its function was translated from 3.0 or checked against
# it (symbols/verified_30.tsv). report/ini_coverage.tsv predates the last verification
# rounds, so its check_0A_use label is recomputed here.
verified = {r['address'].upper() for r in tsv('symbols/verified_30.tsv')}
THREE = {'hook_3.0', 'translated', 'loader'}
uses = defaultdict(list)
use_ok = defaultdict(list)
for r in tsv('report/ini_uses.tsv'):
    if r['source'] == 'loader':
        continue
    ok = r['source'] in THREE or r['function'].upper() in verified
    uses[(r['ext'], r['ini_key'])].append('%s %s [%s%s%s]' % (
        r['function'], r['function_name'], r['source'], ',inferred' if r['via'] == 'fingerprint' else '',
        ',3.0✓' if ok and r['source'] not in THREE else ''))
    use_ok[(r['ext'], r['ini_key'])].append(ok and r['source'] != 'missing')

def use_status(k):
    oks = use_ok.get(k)
    if not oks:
        return 'no_use_found'
    return 'ok' if all(oks) else 'needs_check'

rows, seen, seen_pair = [], set(), set()
fields = tsv('symbols/extdata_fields.tsv')
if os.path.exists(os.path.join(ares, 'symbols/extdata_fields_sw.tsv')):
    fields += tsv('symbols/extdata_fields_sw.tsv')
for r in fields:
    if r['ext'] == 'ext' or (r['ext'], r['ini_key'].lower()) in seen_pair:
        continue
    seen_pair.add((r['ext'], r['ini_key'].lower()))
    cls = EXT_DIR.get(r['ext'], r['ext'])
    typ, dflt = decl_default(cls, r['member'])
    _, cpp = body_paths(cls)
    at = find_read(r['ini_key'], cpp)
    rows.append(dict(key=r['ini_key'], cls=cls, section=SECTION.get(cls, ''), origin='ExtData',
                     type=typ or r['reader'], default=dflt, loader=r['loader'], read_at=at,
                     since=r['status'], use_status=use_status((r['ext'], r['ini_key'])),
                     uses='; '.join(uses.get((r['ext'], r['ini_key']), []))))
    seen.add(r['ini_key'].lower())

# 2. keys read elsewhere in the recovered source (SW types, enums, UI, ...)
READ = re.compile(r'(Read\w*|Process\w*|GetKey\w*)\s*\(([^;]{0,200}?)"([A-Za-z][A-Za-z0-9_.%]*)"')
for root, _, files in os.walk(os.path.join(ares, 'src')):
    for fn in sorted(files):
        if not fn.endswith(('.cpp', '.h')):
            continue
        rel = os.path.relpath(os.path.join(root, fn), ares)
        text = '\n'.join(lines(rel))
        for m in READ.finditer(text):
            key = m.group(3)
            if key in IGNORE or key.lower() in seen:
                continue
            seen.add(key.lower())
            line = text.count('\n', 0, m.start()) + 1
            cls = rel.split('/')[2] if rel.startswith('src/Ext/') else os.path.splitext(fn)[0]
            if rel.startswith('src/Misc/SWTypes/'):
                cls = 'SWType'
            rows.append(dict(key=key, cls=cls, section=SECTION.get(cls, ''), origin='source-scan',
                             type='', default='', loader='', read_at='%s:%d' % (rel, line),
                             since='', use_status='unscanned', uses=''))

# 3. VERA20K leads: non-test .rs files holding the key as a string literal
lit = defaultdict(set)
pat = re.compile(r'"([A-Za-z0-9_.%]+)"')
for root, _, files in os.walk(os.path.join(vera, 'src')):
    for fn in files:
        if fn.endswith('.rs') and not fn.endswith('_tests.rs') and fn != 'tests.rs':
            p = os.path.join(root, fn)
            for s in pat.findall(open(p, encoding='utf-8', errors='replace').read()):
                lit[s.lower()].add(os.path.relpath(p, vera))

mech_info = {m[0]: m for m in MECHANISMS}
for r in rows:
    r['mechanism'] = mechanism(r['cls'], r['key'])
    r['milestone'] = mech_info.get(r['mechanism'], ('', '?'))[1]
    files = sorted(lit.get(r['key'].lower().replace('%s', ''), ()))
    r['vera_literal'] = ', '.join(files[:3]) + (' …' if len(files) > 3 else '')
    r['vera_status'] = ''  # filled by hand during M0 and each milestone: none/read/partial/done/residual

order = {m[0]: i for i, m in enumerate(MECHANISMS)}
rows.sort(key=lambda r: (order.get(r['mechanism'], 999), r['cls'], r['key'].lower()))
cols = ['mechanism', 'milestone', 'key', 'cls', 'section', 'origin', 'type', 'default', 'loader',
        'read_at', 'since', 'use_status', 'uses', 'vera_literal', 'vera_status']
os.makedirs(out, exist_ok=True)

# keep hand-filled vera_status across regenerations
old = {}
cat_path = os.path.join(out, 'catalog.tsv')
if os.path.exists(cat_path):
    with open(cat_path, newline='', encoding='utf-8') as f:
        old = {(r['cls'], r['key']): r.get('vera_status', '') for r in csv.DictReader(f, delimiter='\t')}
for r in rows:
    r['vera_status'] = old.get((r['cls'], r['key']), '')
with open(cat_path, 'w', newline='', encoding='utf-8') as f:
    w = csv.DictWriter(f, cols, delimiter='\t', lineterminator='\n')
    w.writeheader()
    w.writerows(rows)

# 4. hotspots: gamemd functions hooked from more than one mechanism
hook_file = {r['name']: r['file'] for r in tsv('report/hooks.tsv')}
def hook_mech(path):
    for frag, mid in HOOK_FILES:
        if frag in path:
            return mid
    if '/Recovered/' in path:
        return 'recovered:' + os.path.splitext(os.path.basename(path))[0]
    parts = path.split('/')
    return 'ext:' + (parts[2] if len(parts) > 3 and parts[1] == 'Ext' else os.path.splitext(parts[-1])[0])

fn_hooks = defaultdict(list)
for r in tsv('reference/gamemd_hooks/index.tsv'):
    f = hook_file.get(r['hook'])
    if f:
        fn_hooks[(r['gamemd_function_address'], r['gamemd_function'])].append((r['hook_address'], r['hook'], hook_mech(f)))
hot = []
for (addr, name), hs in fn_hooks.items():
    mechs = sorted({m for _, _, m in hs})
    if len(mechs) > 1 or len(hs) >= 5:
        hot.append(dict(gamemd_address=addr, gamemd_function=name, hooks=len(hs), mechanisms=len(mechs),
                        mechanism_list=', '.join(mechs),
                        hook_list=', '.join('%s %s' % (a, n) for a, n, _ in sorted(hs))))
hot.sort(key=lambda h: (-h['mechanisms'], -h['hooks'], h['gamemd_address']))
with open(os.path.join(out, 'hotspots.tsv'), 'w', newline='', encoding='utf-8') as f:
    w = csv.DictWriter(f, list(hot[0]), delimiter='\t', lineterminator='\n')
    w.writeheader()
    w.writerows(hot)

# 5. mechanism summary
per = defaultdict(list)
for r in rows:
    per[r['mechanism']].append(r)
hot_of = defaultdict(int)
for h in hot:
    for m in h['mechanism_list'].split(', '):
        hot_of[m] += 1
with open(os.path.join(out, 'mechanisms.tsv'), 'w', newline='', encoding='utf-8') as f:
    w = csv.writer(f, delimiter='\t', lineterminator='\n')
    w.writerow(['id', 'milestone', 'name', 'depends', 'vera_hint', 'vera_doc', 'keys', 'keys_vera_literal',
                'keys_unused_or_unscanned', 'hotspots'])
    for mid, ms, name, dep, hint, doc in MECHANISMS:
        ks = per.get(mid, [])
        w.writerow([mid, ms, name, dep, hint, doc, len(ks), sum(1 for k in ks if k['vera_literal']),
                    sum(1 for k in ks if k['use_status'] in ('no_use_found', 'unscanned')), hot_of.get(mid, 0)])

c = Counter(r['milestone'] for r in rows)
print('%d keys (%d ExtData, %d source scan); unmapped %d; milestones %s; hotspots %d' % (
    len(rows), sum(r['origin'] == 'ExtData' for r in rows), sum(r['origin'] != 'ExtData' for r in rows),
    sum(r['mechanism'] == '?' for r in rows), dict(sorted(c.items())), len(hot)))
for r in rows:
    if r['mechanism'] == '?':
        print('  unmapped:', r['cls'], r['key'])
