#!/usr/bin/env python3
"""041a's one-time, source-only migration (FLEET-002/005, OBJ-013).

Reads the committed vectors, never engine output. Owners and an appended Fleet are
inserted into submitted documents. Existing registration projections supply their
old allocation order; only Fleet insertion, ID relocation, defaults and records
are computed. Re-running produces the same bytes. Unrecognized shapes fail closed.
"""
import copy
import json
from pathlib import Path
import re
import subprocess

from vector_delta import structured

ROOT = Path(__file__).resolve().parents[1]
CORE = 'github.com/shrav-k/conduit/core'
API = 'github.com/shrav-k/conduit/v0'
OPERATIONS = ['register.v1', 'register-replacement.v1', 'inspect.v1', 'list.v1',
              'set-up.v1', 'retire.v1', 'reconcile.v1', 'sign-in.v1', 'answer.v1']


def baseline(path):
    return subprocess.check_output(['git', 'show', 'HEAD:' + str(path.relative_to(ROOT))], cwd=ROOT)


def fleet():
    return {'apiVersion': API, 'kind': 'Fleet', 'metadata': {'name': 's1'}, 'from': CORE + '#fleet'}


def agents(value):
    if isinstance(value, dict):
        if value.get('kind') == 'Agent' and 'from' in value:
            yield value
        for v in value.values():
            yield from agents(v)
    elif isinstance(value, list):
        for v in value:
            yield from agents(v)


def rename(value, mapping):
    if isinstance(value, str):
        if value.startswith('urn:uuid:') and value[9:] in mapping:
            return 'urn:uuid:' + mapping[value[9:]]
        return mapping.get(value, value)
    if isinstance(value, dict):
        return {k: rename(v, mapping) for k, v in value.items()}
    if isinstance(value, list):
        return [rename(v, mapping) for v in value]
    return value


def relocate_revisions(value, mapping):
    if isinstance(value, dict):
        return {k: (mapping.get(v, v) if k == 'revision' and isinstance(v, str)
                    else v if k == 'evidence' else relocate_revisions(v, mapping))
                for k, v in value.items()}
    if isinstance(value, list):
        return [relocate_revisions(v, mapping) for v in value]
    return value


def object_lists(value):
    if isinstance(value, dict):
        for k, v in value.items():
            if k == 'objects' and isinstance(v, list):
                yield v
            else:
                yield from object_lists(v)
    elif isinstance(value, list):
        for v in value:
            yield from object_lists(v)


def allocated_records(expect):
    return [r for r in expect.get('records', [])
            if r.get('type') == 'object-change' and r.get('data', {}).get('action') == 'registered']


def source_inventory(docs, evidence, stored):
    """Count the existing explicit/nested allocation shape from authored documents."""
    docs = sorted(docs, key=lambda d: (json.dumps(d.get('kind')), json.dumps(d.get('metadata', {}).get('name')), json.dumps(d, sort_keys=True)))
    families = ('Agent', 'Conduit', 'Fleet', 'Interface', 'Pool', 'Resource')
    templates = [d for d in docs if d.get('kind') in families and 'spec' in d]
    registry = dict(stored)
    registry.update({d['metadata']['name']: d for d in templates})
    result = [{'kind': d['kind'], 'name': d['metadata']['name'], 'spec': d['spec']} for d in templates]
    suffix = 0
    reserved = {(d.get('kind'), d.get('metadata', {}).get('name')) for d in docs}
    def expand(d):
        nonlocal suffix
        f = d['from']
        name = f if isinstance(f, str) else f.get('name', '')
        name = name.split('#')[-1]
        spec = registry.get(name, {}).get('spec', {})
        n = d.get('metadata', {}).get('name')
        if not n:
            while True:
                n = name[:57] + '-' + evidence['suffixes'][suffix]
                suffix += 1
                if (d['kind'], n) not in reserved:
                    reserved.add((d['kind'], n))
                    break
        result.append({'kind': d['kind'], 'name': n})
        for slot, family in [('conduits', 'Conduit'), ('entries', 'Resource')]:
            for _, child in sorted(spec.get(slot, {}).items()):
                expand(dict(child, kind=family))
    for d in docs:
        if 'from' in d:
            expand(d)
    for i, o in enumerate(result):
        o['id'] = evidence['ids'][i]
    return result, registry


