#!/usr/bin/env python3
"""Run the local Python, commit identity and secret gates without fetching anything."""

import argparse
from dataclasses import dataclass
from datetime import datetime, timezone
import importlib.util
import json
import os
from pathlib import Path
import platform
import re
import select
import shlex
import shutil
import signal
import stat
import subprocess
import sys
import tempfile
import threading
import time

WORKFLOW = Path('.github/workflows/checks.yml')
OUTPUT = Path('target/gate')
CONFIG = '.gitleaks.toml'
TOOL = 'tools/gate.py'
VECTOR = 'tools/vector_delta.py'
FALLBACK = '/usr/bin/python3'
IDENTITY = ('Conduit work order', 'work-orders@conduit.invalid')
# Direct queries ignore refs/replace, which every worktree shares: git push sends the original.
GIT = ('git', '--no-replace-objects')
IDENTITY_FORMAT = '--format=%H%x00%an%x00%ae%x00%cn%x00%ce'
COMMIT_HASH = re.compile(rb'(?:[0-9a-f]{40}|[0-9a-f]{64})')
GRACE = 10
PIPE_WAIT = 10
NAMED = frozenset(('CONDUIT_TEST_NPM', 'SCR_AGENT_CODEX', 'RUSTUP_TOOLCHAIN',
                   'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'RUSTDOCFLAGS',
                   'CONDUIT_PRIVATE_NAMES'))
CREDENTIAL = re.compile('KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL')
PYTHON_GATES = (
    ('docs', ('tools/check_docs.py',)),
    ('names', ('tools/set_names.py', '--check')),
    ('schemas', ('tools/check_schemas.py',)),
    ('roadmap', ('tools/check_roadmap.py',)),
    ('rulings', ('tools/check_rulings.py',)),
    ('model account', ('tools/check_model_account.py', '--history')),
    ('promise standing', ('tools/promise_standing.py', '--check')),
    ('private paths', ('tools/check_private_paths.py',)),
    ('tool tests', ('-m', 'unittest', 'discover', '-s', 'tools/tests')),
)
PYTHON_REASON = "CI's docs job runs it on each pull request"
RUST_WHY = ('this version of the gate runs no Rust directory and not the engine; '
            'order 193b adds them, and until then their checks run by hand, '
            'as the method as run says')
NO_COMMITS = 'the branch adds no commit since the merge base'
PRIVATE_FILE = 'the file that --private-names-file names'


@dataclass
class CommandResult:
    exit: object
    output: str = ''
    timed_out: bool = False


def end_group(process):
    """End the session's group, including children surviving its leader."""
    def send(sig):
        try:
            os.killpg(process.pid, sig)
            return True
        except (ProcessLookupError, PermissionError):
            # macOS answers EPERM, not ESRCH, for a group whose only members are zombies,
            # such as a leader that died on SIGTERM and is not yet reaped.
            return False

    if send(signal.SIGTERM):
        deadline = time.monotonic() + GRACE
        while time.monotonic() < deadline:
            process.poll()  # Reap the leader before testing whether the group still exists.
            if not send(0):
                break
            time.sleep(0.05)
        send(signal.SIGKILL)
    process.wait()


def read_output(stream, buffer, stop):
    """Reader thread: append the pipe's bytes to buffer until end of file or stop."""
    descriptor = stream.fileno()
    while not stop.is_set():
        ready, _, _ = select.select([descriptor], [], [], 0.1)
        if ready:
            chunk = os.read(descriptor, 65536)
            if not chunk:
                return
            buffer.extend(chunk)


def redact_output(text, names=None):
    """Replace configured names literally, including overlapping names, in one pass."""
    if not names:
        return text
    pattern = '|'.join(re.escape(name) for name in sorted(names.split(','), key=len, reverse=True))
    return re.sub(pattern, '[redacted]', text)


