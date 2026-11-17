import contextlib
import errno
import importlib.util
import io
import json
import os
from pathlib import Path
import runpy
import select
import signal
import subprocess
import sys
import tempfile
import threading
import time
import unittest
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import gate

REPOSITORY = Path(__file__).resolve().parents[2]


class FakeRunner:
    def __init__(self):
        self.calls = []
        self.responses = {}
        self.version = 'v8.24.2\n'
        self.report = []
        self.scan_exit = 0
        self.inspect = None

    def __call__(self, args, *, cwd, env, timeout, log, redact):
        self.calls.append(dict(args=list(args), cwd=cwd, env=dict(env), timeout=timeout, log=log))
        if self.inspect:
            self.inspect(self.calls[-1])
        if args[-1] == 'version':
            result = self.responses.get('version', gate.CommandResult(0, self.version))
        elif 'git' in args and '--report-path' in args:
            if self.report is not None:
                Path(args[args.index('--report-path') + 1]).write_text(json.dumps(self.report))
            result = gate.CommandResult(self.scan_exit, 'planted-scanner-output\n')
        else:
            key = args[1] if len(args) > 1 else args[0]
            result = self.responses.get(key, gate.CommandResult(0, 'checked\n'))
        output = redact(result.output)
        Path(log).write_text(output)
        return gate.CommandResult(result.exit, output, result.timed_out)


class GateTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.parent = Path(temporary.name)
        self.root = self.parent / 'repository'
        self.root.mkdir()
        self.environment = mock.patch.dict(os.environ, {'PATH': os.defpath}, clear=True)
        self.environment.start()
        self.addCleanup(self.environment.stop)
        self.git('init', '-q')
        self.write('.gitignore', 'target/\n')
        self.write(str(gate.WORKFLOW), 'jobs:\n  secrets:\n    scanner_version=8.24.2\n')
        self.write('seed.txt', 'base\n')
        self.base = self.commit()
        self.git('branch', 'base')
        self.git('checkout', '-qb', 'work')
        self.fake = FakeRunner()

    def git(self, *args, env=None):
        return subprocess.run(['git', *args], cwd=self.root, env=env, check=True,
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE).stdout.decode().strip()

    def write(self, name, value):
        target = self.root / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(value, encoding='utf-8')
        return target

    def commit(self, author=gate.IDENTITY, committer=gate.IDENTITY):
        self.git('add', '.')
        environment = dict(os.environ, GIT_AUTHOR_NAME=author[0], GIT_AUTHOR_EMAIL=author[1],
                           GIT_COMMITTER_NAME=committer[0], GIT_COMMITTER_EMAIL=committer[1])
        self.git('-c', 'user.name=' + author[0], '-c', 'user.email=' + author[1],
                 '-c', 'commit.gpgsign=false', 'commit', '-qm', 'test snapshot', env=environment)
        return self.git('rev-parse', 'HEAD')

    def add_commit(self):
        self.write('branch.txt', 'branch\n')
        return self.commit()

    def names_file(self, value='planted-alpha, planted-beta\nplanted-gamma\n', mode=0o600,
                   path=None):
        path = path or self.parent / 'names'
        path.write_text(value, encoding='utf-8')
        path.chmod(mode)
        return path

    def invoke(self, *args, installed=None):
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err), \
                mock.patch.object(gate.shutil, 'which', return_value=installed):
            code = gate.main(self.root, ['--base', 'base', *args], run=self.fake)
        return code, out.getvalue(), err.getvalue()

    def record(self):
        return json.loads(self.record_path().read_text())

    def record_path(self):
        return self.root / gate.OUTPUT / (self.git('rev-parse', 'HEAD') + '.json')

    def row(self, name):
        return next(row for row in self.record()['gates'] if row['name'] == name)

    def assert_private(self, text, *values):
        for value in values:
            self.assertNotIn(str(value), text)

    def logs(self):
        return ''.join(path.read_text() for path in (self.root / gate.OUTPUT).rglob('*.log'))

    def test_changed_paths_cover_commits_edits_renames_and_untracked(self):
        self.write('a/x.txt', 'x\n')
        self.write('b/y.txt', 'y\n')
        self.commit()
        self.git('branch', '-f', 'base', 'HEAD')
        (self.root / 'c').mkdir()
        self.git('mv', 'a/x.txt', 'c/x.txt')
        self.commit()
        self.git('checkout', '-q', 'base')
        self.write('d/base-only.txt', 'base only\n')
        self.commit()
        self.git('checkout', '-q', 'work')
        self.write('b/y.txt', 'edited\n')
        self.write('e/new.txt', 'untracked\n')
        self.write('target/out.txt', 'ignored\n')
        env, _, _ = gate.scrub_environment()
        common = self.git('merge-base', 'base', 'HEAD')
        self.assertEqual(gate.changed_paths(self.root, common, env),
                         [b'a/x.txt', b'b/y.txt', b'c/x.txt', b'e/new.txt'])
        code, output, _ = self.invoke()
        self.assertEqual(code, 0)
        record = self.record()
        self.assertEqual(record['changed_paths'], 4)
        self.assertEqual(record['tree'], {'clean': False, 'uncommitted_paths': 2})
        self.assertFalse(record['contains_base'])
        self.assertEqual(record['behind'], 1)
        self.assertIn('Base: the branch lacks 1 commits of base.', output)
        self.assertIn('The branch lacks 1 commits of base: bring it up to date', output.splitlines()[-1])

    def test_identity_names_each_commit_not_by_the_work_order_identity(self):
        planted = ('Planted Person', 'planted@example.invalid')
        self.write('.mailmap', 'Conduit work order <work-orders@conduit.invalid> '
                   'Planted Person <planted@example.invalid>\n')
        good = self.commit()
        self.write('second.txt', 'second\n')
        second = self.commit(committer=planted)
        self.write('third.txt', 'third\n')
        third = self.commit(author=planted)
        code, output, error = self.invoke()
        line = next(line for line in output.splitlines() if line.startswith('FAIL identity'))
        self.assertEqual(code, 1)
        self.assertIn('2 of 3 commits', line)
        self.assertIn(second[:12] + ' (committer)', line)
        self.assertIn(third[:12] + ' (author)', line)
        self.assertLess(line.index(third[:12]), line.index(second[:12]))
        self.assertNotIn(good[:12], line)
        self.assertEqual(self.record()['result'], 'FAIL')
        self.assertIsNone(self.row('identity')['log'])
        self.assert_private(output + error + self.record_path().read_text(), *planted)
        self.git('reset', '--hard', good)
        self.write('second.txt', 'second\n')
        self.commit()
        self.write('third.txt', 'third\n')
        self.commit()
        self.assertEqual(self.invoke()[0], 0)
        self.assertEqual(self.row('identity')['result'], 'PASS')
        self.assertEqual(self.row('identity')['reason'],
                         'the branch adds 3 commits, each to be by Conduit work order '
                         '<work-orders@conduit.invalid>')
        self.git('reset', '--hard', self.base)
        code, output, _ = self.invoke()
        self.assertEqual(code, 0)
        self.assertIn(gate.NO_COMMITS, output)
        self.assertFalse(self.row('identity')['required'])
        self.assertIn('not run: identity, secrets, rust and engine.', output.splitlines()[-1])

        # Valid Git identities may hold Unicode separators that are not Git row delimiters.
        for separator in ('\u2028', '\u2029', '\x85'):
            with self.subTest(separator=repr(separator)):
                self.git('reset', '--hard', self.base)
                self.write('unicode.txt', 'identity\n')
                unusual = ('Planted' + separator + 'Person', 'unicode@example.invalid')
                commit = self.commit(author=unusual, committer=unusual)
                code, output, error = self.invoke()
                self.assertEqual(code, 1)
                row = self.row('identity')
                self.assertIn('the branch adds 1 commits', row['reason'])
                self.assertIn('1 of 1 commits', row['why'])
                self.assertIn(commit[:12] + ' (author and committer)', row['why'])
                self.assert_private(output + error + self.record_path().read_text(),
                                    *unusual, 'Planted', 'Person')

        direct_git = gate.git

        def malformed_log(root, env, *args):
            if args[0] == 'log':
                return subprocess.CompletedProcess(args, 0, b'planted-fragment\0name\0\0\0\0', b'')
            return direct_git(root, env, *args)

        with mock.patch.object(gate, 'git', side_effect=malformed_log):
            code, output, error = self.invoke()
        self.assertEqual(code, 1)
        self.assertEqual(self.row('identity')['why'],
                         'git log returned a row that is not a commit hash and four identity fields; '
                         'identities are not printed')
        self.assert_private(output + error + self.record_path().read_text(), 'planted-fragment')

        # A tab in a name moves no field: only the author is named as wrong.
        self.git('reset', '--hard', self.base)
        self.write('tab.txt', 'tab\n')
        tabbed = self.commit(author=('Planted\tPerson', 'tab@example.invalid'))
        code, output, error = self.invoke()
        self.assertEqual(code, 1)
        self.assertIn('1 of 1 commits', self.row('identity')['why'])
        self.assertIn(tabbed[:12] + ' (author);', self.row('identity')['why'])
        self.assert_private(output + error + self.record_path().read_text(), 'Planted', 'tab@example.invalid')

        # A replacement under refs/replace does not hide the identity that git push sends.
        self.git('reset', '--hard', self.base)
        self.write('replaced.txt', 'replaced\n')
        original = self.commit(author=planted, committer=planted)
        work_order = dict(os.environ, GIT_AUTHOR_NAME=gate.IDENTITY[0], GIT_AUTHOR_EMAIL=gate.IDENTITY[1],
                          GIT_COMMITTER_NAME=gate.IDENTITY[0], GIT_COMMITTER_EMAIL=gate.IDENTITY[1])
        substitute = self.git('-c', 'commit.gpgsign=false', 'commit-tree', '--no-gpg-sign',
                              original + '^{tree}', '-p', self.base, '-m', 'substitute', env=work_order)
        self.git('replace', original, substitute)
        code, output, error = self.invoke()
        self.assertEqual(code, 1)
        self.assertTrue(self.row('identity')['command'].startswith('git --no-replace-objects log -z '))
        self.assertIn('1 of 1 commits', self.row('identity')['why'])
        self.assertIn(original[:12] + ' (author and committer)', self.row('identity')['why'])
        self.assert_private(output + error + self.record_path().read_text(), *planted)

    def test_private_names_reach_the_private_paths_gate_alone(self):
        names = self.names_file()
        values = {key: 'planted-environment-' + str(index) for index, key in enumerate(
            ('CONDUIT_PRIVATE_NAMES', 'CONDUIT_TEST_NPM', 'SCR_AGENT_CODEX',
             'RUSTUP_TOOLCHAIN', 'RUSTFLAGS', 'EXAMPLE_API_KEY'))}
        with mock.patch.dict(os.environ, values):
            code, output, error = self.invoke('--private-names-file', str(names), installed='gitleaks')
        self.assertEqual(code, 0)
        private_calls = 0
        for call in self.fake.calls:
            if 'tools/check_private_paths.py' in call['args']:
                private_calls += 1
                self.assertEqual(call['env']['CONDUIT_PRIVATE_NAMES'],
                                 'planted-alpha,planted-beta,planted-gamma')
            else:
                self.assertNotIn('CONDUIT_PRIVATE_NAMES', call['env'])
            for key in values.keys() - {'CONDUIT_PRIVATE_NAMES'}:
                self.assertNotIn(key, call['env'])
            self.assertNotIn(values['CONDUIT_PRIVATE_NAMES'], call['env'].values())
        self.assertEqual(private_calls, 1)
        self.assertIn('Private names: read from the file that --private-names-file names, and given to '
                      'tools/check_private_paths.py alone; neither they nor the file\'s path are printed or recorded.', output)
        self.assertIn('Environment: removed from every command: CONDUIT_PRIVATE_NAMES, CONDUIT_TEST_NPM, '
                      'RUSTFLAGS, RUSTUP_TOOLCHAIN, SCR_AGENT_CODEX, and 1 variables named for a credential.\n',
                      output)
        env = self.record()['environment']
        self.assertTrue(env['private_names_configured'])
        self.assertEqual(env['removed'], ['CONDUIT_PRIVATE_NAMES', 'CONDUIT_TEST_NPM', 'RUSTFLAGS',
                                         'RUSTUP_TOOLCHAIN', 'SCR_AGENT_CODEX'])
        self.assertEqual(env['removed_credentials'], 1)
        self.assert_private(output + error + self.logs() + self.record_path().read_text(),
                            *values.values(), 'planted-alpha', 'planted-beta', 'planted-gamma', names)

        # Diagnostics from any checker, including Git, must be redacted before logging.
        self.fake.responses['diff'] = gate.CommandResult(
            2, 'warning: planted-beta\n+planted-alpha \nplanted-gamma\n')
        code, output, error = self.invoke('--private-names-file', str(names))
        self.assertEqual(code, 1)
        self.assertEqual(self.row('whitespace')['result'], 'FAIL')
        self.assertIn('its last line: [redacted]', output)
        self.assertIn('    +[redacted] ', error)
        self.assertIn('+[redacted] ', self.logs())
        self.assert_private(output + error + self.logs() + self.record_path().read_text(),
                            'planted-alpha', 'planted-beta', 'planted-gamma', names)
        self.fake.responses.clear()
        self.fake.calls.clear()
        with mock.patch.dict(os.environ, values):
            _, output, _ = self.invoke()
        self.assertIn('CONDUIT_PRIVATE_NAMES in the environment is not used.', output)
        self.assertTrue(all('CONDUIT_PRIVATE_NAMES' not in call['env'] for call in self.fake.calls))
        self.assertFalse(self.record()['environment']['private_names_configured'])

        # A configured name in the branch or the base is printed and recorded redacted.
        self.git('checkout', '-qb', 'planted-beta-base')
        self.write('beta.txt', 'beta\n')
        self.commit()
        self.git('checkout', '-q', 'work')
        self.git('checkout', '-qb', 'wo/planted-alpha')
        code, output, error = self.invoke('--private-names-file', str(names), '--base', 'planted-beta-base')
        self.assertEqual(code, 0)
        lines = output.splitlines()
        self.assertTrue(lines[0].startswith('Gate: base [redacted]-base at '))
        self.assertIn(' on wo/[redacted]; nothing is fetched.', lines[0])
        self.assertIn('Base: the branch lacks 1 commits of [redacted]-base.', output)
        self.assertIn('The branch lacks 1 commits of [redacted]-base: bring it up to date', lines[-1])
        record = self.record()
        self.assertEqual(record['branch'], 'wo/[redacted]')
        self.assertEqual(record['base']['ref'], '[redacted]-base')
        self.assert_private(output + error + self.logs() + self.record_path().read_text(),
                            'planted-alpha', 'planted-beta', 'planted-gamma', names)

    def test_a_run_that_cannot_start_checks_nothing(self):
        code, output, _ = self.invoke('--base', 'no-such-ref')
        self.assertEqual(code, 2)
        self.assertEqual(output, 'Gate: NOT RUN: the base no-such-ref does not resolve to a commit; '
                         'fetch it or pass --base with a commit. Nothing was checked.\n')
        names = self.names_file('planted-alpha', mode=0o644)
        link = self.parent / 'link'
        target = self.names_file('planted-alpha', path=self.parent / 'regular')
        link.symlink_to(target)
        empty = self.names_file(', ,\n,\n', path=self.parent / 'empty')
        invalid = self.parent / 'invalid'
        invalid.write_bytes(b'\xff')
        invalid.chmod(0o600)
        inside = self.names_file('planted-alpha', path=self.root / 'inside')
        cases = [(names, 'has mode 0644, not 0600; run chmod 600 on it'),
                 (link, 'is not a regular file; a symbolic link, a directory and any other kind are refused'),
                 (self.parent, 'is not a regular file; a symbolic link, a directory and any other kind are refused'),
                 (self.parent / 'absent', 'does not exist or cannot be read'),
                 (inside, 'is inside the repository, where Git could commit it; keep it outside'),
                 (empty, 'holds no name'), (invalid, 'is not UTF-8 text')]
        for filename, fault in cases:
            with self.subTest(fault=fault):
                code, output, error = self.invoke('--private-names-file', str(filename))
                self.assertEqual(code, 2)
                self.assertEqual(output, 'Gate: NOT RUN: ' + gate.PRIVATE_FILE + ' ' + fault
                                 + '. Nothing was checked.\n')
                self.assert_private(output + error, filename, 'planted-alpha')
                self.assertFalse(self.fake.calls)
                self.assertFalse((self.root / gate.OUTPUT).exists())
        with self.assertRaises(SystemExit) as stopped:
            self.invoke('--unknown-option')
        self.assertEqual(stopped.exception.code, 2)
        # A directory without Git and an unrelated history also stop before any gate.
        with contextlib.redirect_stdout(io.StringIO()) as output:
            code = gate.main(self.parent, [], run=self.fake)
        self.assertEqual(code, 2)
        self.assertIn("the gate's root is not a Git working tree", output.getvalue())
        self.git('checkout', '--orphan', 'unrelated')
        self.write('orphan.txt', 'orphan\n')
        self.commit()
        code, output, _ = self.invoke()
        self.assertEqual(code, 2)
        self.assertIn('the base base shares no commit with HEAD.', output)
        self.assertFalse(self.fake.calls)

    def test_python_gates_follow_the_docs_job(self):
        self.invoke()
        expected = [
            ('docs', 'python3 tools/check_docs.py'),
            ('names', 'python3 tools/set_names.py --check'),
            ('schemas', 'python3 tools/check_schemas.py'),
            ('roadmap', 'python3 tools/check_roadmap.py'),
            ('rulings', 'python3 tools/check_rulings.py'),
            ('model account', 'python3 tools/check_model_account.py --history'),
            ('promise standing', 'python3 tools/promise_standing.py --check'),
            ('private paths', 'python3 tools/check_private_paths.py'),
            ('tool tests', 'python3 -m unittest discover -s tools/tests'),
            ('whitespace', 'git diff --check ' + self.base),
            ('vector changes', 'python3 tools/vector_delta.py ' + self.base + ' WORKTREE --declared-changed')]
        actual = [(row['name'], row['command']) for row in self.record()['gates'][1:12]]
        self.assertEqual(actual, expected)
        for call in self.fake.calls:
            self.assertEqual(call['cwd'], self.root.resolve())
            self.assertEqual(call['timeout'], 900)
            if call['args'][0] != 'git':
                self.assertEqual(call['args'][0], sys.executable)
        script = Path(gate.__file__).resolve()
        original = importlib.util.find_spec
        restarted_with = []

        def without_yaml(name, *args):
            return None if name == 'yaml' else original(name, *args)

        def restart_side_effect(*args):
            restarted_with.append(os.environ.get('CONDUIT_GATE_REEXEC'))
            raise RuntimeError('restart')

        with mock.patch.object(importlib.util, 'find_spec', side_effect=without_yaml), \
                mock.patch.object(sys, 'executable', str(self.parent / 'other-python3')), \
                mock.patch.object(sys, 'argv', [str(script), '--base', 'no-such-ref']), \
                mock.patch.object(os.path, 'exists', return_value=True), \
                mock.patch.dict(os.environ, {'CONDUIT_GATE_REEXEC': '0'}), \
                mock.patch.object(os, 'execv', side_effect=restart_side_effect) as restart, \
                contextlib.redirect_stdout(io.StringIO()):
            with self.assertRaisesRegex(RuntimeError, 'restart'):
                runpy.run_path(str(script), run_name='__main__')
            restart.assert_called_once_with('/usr/bin/python3',
                                           ['/usr/bin/python3', str(script), '--base', 'no-such-ref'])
            self.assertEqual(restarted_with, ['1'])
            with mock.patch.dict(os.environ, {'CONDUIT_GATE_REEXEC': '1'}), \
                    mock.patch.object(gate, 'run_command', side_effect=AssertionError('no command')):
                with self.assertRaises(SystemExit) as stopped:
                    runpy.run_path(str(script), run_name='__main__')
            self.assertEqual(stopped.exception.code, 2)
            self.assertEqual(restart.call_count, 1)
        try:
            import yaml
        except ImportError:
            self.skipTest('PyYAML unavailable for the comparison with CI')
        workflow = yaml.safe_load((REPOSITORY / gate.WORKFLOW).read_text())
        steps = workflow['jobs']['docs']['steps']
        start = next(i for i, step in enumerate(steps) if step.get('run', '').startswith('python3 tools/'))
        ci = [step['run'] for step in steps[start:] if 'run' in step]
        ci = [command.replace('HEAD^1 HEAD', '<comparison>') for command in ci]
        local = [command.replace(self.base + ' WORKTREE', '<comparison>').replace(self.base, '<comparison>')
                 for _, command in actual]
        self.assertEqual(ci, local)

    def test_result_is_pass_only_when_every_required_gate_passed(self):
        self.add_commit()
        code, output, _ = self.invoke()
        self.assertEqual(code, 0)
        self.assertEqual(self.row('identity')['result'], 'PASS')
        self.assertIn('gitleaks is not installed; the pin is 8.24.2', output)
        self.assertIn('NOT RUN rust and engine: ' + gate.RUST_WHY, output)
        self.assertIn('Gate: PASS. 12 passed; not run: secrets, rust and engine.', output)
        self.fake.responses['tools/check_schemas.py'] = gate.CommandResult(1, 'detail\nlast failure line\n')
        self.fake.calls.clear()
        code, output, error = self.invoke()
        self.assertEqual(code, 1)
        self.assertIn('Gate: FAIL. Failed: schemas;', output)
        line = next(line for line in output.splitlines() if line.startswith('FAIL schemas'))
        self.assertIn('exit 1; its last line: last failure line; log target/gate/logs/', line)
        self.assertIn('    last failure line', error)
        self.assertEqual(len(self.fake.calls), 11)
        self.assertIn('tools/vector_delta.py', self.fake.calls[-1]['args'])
        self.fake.responses['tools/check_schemas.py'] = gate.CommandResult(None, 'partial\n', True)
        code, output, _ = self.invoke()
        self.assertEqual(code, 1)
        self.assertIn('ended after 15 minutes, with its process group', output)
        self.fake.responses.clear()
        self.write('untracked.txt', 'untracked\n')
        self.fake.responses['tools/check_docs.py'] = gate.CommandResult(0,
            'Notice: skipped repository permalinks at first\nNotice: skipped repository permalinks at second\n')
        self.fake.responses['-m'] = gate.CommandResult(0, 'Ran tests\n\nOK (skipped=3)\n')
        code, output, _ = self.invoke()
        self.assertEqual(code, 0)
        for name, partial in [('whitespace', '1 untracked path(s)'), ('private paths', '1 untracked path(s)'),
                              ('docs', 'the permalinks at 2 commit(s) this clone lacks, which CI fetches'),
                              ('tool tests', '3 test(s) unittest skipped')]:
            line = next(line for line in output.splitlines() if line.startswith('PASS ' + name + ' ('))
            self.assertTrue(line.endswith('; not checked: ' + partial))
            self.assertEqual(self.row(name)['partial'], partial)
        # Interruptions preserve the incomplete record, and do not claim a final result.
        def interrupt(*args, **kwargs):
            raise KeyboardInterrupt
        self.fake = interrupt
        code, output, _ = self.invoke()
        self.assertEqual(code, 130)
        self.assertIn('Gate: interrupted during docs; its process group was ended', output)
        self.assertEqual(self.record()['result'], 'incomplete')
        self.assertIsNone(self.record()['finished'])

    def test_secrets_gate_runs_only_the_pinned_gitleaks_and_prints_no_value(self):
        _, output, _ = self.invoke()
        self.assertIn('gitleaks is not installed; the pin is 8.24.2', output)
        self.fake.version = 'v8.0.0\n'
        _, output, _ = self.invoke(installed='gitleaks')
        self.assertIn('gitleaks 8.0.0 is installed, not the pinned 8.24.2', output)
        self.fake.responses['version'] = gate.CommandResult(2, 'unavailable')
        _, output, _ = self.invoke(installed='gitleaks')
        self.assertIn('gitleaks version did not answer (exit 2); the pin is 8.24.2', output)
        self.fake.responses['version'] = gate.CommandResult(None, '', True)
        _, output, _ = self.invoke(installed='gitleaks')
        self.assertIn('gitleaks version did not answer (ended after 10 minutes, with its process group)', output)
        self.fake.responses.clear()
        self.fake.version = '8.24.2\n'
        _, output, _ = self.invoke(installed='gitleaks')
        self.assertEqual(self.row('secrets')['why'], gate.NO_COMMITS)
        head = self.add_commit()
        self.fake.version = 'v8.24.2\n'
        self.fake.scan_exit = 1
        self.fake.report = [dict(RuleID='planted-rule', File='planted-alpha/x.txt', StartLine=1,
                                 Commit=head, Secret='planted-secret-value', Match='planted-secret-value')]
        names = self.names_file()
        code, output, error = self.invoke('--private-names-file', str(names), installed='gitleaks')
        self.assertEqual(code, 1)
        self.assertIn('1 findings; values are not printed', output)
        self.assertIn('planted-rule [redacted]/x.txt:1 in ' + head[:12], error)
        self.assert_private(output + error + self.logs() + self.record_path().read_text(),
                            'planted-secret-value', 'planted-scanner-output', 'planted-alpha')
        secrets = self.row('secrets')
        self.assertIn('<temporary>/findings.json', secrets['command'])
        self.assertIn(self.base + '..HEAD', secrets['command'])
        self.assertIsNone(secrets['log'])
        scan = next(call for call in reversed(self.fake.calls) if '--report-path' in call['args'])
        expected = ['gitleaks', 'git', '.', '--config', '.gitleaks.toml', '--redact', '--no-banner',
                    '--log-opts=--diff-merges=first-parent ' + self.base + '..HEAD',
                    '--report-format', 'json', '--report-path', str(Path(scan['log']).parent / 'findings.json')]
        self.assertEqual(scan['args'], expected)
        self.assertEqual(scan['timeout'], 600)
        self.assertFalse(Path(scan['log']).parent.exists())
        self.fake.report = None
        code, output, _ = self.invoke(installed='gitleaks')
        self.assertEqual(code, 1)
        self.assertIn('gitleaks exited 1 without reporting a finding: the scan did not complete', output)
        self.fake.report = []
        self.fake.scan_exit = 0
        self.write('untracked.txt', 'untracked\n')
        code, output, _ = self.invoke(installed='gitleaks')
        self.assertEqual(code, 0)
        self.assertEqual(self.row('secrets')['result'], 'PASS')
        self.assertIsNone(self.row('secrets')['log'])
        line = next(line for line in output.splitlines() if line.startswith('PASS secrets'))
        self.assertTrue(line.endswith('; not checked: 1 uncommitted path(s), since gitleaks reads commits alone'))
        self.write(str(gate.WORKFLOW), 'no pin\n')
        _, output, _ = self.invoke(installed='gitleaks')
        self.assertIn('the pinned version could not be read from .github/workflows/checks.yml', output)

    def test_record_says_what_ran_and_holds_no_private_detail(self):
        names = self.names_file()
        observed = []

        def inspect(call):
            if len(self.fake.calls) == 2:
                observed.append(self.record())

        self.fake.inspect = inspect
        with mock.patch.dict(os.environ, {'PATH': os.defpath, 'CONDUIT_PRIVATE_NAMES': 'planted-environment',
                                          'EXAMPLE_TOKEN': 'planted-token'}, clear=True):
            code, output, _ = self.invoke('--private-names-file', str(names))
        self.assertEqual(code, 0)
        self.assertEqual(observed[0]['result'], 'incomplete')
        self.assertIsNone(observed[0]['finished'])
        self.assertEqual([row['name'] for row in observed[0]['gates']], ['identity', 'docs'])
        record = self.record()
        self.assertEqual(set(record), {'format', 'tool', 'started', 'finished', 'result', 'base',
                         'merge_base', 'head', 'branch', 'contains_base', 'behind', 'tree', 'changed_paths',
                         'platform', 'python', 'python_packages', 'environment', 'gates'})
        self.assertEqual(record['format'], 1)
        self.assertEqual(record['tool'], 'tools/gate.py')
        self.assertEqual(record['result'], 'PASS')
        self.assertIsNotNone(record['finished'])
        self.assertEqual(record['base'], dict(ref='base', commit=self.base))
        self.assertEqual(record['tree'], dict(clean=True, uncommitted_paths=0))
        self.assertTrue(record['environment']['private_names_configured'])
        self.assertEqual(record['environment']['removed_credentials'], 1)
        self.assertEqual(record['environment']['set'], {})
        for row in record['gates']:
            self.assertEqual(set(row), {'name', 'directory', 'command', 'required', 'result',
                                       'reason', 'why', 'partial', 'exit', 'seconds', 'log'})
            self.assertIn(row['result'] + ' ' + row['name'], output)
            if row['command']:
                self.assertIn(row['command'], output)
                self.assertEqual(row['directory'], '.')
            else:
                self.assertIsNone(row['directory'])
        self.assert_private(self.record_path().read_text(), 'planted-environment', 'planted-token',
                            'planted-alpha', 'planted-beta', 'planted-gamma', names, self.root)

    def test_plan_lists_the_gates_and_runs_none(self):
        self.add_commit()
        code, output, _ = self.invoke('--plan', installed='gitleaks')
        self.assertEqual(code, 0)
        self.assertEqual([call['args'] for call in self.fake.calls], [['gitleaks', 'version']])
        would = [line for line in output.splitlines() if line.startswith('WOULD RUN ')]
        self.assertEqual(len(would), 13)
        self.assertTrue(would[0].startswith('WOULD RUN identity:'))
        self.assertTrue(would[-1].startswith('WOULD RUN secrets:'))
        self.assertIn('NOT RUN rust and engine: ' + gate.RUST_WHY, output)
        self.assertEqual(output.splitlines()[-1], 'Gate plan: 13 gates would run, 1 would not. '
                         'Nothing was checked, and no record was written.')
        self.assertFalse((self.root / gate.OUTPUT).exists())
        self.fake.calls.clear()
        self.git('reset', '--hard', self.base)
        code, output, _ = self.invoke('--plan')
        self.assertEqual(code, 0)
        self.assertIn('Gate plan: 11 gates would run, 3 would not.', output)
        self.assertFalse(self.fake.calls)
        self.assertFalse((self.root / gate.OUTPUT).exists())

    def test_a_command_past_its_timeout_is_ended_with_its_children(self):
        processes = []

        def exists(pid):
            try:
                os.kill(pid, 0)
                return True
            except ProcessLookupError:
                return False

        def gone(pids):
            deadline = time.monotonic() + 5
            while any(exists(pid) for pid in pids) and time.monotonic() < deadline:
                time.sleep(0.05)
            alive = [pid for pid in pids if exists(pid)]
            for pid in pids:
                if pid not in alive and pid in processes:
                    processes.remove(pid)  # Gone: its number must not be killed after reuse.
            return not alive

        # Also clean up if a mutant leaves a process alive, and the child that left the group.
        def cleanup():
            for pid in processes:
                try:
                    os.kill(pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
        self.addCleanup(cleanup)

        # trap: the leader waits for its child on SIGTERM and the child ignores SIGTERM, so only
        # SIGKILL ends them. plain: neither traps SIGTERM. escape: the child leaves the group.
        # detach: as escape, but the leader exits 0 at once.
        script = self.parent / 'sleeper.py'
        script.write_text(
            'import os, signal, subprocess, sys, time\n'
            'mode, pids, ready = sys.argv[1:4]\n'
            'child = subprocess.Popen([sys.executable, "-c", "import signal, sys, time\\n'
            'if sys.argv[1] == \'trap\': signal.signal(signal.SIGTERM, signal.SIG_IGN)\\n'
            'open(sys.argv[2], \'w\').close()\\ntime.sleep(60)", mode, ready],\n'
            '                         start_new_session=mode in ("escape", "detach"))\n'
            'if mode == "trap":\n'
            '    def stop(sig, frame):\n'
            '        child.wait()\n'
            '        sys.exit(0)\n'
            '    signal.signal(signal.SIGTERM, stop)\n'
            'deadline = time.monotonic() + 5\n'
            'while not os.path.exists(ready) and time.monotonic() < deadline:\n'
            '    time.sleep(0.01)\n'
            'with open(pids, "w") as stream:\n'
            '    stream.write(str(os.getpid()) + " " + str(child.pid))\n'
            'print("before sleeping: planted-alpha", flush=True)\n'
            'if mode != "detach":\n'
            '    time.sleep(60)\n')
        log = self.parent / 'runner.log'

        def run(mode, timeout, label):
            pids, ready = self.parent / ('pids-' + label), self.parent / ('ready-' + label)
            started = time.monotonic()
            result = gate.run_command([sys.executable, str(script), mode, str(pids), str(ready)],
                                      cwd=self.root, env=dict(os.environ), timeout=timeout, log=log,
                                      redact=lambda text: gate.redact_output(text, 'planted-alpha'))
            elapsed = time.monotonic() - started
            started_pids = [int(pid) for pid in pids.read_text().split()] if pids.exists() else []
            processes.extend(started_pids)
            return result, elapsed, started_pids

        result, elapsed, started_pids = run('trap', 1, 'trap')
        self.assertLess(elapsed, 15)
        self.assertEqual(len(started_pids), 2)
        self.assertTrue(result.timed_out)
        self.assertTrue(gone(started_pids))
        self.assertIn('before sleeping', log.read_text())
        self.assertIn('before sleeping', result.output)
        self.assertIn('[redacted]', result.output)
        self.assert_private(log.read_text() + result.output, 'planted-alpha')

        # A group whose only member is a leader that exited and is not reaped: macOS's killpg
        # answers EPERM, which end_group must take as the group's end. The leader exits only when
        # its input closes, after kqueue watches it, and kqueue reports the exit without reaping.
        if hasattr(select, 'kqueue'):
            zombie = subprocess.Popen([sys.executable, '-c', 'import sys; sys.stdin.read()'],
                                      stdin=subprocess.PIPE, start_new_session=True)
            queue = select.kqueue()
            try:
                queue.control([select.kevent(zombie.pid, filter=select.KQ_FILTER_PROC,
                                             flags=select.KQ_EV_ADD | select.KQ_EV_ONESHOT,
                                             fflags=select.KQ_NOTE_EXIT)], 0)
                zombie.stdin.close()
                self.assertEqual(len(queue.control(None, 1, 30)), 1)
            finally:
                queue.close()
            gate.end_group(zombie)
            self.assertEqual(zombie.returncode, 0)

        # The same answer to the probe after a SIGTERM that was taken, on every platform: a stub
        # gives it, for a leader that has exited, so that no real signal is sent.
        exited = subprocess.Popen([sys.executable, '-c', 'pass'], start_new_session=True)
        exited.wait()
        signals = []

        def killpg(pid, sig):
            signals.append(sig)
            if sig == 0:
                raise PermissionError(errno.EPERM, os.strerror(errno.EPERM))

        with mock.patch.object(gate.os, 'killpg', side_effect=killpg):
            gate.end_group(exited)
        self.assertEqual(signals[:2], [signal.SIGTERM, 0])
        self.assertEqual(exited.returncode, 0)

        # A leader and child that trap nothing end on SIGTERM; the group they leave may meet
        # the answer that the two cases above make certain.
        for attempt in range(5):
            with self.subTest(attempt=attempt):
                result, elapsed, started_pids = run('plain', 0.5, 'plain-' + str(attempt))
                self.assertTrue(result.timed_out)
                self.assertLess(elapsed, 15)
                self.assertTrue(gone(started_pids))

        # A child that left the group and holds the output pipe does not hold the result.
        result, elapsed, started_pids = run('escape', 1, 'escape')
        self.assertLess(elapsed, 15)
        self.assertTrue(result.timed_out)
        self.assertIn('before sleeping', log.read_text())
        self.assertTrue(gone(started_pids[:1]))

        # The timeout counts the leader alone: a leader that exits 0 at once is not a timeout.
        result, elapsed, started_pids = run('detach', 3, 'detach')
        self.assertLess(elapsed, 15)
        self.assertFalse(result.timed_out)
        self.assertEqual(result.exit, 0)
        self.assertIn('before sleeping', result.output)
        self.assertTrue(gone(started_pids[:1]))

        # Interrupted, main ends the real runner's group, says so, and returns 130.
        previous = signal.signal(signal.SIGINT, signal.default_int_handler)
        self.addCleanup(signal.signal, signal.SIGINT, previous)
        interrupted = self.parent / 'interrupted-pid'

        def interrupt_once_started():
            deadline = time.monotonic() + 5
            while not (interrupted.exists() and interrupted.read_text()) and time.monotonic() < deadline:
                time.sleep(0.01)
            os.kill(os.getpid(), signal.SIGINT)

        def interrupted_runner(args, **kwargs):
            threading.Thread(target=interrupt_once_started, daemon=True).start()
            return gate.run_command([sys.executable, '-c', 'import os, sys, time\n'
                                     'with open(sys.argv[1], "w") as stream:\n'
                                     '    stream.write(str(os.getpid()))\n'
                                     'time.sleep(60)', str(interrupted)], **kwargs)

        started = time.monotonic()
        with contextlib.redirect_stdout(io.StringIO()) as output, \
                contextlib.redirect_stderr(io.StringIO()), \
                mock.patch.object(gate.shutil, 'which', return_value=None):
            code = gate.main(self.root, ['--base', 'base'], run=interrupted_runner)
        self.assertLess(time.monotonic() - started, 15)
        self.assertEqual(code, 130)
        self.assertEqual(output.getvalue().splitlines()[-1],
                         'Gate: interrupted during docs; its process group was ended, '
                         'and the record says incomplete.')
        self.assertEqual(self.record()['result'], 'incomplete')
        processes.append(int(interrupted.read_text()))
        self.assertTrue(gone(processes[-1:]))

        # Interrupted during the bounded wait on the pipe, after the leader exited, the runner
        # still ends the group: here a process left in it that holds the pipe.
        background = self.parent / 'background-pid'
        timer = threading.Timer(3, os.kill, (os.getpid(), signal.SIGINT))
        timer.start()
        self.addCleanup(timer.cancel)
        with self.assertRaises(KeyboardInterrupt):
            gate.run_command(['/bin/sh', '-c', 'sleep 30 & echo $! > "$1"; echo started', 'sh',
                              str(background)],
                             cwd=self.root, env=dict(os.environ), timeout=60, log=log)
        processes.append(int(background.read_text()))
        self.assertTrue(gone(processes[-1:]))

        # The runner must also redact a successful command, including both output streams.
        result = gate.run_command(
            [sys.executable, '-c', 'import sys; print("planted-alpha"); '
             'print("planted-alpha-beta", file=sys.stderr)'],
            cwd=self.root, env=dict(os.environ), timeout=5, log=log,
            redact=lambda text: gate.redact_output(text, 'planted-alpha,planted-alpha-beta'))
        self.assertEqual(result.exit, 0)
        self.assertEqual(result.output, '[redacted]\n[redacted]\n')
        self.assertEqual(log.read_text(), result.output)

if __name__ == '__main__':
    unittest.main()