def registration(case, index, add_fleet, inventory):
    """Insert known allocation shapes; retain the five boundary-accounting IDs."""
    step = case['steps'][index]
    e, inp = step['expect'], step['input']
    old = copy.deepcopy(inventory)
    n = len(old)
    old_ids = [o['id'] for o in old]
    records = [{'id': inp['evidence']['ids'][n+i], 'data': {'target': o['id']}} for i, o in enumerate(old)]
    if any('kind' not in o or 'name' not in o for o in old):
        raise ValueError('registration lacks a known object inventory')
    targets = [o for o in old if o['kind'] in ('Pool', 'Fleet') and 'spec' not in o]
    if not targets and not add_fleet:
        return {}, [], []
    n = len(old)
    # Templates precede all instances. Fleet sorts after Agent/Conduit and before Pool.
    insertion = next((i for i, o in enumerate(old)
                      if 'spec' not in o and o['kind'] in ('Pool', 'Resource')), n)
    ev = inp['evidence']
    start = int(old_ids[0].rsplit('-', 1)[1])
    prefix = old_ids[0].rsplit('-', 1)[0] + '-'
    boundary_ids = set(inp['evidence']['ids'][2*n:2*n+5])
    def uid(i):
        number = start
        while True:
            candidate = prefix + f'{number:012d}'
            number += 1
            if candidate not in boundary_ids:
                if i == 0:
                    return candidate
                i -= 1
    new_n = n + int(add_fleet)
    mapping = {v: uid(i + (add_fleet and i >= insertion)) for i, v in enumerate(old_ids)}
    mapping.update({r['id']: uid(new_n+i+(add_fleet and i >= insertion)) for i, r in enumerate(records)})
    while len(ev['revisions']) < new_n:
        ev['revisions'].append(f'r{len(ev["revisions"])+1}')
    revisions = {ev['revisions'][i]: ev['revisions'][i+1] for i in range(insertion, n)} if add_fleet else {}
    REVISION_MAPS.setdefault(case['id'], {}).update(revisions)
    # Apply allocation relocation to later inputs/expectations as well as this registration.
    for j in range(index, len(case['steps'])):
        case['steps'][j] = relocate_revisions(rename(case['steps'][j], mapping), revisions)
    step = case['steps'][index]
    e, inp = step['expect'], step['input']
    ev = inp['evidence']
    fleet_object = {'kind': 'Fleet', 'name': 's1', 'id': uid(insertion),
                    'from': '@core#fleet', 'state': 'registered'} if add_fleet else None
    new = rename(old, mapping)
    if add_fleet:
        new.insert(insertion, fleet_object)
    targets = [o for o in new if o['kind'] in ('Pool', 'Fleet') and 'spec' not in o]
    # Generated suffixes already consumed by authored unnamed instances stay consumed.
    used = [i for i, s in enumerate(ev['suffixes'])
            if any(o['name'].endswith('-' + s) for o in old)]
    suffix_index = max(used, default=-1) + 1
    needed = suffix_index + len(targets)
    while len(ev['suffixes']) < needed:
        ev['suffixes'].append(f'a{len(ev["suffixes"]):04d}')
    while len(ev['revisions']) < new_n + len(targets):
        ev['revisions'].append(f'r{len(ev["revisions"])+1}')
    defaults = []
    default_records = []
    invocation = ORIGINAL_CASES[case['id']]['steps'][index]['input']['evidence']['ids'][2*n]
    for i, target in enumerate(targets):
        template = 'root-pool' if target['kind'] == 'Pool' else 'root-fleet'
        ops = [op for op in OPERATIONS if target['kind'] == 'Pool' or op != 'register-replacement.v1']
        at = 2 * new_n + 3 * i
        obj = {'kind': 'Conduit', 'name': template + '-' + ev['suffixes'][suffix_index+i],
               'id': uid(at), 'from': '@core#' + template, 'state': 'ready',
               'consumer': '00000000-0000-4000-8000-000000000005', 'target': target['id'],
               'local_name': target['name'], 'open': True, 'provenance': {'cause': 'default'},
               'operations': ['@core#' + op for op in ops]}
        defaults.append(obj)
        for j, action in enumerate(('registered', 'lifecycle-changed')):
            default_records.append({'id': uid(at+j+1), 'type': 'object-change', 'specversion': '1.0', 'source': 'urn:conduit:first-build', 'time': ev['time'],
                                    'data': {'target': uid(at), 'cause': 'default', 'action': action,
                                             'state': 'registered' if j == 0 else 'ready', 'invocation': invocation}})
    # Replace only the consumed allocation prefix. The existing five request IDs keep
    # their meaning, and unused evidence remains available without duplicate IDs.
    consumed = 2 * new_n + 3 * len(targets)
    original_ev = ORIGINAL_CASES[case['id']]['steps'][index]['input']['evidence']
    boundary = original_ev['ids'][2*n:2*n+5]
    ids = [uid(i) for i in range(consumed)] + boundary
    ev['ids'] = ids + [i for i in original_ev['ids'][2*n+5:] if i not in ids]
    # Object arrays are exact; some authored projections identify objects only by name.
    state_objects = e.get('state', {}).get('objects')
    for objects in object_lists(e):
        def match(o, known):
            return isinstance(o, dict) and (o.get('id') == known['id'] or o.get('name') == known['name'])
        positions = [i for i, o in enumerate(objects) if any(match(o, known) for known in new)]
        if not positions:
            continue
        if add_fleet:
            pos = next((i for i, o in enumerate(objects) if any(match(o, v) for v in new[insertion+1:])), positions[-1]+1)
            objects.insert(pos, copy.deepcopy(fleet_object if objects is state_objects else
                                           {k: fleet_object[k] for k in ('kind', 'name', 'id')}))
        objects.extend(copy.deepcopy(defaults if objects is state_objects else
                                     [{k: o[k] for k in ('kind', 'name', 'id')} for o in defaults]))
        if objects is state_objects:
            for o in objects:
                if not isinstance(o, dict):
                    continue
                source = next((v for v in new if match(o, v)), {})
                if source.get('kind') == 'Agent' and 'spec' not in source:
                    owner = fleet_object or next((o for o in inventory if o.get('kind') == 'Fleet'), None)
                    if owner:
                        o['owner'] = {'kind': 'Fleet', 'id': owner['id']}
    rs = e.get('records')
    if isinstance(rs, list) and add_fleet:
        rs.insert(2+insertion, {'id': uid(new_n+insertion), 'type': 'object-change', 'specversion': '1.0', 'source': 'urn:conduit:first-build', 'time': ev['time'],
                              'data': {'target': fleet_object['id'], 'action': 'registered', 'invocation': invocation}})
    if isinstance(rs, list):
        rs[-2:-2] = default_records
    return mapping, ([fleet_object] if add_fleet else []) + defaults, new


