import contextlib
import errno
import importlib.util
import io
import json
import os
from pathlib import Path
import runpy
import select
import shutil
import signal
import subprocess
import sys
import tempfile
import threading
import time
from types import SimpleNamespace
import unittest
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import gate
import set_names

REPOSITORY = Path(__file__).resolve().parents[2]


class FakeRunner:
    def __init__(self, root):
        self.root = root
        self.calls = []
        self.responses = {}
        self.version = 'v8.24.2\n'
        self.report = []
        self.scan_exit = 0
        self.inspect = None
        self.workspaces = {}
        self.dependencies = {}
        self.package_names = {}
        self.resolved_packages = {}

    def __call__(self, args, *, cwd, env, timeout, log, redact):
        self.calls.append(dict(args=list(args), cwd=cwd, env=dict(env), timeout=timeout, log=log))
        if self.inspect:
            self.inspect(self.calls[-1])
        if len(args) > 1 and args[1] == 'locate-project':
            manifest = args[-1]
            workspace = self.workspaces.get(manifest, manifest)
            result = self.responses.get('locate-project', gate.CommandResult(0, str(cwd / workspace) + '\n'))
        elif len(args) > 1 and args[1] == 'metadata':
            directory = (Path(args[-1]).parent.as_posix() if '--no-deps' in args
                         else cwd.relative_to(self.root).as_posix())
            dependencies = [dict(path=str(self.root / path), kind=kind, name=Path(path).name)
                            for path, kind in self.dependencies.get(directory, [])]
            packages = [dict(name=name, source=None, dependencies=dependencies)
                        for name in self.package_names.get(directory, [Path(directory).name])]
            if '--no-deps' not in args:
                packages = self.resolved_packages.get(directory, packages)
            result = self.responses.get('metadata', gate.CommandResult(
                0, json.dumps(dict(packages=packages))))
        elif args[-1] == 'version':
            result = self.responses.get('version', gate.CommandResult(0, self.version))
        elif args[-1] == '-vV':
            result = self.responses.get('-vV', gate.CommandResult(0, 'host: aarch64-apple-darwin\n'))
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


class FakeMachine(gate.Machine):
    def __init__(self, *, account=('planted-short', ''), host='planted-host', macos=False,
                 computer_name=(1, b''), local_host_name=(1, b''),
                 git_name=(1, b''), git_email=(1, b''), list_error=None,
                 tailscale=None, peers=(0, b'{}')):
        super().__init__(os.environ)
        self.calls = []
        self.account_value = account
        self.host = host
        self.macos = macos
        self.list_error = list_error
        self.tailscale = tailscale
        self.responses = {
            ('git', 'config', '--global', '--get', 'user.name'): git_name,
            ('git', 'config', '--global', '--get', 'user.email'): git_email,
            ('scutil', '--get', 'ComputerName'): computer_name,
            ('scutil', '--get', 'LocalHostName'): local_host_name,
        }
        if tailscale is not None:
            self.responses[tailscale, 'status', '--json'] = peers

    def account(self):
        self.calls.append(('account',))
        if isinstance(self.account_value, gate.SourceSkipped):
            raise self.account_value
        return self.account_value

    def host_name(self):
        self.calls.append(('host_name',))
        if isinstance(self.host, OSError):
            raise self.host
        return self.host

    def is_macos(self):
        self.calls.append(('is_macos',))
        return self.macos

    def which(self, name):
        self.calls.append(('which', name))
        if name != 'tailscale':
            raise AssertionError('unexpected command lookup')
        return self.tailscale

    def list_directory(self, path):
        self.calls.append(('list_directory', path))
        if self.list_error is not None:
            raise self.list_error
        return super().list_directory(path)

    def query(self, args, extra=None):
        command = tuple(args)
        self.calls.append(('query', command, extra))
        if command in self.responses:
            return self.responses[command]
        if args[0] == 'git' and '--global' not in args:
            return super().query(args, extra)
        raise AssertionError('unexpected machine command')


class CargoCacheTests(unittest.TestCase):
    def test_target_cache_rebuilds_for_a_second_tree_with_an_explicit_host_target(self):
        if not shutil.which('cargo') or not shutil.which('rustc'):
            self.skipTest('Cargo and rustc are required for the cache regression')
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            cache = root / 'cache'
            environment, _, _ = gate.scrub_environment()
            environment.update(CARGO_NET_OFFLINE='true', CARGO_TERM_COLOR='never',
                               RUSTUP_AUTO_INSTALL='0')
            version = subprocess.check_output(['rustc', '-vV'], cwd=root, env=environment,
                                              text=True, timeout=60)
            host = next(line.removeprefix('host: ') for line in version.splitlines()
                        if line.startswith('host: '))
            environment.update(CARGO_BUILD_TARGET=host, CARGO_TARGET_DIR=str(cache))

            first, second = root / 'first', root / 'second'
            for tree, expected in ((first, 1), (second, 2)):
                (tree / 'src').mkdir(parents=True)
                manifest = tree / 'Cargo.toml'
                manifest.write_text('[package]\nname = "gate-cache-probe"\nversion = "0.0.0"\n',
                                    encoding='utf-8')
                source = tree / 'src/lib.rs'
                source.write_text('#[test]\nfn identifies_its_tree() {\n'
                                  f'    assert_eq!(1, {expected}, "source tree assertion");\n'
                                  '}\n', encoding='utf-8')
                # Give both trees timestamps older than the shared cached build.
                for file in (manifest, source):
                    os.utime(file, (1_700_000_000, 1_700_000_000))

            def test_tree(tree):
                return subprocess.run(['cargo', 'test', '--release', '--offline'],
                                      cwd=tree, env=environment, text=True, timeout=60,
                                      stdout=subprocess.PIPE, stderr=subprocess.STDOUT)

            passing = test_tree(first)
            self.assertEqual(passing.returncode, 0, passing.stdout)
            fingerprints = cache / host / 'release' / '.fingerprint'
            self.assertTrue(any(fingerprints.glob('gate-cache-probe-*')))
            gate.invalidate_path_fingerprints(cache, {'gate-cache-probe'})
            failing = test_tree(second)
            self.assertEqual(failing.returncode, 101, failing.stdout)
            self.assertIn('source tree assertion', failing.stdout)


class GateTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.parent = Path(temporary.name).resolve()
        self.root = self.parent / 'repository'
        self.root.mkdir()
        self.environment = mock.patch.dict(os.environ, {'PATH': os.defpath}, clear=True)
        self.environment.start()
        self.addCleanup(self.environment.stop)
        self.git('init', '-q')
        self.write('.gitignore', 'target/\n')
        self.write(str(gate.WORKFLOW), 'jobs:\n  secrets:\n    scanner_version=8.24.2\n'
                   '    macos_only=" experiments/005-macos-keychain "\n')
        self.write('seed.txt', 'base\n')
        self.base = self.commit()
        self.git('branch', 'base')
        self.git('checkout', '-qb', 'work')
        self.fake = FakeRunner(self.root)

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

    def invoke(self, *args, installed=None, cargo='cargo', machine=None):
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err), \
                mock.patch.object(gate.shutil, 'which',
                                  side_effect=lambda name: cargo if name == 'cargo' else installed):
            code = gate.main(self.root, ['--base', 'base', *args], run=self.fake, machine=machine)
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

    def add_origin(self, url='https://example.invalid/planted-owner/repository.git'):
        self.git('remote', 'add', 'origin', url)

    def add_sibling(self, name, **remotes):
        sibling = self.parent / name
        self.git('init', '-q', str(sibling))
        for remote, url in remotes.items():
            self.git('-C', str(sibling), 'remote', 'add', remote, url)
        return sibling

    def assert_names_given(self, expected):
        calls = []
        for call in self.fake.calls:
            if 'tools/check_private_paths.py' in call['args']:
                calls.append(call['env'].get('CONDUIT_PRIVATE_NAMES'))
            else:
                self.assertNotIn('CONDUIT_PRIVATE_NAMES', call['env'])
        self.assertEqual(calls, [expected])

    def private_lines(self, output):
        return [line for line in output.splitlines() if line.startswith('Private names:')]

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
        self.assertIn('not run: identity, tool tests, secrets, engine.', output.splitlines()[-1])

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
        self.assertIn('Environment: cargo runs with CARGO_NET_OFFLINE=true, RUSTUP_AUTO_INSTALL=0, '
                      'CARGO_TERM_COLOR=never and CARGO_BUILD_JOBS=6, and cargo test with '
                      'RUST_TEST_THREADS=4. Removed from every command: CONDUIT_PRIVATE_NAMES, CONDUIT_TEST_NPM, '
                      'RUSTFLAGS, RUSTUP_TOOLCHAIN, SCR_AGENT_CODEX, and 1 variables named for a credential.\n',
                      output)
        env = self.record()['environment']
        self.assertTrue(env['private_names_configured'])
        self.assertEqual(env['private_names'], dict(file=True, derived=None))
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
        machine = FakeMachine()
        with mock.patch.dict(os.environ, values):
            _, output, _ = self.invoke(machine=machine)
        self.assertIn('CONDUIT_PRIVATE_NAMES in the environment is not used.', output)
        self.assertEqual(machine.calls, [])
        self.assertTrue(all('CONDUIT_PRIVATE_NAMES' not in call['env'] for call in self.fake.calls))
        self.assertFalse(self.record()['environment']['private_names_configured'])
        self.assertEqual(self.record()['environment']['private_names'], dict(file=False, derived=None))

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

    def test_derived_names_come_from_each_source(self):
        self.add_origin()
        first = self.add_sibling('planted-sibling-one', origin=(
            'https://planted-user:planted-secret@example.invalid/planted-owner/planted-remote-one.git'))
        self.add_sibling('planted-sibling-two',
                         origin='git@example.invalid:planted-owner/planted-remote-two.git',
                         upstream='https://example.invalid/planted-owner/planted-remote-three')
        (self.parent / 'planted-plain-directory').mkdir()
        (self.parent / 'planted-link').symlink_to(first, target_is_directory=True)
        tailscale = 'planted-bin/tailscale'
        machine = FakeMachine(
            account=('planted-short', 'Fakefirst Fakelast,Room 1,,'), host='planted-host.example.invalid',
            macos=True, computer_name=(0, b'Planted Computer\n'), local_host_name=(0, b'planted-local-host\n'),
            git_name=(0, b'Planted Gitname\n'), git_email=(0, b'planted-git@example.invalid\n'),
            tailscale=tailscale, peers=(0, json.dumps({
                'Self': {'HostName': 'planted-self', 'DNSName': 'planted-self-dns.example.invalid.'},
                'Peer': {'planted-key': {'HostName': 'planted-peer',
                                         'DNSName': 'planted-peer-dns.example.invalid.'}},
            }).encode()))

        code, output, error = self.invoke('--derive-private-names', machine=machine)

        self.assertEqual(code, 0)
        self.assert_names_given(
            'planted-short,Fakefirst Fakelast,fakefirst fakelast,Fakefirst,fakefirst,Fakelast,fakelast,'
            'planted-host.example.invalid,planted-host,Planted Computer,planted computer,planted-local-host,'
            'Planted Gitname,planted gitname,planted-git,planted-sibling-one,planted-remote-one,'
            'planted-sibling-two,planted-remote-two,planted-remote-three,'
            'planted-self,planted-self-dns,planted-peer,planted-peer-dns')
        self.assertEqual(self.private_lines(output), [
            'Private names: 19 derived from this machine (account 4, host name 2, macOS computer names 2, '
            'git identity 2, sibling repositories 5, tailnet peers 4), and given to '
            'tools/check_private_paths.py alone; 1 dropped as public or short (0 shorter than 4 characters, '
            "1 in the origin remote's URL, 0 on the gate's list of public words); none is printed or recorded.",
        ])
        self.assertIn(('query', (tailscale, 'status', '--json'), None), machine.calls)
        queries = [(args[3], extra) for call, args, extra in
                   (entry for entry in machine.calls if entry[0] == 'query')
                   if args[:3] == (*gate.GIT, '-C')]
        self.assertEqual(queries, [
            (str(self.parent / name), {'GIT_CEILING_DIRECTORIES': str(self.parent)})
            for name in ('planted-sibling-one', 'planted-sibling-two', 'repository')
        ])
        self.assertEqual(self.record()['environment']['private_names'], {
            'file': False, 'derived': {
                'kept': 19, 'dropped': {'short': 0, 'origin': 1, 'public': 0}, 'origin_remote': True,
                'unreadable_sibling_remotes': 0,
                'sources': [dict(name=name, result='read', kept=count, why=None)
                            for name, count in [('account', 4), ('host name', 2),
                                                ('macOS computer names', 2), ('git identity', 2),
                                                ('sibling repositories', 5), ('tailnet peers', 4)]],
            },
        })
        self.assert_private(output + error + self.logs() + self.record_path().read_text(),
                            'planted-user', 'planted-secret', 'planted-owner', 'planted-key', 'Room 1')

        trace_paths = {key: self.parent / key for key in
                       ('GIT_TRACE', 'GIT_TRACE2', 'GIT_TRACE2_EVENT', 'GIT_TRACE2_PERF')}
        trace_environment = {key: str(path) for key, path in trace_paths.items()}
        command = ['git', '-C', str(first), 'config', '--local', '--get', 'remote.origin.url']
        subprocess.run(command, env=dict(machine.environment, **trace_environment), check=True,
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        for key, path in trace_paths.items():
            trace = path.read_bytes()
            self.assertTrue(trace)
            if key == 'GIT_TRACE2_EVENT':
                self.assertTrue(os.fsencode(first) in trace)
            path.unlink()
        machine.environment.update(trace_environment)
        derived = gate.derive_private_names(self.root, machine)
        self.assertIn('planted-sibling-one', derived.kept)
        self.assertIn('planted-remote-one', derived.kept)
        self.assertTrue(derived.origin_remote)
        self.assertFalse(any(path.exists() for path in trace_paths.values()))

    def test_derived_names_drop_short_public_and_origin_words(self):
        self.assertEqual(set(gate.PUBLIC_WORDS), {'agentwatch', 'localhost', 'pi-reference'})
        for reason in gate.PUBLIC_WORDS.values():
            self.assertIsInstance(reason, str)
            self.assertTrue(reason)
        self.add_origin('https://example.invalid/planted-origin-word/repository.git')
        machine = FakeMachine(
            account=('abc', ''), host='Planted-Origin-Word',
            git_name=(0, b'AgentWatch\n'), git_email=(0, b'LOCALHOST@example.invalid\n'),
            tailscale='planted-bin/tailscale', peers=(0, json.dumps({
                'Self': {'HostName': 'planted-peer'},
                'Peer': {'planted-key': {'HostName': 'PLANTED-PEER', 'DNSName': 'Pi-Reference.example.invalid.'},
                         'planted-null': None},
            }).encode()))

        code, output, _ = self.invoke('--derive-private-names', machine=machine)
        self.assertEqual(code, 0)
        self.assert_names_given('planted-peer')
        self.assertEqual(self.private_lines(output), [
            'Private names: 1 derived from this machine (account 0, host name 0, git identity 0, '
            'sibling repositories 0, tailnet peers 1), and given to tools/check_private_paths.py alone; '
            "6 dropped as public or short (1 shorter than 4 characters, 2 in the origin remote's URL, "
            "3 on the gate's list of public words); none is printed or recorded.",
            'Private names: NOT RUN macOS computer names: this is not macOS.',
        ])
        self.git('remote', 'remove', 'origin')
        self.fake.calls.clear()

        code, output, _ = self.invoke('--derive-private-names', machine=machine)
        self.assertEqual(code, 0)
        self.assert_names_given('Planted-Origin-Word,planted-origin-word,repository,planted-peer')
        self.assertEqual(self.private_lines(output), [
            'Private names: 3 derived from this machine (account 0, host name 1, git identity 0, '
            'sibling repositories 1, tailnet peers 1), and given to tools/check_private_paths.py alone; '
            "4 dropped as public or short (1 shorter than 4 characters, 0 in the origin remote's URL, "
            "3 on the gate's list of public words); none is printed or recorded.",
            'Private names: NOT RUN macOS computer names: this is not macOS.',
            'Private names: this repository has no origin remote, so no derived name was dropped '
            'as already public in its URL.',
        ])

    def test_derived_names_combine_with_a_names_file(self):
        self.add_origin()
        names = self.names_file('xyz, Planted-Peer\n')
        readings = dict(host=OSError(), tailscale='planted-bin/tailscale',
                        peers=(0, b'{"Self": {"HostName": "planted-peer"}}'))
        options = ('--derive-private-names', '--private-names-file', str(names))

        code, output, error = self.invoke(*options, machine=FakeMachine(**readings))

        self.assertEqual(code, 0)
        self.assert_names_given('xyz,Planted-Peer,planted-short,planted-peer')
        self.assertEqual(self.private_lines(output), [
            'Private names: read from the file that --private-names-file names, and 2 more derived from this '
            'machine (account 1, sibling repositories 0, tailnet peers 1), all given to '
            'tools/check_private_paths.py alone; 1 dropped as public or short (0 shorter than 4 characters, '
            "1 in the origin remote's URL, 0 on the gate's list of public words); neither the names nor the "
            "file's path are printed or recorded.",
            'Private names: NOT RUN host name: socket.gethostname() failed.',
            'Private names: NOT RUN macOS computer names: this is not macOS.',
            "Private names: NOT RUN git identity: git's global configuration sets neither user.name nor "
            'user.email.',
        ])
        environment = self.record()['environment']
        self.assertTrue(environment['private_names_configured'])
        self.assertTrue(environment['private_names']['file'])
        self.assertEqual(environment['private_names']['derived']['kept'], 2)
        self.assert_private((output + error + self.logs() + self.record_path().read_text()).lower(),
                            'xyz', 'planted-peer', 'planted-short', str(names).lower())

        self.record_path().unlink()
        self.fake.calls.clear()
        names.chmod(0o644)
        machine = FakeMachine(**readings)
        code, output, _ = self.invoke(*options, machine=machine)
        self.assertEqual(code, 2)
        self.assertEqual(output.splitlines(), [
            'Gate: NOT RUN: the file that --private-names-file names has mode 0644, not 0600; run chmod '
            '600 on it. Nothing was checked.',
        ])
        self.assertEqual(machine.calls, [])
        self.assertEqual(self.fake.calls, [])
        self.assertFalse(self.record_path().exists())

    def test_a_missing_source_is_skipped_by_name(self):
        source_order = ('account', 'host name', 'macOS computer names', 'git identity',
                        'sibling repositories', 'tailnet peers')
        base_skips = {
            'macOS computer names': 'this is not macOS.',
            'git identity': "git's global configuration sets neither user.name nor user.email.",
            'tailnet peers': 'the tailscale command is not on the PATH.',
        }

        def assert_skips(output, **changes):
            expected = dict(base_skips, **changes)
            self.assertEqual([line for line in self.private_lines(output)
                              if line.startswith('Private names: NOT RUN ')],
                             [f'Private names: NOT RUN {name}: {expected[name]}'
                              for name in source_order if name in expected])

        code, output, _ = self.invoke('--derive-private-names', machine=FakeMachine())
        self.assertEqual(code, 0)
        assert_skips(output)
        tailscale = 'planted-bin/tailscale'
        for changes, line in [
            (dict(host=OSError()), 'host name: socket.gethostname() failed.'),
            (dict(macos=True), 'macOS computer names: scutil answered neither ComputerName nor LocalHostName.'),
            (dict(git_name=(128, b'')), 'git identity: git config --global exited 128.'),
            (dict(git_name=(None, b'')), 'git identity: git config --global did not answer within 10 seconds.'),
            (dict(tailscale=tailscale, peers=(1, b'planted-tailscale-output')),
             'tailnet peers: tailscale status --json exited 1.'),
            (dict(tailscale=tailscale, peers=(None, b'')),
             'tailnet peers: tailscale status --json did not answer within 10 seconds.'),
            (dict(tailscale=tailscale, peers=(0, b'[]')),
             'tailnet peers: tailscale status --json did not print a JSON object.'),
            (dict(list_error=OSError()),
             'sibling repositories: the directory that holds this repository could not be listed.'),
            (dict(account=gate.SourceSkipped("this process's user ID has no account entry")),
             "account: this process's user ID has no account entry."),
        ]:
            with self.subTest(source=line):
                code, output, error = self.invoke('--derive-private-names', machine=FakeMachine(**changes))
                self.assertEqual(code, 0)
                source, why = line.split(': ', 1)
                assert_skips(output, **{source: why})
                self.assertTrue(output.splitlines()[-1].startswith('Gate: PASS.'))
                self.assert_private(output + error + self.logs() + self.record_path().read_text(),
                                    'planted-tailscale-output')

        unreadable = ('Private names: the remotes of 1 sibling repositories could not be read; '
                      'their directory names alone were taken.')
        invalid_url = ('https://planted-user:planted-secret@'
                       'planted\uff03host.example.invalid/planted-owner/planted-remote.git')
        for broken in (True, False):
            with self.subTest(broken_gitdir=broken):
                if broken:
                    sibling = self.parent / 'planted-broken'
                    sibling.mkdir()
                    (sibling / '.git').write_text('gitdir: planted-nowhere\n')
                else:
                    sibling = self.add_sibling('planted-sibling-one', origin=invalid_url)
                try:
                    self.fake.calls.clear()
                    code, output, error = self.invoke('--derive-private-names', machine=FakeMachine())
                    self.assertEqual(code, 0)
                    assert_skips(output)
                    self.assertIn(unreadable, self.private_lines(output))
                    self.assert_names_given(','.join(['planted-short', 'planted-host', sibling.name,
                                                      'repository']))
                    derived = self.record()['environment']['private_names']['derived']
                    self.assertEqual(derived['unreadable_sibling_remotes'], 1)
                    self.assertTrue(output.splitlines()[-1].startswith('Gate: PASS.'))
                    self.assert_private(output + error + self.logs() + self.record_path().read_text(),
                                        'planted-secret', 'planted-remote')
                finally:
                    shutil.rmtree(sibling)

        self.add_origin()
        original_git = gate.git
        for failed_argument in ('--git-common-dir', 'remote.origin.url'):
            with self.subTest(failed_argument=failed_argument):
                def git(root, env, *args):
                    if args[-1] == failed_argument:
                        raise OSError('planted-git-error')
                    return original_git(root, env, *args)

                self.fake.calls.clear()
                with mock.patch.object(gate, 'git', side_effect=git):
                    code, output, error = self.invoke('--derive-private-names', machine=FakeMachine())
                self.assertEqual(code, 0)
                if failed_argument == '--git-common-dir':
                    assert_skips(output, **{'sibling repositories': 'this repository has no main '
                                            'working tree, so its sibling directories are not known.'})
                    self.assert_names_given('planted-short,planted-host')
                else:
                    assert_skips(output)
                    self.assertIn('Private names: this repository has no origin remote, so no derived '
                                  'name was dropped as already public in its URL.', self.private_lines(output))
                    self.assert_names_given('planted-short,planted-host,repository')
                self.assert_private(output + error + self.logs() + self.record_path().read_text(),
                                    'planted-git-error')

        self.record_path().unlink()
        self.fake.calls.clear()
        machine = FakeMachine(account=gate.SourceSkipped("this process's user ID has no account entry"),
                              host=OSError(), list_error=OSError())
        code, output, _ = self.invoke('--derive-private-names', machine=machine)
        self.assertEqual(code, 2)
        self.assertEqual(output.splitlines(), [
            'Gate: NOT RUN: --derive-private-names read none of its sources on this machine. Nothing was checked.',
        ])
        self.assertEqual(self.fake.calls, [])
        self.assertFalse(self.record_path().exists())

        for invalid_name in ('planted-\0peer', 'planted-\ud800peer'):
            with self.subTest(invalid_name=repr(invalid_name)):
                peers = json.dumps({'Self': {'HostName': invalid_name}}).encode()
                machine = FakeMachine(tailscale=tailscale, peers=(0, peers))
                code, output, error = self.invoke('--derive-private-names', machine=machine)
                self.assertEqual(code, 2)
                self.assertEqual(output.splitlines(), [
                    'Gate: NOT RUN: --derive-private-names read a name from tailnet peers that '
                    'cannot be passed to tools/check_private_paths.py. Nothing was checked.',
                ])
                self.assertEqual(error, '')
                self.assertEqual(self.fake.calls, [])
                self.assertFalse(self.record_path().exists())

    def test_derived_names_are_redacted_and_never_recorded(self):
        self.git('checkout', '-qb', 'wo/planted-short-work')
        machine = FakeMachine(account=('planted-short', 'Fakefirst Fakelast'))
        self.fake.responses['tools/check_docs.py'] = gate.CommandResult(
            1, 'PLANTED-SHORT and fakefirst FAKELAST here\n')

        code, output, error = self.invoke('--derive-private-names', machine=machine)

        self.assertEqual(code, 1)
        self.assertIn(' on wo/[redacted]-work', output.splitlines()[0])
        docs_line = next(line for line in output.splitlines() if line.startswith('FAIL docs '))
        self.assertIn('[redacted] and [redacted] here', docs_line)
        self.assertIn('[redacted] and [redacted] here', (self.root / self.row('docs')['log']).read_text())
        self.assert_names_given('planted-short,Fakefirst Fakelast,fakefirst fakelast,'
                                'Fakefirst,fakefirst,Fakelast,fakelast,planted-host,repository')
        self.assertEqual(self.record()['branch'], 'wo/[redacted]-work')
        self.assert_private((output + error + self.logs() + self.record_path().read_text()).lower(),
                            'planted-short', 'fakefirst fakelast', 'fakefirst', 'fakelast', 'planted-host')

        self.record_path().unlink()
        calls = machine.calls[:]
        code, plan, _ = self.invoke('--plan', '--derive-private-names', machine=machine)
        self.assertEqual(code, 0)
        self.assertEqual(self.private_lines(plan), self.private_lines(output))
        self.assertEqual(machine.calls, calls + calls)
        self.assertFalse(self.record_path().exists())

    def test_the_real_readers_take_planted_values(self):
        import pwd

        machine = gate.Machine({})
        entry = SimpleNamespace(pw_name='planted-short', pw_gecos='Fakefirst Fakelast,,,')
        with mock.patch.object(pwd, 'getpwuid', return_value=entry):
            self.assertEqual(machine.account(), ('planted-short', 'Fakefirst Fakelast,,,'))
        with mock.patch.object(pwd, 'getpwuid', side_effect=KeyError):
            with self.assertRaises(gate.SourceSkipped) as skipped:
                machine.account()
        self.assertEqual(str(skipped.exception), "this process's user ID has no account entry")
        with mock.patch.object(gate.socket, 'gethostname', return_value='planted-host'):
            self.assertEqual(machine.host_name(), 'planted-host')

        with tempfile.TemporaryDirectory(dir=self.parent) as temporary:
            directory = Path(temporary)
            planted_bin = directory / 'planted-bin'
            planted_bin.mkdir()
            tailscale = planted_bin / 'tailscale'
            tailscale.write_text('#!/bin/sh\nprintf \'%s\\n\' \'{"Self": {"HostName": "planted-self"}}\'\n')
            tailscale.chmod(0o700)
            config = directory / 'planted-gitconfig'
            config.write_text('[user]\n\tname = Planted Gitname\n')
            git_bin = Path(shutil.which('git')).parent
            machine = gate.Machine(dict(PATH=os.pathsep.join(map(str, (planted_bin, git_bin))),
                                        HOME=temporary, GIT_CONFIG_GLOBAL=str(config), GIT_CONFIG_NOSYSTEM='1'))
            self.assertNotIn(str(planted_bin), os.get_exec_path())
            self.assertEqual(machine.which('tailscale'), str(tailscale))
            self.assertEqual(machine.query([str(tailscale), 'status', '--json']),
                             (0, b'{"Self": {"HostName": "planted-self"}}\n'))
            self.assertEqual(machine.query(['git', 'config', '--global', '--get', 'user.name']),
                             (0, b'Planted Gitname\n'))
            tailscale.write_text('#!/bin/sh\nexec /bin/sleep 30\n')
            started = time.monotonic()
            with mock.patch.object(gate, 'DERIVE_TIMEOUT', 1):
                self.assertEqual(machine.query([str(tailscale), 'status', '--json']), (None, b''))
            self.assertLess(time.monotonic() - started, 15)
            self.assertEqual(machine.query(['planted-no-such-command']), (127, b''))

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
            ('vector changes', 'python3 tools/vector_delta.py ' + self.base + ' WORKTREE --declared-changed'),
            ('decisions', 'python3 tools/check_decisions.py ' + self.base + ' WORKTREE')]
        actual = [(row['name'], row['command']) for row in self.record()['gates'][2:14]]
        self.assertEqual(actual, expected)
        for call in self.fake.calls:
            self.assertEqual(call['cwd'], self.root.resolve())
            if call['args'][0] == 'cargo':
                self.assertEqual(call['timeout'], 300)
                continue
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
        self.assertIn('NOT RUN engine: ' + gate.ENGINE_WHY, output)
        self.assertNotIn('rust and engine', output)
        self.assertIn('Gate: PASS. 13 passed; not run: tool tests, secrets, engine.', output)
        self.fake.responses['tools/check_schemas.py'] = gate.CommandResult(1, 'detail\nlast failure line\n')
        self.fake.calls.clear()
        code, output, error = self.invoke()
        self.assertEqual(code, 1)
        self.assertIn('Gate: FAIL. Failed: schemas;', output)
        line = next(line for line in output.splitlines() if line.startswith('FAIL schemas'))
        self.assertIn('exit 1; its last line: last failure line; log target/gate/logs/', line)
        self.assertIn('    last failure line', error)
        self.assertEqual(len(self.fake.calls), 12)
        self.assertIn('tools/vector_delta.py', self.fake.calls[-2]['args'])
        self.fake.responses['tools/check_schemas.py'] = gate.CommandResult(None, 'partial\n', True)
        code, output, _ = self.invoke()
        self.assertEqual(code, 1)
        self.assertIn('ended after 15 minutes, with its process group', output)
        self.fake.responses.clear()
        self.write('tools/untracked.py', '# untracked\n')
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
        # Derived names equal to a gate label or inside a skip line change no log path and no count.
        code, output, _ = self.invoke('--derive-private-names', machine=FakeMachine(account=('docs', 'Skipped')))
        self.assertEqual(code, 0)
        rows = self.record()['gates']
        partials = [row['partial'] for row in rows]
        self.assertIn('the permalinks at 2 commit(s) this clone lacks, which CI fetches', partials)
        self.assertIn('3 test(s) unittest [redacted]', partials)
        self.assertIn('target/gate/logs/' + self.git('rev-parse', 'HEAD') + '/03-[redacted].log',
                      [row['log'] for row in rows])
        for row in rows:
            if row['log'] is not None:
                self.assertTrue((self.root / row['log']).is_file(), row['log'])
        # Interruptions preserve the incomplete record, and do not claim a final result.
        def interrupt(*args, **kwargs):
            raise KeyboardInterrupt
        self.fake = interrupt
        code, output, _ = self.invoke(cargo=None)
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
        version_logs = []

        def inspect_scan(call):
            if '--report-path' in call['args']:
                version_logs.append(Path(call['log']).with_name('version.log').read_text())

        self.fake.inspect = inspect_scan
        code, output, error = self.invoke('--derive-private-names', installed='gitleaks',
                                          machine=FakeMachine(account=('8.24', '')))
        self.fake.inspect = None
        self.assertEqual(code, 0)
        self.assertEqual(self.row('secrets')['result'], 'PASS')
        self.assertTrue(self.row('secrets')['required'])
        self.assertEqual(version_logs, ['v[redacted].2\n'])
        self.assert_private(output + error + self.logs() + self.record_path().read_text(), '8.24')

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
            if len(self.fake.calls) == 3:
                observed.append(self.record())

        self.fake.inspect = inspect
        with mock.patch.dict(os.environ, {'PATH': os.defpath, 'CONDUIT_PRIVATE_NAMES': 'planted-environment',
                                          'EXAMPLE_TOKEN': 'planted-token'}, clear=True):
            code, output, _ = self.invoke('--private-names-file', str(names))
        self.assertEqual(code, 0)
        self.assertEqual(observed[0]['result'], 'incomplete')
        self.assertIsNone(observed[0]['finished'])
        self.assertEqual([row['name'] for row in observed[0]['gates']], ['rust discovery', 'identity', 'docs'])
        self.assertFalse(observed[0]['environment']['private_names_configured'])
        record = self.record()
        self.assertEqual(set(record), {'format', 'tool', 'started', 'finished', 'result', 'base',
                         'merge_base', 'head', 'branch', 'contains_base', 'behind', 'tree', 'changed_paths',
                         'platform', 'python', 'python_packages', 'environment', 'settings', 'gates'})
        self.assertEqual(record['format'], 1)
        self.assertEqual(record['tool'], 'tools/gate.py')
        self.assertEqual(record['result'], 'PASS')
        self.assertIsNotNone(record['finished'])
        self.assertEqual(record['base'], dict(ref='base', commit=self.base))
        self.assertEqual(record['tree'], dict(clean=True, uncommitted_paths=0))
        self.assertTrue(record['environment']['private_names_configured'])
        self.assertEqual(record['environment']['removed_credentials'], 1)
        self.assertEqual(record['environment']['set'], dict(CARGO_NET_OFFLINE='true', RUSTUP_AUTO_INSTALL='0',
                         CARGO_TERM_COLOR='never', CARGO_BUILD_JOBS='6', RUST_TEST_THREADS='4'))
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

        for interrupted_gate in ('metadata', 'tools/check_private_paths.py'):
            with self.subTest(interrupted_gate=interrupted_gate):
                def interrupt(call):
                    if call['args'][1] == interrupted_gate:
                        raise KeyboardInterrupt

                self.fake.inspect = interrupt
                code, _, _ = self.invoke('--derive-private-names', machine=FakeMachine())
                self.assertEqual(code, 130)
                record = self.record()
                self.assertEqual(record['result'], 'incomplete')
                reached_checker = interrupted_gate == 'tools/check_private_paths.py'
                self.assertEqual(record['environment']['private_names_configured'], reached_checker)

    def test_plan_lists_the_gates_and_runs_none(self):
        self.add_commit()
        code, output, _ = self.invoke('--plan', installed='gitleaks')
        self.assertEqual(code, 0)
        self.assertEqual([call['args'] for call in self.fake.calls],
                         [['cargo', 'metadata', '--no-deps', '--format-version', '1', '--offline',
                           '--manifest-path', 'engine/Cargo.toml'], ['gitleaks', 'version']])
        would = [line for line in output.splitlines() if line.startswith('WOULD RUN ')]
        self.assertEqual(len(would), 13)
        self.assertTrue(would[0].startswith('WOULD RUN identity:'))
        self.assertTrue(would[-1].startswith('WOULD RUN secrets:'))
        self.assertTrue(any(line.startswith('PASS rust discovery') for line in output.splitlines()))
        self.assertIn('NOT RUN engine: ' + gate.ENGINE_WHY, output)
        self.assertEqual(output.splitlines()[-1], 'Gate plan: rust discovery passed; 13 other gates '
                         'would run, 2 would not. No other gate was checked, and no record was written.')
        self.assertFalse((self.root / gate.OUTPUT).exists())
        self.fake.calls.clear()
        code, plan, _ = self.invoke('--plan', '--derive-private-names', installed='gitleaks',
                                    machine=FakeMachine(account=('discovery', '')))
        self.assertEqual(code, 0)
        self.assertIn('PASS rust [redacted] ', plan)
        self.assertNotIn('WOULD RUN rust [redacted]', plan)
        self.assertEqual(plan.splitlines()[-1], output.splitlines()[-1])
        self.assertFalse((self.root / gate.OUTPUT).exists())

        self.fake.calls.clear()
        self.git('reset', '--hard', self.base)
        code, output, _ = self.invoke('--plan')
        self.assertEqual(code, 0)
        self.assertIn('Gate plan: rust discovery passed; 11 other gates would run, 4 would not.', output)
        self.assertEqual([call['args'][1] for call in self.fake.calls], ['metadata'])
        self.assertFalse((self.root / gate.OUTPUT).exists())

    def crates(self, *directories, not_built=('experiments/005-macos-keychain',)):
        for directory in directories:
            self.write(directory + '/Cargo.toml', '[package]\nname="fake"\nversion="0.1.0"\n')
        self.write(str(gate.WORKFLOW), 'scanner_version=8.24.2\nmacos_only=" '
                   + ' '.join(not_built) + ' "\n')
        self.commit()
        self.git('branch', '-f', 'base', 'HEAD')

    def test_rust_directories_are_found_as_ci_finds_them(self):
        self.crates('packages/p', 'packages/w', 'packages/w/m', 'packages/p/target',
                    'experiments/e', 'experiments/f/g', 'experiments/005-macos-keychain')
        self.fake.workspaces['packages/w/m/Cargo.toml'] = 'packages/w/Cargo.toml'
        code, output, _ = self.invoke('--all')
        self.assertEqual(code, 0)
        self.assertEqual([row['name'] for row in self.record()['gates']][0], 'rust discovery')
        expected = ['rust experiments/005-macos-keychain', 'rust experiments/e test',
                    'rust experiments/f/g test', 'rust packages/p fmt', 'rust packages/p clippy',
                    'rust packages/p test', 'rust packages/w fmt', 'rust packages/w clippy',
                    'rust packages/w test', 'engine fmt', 'engine clippy', 'engine test']
        self.assertEqual([row['name'] for row in self.record()['gates']][15:], expected)
        self.assertFalse(self.row('rust experiments/005-macos-keychain')['required'])
        self.assertEqual(self.row('rust experiments/005-macos-keychain')['why'], gate.NOT_BUILT_WHY)
        locate = [call for call in self.fake.calls if call['args'][1] == 'locate-project']
        self.assertEqual(len(locate), 6)
        self.assertTrue(all(call['cwd'] == self.root and call['timeout'] == 300 for call in locate))
        self.assertTrue(all('target' not in call['args'][-1] for call in locate))
        log = (self.root / self.row('rust discovery')['log']).read_text()
        self.assertIn('cargo locate-project --workspace --message-format plain', log)
        self.assertIn('cargo metadata --no-deps --format-version 1 --offline', log)
        self.fake.calls.clear()
        code, output, _ = self.invoke('--plan', '--all', installed='gitleaks')
        self.assertEqual(code, 0)
        self.assertEqual({call['args'][1] for call in self.fake.calls},
                         {'locate-project', 'metadata', 'version'})
        self.assertTrue(all(call['timeout'] == 300 for call in self.fake.calls
                            if call['args'][1] != 'version'))
        self.assertIn('PASS rust discovery', output)
        for option in ((), ('--plan',)):
            with self.subTest(option=option):
                code, output, _ = self.invoke(*option, cargo=None)
                self.assertEqual(code, 1)
                self.assertIn('NOT RUN rust discovery:', output)
                self.assertIn('cargo was not found on the PATH; install Rust through rustup', output)
                self.assertIn('NOT RUN rust and engine: ' + gate.RUST_WHY, output)
                if not option:
                    self.assertIn('required, not run: rust discovery;', output.splitlines()[-1])
                    self.assertIsNone(self.row('rust and engine')['directory'])
                    self.assertIsNone(self.row('rust and engine')['command'])
                else:
                    self.assertTrue(output.splitlines()[-1].startswith(
                        'Gate plan: rust discovery was not run; '))
        self.write(str(gate.WORKFLOW), 'scanner_version=8.24.2\n')
        code, output, _ = self.invoke()
        self.assertEqual(code, 1)
        line = next(line for line in output.splitlines() if line.startswith('FAIL rust discovery'))
        self.assertIn('macos_only could not be read from .github/workflows/checks.yml', line)
        self.assertIn('NOT RUN rust and engine: ' + gate.RUST_WHY, output)
        self.assertEqual(self.record()['result'], 'FAIL')
        self.write(str(gate.WORKFLOW), 'scanner_version=8.24.2\nmacos_only=""\n')
        for command in ('locate-project', 'metadata'):
            for result in (gate.CommandResult(7, 'discovery failure\n'),
                           gate.CommandResult(-15, 'timed out\n', True)):
                with self.subTest(command=command, timeout=result.timed_out):
                    self.fake.responses[command] = result
                    code, output, _ = self.invoke('--plan')
                    self.assertEqual(code, 1)
                    self.assertIn('FAIL rust discovery', output)
                    self.assertIn(gate.timeout_words(5) if result.timed_out else 'exit 7', output)
                    self.assertNotIn('WOULD RUN engine', output)
                    self.assertTrue(output.splitlines()[-1].startswith(
                        'Gate plan: rust discovery failed; '))
                    self.fake.responses.clear()
        # A failed command's own last line is shown, not the gate's words for its exit.
        self.fake.responses['metadata'] = gate.CommandResult(101, 'error: failed to parse manifest\n')
        code, output, _ = self.invoke()
        self.assertEqual(code, 1)
        line = next(line for line in output.splitlines() if line.startswith('FAIL rust discovery'))
        self.assertIn('exit 101; its last line: error: failed to parse manifest; log ', line)
        self.fake.responses.clear()

    def test_dependents_by_path_run_transitively(self):
        self.crates('packages/a', 'packages/b', 'packages/c', 'packages/d')
        self.fake.dependencies = {'packages/b': [('packages/a', None)],
                                  'packages/c': [('packages/b', 'dev')]}
        self.write('packages/a/src/lib.rs', '// changed\n')
        code, _, _ = self.invoke()
        self.assertEqual(code, 0)
        names = {row['name'] for row in self.record()['gates']}
        self.assertTrue({'rust packages/a test', 'rust packages/b test', 'rust packages/c test'} <= names)
        self.assertEqual(self.row('rust packages/a test')['reason'], 'the branch changes 1 of its paths')
        self.assertEqual(self.row('rust packages/b test')['reason'],
                         'it depends by path on packages/a, which the branch changes')
        self.assertEqual(self.row('rust packages/c test')['reason'],
                         'it depends by path, through packages/b, on packages/a, which the branch changes')
        self.assertEqual(self.row('rust unreached')['why'], gate.UNREACHED_WHY + 'packages/d')
        self.assertFalse(self.row('rust unreached')['required'])
        self.assertIsNone(self.row('rust unreached')['command'])
        # A cycle cannot select an otherwise unreached directory or prevent termination.
        self.fake.dependencies['packages/d'] = [('packages/d/nested', 'build')]
        (self.root / 'packages/a/src/lib.rs').unlink()
        self.invoke()
        self.assertTrue(all(row['result'] == 'NOT RUN' for row in self.record()['gates']
                            if row['name'].startswith('rust ') and row['name'] != 'rust discovery'))
        self.fake.dependencies['packages/b'].append(('packages/c', 'build'))
        self.write('packages/a/src/lib.rs', '// changed\n')
        self.assertEqual(self.invoke()[0], 0)
        self.assertEqual(self.row('rust packages/c test')['result'], 'PASS')
        outside = self.parent / 'outside'
        self.fake.dependencies['packages/d'] = [(str(outside), 'build')]
        code, output, _ = self.invoke()
        self.assertEqual(code, 1)
        self.assertIn('FAIL rust discovery', output)
        self.assertIn(str(outside), output)
        self.assertIn('NOT RUN rust and engine: ' + gate.RUST_WHY, output)
        self.assertFalse(self.record()['environment']['private_names_configured'])
        self.assertNotIn(str(outside), self.record_path().read_text())
        self.assertEqual(self.row('rust discovery')['why'],
                         'path dependency outside the repository')
        discovery_log = self.root / self.row('rust discovery')['log']
        self.assertIn('path dependency outside the repository: ' + str(outside),
                      discovery_log.read_text())
        # One inside the repository that no discovered directory or engine holds fails too.
        self.fake.dependencies['packages/d'] = [('tools/crate', None)]
        code, output, _ = self.invoke()
        self.assertEqual(code, 1)
        self.assertIn('path dependency has no discovered directory: tools/crate', output)
        self.assertEqual(self.row('rust discovery')['why'],
                         'path dependency has no discovered directory: tools/crate')

    def test_reads_and_the_engine_rule(self):
        self.crates('packages/e', 'packages/f', 'packages/g', 'packages/h')
        self.fake.dependencies['packages/h'] = [('engine/crates/core', None)]
        reads = (('packages/e', 'spec/', 'tests/read.rs', 'spec'),
                 ('packages/f', 'packages/g/contracts.yaml', 'tests/read.rs', 'contracts'))
        cases = [('spec/x.yaml', {'packages/e'}, True),
                 ('packages/g/contracts.yaml', {'packages/f', 'packages/g'}, False),
                 ('packages/g/README.md', {'packages/g'}, False),
                 ('specs/x.yaml', set(), False), ('engine/x', {'packages/h'}, True),
                 ('conformance/x', set(), True), ('packages/agent-claude-code/x', set(), True),
                 ('packages/agent-codex/x', set(), True), ('packages/provider-file/x', set(), True),
                 ('packages/github/x', set(), True),
                 ('packages/git/x', set(), True),
                 ('packages/scenario-s2-linear/x', set(), True),
                 ('packages/scenario-s2-linear-restrictions/x', set(), True),
                 ('experiments/013-s2-shared-login/linear-stub/x', set(), True),
                 ('packages/provider-local-process/x', set(), True),
                 ('packages/runtime-provider-kit/x', set(), True),
                 ('packages/scenario-s4-effect/x', set(), True),
                 ('docs/x.md', set(), False),
                 ('tools/x.py', set(), False)]
        with mock.patch.object(gate, 'RUST_READS', reads):
            for path, expected, engine in cases:
                with self.subTest(path=path):
                    self.write(path, 'changed\n')
                    self.assertEqual(self.invoke()[0], 0)
                    rows = self.record()['gates']
                    chosen = {row['directory'] for row in rows if row['name'].startswith('rust packages/')}
                    self.assertEqual(chosen, expected)
                    self.assertEqual(any(row['name'] == 'engine test' for row in rows), engine)
                    if path == 'spec/x.yaml':
                        self.assertEqual(self.row('rust packages/e test')['reason'],
                                         'it reads spec/ (tests/read.rs), which the branch changes')
                    if path == 'packages/g/contracts.yaml':
                        self.assertEqual(self.row('rust packages/f test')['reason'],
                                         'it reads packages/g/contracts.yaml (tests/read.rs), which the branch changes')
                    if path == 'engine/x':
                        self.assertEqual(self.row('rust packages/h test')['reason'],
                                         'it depends by path on engine, which the branch changes')
                    if engine:
                        prefix = next(prefix for prefix in gate.ENGINE_PREFIXES if path.startswith(prefix))
                        self.assertEqual(self.row('engine test')['reason'],
                                         'the branch changes 1 paths under ' + prefix)
                    else:
                        self.assertEqual(self.row('engine')['why'], gate.ENGINE_WHY)
                        self.assertIsNone(self.row('engine')['command'])
                        self.assertIsNone(self.row('engine')['directory'])
                    if not expected:
                        self.assertEqual(self.row('rust unreached')['why'], gate.UNREACHED_WHY
                                         + 'packages/e, packages/f, packages/g, packages/h')
                (self.root / path).unlink()
            self.assertEqual(self.invoke('--all')[0], 0)
            rows = [row for row in self.record()['gates'] if row['name'].startswith('rust packages/')
                    or row['name'].startswith('engine ')]
            self.assertEqual(len(rows), 15)
            self.assertTrue(all(row['reason'] == '--all was given' for row in rows))
            self.assertNotIn('rust unreached', [row['name'] for row in self.record()['gates']])
            self.fake.dependencies['engine'] = [('packages/e', 'build')]
            self.write('packages/e/src/lib.rs', '// changed\n')
            self.assertEqual(self.invoke()[0], 0)
            self.assertEqual(self.row('engine test')['reason'],
                             'it depends by path on packages/e, which the branch changes')

    def test_each_kind_of_directory_gets_ci_commands_and_environment(self):
        self.crates('packages/p', 'experiments/e', 'experiments/005-macos-keychain', 'packages/skipped',
                    not_built=('experiments/005-macos-keychain', 'packages/skipped'))
        planted = {key: 'planted-value-' + str(index) for index, key in enumerate(
            ('CONDUIT_TEST_NPM', 'SCR_AGENT_CODEX', 'RUSTUP_TOOLCHAIN', 'RUSTFLAGS', 'EXAMPLE_API_KEY'))}
        with mock.patch.dict(os.environ, planted):
            code, output, error = self.invoke('--all', '--jobs', '5', '--test-threads', '3')
        self.assertEqual(code, 0)
        cargo_calls = [call for call in self.fake.calls if call['args'][0] == 'cargo']
        checks = [call for call in cargo_calls if call['args'][1] not in ('metadata', 'locate-project')]
        triple = [['cargo', 'fmt', '--check'], ['cargo', 'clippy', '--all-targets', '--', '-D', 'warnings'],
                  ['cargo', 'test', '--release']]
        for directory, commands in (('packages/p', triple), ('experiments/e', [triple[-1]]),
                                    ('experiments/005-macos-keychain', []),
                                    ('packages/skipped', [triple[0]]), ('engine', triple)):
            self.assertEqual([call['args'] for call in checks if call['cwd'] == self.root / directory], commands)
        self.assertTrue(all(call['timeout'] == 5400 for call in checks))
        for call in cargo_calls:
            for name, value in dict(CARGO_NET_OFFLINE='true', RUSTUP_AUTO_INSTALL='0',
                                    CARGO_TERM_COLOR='never', CARGO_BUILD_JOBS='5').items():
                self.assertEqual(call['env'][name], value)
            if call['args'][1] == 'test':
                self.assertEqual(call['env']['RUST_TEST_THREADS'], '3')
            self.assertFalse(planted.keys() & call['env'].keys())
        self.assertEqual(self.record()['settings'], dict(jobs=5, test_threads=3, all=True, target_cache=None))
        self.assertEqual(self.record()['environment']['set'],
                         dict(CARGO_NET_OFFLINE='true', RUSTUP_AUTO_INSTALL='0', CARGO_TERM_COLOR='never',
                              CARGO_BUILD_JOBS='5', RUST_TEST_THREADS='3'))
        self.assert_private(output + error + self.logs() + self.record_path().read_text(), *planted.values())
        self.assertIn('RUST_TEST_THREADS=3 cargo test --release in packages/p', output)
        for name in ('rust experiments/005-macos-keychain', 'rust packages/skipped'):
            self.assertEqual(self.row(name)['result'], 'NOT RUN')
            self.assertFalse(self.row(name)['required'])
        for row in self.record()['gates']:
            if row['name'].startswith(('rust packages/', 'rust experiments/', 'engine ')) and row['command']:
                self.assertTrue(row['required'])
        # Names in cargo's absolute paths must be parsed before redaction, and never escape.
        names = self.names_file(self.parent.name + ',packages/p')
        self.fake.dependencies['packages/p'] = [('experiments/e', None)]
        code, output, error = self.invoke('--all', '--private-names-file', str(names))
        self.assertEqual(code, 0)
        self.assertIn('rust [redacted] test', [row['name'] for row in self.record()['gates']])
        self.assertEqual(self.row('rust [redacted] test')['directory'], '[redacted]')
        self.assert_private(output + error + self.logs() + self.record_path().read_text(),
                            self.parent.name, 'packages/p', names)
        # A failed command never skips the next check, and cargo timeouts use the runner's limit.
        self.fake.responses['clippy'] = gate.CommandResult(4, 'failed clippy\n')
        self.fake.responses['test'] = gate.CommandResult(-15, 'timeout\n', True)
        code, output, _ = self.invoke('--all')
        self.assertEqual(code, 1)
        self.assertEqual(self.row('rust packages/p clippy')['result'], 'FAIL')
        self.assertEqual(self.row('engine test')['why'], gate.timeout_words(90))
        self.assertIn('ended after 90 minutes, with its process group', output)
        for args in (('--jobs', '0'), ('--test-threads', 'x')):
            with self.subTest(args=args), self.assertRaises(SystemExit) as stopped:
                self.invoke(*args)
            self.assertEqual(stopped.exception.code, 2)

    def test_target_cache_commands_and_redaction(self):
        self.crates('packages/a')
        cache = self.parent / 'planted-cache-name'
        names = self.names_file(cache.name)
        options = ('--all', '--target-cache', str(cache), '--private-names-file', str(names))
        code, output, _ = self.invoke('--plan', *options)
        self.assertEqual(code, 0)
        self.assertFalse(cache.exists())
        self.assertFalse(self.record_path().exists())
        self.assertIn('WOULD RUN rust packages/a clippy:', output)
        self.assertIn('<target-cache>/packages-a', output)
        self.assertIn('<target-cache>/engine', output)
        lock = cache / 'gate.lock'
        observed = []

        def inspect(call):
            if call['args'][1] in ('clippy', 'test'):
                observed.append(json.loads(lock.read_text()))
        self.fake.inspect = inspect
        self.fake.responses['test'] = gate.CommandResult(0, f'checked in {cache}\n')
        self.fake.calls.clear()
        code, output, error = self.invoke(*options)
        self.assertEqual(code, 0)
        self.assertFalse(lock.exists())
        self.assertEqual(len(observed), 4)
        self.assertEqual(observed, [dict(pid=os.getpid(), head=self.record()['head'],
                                        started=self.record()['started'])] * 4)
        for directory, prefix in (('packages/a', 'rust packages/a'), ('engine', 'engine')):
            for subcommand in ('fmt', 'clippy', 'test'):
                call = next(call for call in self.fake.calls
                            if call['cwd'] == self.root / directory and call['args'][1] == subcommand)
                command = self.row(f'{prefix} {subcommand}')['command']
                if directory == 'packages/a' and subcommand == 'test':
                    self.assertEqual(call['args'], ['cargo', 'test', '--release'])
                    self.assertEqual(call['env']['CARGO_TARGET_DIR'], str(cache / 'packages-a'))
                    self.assertIn("CARGO_TARGET_DIR='<target-cache>/packages-a'", command)
                elif subcommand == 'fmt':
                    self.assertNotIn('CARGO_TARGET_DIR', call['env'])
                    self.assertEqual(call['args'], ['cargo', 'fmt', '--check'])
                    self.assertNotIn('--target-dir', command)
                else:
                    self.assertNotIn('CARGO_TARGET_DIR', call['env'])
                    name = directory.replace('/', '-')
                    self.assertEqual(call['args'][2:4], ['--target-dir', str(cache / name)])
                    self.assertIn(f'<target-cache>/{name}', command)
        shown = str(cache.parent / '[redacted]')
        self.assertEqual(output.count(shown), 1)
        self.assertEqual(self.record()['settings']['target_cache'], shown)
        self.assertIn('checked in <target-cache>', self.logs())
        self.assert_private(output + error + self.record_path().read_text() + self.logs(), cache)

    def test_target_cache_resolves_metadata_for_each_directories_host(self):
        self.crates('packages/a')
        cache = self.parent / 'cache'
        hosts = {'packages/a': 'aarch64-apple-darwin', 'engine': 'x86_64-unknown-linux-gnu'}

        def inspect(call):
            if call['args'] == ['rustc', '-vV']:
                directory = call['cwd'].relative_to(self.root).as_posix()
                self.fake.responses['-vV'] = gate.CommandResult(0, f'host: {hosts[directory]}\n')
        self.fake.inspect = inspect
        self.assertEqual(self.invoke('--all', '--target-cache', str(cache))[0], 0)
        for directory, host in hosts.items():
            probes = [call['args'] for call in self.fake.calls
                      if call['cwd'] == self.root / directory
                      and call['args'][1] in ('-vV', 'metadata')]
            self.assertEqual(probes, [
                ['rustc', '-vV'],
                ['cargo', 'metadata', '--format-version', '1', '--offline',
                 '--filter-platform', host],
            ])

    def test_target_cache_fails_closed_when_host_is_unknown(self):
        self.crates('packages/a')
        self.write('packages/a/README.md', 'changed\n')
        cache = self.parent / 'cache'
        for host_result in (gate.CommandResult(1, 'rustc unavailable\n'),
                            gate.CommandResult(0, 'no host line\n'),
                            gate.CommandResult(0, 'host: a-b-c\nhost: d-e-f\n'),
                            gate.CommandResult(1, 'rustc timed out\n', timed_out=True)):
            with self.subTest(host_result=host_result):
                self.fake.calls.clear()
                self.fake.responses['-vV'] = host_result
                self.assertEqual(self.invoke('--target-cache', str(cache))[0], 1)
                for verb in ('clippy', 'test'):
                    row = self.row('rust packages/a ' + verb)
                    self.assertEqual(row['result'], 'FAIL')
                    self.assertIn('cached Rust host:', row['why'])
                package_calls = [call['args'][1:] for call in self.fake.calls
                                 if call['cwd'] == self.root / 'packages/a']
                self.assertEqual(package_calls, [['fmt', '--check'], ['-vV'], ['-vV']])
                self.assertFalse((cache / 'gate.lock').exists())

    def test_without_target_cache_keeps_rust_commands_and_environment(self):
        self.crates('packages/a')
        with mock.patch.dict(os.environ, CARGO_TARGET_DIR='planted-target'):
            self.assertEqual(self.invoke('--all')[0], 0)
        self.assertFalse(any(call['args'][1] == '-vV' or '--filter-platform' in call['args']
                             for call in self.fake.calls))
        tests = [call for call in self.fake.calls if call['args'][1] == 'test']
        self.assertEqual(len(tests), 2)
        for call in tests:
            self.assertEqual(call['args'], ['cargo', 'test', '--release'])
            self.assertEqual(call['env']['CARGO_TARGET_DIR'], 'planted-target')

    def test_target_cache_invalidates_transitive_path_fingerprints_once(self):
        self.crates('packages/a', 'packages/b', 'packages/c')
        self.write('packages/a/README.md', 'changed\n')
        self.fake.dependencies.update({'packages/a': [('packages/b', None)],
                                       'packages/b': [('packages/c', None)]})
        self.fake.package_names['packages/c'] = ['c', 'c-member']
        self.fake.resolved_packages['packages/a'] = [
            dict(name=name, source=None) for name in ('a', 'b', 'c', 'c-member')]
        cache = self.parent / 'cache'
        stale, kept = [], []
        for root in (cache / 'packages-a', cache / 'packages-a' / 'aarch64-apple-darwin'):
            for profile in ('debug', 'release'):
                fingerprints = root / profile / '.fingerprint'
                for name in ('a-123', 'b-123', 'c-123', 'c-member-123', 'a-registry-456', 'registry-456'):
                    path = fingerprints / name
                    path.mkdir(parents=True)
                    (stale if name.endswith('123') else kept).append(path)

        def inspect(call):
            if call['args'][1] == 'clippy':
                self.assertFalse(any(path.exists() for path in stale))
                stale[0].mkdir()
            if call['args'][1] in ('clippy', 'test'):
                self.assertTrue(all(path.is_dir() for path in kept))
        self.fake.inspect = inspect
        self.assertEqual(self.invoke('--target-cache', str(cache))[0], 0)
        self.assertTrue(stale[0].is_dir())
        self.assertFalse(any(path.exists() for path in stale[1:]))

    def test_target_cache_invalidates_crates_outside_workspace_membership(self):
        self.crates('packages/a')
        self.write('packages/a/README.md', 'changed\n')
        # The engine workspace metadata lists neither excluded crate. A direct edge names
        # probe-b, but only the package's resolved metadata contains its dependency probe-c.
        self.fake.dependencies['packages/a'] = [('engine/probe-b', None)]
        self.fake.resolved_packages['packages/a'] = [
            dict(name=name, source=None) for name in ('a', 'probe-b', 'probe-c')]
        self.fake.resolved_packages['packages/a'].append(
            dict(name='registry-only', source='registry+https://example.invalid/index'))
        cache = self.parent / 'cache'
        stale, kept = [], []
        for profile in ('debug', 'release'):
            fingerprints = cache / 'packages-a' / profile / '.fingerprint'
            stale.append(fingerprints / 'probe-c-123')
            kept.append(fingerprints / 'registry-only-456')
            stale[-1].mkdir(parents=True)
            kept[-1].mkdir()

        def inspect(call):
            if call['args'][1] == 'clippy':
                self.assertFalse(any(path.exists() for path in stale))
                self.assertTrue(all(path.is_dir() for path in kept))
        self.fake.inspect = inspect
        self.assertEqual(self.invoke('--target-cache', str(cache))[0], 0)
        resolved = [call['args'] for call in self.fake.calls
                    if call['args'][1] == 'metadata' and '--no-deps' not in call['args']]
        self.assertEqual(len(resolved), 1)
        self.assertIn('--offline', resolved[0])

    def test_target_cache_interrupted_before_lock_publication(self):
        cache = self.parent / 'cache'
        with contextlib.ExitStack() as cleanup:
            with mock.patch.object(gate.os, 'fstat', side_effect=KeyboardInterrupt):
                with self.assertRaises(KeyboardInterrupt):
                    gate.claim_target_cache(cache, 'cache', self.base, 'start', cleanup)
        self.assertEqual(list(cache.iterdir()), [])

        link = os.link

        def interrupt_after_link(source, destination):
            link(source, destination)
            raise KeyboardInterrupt

        with contextlib.ExitStack() as cleanup:
            with mock.patch.object(gate.os, 'link', side_effect=interrupt_after_link):
                with self.assertRaises(KeyboardInterrupt):
                    gate.claim_target_cache(cache, 'cache', self.base, 'start', cleanup)
        self.assertEqual(list(cache.iterdir()), [])

    def test_target_cache_sigterm_ends_the_child_group_and_releases_lock(self):
        cache = self.parent / 'cache'
        ready = self.root / 'ready'
        self.write('tools/check_docs.py',
                   'import os, signal, subprocess, sys, time\n'
                   'from pathlib import Path\n'
                   'child = subprocess.Popen([sys.executable, "-c", "import time; time.sleep(60)"])\n'
                   'def stop(sig, frame):\n'
                   '    child.wait()\n'
                   '    sys.exit(0)\n'
                   'signal.signal(signal.SIGTERM, stop)\n'
                   'Path("ready").write_text(f"{os.getpid()} {child.pid}\\n")\n'
                   'time.sleep(60)\n')
        script = (
            'import signal, sys\n'
            'sys.path.insert(0, sys.argv[1])\n'
            'import gate\n'
            'gate.shutil.which = lambda name: None\n'
            'previous = signal.getsignal(signal.SIGTERM)\n'
            'status = gate.main(sys.argv[2], ["--base", "base", "--target-cache", sys.argv[3]])\n'
            'assert signal.getsignal(signal.SIGTERM) == previous\n'
            'sys.exit(status)\n'
        )
        with subprocess.Popen([sys.executable, '-c', script, str(REPOSITORY / 'tools'),
                               str(self.root), str(cache)], cwd=self.root,
                              stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
                              env=dict(os.environ, PYTHONDONTWRITEBYTECODE='1')) as process:
            pids = []
            try:
                deadline = time.monotonic() + 10
                while len(pids) != 2 and process.poll() is None and time.monotonic() < deadline:
                    if ready.exists():
                        pids = [int(pid) for pid in ready.read_text().split()]
                    if len(pids) != 2:
                        time.sleep(0.02)
                self.assertEqual(len(pids), 2)
                self.assertTrue((cache / 'gate.lock').is_file())
                process.terminate()
                output, _ = process.communicate(timeout=20)
                self.assertEqual(process.returncode, 130)
                self.assertIn('Gate: interrupted during docs', output)
                self.assertFalse((cache / 'gate.lock').exists())
                for pid in pids:
                    with self.assertRaises(ProcessLookupError):
                        os.kill(pid, 0)
                pids.clear()
            finally:
                if pids:
                    try:
                        os.killpg(pids[0], signal.SIGTERM)
                    except ProcessLookupError:
                        pass
                if process.poll() is None:
                    process.kill()
                process.communicate(timeout=5)

    def test_set_names_excludes_target_only_within_repository(self):
        root = self.parent / 'target' / 'repository'
        source = root / 'engine' / 'source.rs'
        build = root / 'engine' / 'target' / 'generated.rs'
        for path in (source, build):
            path.parent.mkdir(parents=True, exist_ok=True)
            path.touch()
        with mock.patch.object(set_names, 'ROOT', root):
            self.assertEqual(set_names.files(['engine']), [source])

    def test_target_cache_refuses_a_held_lock(self):
        cache = self.parent / 'cache'
        cache.mkdir()
        lock = cache / 'gate.lock'
        text = json.dumps(dict(pid=os.getpid()))
        lock.write_text(text)
        code, output, _ = self.invoke('--target-cache', str(cache))
        self.assertEqual(code, 2)
        self.assertIn('Gate: NOT RUN:', output)
        self.assertIn(f'{lock} exists for PID {os.getpid()}, which is running', output)
        self.assertIn('Remove the lock if no gate is running', output)
        with mock.patch.object(gate.os, 'kill', side_effect=ProcessLookupError):
            code, output, _ = self.invoke('--target-cache', str(cache))
        self.assertEqual(code, 2)
        self.assertIn('which is not running', output)
        self.assertEqual(lock.read_text(), text)
        self.assertEqual(list(cache.iterdir()), [lock])
        self.assertEqual(self.fake.calls, [])

    def test_target_cache_releases_lock_after_failure_interrupt_or_error(self):
        cache = self.parent / 'cache'
        lock = cache / 'gate.lock'
        self.fake.responses['tools/check_docs.py'] = gate.CommandResult(1, 'failed\n')
        self.assertEqual(self.invoke('--target-cache', str(cache))[0], 1)
        self.assertFalse(lock.exists())

        for fault, exit_code in ((KeyboardInterrupt, 130), (RuntimeError, None)):
            with self.subTest(fault=fault):
                def inspect(call):
                    self.assertTrue(lock.exists())
                    raise fault
                self.fake.inspect = inspect
                if exit_code is None:
                    with self.assertRaises(RuntimeError):
                        self.invoke('--target-cache', str(cache))
                else:
                    self.assertEqual(self.invoke('--target-cache', str(cache))[0], exit_code)
                self.assertFalse(lock.exists())

    def test_target_cache_rejects_invalid_paths_and_colliding_names(self):
        inside = self.root / 'target' / 'cache'
        file = self.parent / 'file'
        file.touch()
        for path, reason in ((inside, "inside the gate's root"), (file, 'is not a directory')):
            for options in ((), ('--plan',)):
                with self.subTest(path=path, options=options):
                    code, output, _ = self.invoke('--target-cache', str(path), *options)
                    self.assertEqual(code, 2)
                    self.assertIn(reason, output)
        self.assertEqual(self.fake.calls, [])
        self.assertFalse(inside.exists())
        self.crates('packages/a/b', 'packages/a-b')
        cache = self.parent / 'cache'
        code, output, _ = self.invoke('--target-cache', str(cache))
        self.assertEqual(code, 1)
        self.assertIn('FAIL rust discovery', output)
        self.assertIn('target-cache name packages-a-b matches both packages/a-b and packages/a/b', output)
        self.assertFalse((cache / 'gate.lock').exists())

    def test_tool_tests_run_for_their_inputs_and_all(self):
        for path, reached in (('tools/x.py', True), ('.github/workflows/other.yml', True),
                              ('packages/p/src/x.rs', True), ('packages/p/Cargo.toml', True),
                              ('packages/p/Cargo.lock', True), ('experiments/e/src/x.rs', True),
                              ('experiments/e/Cargo.toml', True), ('experiments/e/Cargo.lock', True),
                              ('docs/x.md', False), ('packages/p/README.md', False),
                              ('engine/src/x.rs', False), ('spec/x.yaml', False), (None, True)):
            with self.subTest(path=path):
                changed = self.write(path, 'changed\n') if path else None
                self.fake.calls.clear()
                code, output, _ = self.invoke(*(() if path else ('--all',)))
                self.assertEqual(code, 0)
                row = self.row('tool tests')
                self.assertEqual(row['required'], reached)
                self.assertEqual(row['result'], 'PASS' if reached else 'NOT RUN')
                self.assertEqual(row['command'], 'python3 -m unittest discover -s tools/tests')
                self.assertEqual(any(call['args'][1:3] == ['-m', 'unittest']
                                     for call in self.fake.calls), reached)
                if not reached:
                    self.assertEqual(row['why'], gate.TOOL_TESTS_WHY)
                    self.assertIn(gate.PYTHON_REASON, output)
                    self.assertIn('tool tests', output.splitlines()[-1].split('not run: ')[1])
                if changed:
                    changed.unlink()

    def test_rust_own_paths_only_reports_the_dependency_and_read_routes(self):
        spike = 'experiments/011-short-form-spike'
        name = 'rust ' + spike
        self.crates(spike, 'experiments/other')
        self.fake.dependencies[spike] = [('engine/crates/core', None)]
        for path, reason in (('engine/README.md', 'depends by path on engine'),
                             ('spec/x.yaml', 'reads spec/'),
                             ('conformance/fixtures/s1/x.json', 'reads conformance/fixtures/s1/')):
            with self.subTest(path=path):
                changed = self.write(path, 'changed\n')
                self.assertEqual(self.invoke()[0], 0)
                row = self.row(name)
                self.assertEqual(row['result'], 'NOT RUN')
                self.assertFalse(row['required'])
                self.assertIn(reason, row['why'])
                self.assertIn(gate.OWN_PATHS_ONLY[spike], row['why'])
                self.assertNotIn(spike, self.row('rust unreached')['why'])
                changed.unlink()
        self.assertEqual(self.invoke('--all')[0], 0)
        self.assertEqual(self.row(name + ' test')['result'], 'PASS')
        self.write(spike + '/README.md', 'its own change\n')
        self.assertEqual(self.invoke()[0], 0)
        self.assertEqual(self.row(name + ' test')['result'], 'PASS')

    def test_reads_table_agrees_with_the_tree(self):
        tracked = set(subprocess.run(['git', 'ls-files', '-z'], cwd=REPOSITORY, check=True,
                                    stdout=subprocess.PIPE, stderr=subprocess.PIPE).stdout.decode().split('\0'))
        directories = {str(Path(path).parent) for path in tracked if path.endswith('/Cargo.toml')
                       and path.split('/')[0] in ('packages', 'experiments')}
        self.assertLessEqual(gate.OWN_PATHS_ONLY.keys(), directories)
        for directory, path, evidence, text in gate.RUST_READS:
            with self.subTest(directory=directory, path=path):
                self.assertIn(directory, directories)
                source = directory + '/' + evidence
                self.assertIn(source, tracked)
                self.assertIn(text, (REPOSITORY / source).read_text())
        for directory in sorted(directories):
            sources = [path for path in tracked if path.startswith(directory + '/') and path.endswith('.rs')]
            text = '\n'.join((REPOSITORY / source).read_text() for source in sources)
            reads = [path for reader, path, _, _ in gate.RUST_READS if reader == directory]
            if '"../../spec' in text or '.join("spec' in text:
                self.assertIn('spec/', reads, directory)
            if '"../../conformance' in text or '.join("conformance' in text:
                self.assertTrue(any(path.startswith('conformance/') for path in reads), directory)
            for sibling in directories - {directory}:
                if Path(sibling).parent == Path(directory).parent and '"../' + Path(sibling).name + '/' in text:
                    self.assertTrue(any(path.startswith(sibling + '/') for path in reads), (directory, sibling))
        self.assertIn(('packages/provider-local-process', 'tools/check_schemas.py', 'tests/package.rs',
                       'import check_schemas'), gate.RUST_READS)

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

        def run_quickly(mode, timeout, label):
            with mock.patch.object(gate, 'GRACE', 1), mock.patch.object(gate, 'PIPE_WAIT', 1):
                return run(mode, timeout, label)

        result, elapsed, started_pids = run_quickly('trap', 1, 'trap')
        self.assertLess(elapsed, 8)
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
                result, elapsed, started_pids = run_quickly('plain', 0.5, 'plain-' + str(attempt))
                self.assertTrue(result.timed_out)
                self.assertLess(elapsed, 8)
                self.assertTrue(gone(started_pids))

        # A child that left the group and holds the output pipe does not hold the result.
        result, elapsed, started_pids = run_quickly('escape', 1, 'escape')
        self.assertLess(elapsed, 8)
        self.assertTrue(result.timed_out)
        self.assertIn('before sleeping', log.read_text())
        self.assertTrue(gone(started_pids[:1]))

        # The timeout counts the leader alone: a leader that exits 0 at once is not a timeout.
        result, elapsed, started_pids = run_quickly('detach', 3, 'detach')
        self.assertLess(elapsed, 8)
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
