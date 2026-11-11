"""The 041a migration preserves authored expectations and computes only known additions."""
import copy
import fnmatch
import sys
from pathlib import Path
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import migrate_041a as migration


def uid(n):
    return f'00000000-0000-4000-8000-{n:012d}'


class MigrationTests(unittest.TestCase):
    def test_source_order_and_nested_suffixes(self):
        docs = [
            {'kind': 'Agent', 'metadata': {'name': 'worker'}, 'spec': {
                'conduits': {'z': {'from': 'link'}, 'a': {'from': 'link'}}}},
            {'kind': 'Conduit', 'metadata': {'name': 'link'}, 'spec': {}},
            {'kind': 'Agent', 'metadata': {'name': 'a'}, 'from': 'worker'},
        ]
        evidence = {'ids': [uid(i) for i in range(10)], 'suffixes': ['a0000', 'a0001']}
        inventory, _ = migration.source_inventory(docs, evidence, {})
        self.assertEqual([o['name'] for o in inventory],
                         ['worker', 'link', 'a', 'link-a0000', 'link-a0001'])
        self.assertEqual([o['id'] for o in inventory], [uid(i) for i in range(5)])

    def test_fleet_and_defaults_preserve_request_accounting(self):
        case = {'id': 'test', 'steps': [{
            'input': {'evidence': {'time': '2026-11-11T12:00:00Z',
                      'ids': [uid(i) for i in range(100, 140)],
                      'revisions': ['t', 'a'], 'suffixes': []}},
            'expect': {'reply': {'accepted': True}, 'state': {'objects': [
                {'kind': 'Agent', 'name': 'worker', 'id': uid(100), 'spec': {}},
                {'kind': 'Agent', 'name': 'a', 'id': uid(101)}]},
                'records': [{'type': 'access-decision'}, {'type': 'invocation-dispatched'},
                            {'id': uid(102), 'type': 'object-change', 'data': {'target': uid(100), 'action': 'registered'}},
                            {'id': uid(103), 'type': 'object-change', 'data': {'target': uid(101), 'action': 'registered'}},
                            {'type': 'invocation-ended'}, {'type': 'request-ended'}]}}]}
        before = copy.deepcopy(case)
        migration.ORIGINAL_CASES['test'] = before
        inventory = before['steps'][0]['expect']['state']['objects']
        mapping, made, objects = migration.registration(case, 0, True, inventory)
        s = case['steps'][0]
        self.assertEqual([o['name'] for o in objects], ['worker', 'a', 's1'])
        self.assertEqual([o['name'] for o in made], ['s1', 'root-fleet-a0000'])
        self.assertEqual(s['input']['evidence']['ids'][9:14], before['steps'][0]['input']['evidence']['ids'][4:9])
        self.assertEqual(s['expect']['records'][-4]['data']['cause'], 'default')
        self.assertTrue(s['expect']['reply']['accepted'])
        self.assertEqual(s['expect']['state']['objects'][1]['owner'], {'kind': 'Fleet', 'id': uid(102)})
        self.assertEqual(mapping[uid(102)], uid(103))
        self.assertEqual(len(set(s['input']['evidence']['ids'])), len(s['input']['evidence']['ids']))
        self.assertEqual(before['steps'][0]['expect']['state']['objects'], inventory)

    def test_only_agent_instances_receive_owners(self):
        doc = [{'kind': 'Agent', 'spec': {}}, {'kind': 'Agent', 'from': 'worker'},
               {'kind': 'Resource', 'from': 'thing'}]
        self.assertEqual(list(migration.agents(doc)), [doc[1]])

    def test_authored_names_consume_collision_suffixes(self):
        docs = [
            {'kind': 'Resource', 'metadata': {'name': 'thing'}, 'spec': {}},
            {'kind': 'Resource', 'metadata': {'name': 'thing-a0000'}, 'from': 'thing'},
            {'kind': 'Resource', 'from': 'thing'},
        ]
        inventory, _ = migration.source_inventory(docs, {
            'ids': [uid(i) for i in range(3)], 'suffixes': ['a0000', 'a0001']}, {})
        self.assertEqual([o['name'] for o in inventory], ['thing', 'thing-a0000', 'thing-a0001'])
        # The reserved authored name makes the unnamed instance consume a0001.
        self.assertEqual(inventory[2]['name'], 'thing-a0001')

    def test_revisions_move_without_rewriting_supplied_evidence(self):
        value = {'expect': {'object_facts': [{'id': uid(1), 'revision': 'reg-16'}]},
                 'input': {'evidence': {'revisions': ['reg-16', 'reg-17']}}}
        after = migration.relocate_revisions(value, {'reg-16': 'reg-17'})
        self.assertEqual(after['expect']['object_facts'][0]['revision'], 'reg-17')
        self.assertEqual(after['input']['evidence'], value['input']['evidence'])

    def test_declaration_globs_cover_only_the_selected_existing_cases(self):
        inventory = {'s1', 's1-forward', 's1-reverse', 's2', 'other'}
        selected = {'s1', 's1-reverse', 'other'}
        patterns = migration.compress_cases(selected, inventory)
        covered = {c for c in inventory if any(fnmatch.fnmatchcase(c, p) for p in patterns)}
        self.assertEqual(covered, selected)

    def test_id_relocation_is_simultaneous_and_exact(self):
        self.assertEqual(migration.rename([uid(1), uid(2), 'prefix ' + uid(1), 'urn:uuid:' + uid(1)],
                         {uid(1): uid(2), uid(2): uid(3)}),
                         [uid(2), uid(3), 'prefix ' + uid(1), 'urn:uuid:' + uid(2)])


if __name__ == '__main__':
    unittest.main()