ORIGINAL_CASES = {}
REVISION_MAPS = {}


def main():
    import yaml
    paths = subprocess.check_output(['git', 'ls-files', 'conformance/'], cwd=ROOT).decode().splitlines()
    cache, changed_docs, additions = {}, set(), set()
    def read(p):
        if p not in cache:
            cache[p] = baseline(p)
        return cache[p]
    def data(p):
        return structured(read(p))
    cases = [(ROOT / p, data(ROOT / p)) for p in paths if Path(p).name in ('case.yaml', 'case.yml', 'case.json')]
    ORIGINAL_CASES.update({c['id']: copy.deepcopy(c) for _, c in cases})
    projects = {}
    # Determine project files and whether each submission introduces its Fleet.
    for p, c in cases:
        has_fleet = False
        for i, s in enumerate(c['steps']):
            inp = s['input']
            if inp.get('kind') not in ('register-project', 'registration-orders', 'resolve-project'):
                continue
            fixture = data((p.parent / c['files'][inp['file']]).resolve())
            docs = []
            docpaths = []
            for name, alias in fixture['project']['files'].items():
                if name in ('conduit.yaml', 'conduit.yml', 'conduit.json'):
                    continue
                q = (p.parent / c['files'][alias]).resolve()
                content = read(q)
                try:
                    v = json.loads(content)
                    ds = v if isinstance(v, list) else [v]
                except ValueError:
                    ds = list(yaml.safe_load_all(content))
                docs.extend(ds); docpaths.append(q)
            ownerless = any('owner' not in a for a in agents(docs))
            add = ownerless and not has_fleet
            if add:
                additions.add(docpaths[-1])
            if ownerless:
                changed_docs.update(docpaths)
            projects[(c['id'], i)] = (add, docs)
            if inp['kind'] != 'resolve-project' and s['expect'].get('reply', {}).get('accepted') and add:
                has_fleet = True
    # Patch submitted files while preserving YAML layout/comments.
    for q in sorted(changed_docs):
        content = read(q)
        try:
            v = json.loads(content)
        except ValueError:
            text = content.decode()
            parts = re.split(r'(?m)^---\s*\n', text)
            for j, part in enumerate(parts):
                d = yaml.safe_load(part)
                if isinstance(d, dict) and d.get('kind') == 'Agent' and 'from' in d and 'owner' not in d:
                    parts[j] = re.sub(r'(?m)^(from:.*)$', r'\1\nowner: s1', part)
            text = '---\n'.join(parts)
            if q in additions:
                text = text.rstrip() + '\n---\n' + yaml.safe_dump(fleet(), sort_keys=False)
            output = text.encode()
        else:
            for a in agents(v):
                a.setdefault('owner', 's1')
            if q in additions:
                if not isinstance(v, list):
                    raise ValueError(f'cannot append Fleet to {q}')
                v.append(fleet())
            output = (json.dumps(v, indent=2, ensure_ascii=False) + '\n').encode()
        if output != content and q.read_bytes() != output:
            q.write_bytes(output)
    unsupported = []
    for p, c in cases:
        stored = {}
        extra, mappings, new_inventory = [], {}, []
        extra_positions = []
        for i, s in enumerate(c['steps']):
            if extra:
                if 'object_count' in s['expect']:
                    s['expect']['object_count'] += len(extra)
                os = s['expect'].get('state', {}).get('objects')
                if isinstance(os, list):
                    for pos, obj in extra_positions:
                        if not any(isinstance(o, dict) and o.get('id') == obj['id'] for o in os):
                            os.insert(min(pos, len(os)), copy.deepcopy(obj))
            key = c['id'], i
            if key in projects and projects[key][0] and s['input']['kind'] != 'resolve-project' and not s['expect'].get('reply', {}).get('accepted'):
                ev = s['input']['evidence']
                ev['revisions'].append(f'r{len(ev["revisions"])+1}')
            if key in projects and s['input']['kind'] != 'resolve-project' and s['expect'].get('reply', {}).get('accepted'):
                add, docs = projects[key]
                inventory, stored = source_inventory(docs, ORIGINAL_CASES[c['id']]['steps'][i]['input']['evidence'], stored)
                try:
                    mapping, made, objects = registration(c, i, add, inventory)
                except (ValueError, KeyError, StopIteration) as err:
                    unsupported.append(f'{c["id"]} step {i}: {err}')
                    continue
                mappings.update(mapping); extra.extend(made); new_inventory.extend(objects)
                if 'object_count' in c['steps'][i]['expect']:
                    c['steps'][i]['expect']['object_count'] += len(made)
                current = c['steps'][i]['expect'].get('state', {}).get('objects', [])
                for obj in made:
                    position = next((j for j, o in enumerate(current) if isinstance(o, dict) and o.get('id') == obj['id']), None)
                    if position is not None:
                        extra_positions.append((position, obj))
        # All indexed request observations use the relocated registered IDs, too.
        for alias, rel in c.get('files', {}).items():
            q = (p.parent / rel).resolve()
            if q.suffix not in ('.json', '.yaml') or q == p or '/fixtures/' in str(q):
                continue
            try:
                v = data(q)
            except Exception:
                continue
            if not isinstance(v, dict) or not ({'requests', 'observations'} & v.keys()):
                continue
            v = relocate_revisions(rename(v, mappings), REVISION_MAPS.get(c['id'], {}))
            for a in agents(v):
                a.setdefault('owner', 's1')
            output = (json.dumps(v, indent=2, ensure_ascii=False)+'\n').encode()
            if output != read(q) and q.read_bytes() != output:
                q.write_bytes(output)
        if c != ORIGINAL_CASES[c['id']]:
            output = (json.dumps(c, indent=2, ensure_ascii=False)+'\n').encode()
            if p.read_bytes() != output:
                p.write_bytes(output)
    for item in unsupported:
        print('UNHANDLED', item)
    print(f'Migrated {sum(c != ORIGINAL_CASES[c["id"]] for _, c in cases)} cases; {len(unsupported)} unhandled registrations')
    return bool(unsupported)