def run_command(args, *, cwd, env, timeout, log, redact=redact_output):
    """Runner: argv, cwd/env/timeout/log/redact -> CommandResult; may interrupt.

    The timeout counts the leader alone. A reader thread collects the output, and a process
    that left the group and still holds the pipe delays the result by PIPE_WAIT at most.
    Output stays in memory until redacted, before any log is written.
    """
    buffer = bytearray()
    timed_out = False
    process = None
    try:
        try:
            process = subprocess.Popen(args, cwd=cwd, env=env, stdin=subprocess.DEVNULL,
                                       stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                       start_new_session=True)
        except OSError:
            buffer.extend(b'could not start the command\n')
        if process is not None:
            stop = threading.Event()
            reader = threading.Thread(target=read_output, args=(process.stdout, buffer, stop),
                                      daemon=True)
            try:
                reader.start()
                try:
                    process.wait(timeout=timeout)
                except subprocess.TimeoutExpired:
                    timed_out = True
                    end_group(process)
                # Inside the try: an interrupt during this wait still ends the group (decision 8).
                reader.join(PIPE_WAIT)
            except KeyboardInterrupt:
                end_group(process)
                if reader.ident is not None:
                    reader.join(PIPE_WAIT)
                raise
            finally:
                stop.set()
                if reader.ident is not None:
                    reader.join()
                process.stdout.close()
    finally:
        output = redact(bytes(buffer).decode('utf-8', errors='replace'))
        Path(log).write_text(output, encoding='utf-8')
    return CommandResult(process.returncode if process else 127,
                         output, timed_out)


def scrub_environment():
    removed = sorted(NAMED.intersection(os.environ), key=os.fsencode)
    credentials = [key for key in os.environ if CREDENTIAL.search(key)]
    clean = {key: value for key, value in os.environ.items()
             if key not in NAMED and not CREDENTIAL.search(key)}
    return clean, removed, len(credentials)


def git(root, env, *args):
    return subprocess.run([*GIT, *args], cwd=root, env=env, stdin=subprocess.DEVNULL,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE)


def changed_paths(root, merge_base, env):
    tracked = git(root, env, 'diff', '--name-only', '--no-renames', '-z', merge_base).stdout
    untracked = git(root, env, 'ls-files', '--others', '--exclude-standard', '-z').stdout
    return sorted(set(filter(None, (tracked + untracked).split(b'\0'))))


def read_private_names(filename, root):
    try:
        info = os.lstat(filename)
    except OSError:
        return None, 'does not exist or cannot be read'
    if not stat.S_ISREG(info.st_mode):
        return None, ('is not a regular file; a symbolic link, a directory '
                      'and any other kind are refused')
    mode = stat.S_IMODE(info.st_mode)
    if mode != 0o600:
        return None, f'has mode {mode:04o}, not 0600; run chmod 600 on it'
    path = Path(filename).resolve()
    if path == root or root in path.parents:
        return None, 'is inside the repository, where Git could commit it; keep it outside'
    try:
        descriptor = os.open(filename, os.O_RDONLY | os.O_NOFOLLOW)
        with os.fdopen(descriptor, 'rb') as stream:
            data = stream.read()
    except OSError:
        return None, 'does not exist or cannot be read'
    try:
        text = data.decode('utf-8')
    except UnicodeDecodeError:
        return None, 'is not UTF-8 text'
    names = list(dict.fromkeys(part.strip() for part in re.split(r'[,\r\n]', text)
                               if part.strip()))
    if not names:
        return None, 'holds no name'
    return ','.join(names), None


def packages_missing():
    return [label for module, label in (('yaml', 'PyYAML'), ('jsonschema', 'jsonschema'))
            if importlib.util.find_spec(module) is None]


def utc_now():
    return datetime.now(timezone.utc).isoformat()


def write_record(path, record):
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary = tempfile.mkstemp(dir=path.parent, prefix='.record-', suffix='.json')
    try:
        with os.fdopen(descriptor, 'w', encoding='utf-8') as stream:
            json.dump(record, stream, indent=2)
            stream.write('\n')
        os.replace(temporary, path)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def command_text(args):
    return shlex.join(args)


def timeout_words(minutes):
    return f'ended after {minutes} minutes, with its process group'


def gate_row(name, command, required=True, reason=None, why=None):
    return dict(name=name, directory='.' if command else None, command=command,
                required=required, result='NOT RUN', reason=reason, why=why,
                partial=None, exit=None, seconds=None, log=None)