def compress_cases(selected, inventory):
    """Prefix globs covering exactly selected IDs in the current manifest."""
    def visit(prefix, candidates):
        wanted = candidates & selected
        if not wanted:
            return []
        if wanted == candidates:
            return [prefix + '*'] if len(candidates) > 1 else list(wanted)
        result = [prefix] if prefix in wanted else []
        for letter in sorted({c[len(prefix)] for c in candidates if len(c) > len(prefix)}):
            result.extend(visit(prefix + letter, {c for c in candidates if c.startswith(prefix + letter)}))
        return result
    return visit('', set(inventory))


def write_declaration():
    """Freeze exact source deltas, grouped by identical before/after values.

    No wildcard accepts an arbitrary expectation. Every rule fixes its operation,
    values, case sets and paths; even fixture bytes are pinned by both hashes.
    Flips and read errors cannot be declared by this migration.
    """
    from vector_delta import compare
    changes = compare(ROOT, 'HEAD', 'WORKTREE')
    groups = {}
    for change in changes:
        if change['kind'] in ('flip', 'error', 'removed', 'renamed'):
            raise ValueError('041a does not declare ' + change['kind'])
        shape = {k: change[k] for k in ('kind', 'operation', 'before', 'after') if k in change}
        key = json.dumps(shape, sort_keys=True)
        group = groups.setdefault(key, dict(shape, cases=set(), paths=set()))
        group['cases'].add(change['case'])
        group['paths'].add(re.sub(r'/[0-9]+(?=/|$)', '/*', change['path']).replace('[', '[[]'))
    inventory = {c['id'] for piece in sorted((ROOT / 'conformance').rglob('index.yaml'))
                 for c in structured(piece.read_bytes())['cases']}
    result = []
    ordered = sorted(groups.values(), key=lambda g: -len(g['cases']) * len(g['paths']))
    for i, group in enumerate(ordered):
        group['cases'] = compress_cases(group['cases'], inventory)
        kind = group['kind']
        path = min(group['paths'])
        label = ('new promise vector' if kind == 'added' else
                 'owners, Fleet or relocated observation fixture' if kind == 'fixture' else
                 'registration evidence' if '/input/evidence/' in path else
                 'object and default count' if path.endswith('/object_count') else
                 'Fleet insertion, owners, defaults and relocated registration references')
        result.append(dict(name=f'{label} {i+1:04d}', **{k: sorted(v) if isinstance(v, set) else v for k,v in group.items()}))
    path = ROOT / 'docs/work-orders/041a-declared.yaml'
    header = ("# 041a: FLEET-002/003, OBJ-013, FLEET-005, ACC-024 and ACC-057.\n"
              "# Generated from source changes only: python3 tools/migrate_041a.py --declare\n"
              "# Exact values and field paths pin Fleet/owner insertion, defaults, allocation and\n"
              "# revision relocation, object counts, evidence growth and the eight new vectors.\n"
              "# Each fixture rule fixes both byte counts and SHA-256 hashes. No flips allowed.\n")
    path.write_text(header + '\n'.join('- ' + json.dumps(g, ensure_ascii=True, separators=(',', ':')) for g in result) + '\n')
    print(f'Declared {len(changes)} source changes in {len(result)} exact shapes')


if __name__ == '__main__':
    import sys
    if sys.argv[1:] == ['--declare']:
        write_declaration()
    elif sys.argv[1:]:
        raise SystemExit('usage: migrate_041a.py [--declare]')
    else:
        raise SystemExit(main())