def show(row, output='', findings=(), plan=False):
    name, command = row['name'], row['command']
    label = f'{name}: {command}' if command else name + ':'
    if row['result'] == 'NOT RUN':
        print(f"NOT RUN {label}; {row['why']}" if command else f"NOT RUN {label} {row['why']}")
        # A command's NOT RUN line uses a semicolon after its command, as in the order.
        return
    if plan:
        print(f"WOULD RUN {label}; because {row['reason']}")
        return
    label = f"{name} ({row['seconds']:.1f} s): {command}"
    line = f"{row['result']} {label}; because {row['reason']}"
    lines = []
    if row['result'] == 'FAIL':
        line += '; ' + row['why']
        lines = [line for line in output.splitlines() if line.strip()]
        if row['log']:
            if lines and not row['why'].startswith('ended after'):
                line += '; its last line: ' + lines[-1]
            line += '; log ' + row['log']
    if row['partial']:
        line += '; not checked: ' + row['partial']
    print(line)
    if row['result'] == 'FAIL':
        for tail in lines[-20:] if row['log'] else []:
            print('    ' + tail, file=sys.stderr)
        for finding in findings:
            print(finding, file=sys.stderr)


def main(root, argv, run=run_command):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--base', default='origin/main')
    parser.add_argument('--plan', action='store_true')
    parser.add_argument('--private-names-file')
    options = parser.parse_args(argv)
    root = Path(root).resolve()
    environment, removed, credentials = scrub_environment()

    def not_started(why):
        print('Gate: NOT RUN: ' + why + '. Nothing was checked.')
        return 2

    if not root.is_dir():
        return not_started("the gate's root is not a Git working tree, so there is no branch to compare")
    working = git(root, environment, 'rev-parse', '--is-inside-work-tree')
    if working.returncode or working.stdout.strip() != b'true':
        return not_started("the gate's root is not a Git working tree, so there is no branch to compare")
    resolved = git(root, environment, 'rev-parse', '--verify', '--quiet', options.base + '^{commit}')
    if resolved.returncode:
        return not_started(f'the base {options.base} does not resolve to a commit; '
                           'fetch it or pass --base with a commit')
    base = resolved.stdout.decode().strip()
    common = git(root, environment, 'merge-base', base, 'HEAD')
    if common.returncode:
        return not_started(f'the base {options.base} shares no commit with HEAD')
    merge_base = common.stdout.decode().strip()
    private_names = None
    if options.private_names_file is not None:
        private_names, fault = read_private_names(options.private_names_file, root)
        if fault:
            return not_started(PRIVATE_FILE + ' ' + fault)
    head = git(root, environment, 'rev-parse', 'HEAD').stdout.decode().strip()
    branch_result = git(root, environment, 'symbolic-ref', '--quiet', '--short', 'HEAD')
    branch = branch_result.stdout.decode().strip() if branch_result.returncode == 0 else None
    contains_base = git(root, environment, 'merge-base', '--is-ancestor', base, 'HEAD').returncode == 0
    behind = 0 if contains_base else int(git(root, environment, 'rev-list', '--count',
                                           'HEAD..' + base).stdout)
    changed = changed_paths(root, merge_base, environment)
    status = git(root, environment, '--no-optional-locks', 'status', '--porcelain', '-z',
                 '--no-renames', '--untracked-files=all').stdout
    entries = set(filter(None, status.split(b'\0')))
    dirty = len(entries)
    untracked = sum(entry.startswith(b'?? ') for entry in entries)
    missing = packages_missing()
    # Git queries take the raw refs; what is printed or recorded hides the configured names.
    shown_branch = redact_output(branch, private_names) if branch else None
    shown_base = redact_output(options.base, private_names)
    location = f' on {shown_branch}' if branch else ', detached'
    print(f'Gate: base {shown_base} at {base[:12]}, merge base {merge_base[:12]}, '
          f'head {head[:12]}{location}; nothing is fetched.')
    print(f'Tree: {dirty} uncommitted paths. The gates read the working tree; secrets reads commits only.'
          if dirty else 'Tree: clean.')
    if behind:
        print(f'Base: the branch lacks {behind} commits of {shown_base}. CI tests a pull request '
              'merged with them; the gate tests the branch alone.')
    print(f'Changed since the merge base: {len(changed)} paths.')
    if private_names is not None:
        print('Private names: read from the file that --private-names-file names, and given to '
              'tools/check_private_paths.py alone; neither they nor the file\'s path are printed or recorded.')
    else:
        line = ('Private names: no --private-names-file was given, so tools/check_private_paths.py '
                'checks its built-in patterns only.')
        if 'CONDUIT_PRIVATE_NAMES' in os.environ:
            line += ' CONDUIT_PRIVATE_NAMES in the environment is not used.'
        print(line)
    if missing:
        print(f"Python: {platform.python_version()} cannot import {' and '.join(missing)}; "
              'the checks that need them fail, or skip tests that CI runs.')
    else:
        print(f'Python: {platform.python_version()} imports PyYAML and jsonschema, as CI\'s docs job does.')
    print(f"Environment: removed from every command: {', '.join(removed) or 'no named variable'}, "
          f'and {credentials} variables named for a credential.')

    record = dict(format=1, tool=TOOL, started=utc_now(), finished=None, result='incomplete',
                  base=dict(ref=shown_base, commit=base), merge_base=merge_base, head=head,
                  branch=shown_branch, contains_base=contains_base, behind=behind,
                  tree=dict(clean=not dirty, uncommitted_paths=dirty), changed_paths=len(changed),
                  platform=platform.system() + ' ' + platform.machine(), python=platform.python_version(),
                  python_packages=not missing,
                  environment=dict(set={}, removed=removed, removed_credentials=credentials,
                                   private_names_configured=private_names is not None), gates=[])
    record_path = root / OUTPUT / (head + '.json')
    log_directory = root / OUTPUT / 'logs' / head
    if not options.plan:
        log_directory.mkdir(parents=True, exist_ok=True)
        for old in log_directory.glob('*.log'):
            old.unlink()
        write_record(record_path, record)
    current = 'identity'

    def finish(row, output='', findings=()):
        record['gates'].append(row)
        if not options.plan:
            write_record(record_path, record)
        show(row, output, (redact_output(item, private_names) for item in findings), options.plan)

    def execute(args, log, minutes, env=None, display=None):
        print(f'running {current}: {display or command_text(args)}', file=sys.stderr)
        return run(args, cwd=root, env=dict(environment if env is None else env),
                   timeout=minutes * 60, log=log,
                   redact=lambda text: redact_output(text, private_names))

    try:
        identity_args = ['log', '-z', IDENTITY_FORMAT, merge_base + '..HEAD']
        identity = gate_row('identity', command_text([*GIT, *identity_args]), required=head != merge_base)
        if head == merge_base:
            identity['why'] = NO_COMMITS
        elif options.plan:
            count = int(git(root, environment, 'rev-list', '--count', merge_base + '..HEAD').stdout)
            identity.update(result='PASS', reason=(f'the branch adds {count} commits, each to be by '
                            'Conduit work order <work-orders@conduit.invalid>'))
        else:
            started = time.monotonic()
            print('running identity: ' + identity['command'], file=sys.stderr)
            commits = git(root, environment, *identity_args)
            # With -z and the format, NUL ends each of a commit's five fields, so no tab, line end
            # or Unicode line separator in a name can move a field or split a commit.
            values = commits.stdout.split(b'\0')
            if values[-1:] == [b'']:
                values.pop()
            rows = [values[index:index + 5] for index in range(0, len(values), 5)]
            identity['reason'] = (f'the branch adds {len(rows)} commits, each to be by '
                                  'Conduit work order <work-orders@conduit.invalid>')
            failures = []
            invalid_hash = False
            expected = tuple(field.encode('utf-8') for field in IDENTITY)
            for fields in rows:
                if len(fields) != 5 or not COMMIT_HASH.fullmatch(fields[0]):
                    invalid_hash = True
                    continue
                commit = fields[0][:12].decode('ascii')
                author = tuple(fields[1:3]) != expected
                committer = tuple(fields[3:5]) != expected
                if author or committer:
                    role = 'author and committer' if author and committer else 'author' if author else 'committer'
                    failures.append(commit + f' ({role})')
            identity.update(result='PASS', exit=commits.returncode,
                            seconds=round(time.monotonic() - started, 1))
            if commits.returncode:
                identity.update(result='FAIL', why=f'git log exited {commits.returncode}')
            elif invalid_hash:
                identity.update(result='FAIL', why=('git log returned a row that is not a commit hash and four '
                                                    'identity fields; identities are not printed'))
            elif failures:
                identity.update(result='FAIL', why=(f'{len(failures)} of {len(rows)} commits are not by '
                                'Conduit work order <work-orders@conduit.invalid>: ' + ', '.join(failures)
                                + '; the identities found are not printed; squash again with that identity, '
                                "as the method as run's squash does"))
        finish(identity)

        commands = [(name, [sys.executable, *args], ['python3', *args]) for name, args in PYTHON_GATES]
        commands += [('whitespace', ['git', 'diff', '--check', merge_base],
                      ['git', 'diff', '--check', merge_base]),
                     ('vector changes', [sys.executable, VECTOR, merge_base,
                                         'WORKTREE', '--declared-changed'],
                      ['python3', VECTOR, merge_base, 'WORKTREE', '--declared-changed'])]
        for current, args, printed in commands:
            row = gate_row(current, command_text(printed), reason=PYTHON_REASON)
            if options.plan:
                row['result'] = 'PASS'  # Only show WOULD RUN; no check is claimed or recorded.
                finish(row)
                continue
            position = len(record['gates']) + 1
            log = log_directory / (f'{position:02d}-' + current.replace(' ', '-').replace('/', '-') + '.log')
            env = dict(environment)
            if current == 'private paths' and private_names is not None:
                env['CONDUIT_PRIVATE_NAMES'] = private_names
            started = time.monotonic()
            result = execute(args, log, 15, env, row['command'])
            row.update(result='FAIL' if result.timed_out or result.exit != 0 else 'PASS',
                       exit=result.exit, seconds=round(time.monotonic() - started, 1),
                       log=log.relative_to(root).as_posix())
            if row['result'] == 'FAIL':
                row['why'] = timeout_words(15) if result.timed_out else f'exit {result.exit}'
            if current in ('whitespace', 'private paths') and untracked:
                row['partial'] = f'{untracked} untracked path(s)'
            if current == 'docs':
                skipped = sum(line.startswith('Notice: skipped repository permalinks at')
                              for line in result.output.splitlines())
                if skipped:
                    row['partial'] = f'the permalinks at {skipped} commit(s) this clone lacks, which CI fetches'
            if current == 'tool tests':
                skipped = re.search(r'OK \(skipped=(\d+)\)\s*$', result.output)
                if skipped:
                    row['partial'] = f'{skipped[1]} test(s) unittest skipped'
            finish(row, result.output)

        current = 'secrets'
        try:
            workflow = (root / WORKFLOW).read_text(encoding='utf-8')
        except OSError:
            workflow = ''
        pin_match = re.search(r'^\s*scanner_version=([^\s]+)\s*$', workflow, re.MULTILINE)
        pin = pin_match[1] if pin_match else None
        scan_args = ['gitleaks', 'git', '.', '--config', CONFIG, '--redact', '--no-banner',
                     '--log-opts=--diff-merges=first-parent ' + merge_base + '..HEAD',
                     '--report-format', 'json', '--report-path', '<temporary>/findings.json']
        secrets = gate_row(current, command_text(scan_args), required=False)
        findings = []
        started = time.monotonic()
        with tempfile.TemporaryDirectory() as temporary:
            temp = Path(temporary)
            executable = shutil.which('gitleaks') if pin else None
            if pin is None:
                secrets['why'] = 'the pinned version could not be read from .github/workflows/checks.yml'
            elif executable is None:
                secrets['why'] = (f'gitleaks is not installed; the pin is {pin} '
                                  "(.github/workflows/checks.yml), and the pull request's secrets job "
                                  'scans these commits after the push')
            else:
                version = execute([executable, 'version'], temp / 'version.log', 10, display='gitleaks version')
                found = version.output.strip().lstrip('v')
                if version.exit != 0 or version.timed_out:
                    detail = timeout_words(10) if version.timed_out else f'exit {version.exit}'
                    secrets['why'] = f'gitleaks version did not answer ({detail}); the pin is {pin}'
                elif found != pin:
                    secrets['why'] = f'gitleaks {found} is installed, not the pinned {pin}'
                elif head == merge_base:
                    secrets['why'] = NO_COMMITS
                else:
                    secrets.update(required=True, reason=f'gitleaks {pin}, the pinned release, is installed',
                                   result='PASS')
                    if not options.plan:
                        actual = [executable, *scan_args[1:-1], str(temp / 'findings.json')]
                        result = execute(actual, temp / 'scan.log', 10, display=secrets['command'])
                        try:
                            report = json.loads((temp / 'findings.json').read_text(encoding='utf-8'))
                            if not isinstance(report, list):
                                raise ValueError('not a list')
                        except FileNotFoundError:
                            report = []
                        except (ValueError, OSError):
                            report = None
                        secrets.update(exit=result.exit, seconds=round(time.monotonic() - started, 1))
                        if result.timed_out:
                            secrets.update(result='FAIL', why=timeout_words(10))
                        elif report:
                            secrets.update(result='FAIL', why=f'{len(report)} findings; values are not printed')
                            findings = [f"{item.get('RuleID', '')} {item.get('File', '')}:"
                                        f"{item.get('StartLine', '')} in {str(item.get('Commit', ''))[:12]}"
                                        for item in report]
                        elif result.exit != 0 or report is None:
                            secrets.update(result='FAIL', why=(f'gitleaks exited {result.exit} without reporting '
                                           'a finding: the scan did not complete'))
                        if dirty:
                            secrets['partial'] = f'{dirty} uncommitted path(s), since gitleaks reads commits alone'
        finish(secrets, findings=findings)
        current = 'rust and engine'
        finish(gate_row(current, None, required=False, why=RUST_WHY))
    except KeyboardInterrupt:
        print(f'Gate: interrupted during {current}; its process group was ended, '
              'and the record says incomplete.')
        return 130

    if options.plan:
        would = sum(row['result'] != 'NOT RUN' for row in record['gates'])
        print(f"Gate plan: {would} gates would run, {len(record['gates']) - would} would not. "
              'Nothing was checked, and no record was written.')
        return 0
    failed = [row['name'] for row in record['gates'] if row['result'] == 'FAIL']
    required_missing = [row['name'] for row in record['gates']
                        if row['required'] and row['result'] == 'NOT RUN']
    not_run = [row['name'] for row in record['gates'] if row['result'] == 'NOT RUN']
    passed = sum(row['result'] == 'PASS' for row in record['gates'])
    record.update(result='FAIL' if failed or required_missing else 'PASS', finished=utc_now())
    write_record(record_path, record)
    relative_record = record_path.relative_to(root).as_posix()
    if record['result'] == 'PASS':
        line = f"Gate: PASS. {passed} passed; not run: {', '.join(not_run) or 'none'}. Record: {relative_record}"
    else:
        line = (f"Gate: FAIL. Failed: {', '.join(failed) or 'none'}; required, not run: "
                f"{', '.join(required_missing) or 'none'}; {passed} passed. Record: {relative_record}")
    if dirty:
        line += (f' The tree has {dirty} uncommitted paths: commit them and run the gate again '
                 'before citing the record.')
    if behind:
        line += (f' The branch lacks {behind} commits of {shown_base}: bring it up to date with '
                 "the method as run's guarded squash, not a plain rebase, and run the gate again.")
    print(line)
    return 0 if record['result'] == 'PASS' else 1


if __name__ == '__main__':
    if (packages_missing() and os.environ.get('CONDUIT_GATE_REEXEC') != '1'
            and os.path.exists(FALLBACK)
            and os.path.realpath(sys.executable) != os.path.realpath(FALLBACK)):
        os.environ['CONDUIT_GATE_REEXEC'] = '1'
        os.execv(FALLBACK, [FALLBACK, str(Path(__file__).resolve()), *sys.argv[1:]])
    sys.exit(main(Path(__file__).resolve().parents[1], sys.argv[1:]))
