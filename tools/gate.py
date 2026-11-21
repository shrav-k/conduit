#!/usr/bin/env python3
"""Run the local merge gates for what the branch changes, without fetching anything."""

import argparse
from contextlib import ExitStack
from dataclasses import dataclass
from datetime import datetime, timezone
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import platform
import posixpath
import re
import select
import shlex
import shutil
import signal
import socket
import stat
import subprocess
import sys
import tempfile
import threading
import time
from urllib.parse import urlsplit

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
DERIVE_TIMEOUT = 10
PUBLIC_WORDS = {
    'agentwatch': "ruling 68's record names it among ruling 1's reference clones",
    'localhost': 'the documents and examples use it, and phones and some machines report it as their host name',
    'pi-reference': "ruling 68's record names it among ruling 1's reference clones",
}
NAMED = frozenset(('CONDUIT_TEST_NPM', 'SCR_AGENT_CODEX', 'RUSTUP_TOOLCHAIN',
                   'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'RUSTDOCFLAGS',
                   'CONDUIT_PRIVATE_NAMES'))
CREDENTIAL = re.compile('KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL')
PYTHON_GATES = (
    ('docs', ('tools/check_docs.py',)),
    ('names', ('tools/set_names.py', '--check')),
    ('schemas', ('tools/check_schemas.py',)),
    ('roadmap', ('tools/check_roadmap.py',)),
    ('model account', ('tools/check_model_account.py', '--history')),
    ('promise standing', ('tools/promise_standing.py', '--check')),
    ('private paths', ('tools/check_private_paths.py',)),
    ('tool tests', ('-m', 'unittest', 'discover', '-s', 'tools/tests')),
)
PYTHON_REASON = "CI's docs job runs it on each pull request"
TOOL_TESTS_WHY = ('the branch changes nothing under tools/ or .github/workflows/, and no .rs, '
                  'Cargo.toml or Cargo.lock under packages/ or experiments/. ' + PYTHON_REASON)
RUST_WHY = 'rust discovery did not pass, so no Rust directory or engine gate was chosen'
ENGINE_ROOTS = ('engine/', 'spec/', 'conformance/')
UNREACHED_WHY = ('the branch changes nothing in these, in a directory they depend on by path, '
                 'or in a path they read: ')
NOT_BUILT_WHY = ('listed in macos_only of checks.yml, which CI does not build; '
                 'the gate does not run it either')
OWN_PATHS_ONLY = {
    'experiments/011-short-form-spike': ('A79 is finished and its result is recorded. '
                                        'Order 049a1 still runs its tests. '
                                        "CI's rust job tests it on each pull request"),
}
# directory, path read, evidence file within the directory, literal evidence in that file.
RUST_READS = (
    ('packages/git', 'spec/', 'tests/schema_drift.rs', '"../../spec'),
    ('packages/github', 'spec/', 'tests/schema_drift.rs', '"../../spec'),
    ('packages/provider-file', 'spec/', 'tests/schema_drift.rs', '"../../spec'),
    ('packages/provider-keychain', 'spec/', 'tests/schema_drift.rs', '"../../spec'),
    ('packages/provider-local-directory', 'spec/', 'tests/schema_drift.rs', '"../../spec'),
    ('packages/provider-local-machine', 'spec/', 'tests/schema_drift.rs', '"../../spec'),
    ('packages/provider-local-process', 'spec/', 'tests/package.rs', '"../../spec'),
    ('packages/provider-local-process', 'tools/check_schemas.py', 'tests/package.rs', 'import check_schemas'),
    ('packages/provider-ssh-runtime', 'spec/', 'tests/package.rs', '"../../spec'),
    ('packages/runtime-provider-kit', 'spec/', 'tests/schema_drift.rs', '"../../spec'),
    ('packages/scenario-s2-linear', 'spec/', 'tests/schema_drift.rs', '"../../spec'),
    ('packages/scenario-s2-linear', 'packages/scenario-s2-linear-restrictions/contracts.yaml',
     'tests/schema_drift.rs', '"../scenario-s2-linear-restrictions/contracts.yaml'),
    ('packages/scenario-s2-linear-adapter', 'spec/', 'tests/package.rs', '"../../spec'),
    ('packages/scenario-s2-linear-adapter', 'packages/scenario-s2-linear-restrictions/contracts.yaml',
     'tests/package.rs', '"../scenario-s2-linear-restrictions/contracts.yaml'),
    ('packages/scenario-s2-linear-restrictions', 'spec/', 'tests/schema_drift.rs', '"../../spec'),
    ('packages/scenario-s4-effect', 'spec/', 'tests/declarations.rs', '"../../spec'),
    ('experiments/011-short-form-spike', 'spec/', 'src/evidence.rs', '.join("spec")'),
    ('experiments/011-short-form-spike', 'conformance/fixtures/s1/', 'src/evidence.rs',
     '.join("conformance/fixtures/s1")'),
    ('experiments/022-provider-contracts', 'spec/', 'tests/harness.rs', '.join("spec/'),
)
NO_COMMITS = 'the branch adds no commit since the merge base'
PRIVATE_FILE = 'the file that --private-names-file names'


@dataclass
class CommandResult:
    exit: object
    output: str = ''
    timed_out: bool = False


@dataclass
class RustDirectory:
    path: str
    build: bool
    lint: bool


@dataclass
class Derivation:
    kept: list
    values: list
    dropped: dict
    sources: list
    origin_remote: bool
    unreadable_remotes: int


class SourceSkipped(Exception):
    pass


class UnusablePrivateName(Exception):
    pass


def without_git_tracing(environment):
    clean = {key: value for key, value in environment.items() if not key.startswith('GIT_TRACE')}
    # Explicitly override Trace2 targets from Git's system or global configuration too.
    return dict(clean, GIT_TRACE2='0', GIT_TRACE2_EVENT='0', GIT_TRACE2_PERF='0')


class Machine:
    def __init__(self, environment):
        self.environment = dict(environment)

    def account(self):
        try:
            import pwd
        except ImportError:
            raise SourceSkipped('this system has no pwd module') from None
        try:
            entry = pwd.getpwuid(os.getuid())
        except KeyError:
            raise SourceSkipped("this process's user ID has no account entry") from None
        return entry.pw_name, entry.pw_gecos

    def host_name(self):
        return socket.gethostname()

    def is_macos(self):
        return sys.platform == 'darwin'

    def which(self, name):
        return shutil.which(name, path=self.environment.get('PATH', os.defpath))

    def list_directory(self, path):
        return sorted(os.listdir(path), key=os.fsencode)

    def query(self, args, extra=None):
        environment = dict(self.environment, **(extra or {}))
        if args[0] == 'git':
            environment = without_git_tracing(environment)
        try:
            result = subprocess.run(args, env=environment,
                                    stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
                                    stderr=subprocess.DEVNULL, timeout=DERIVE_TIMEOUT)
        except subprocess.TimeoutExpired:
            return None, b''
        except OSError:
            return 127, b''
        return result.returncode, result.stdout


class DiscoveryError(Exception):
    def __init__(self, message, *, record_reason=None, from_command=False):
        super().__init__(message)
        self.record_reason = message if record_reason is None else record_reason
        # A command's exit or timeout: its own output, already in the log, ends the log.
        self.from_command = from_command


def rust_manifests(root):
    return sorted({path for area in ('packages', 'experiments')
                   for pattern in ('*/Cargo.toml', '*/*/Cargo.toml')
                   for path in (root / area).glob(pattern)
                   if path.is_file() and 'target' not in path.relative_to(root).parts},
                  key=lambda path: os.fsencode(path.relative_to(root)))


def discover_rust(root, manifests, query, *, target_cache=False):
    """Discover CI's workspace roots and their path dependencies, without building."""
    try:
        workflow = (root / WORKFLOW).read_text(encoding='utf-8')
    except OSError:
        workflow = ''
    match = re.search(r'^\s*macos_only="([^"\n]*)"\s*$', workflow, re.MULTILINE)
    if not match:
        raise DiscoveryError('macos_only could not be read from .github/workflows/checks.yml')
    not_built = match[1].split()
    directories = []
    for manifest in manifests:
        relative = manifest.relative_to(root).as_posix()
        workspace = query(['locate-project', '--workspace', '--message-format', 'plain',
                           '--manifest-path', relative]).strip()
        if Path(workspace).resolve() == manifest.resolve():
            directory = manifest.parent.relative_to(root).as_posix()
            directories.append(RustDirectory(directory, directory not in not_built,
                                              directory.startswith('packages/')))
    directories.sort(key=lambda entry: os.fsencode(entry.path))
    holders = [entry.path for entry in directories] + ['engine']
    if target_cache:
        cache_names = {}
        for directory in holders:
            name = directory.replace('/', '-')
            if name in cache_names:
                raise DiscoveryError(f'target-cache name {name} matches both '
                                     f'{cache_names[name]} and {directory}')
            cache_names[name] = directory
    dependencies = {directory: set() for directory in holders}
    for directory in holders:
        output = query(['metadata', '--no-deps', '--format-version', '1', '--offline',
                        '--manifest-path', directory + '/Cargo.toml'])
        try:
            metadata = json.loads(output)
            for package in metadata['packages']:
                for dependency in package['dependencies']:
                    if dependency.get('path') is None:
                        continue
                    path = Path(dependency['path']).resolve()
                    try:
                        relative = path.relative_to(root).as_posix()
                    except ValueError:
                        raise DiscoveryError(
                            f'path dependency outside the repository: {path}',
                            record_reason='path dependency outside the repository') from None
                    if relative == directory or relative.startswith(directory + '/'):
                        continue
                    owners = [holder for holder in holders
                              if relative == holder or relative.startswith(holder + '/')]
                    if not owners:
                        raise DiscoveryError(f'path dependency has no discovered directory: {relative}')
                    dependencies[directory].add(max(owners, key=len))
        except (ValueError, KeyError, TypeError, AttributeError) as error:
            raise DiscoveryError(f'cargo metadata for {directory} returned invalid metadata') from error
    return directories, dependencies


# Skip comments and character literals while finding ordinary and raw strings.
RUST_SOURCE_TOKEN = re.compile(
    r'//[^\n]*|/\*|\b(?:b|c)?r(?P<hashes>\#{0,255})"(?P<raw>.*?)"(?P=hashes)'
    r'|"(?P<string>(?:[^"\\]|\\.)*)"|\'(?:[^\'\\\n]|\\(?:x..|u\{[^}]*\}|.))\'',
    re.DOTALL,
)
RUST_BLOCK_MARKER = re.compile(r'/\*|\*/')
RUST_ESCAPE = re.compile(
    r'\\(?:\n[ \t\n\r]*|x[0-9a-fA-F]{2}|u\{[0-9a-fA-F][0-9a-fA-F_]*\}|.)', re.DOTALL)
RUST_SIMPLE_ESCAPES = {'\\': '\\', '"': '"', "'": "'", 'n': '\n', 't': '\t', 'r': '\r', '0': '\0'}


def decode_rust_string(text, match, source):
    def replace(escape):
        code = escape[0][1:]
        if code in RUST_SIMPLE_ESCAPES:
            return RUST_SIMPLE_ESCAPES[code]
        if code.startswith('\n'):
            return ''  # Rust discards whitespace after a string continuation.
        if len(code) == 3 and code.startswith('x'):
            return chr(int(code[1:], 16))
        if code.startswith('u{'):
            digits = code[2:-1].replace('_', '')
            if len(digits) <= 6:
                scalar = int(digits, 16)
                if scalar <= 0x10ffff and not 0xd800 <= scalar <= 0xdfff:
                    return chr(scalar)
        line = text.count('\n', 0, match.start('string') + escape.start()) + 1
        raise DiscoveryError(f'{source}:{line}: cannot decode Rust string escape')

    return RUST_ESCAPE.sub(replace, match['string'])


def rust_string_literals(text, source):
    offset = 0
    while match := RUST_SOURCE_TOKEN.search(text, offset):
        offset = match.end()
        if match[0] == '/*':
            depth = 1
            for marker in RUST_BLOCK_MARKER.finditer(text, offset):
                depth += 1 if marker[0] == '/*' else -1
                if depth == 0:
                    offset = marker.end()
                    break
            else:
                return
        elif match['raw'] is not None:
            yield match['raw']
        elif match['string'] is not None:
            yield decode_rust_string(text, match, source)


def engine_paths(root, dependencies, sources):
    """Find the directories the engine's Rust sources read and their path dependencies."""
    areas = ('packages', 'experiments')
    package_names = {path.name for path in (root / 'packages').glob('*') if path.is_dir()}
    reads = set()
    for source in sources:
        source_path = root / source
        try:
            text = source_path.read_text(encoding='utf-8')
        except (OSError, UnicodeError) as error:
            if isinstance(error, FileNotFoundError) and not os.path.lexists(source_path):
                continue  # A tracked source may have been removed in the working tree.
            raise DiscoveryError(f'could not read engine source: {source}') from None
        literals = set(rust_string_literals(text, source))
        if 'packages' in literals:
            reads.update('packages/' + name for name in package_names & literals)
        for literal in literals:
            parts = literal.split('/')
            for index, area in enumerate(parts[:-1]):
                if area not in areas:
                    continue
                path = PurePosixPath(posixpath.normpath('/'.join(parts[index:])))
                if len(path.parts) < 2 or path.parts[0] != area:
                    continue
                top = PurePosixPath(*path.parts[:2])
                if not (root / top).is_dir():
                    continue
                candidates = [part.as_posix() for part in (path, *path.parents) if len(part.parts) > 1]
                owners = dependencies.keys() & candidates
                if owners:
                    reads.update(owners)
                elif area == 'packages':
                    reads.add(top.as_posix())
                else:
                    directory = next((part for part in candidates if (root / part).is_dir()), None)
                    if directory is not None:
                        reads.add(directory)

    pending = list(reads)
    for directory in pending:
        added = dependencies.get(directory, set()) - reads
        pending.extend(added)
        reads.update(added)
    reads.discard('engine')
    return ENGINE_ROOTS + tuple(sorted((path + '/' for path in reads), key=os.fsencode))


def engine_why(prefixes):
    return 'the branch changes nothing under ' + ', '.join(prefixes[:-1]) + ' or ' + prefixes[-1]


def selects_path(path, entry):
    return path.startswith(entry) if entry.endswith('/') else path == entry


def tool_tests_reached(changed):
    return any(path.startswith((b'tools/', b'.github/workflows/'))
               or (path.startswith((b'packages/', b'experiments/'))
                   and (path.endswith(b'.rs') or os.path.basename(path) in (b'Cargo.toml', b'Cargo.lock')))
               for path in changed)


def select_rust(directories, dependencies, changed, all_directories, engine_prefixes):
    paths = [os.fsdecode(path) for path in changed]
    holders = [entry.path for entry in directories] + ['engine']
    direct = {directory: sum(path.startswith(directory + '/') for path in paths)
              for directory in holders}

    def dependency_reason(directory):
        # Breadth first, then byte order: cycles cannot recur and equal routes are stable.
        queue = [(directory, [])]
        visited = {directory}
        for holder, route in queue:
            for dependency in sorted(dependencies[holder], key=os.fsencode):
                if dependency in visited:
                    continue
                visited.add(dependency)
                if direct[dependency]:
                    through = ', through ' + ', '.join(route) + ',' if route else ''
                    return f'it depends by path{through} on {dependency}, which the branch changes'
                queue.append((dependency, route + [dependency]))
        return None

    selected, skipped_own_paths = {}, {}
    for entry in directories:
        directory = entry.path
        reason = f'the branch changes {direct[directory]} of its paths' if direct[directory] else None
        if reason is None and all_directories and directory in OWN_PATHS_ONLY:
            reason = '--all was given'
        via = None if reason else dependency_reason(directory)
        if reason is None and via is None:
            for reader, path, evidence, _ in RUST_READS:
                if reader == directory and any(selects_path(changed_path, path) for changed_path in paths):
                    via = f'it reads {path} ({evidence}), which the branch changes'
                    break
        if via and directory in OWN_PATHS_ONLY:
            skipped_own_paths[directory] = (f'{via}. It runs here only on its own changes or with '
                                            f'--all. {OWN_PATHS_ONLY[directory]}.')
            continue
        reason = reason or via
        if reason is None and all_directories:
            reason = '--all was given'
        if reason:
            selected[directory] = reason
    prefixes = [prefix for prefix in engine_prefixes if any(path.startswith(prefix) for path in paths)]
    engine_reason = None
    if prefixes:
        count = sum(any(path.startswith(prefix) for prefix in prefixes) for path in paths)
        engine_reason = f"the branch changes {count} paths under {', '.join(prefixes)}"
    engine_reason = engine_reason or dependency_reason('engine')
    if engine_reason is None and all_directories:
        engine_reason = '--all was given'
    return selected, skipped_own_paths, engine_reason


def target_cache_lock_status(lock):
    try:
        if not stat.S_ISREG(lock.lstat().st_mode):
            return ''
        with lock.open(encoding='utf-8') as stream:
            details = json.loads(stream.read(4096))
    except (OSError, ValueError):
        return ''
    pid = details.get('pid') if isinstance(details, dict) else None
    if type(pid) is not int or not 0 < pid < 2**31:
        return ''
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return f' for PID {pid}, which is not running'
    except PermissionError:
        pass
    return f' for PID {pid}, which is running'


def claim_target_cache(directory, shown, head, started, cleanup):
    lock = directory / 'gate.lock'
    def remove_owned_lock(identity):
        try:
            if os.path.samestat(identity, lock.lstat()):
                lock.unlink()
        except FileNotFoundError:
            pass

    try:
        directory.mkdir(parents=True, exist_ok=True)
        with tempfile.NamedTemporaryFile(mode='w', dir=directory, prefix='.gate.lock-',
                                         encoding='utf-8') as stream:
            json.dump(dict(pid=os.getpid(), head=head, started=started), stream)
            stream.write('\n')
            stream.flush()
            cleanup.callback(remove_owned_lock, os.fstat(stream.fileno()))
            os.link(stream.name, lock)
    except FileExistsError:
        if directory.is_dir():
            return (f'target-cache lock {shown}/gate.lock exists{target_cache_lock_status(lock)}. '
                    'Remove the lock if no gate is running')
        return f'target-cache directory {shown} is not a directory'
    except OSError as fault:
        return f'cannot lock target-cache directory {shown}: {fault.strerror}'
    return None


def invalidate_path_fingerprints(target, names):
    try:
        # Cargo uses one extra directory level for explicit targets, including
        # targets selected by Cargo configuration rather than the command line.
        build_roots = [target, *target.iterdir()]
    except FileNotFoundError:
        return
    for root in build_roots:
        for profile in ('debug', 'release'):
            fingerprints = root / profile / '.fingerprint'
            if fingerprints.is_dir():
                for fingerprint in fingerprints.iterdir():
                    if fingerprint.name.rpartition('-')[0] in names and fingerprint.is_dir():
                        shutil.rmtree(fingerprint)


def positive_integer(text):
    try:
        value = int(text)
    except ValueError:
        raise argparse.ArgumentTypeError('must be a positive integer') from None
    if value <= 0:
        raise argparse.ArgumentTypeError('must be a positive integer')
    return value


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
    return re.sub(pattern, '[redacted]', text, flags=re.IGNORECASE)


def run_command(args, *, cwd, env, timeout, log, redact=redact_output):
    """Runner: argv, cwd/env/timeout/log/redact -> CommandResult; may interrupt.

    The timeout counts the leader alone. A reader thread collects the output, and a process
    that left the group and still holds the pipe delays the result by PIPE_WAIT at most.
    Output stays in memory until redacted, before any log is written.
    """
    buffer = bytearray()
    timed_out = False
    process = reader = stop = None
    try:
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
                reader.start()
                try:
                    process.wait(timeout=timeout)
                except subprocess.TimeoutExpired:
                    timed_out = True
                    end_group(process)
                # Inside the try: an interrupt during this wait still ends the group (decision 8).
                reader.join(PIPE_WAIT)
        except KeyboardInterrupt:
            if process is not None:
                end_group(process)
            if reader is not None and reader.ident is not None:
                reader.join(PIPE_WAIT)
            raise
        finally:
            if stop is not None:
                stop.set()
            if reader is not None and reader.ident is not None:
                reader.join()
            if process is not None:
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


def account_names(machine):
    short, gecos = machine.account()
    full = gecos.partition(',')[0]
    return [short, full, *full.split()]


def host_names(machine):
    try:
        host = machine.host_name()
    except OSError:
        raise SourceSkipped('socket.gethostname() failed') from None
    label, dot, _ = host.partition('.')
    return [host, label] if dot else [host]


def macos_names(machine):
    if not machine.is_macos():
        raise SourceSkipped('this is not macOS')
    names = []
    for field in ('ComputerName', 'LocalHostName'):
        code, output = machine.query(['scutil', '--get', field])
        if code == 0:
            names.append(output.decode('utf-8', errors='replace').strip())
    if not names:
        raise SourceSkipped('scutil answered neither ComputerName nor LocalHostName')
    return names


def git_identity_names(machine):
    name_exit, name = machine.query(['git', 'config', '--global', '--get', 'user.name'])
    email_exit, email = machine.query(['git', 'config', '--global', '--get', 'user.email'])
    if None in (name_exit, email_exit):
        raise SourceSkipped('git config --global did not answer within 10 seconds')
    for code in (name_exit, email_exit):
        if code not in (0, 1):
            raise SourceSkipped(f'git config --global exited {code}')
    if name_exit == email_exit == 1:
        raise SourceSkipped("git's global configuration sets neither user.name nor user.email")
    names = []
    if name_exit == 0:
        names.append(name.decode('utf-8', errors='replace').strip())
    if email_exit == 0:
        names.append(email.decode('utf-8', errors='replace').strip().rsplit('@', 1)[0])
    return names


def remote_repository_name(url):
    url = url.strip()
    prefix, colon, path = url.partition(':')
    if '://' in url:
        path = urlsplit(url).path
    elif not colon or '/' in prefix:
        path = url
    name = path.rstrip('/').rsplit('/', 1)[-1]
    return name[:-4] if name.endswith('.git') else name


def sibling_names(root, machine):
    unavailable = 'this repository has no main working tree, so its sibling directories are not known'
    try:
        common = git(root, without_git_tracing(machine.environment),
                     'rev-parse', '--path-format=absolute', '--git-common-dir')
    except OSError:
        raise SourceSkipped(unavailable) from None
    directory = Path(common.stdout.decode('utf-8', errors='replace').rstrip('\r\n'))
    if common.returncode or directory.name != '.git':
        raise SourceSkipped(unavailable)
    holder = directory.parent.parent
    try:
        entries = machine.list_directory(holder)
    except OSError:
        raise SourceSkipped('the directory that holds this repository could not be listed') from None
    names = []
    unreadable = 0
    for entry in entries:
        path = holder / entry
        if not os.path.isdir(path) or os.path.islink(path) or not os.path.lexists(path / '.git'):
            continue
        names.append(entry)
        code, output = machine.query(
            [*GIT, '-C', str(path), 'config', '--local', '--get-regexp', r'^remote\..*\.url$'],
            extra={'GIT_CEILING_DIRECTORIES': str(holder)})
        if code == 1:
            continue
        if code != 0:
            unreadable += 1
            continue
        for line in output.decode('utf-8', errors='replace').split('\n'):
            try:
                names.append(remote_repository_name(line.partition(' ')[2]))
            except ValueError:
                unreadable += 1
    return names, unreadable


def tailnet_names(machine):
    executable = machine.which('tailscale')
    if executable is None:
        raise SourceSkipped('the tailscale command is not on the PATH')
    code, output = machine.query([executable, 'status', '--json'])
    if code is None:
        raise SourceSkipped('tailscale status --json did not answer within 10 seconds')
    if code != 0:
        raise SourceSkipped(f'tailscale status --json exited {code}')
    invalid = 'tailscale status --json did not print a JSON object'
    try:
        status = json.loads(output.decode('utf-8', errors='replace'))
    except (ValueError, RecursionError):
        raise SourceSkipped(invalid) from None
    if not isinstance(status, dict):
        raise SourceSkipped(invalid)
    peers = status.get('Peer')
    entries = [status.get('Self')]
    if isinstance(peers, dict):
        entries.extend(peers.values())
    names = []
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        host = entry.get('HostName')
        dns = entry.get('DNSName')
        if isinstance(host, str) and host:
            names.append(host)
        if isinstance(dns, str) and dns:
            names.append(dns.partition('.')[0])
    return names


def origin_public_text(root, machine):
    try:
        result = git(root, without_git_tracing(machine.environment),
                     'config', '--local', '--get', 'remote.origin.url')
    except OSError:
        return None
    if result.returncode:
        return None
    url = result.stdout.decode('utf-8', errors='replace').strip()
    if '://' in url:
        try:
            parsed = urlsplit(url)
            return (parsed.hostname or '') + parsed.path
        except ValueError:
            return None
    prefix, colon, _ = url.partition(':')
    if colon and '@' in prefix:
        return url.partition('@')[2]
    return url


def derive_private_names(root, machine):
    sources = []
    candidates = {}
    unreadable = 0
    for name, read in (('account', account_names), ('host name', host_names),
                       ('macOS computer names', macos_names), ('git identity', git_identity_names),
                       ('sibling repositories', sibling_names), ('tailnet peers', tailnet_names)):
        source = dict(name=name, result='read', kept=0, why=None)
        sources.append(source)
        try:
            if read is sibling_names:
                names, unreadable = read(root, machine)
            else:
                names = read(machine)
        except SourceSkipped as skipped:
            source.update(result='NOT RUN', why=str(skipped))
            continue
        for candidate in names:
            for part in re.split(r'[,\r\n]', candidate):
                part = part.strip()
                if part:
                    candidates.setdefault(part.lower(), (part, source))

    origin = origin_public_text(root, machine)
    public_origin = origin.lower() if origin is not None else ''
    kept, values = [], []
    dropped = dict(short=0, origin=0, public=0)
    for lower, (name, source) in candidates.items():
        if len(name) < 4:
            dropped['short'] += 1
        elif lower in public_origin:
            dropped['origin'] += 1
        elif lower in PUBLIC_WORDS:
            dropped['public'] += 1
        else:
            try:
                name.encode('utf-8')
            except UnicodeEncodeError:
                raise UnusablePrivateName(source['name']) from None
            if '\0' in name:
                raise UnusablePrivateName(source['name'])
            source['kept'] += 1
            kept.append(name)
            values.append(name)
            if lower != name:
                values.append(lower)
    return Derivation(kept, values, dropped, sources, origin is not None, unreadable)


def show_derivation(derivation, names_file, ignored_environment_names):
    count = len(derivation.kept)
    read = ', '.join(f"{source['name']} {source['kept']}"
                     for source in derivation.sources if source['result'] == 'read')
    short, origin, public = (derivation.dropped[key] for key in ('short', 'origin', 'public'))
    dropped = (f'{short + origin + public} dropped as public or short '
               f"({short} shorter than 4 characters, {origin} in the origin remote's URL, "
               f'{public} on the gate\'s list of public words)')
    if names_file:
        line = (f'Private names: read from {PRIVATE_FILE}, and {count} more derived from this machine '
                f'({read}), all given to tools/check_private_paths.py alone; {dropped}; '
                "neither the names nor the file's path are printed or recorded.")
    else:
        line = (f'Private names: {count} derived from this machine ({read}), and given to '
                f'tools/check_private_paths.py alone; {dropped}; none is printed or recorded.')
        if count == 0:
            line += ' So tools/check_private_paths.py checks its built-in patterns only.'
        if ignored_environment_names:
            line += ' CONDUIT_PRIVATE_NAMES in the environment is not used.'
    print(line)
    for source in derivation.sources:
        if source['result'] == 'NOT RUN':
            print(f"Private names: NOT RUN {source['name']}: {source['why']}.")
    if derivation.unreadable_remotes:
        print(f'Private names: the remotes of {derivation.unreadable_remotes} sibling repositories '
              'could not be read; their directory names alone were taken.')
    if not derivation.origin_remote:
        print('Private names: this repository has no origin remote, so no derived name was dropped '
              'as already public in its URL.')


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
    if command and row['directory'] != '.':
        command += ' in ' + row['directory']
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


def main(root, argv, run=run_command, machine=None):
    with ExitStack() as cleanup:
        return run_gate(root, argv, run, machine, cleanup)


def run_gate(root, argv, run, machine, cleanup):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--base', default='origin/main', metavar='REF')
    parser.add_argument('--plan', action='store_true')
    parser.add_argument('--private-names-file', metavar='FILE')
    parser.add_argument('--derive-private-names', action='store_true')
    parser.add_argument('--all', action='store_true')
    parser.add_argument('--target-cache', metavar='DIR')
    parser.add_argument('--jobs', type=positive_integer, default=6, metavar='N')
    parser.add_argument('--test-threads', type=positive_integer, default=4, metavar='N')
    options = parser.parse_args(argv)
    root = Path(root).resolve()
    clean_environment, removed, credentials = scrub_environment()
    settings = dict(jobs=options.jobs, test_threads=options.test_threads, all=options.all,
                    target_cache=None)
    environment_set = dict(CARGO_NET_OFFLINE='true', RUSTUP_AUTO_INSTALL='0',
                           CARGO_TERM_COLOR='never', CARGO_BUILD_JOBS=str(options.jobs),
                           RUST_TEST_THREADS=str(options.test_threads))
    environment = dict(clean_environment, **{key: value for key, value in environment_set.items()
                                             if key != 'RUST_TEST_THREADS'})

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
    names_file = options.private_names_file is not None
    if names_file:
        private_names, fault = read_private_names(options.private_names_file, root)
        if fault:
            return not_started(PRIVATE_FILE + ' ' + fault)
    derivation = None
    derived_record = None
    if options.derive_private_names:
        if machine is None:
            machine = Machine(clean_environment)
        try:
            derivation = derive_private_names(root, machine)
        except UnusablePrivateName as fault:
            return not_started(f'--derive-private-names read a name from {fault} that cannot be '
                               'passed to tools/check_private_paths.py')
        if all(source['result'] == 'NOT RUN' for source in derivation.sources):
            return not_started('--derive-private-names read none of its sources on this machine')
        file_values = private_names.split(',') if private_names else []
        private_names = ','.join(dict.fromkeys(file_values + derivation.values)) or None
        derived_record = dict(kept=len(derivation.kept), dropped=derivation.dropped,
                              origin_remote=derivation.origin_remote,
                              unreadable_sibling_remotes=derivation.unreadable_remotes,
                              sources=derivation.sources)
    head = git(root, environment, 'rev-parse', 'HEAD').stdout.decode().strip()
    started = utc_now()
    target_cache = None
    if options.target_cache is not None:
        try:
            target_cache = Path(options.target_cache).resolve()
            shown_cache = redact_output(str(target_cache), private_names)
            if target_cache == root or root in target_cache.parents:
                return not_started(f'target-cache directory {shown_cache} is inside the gate\'s root')
            if target_cache.exists() and not target_cache.is_dir():
                return not_started(f'target-cache directory {shown_cache} is not a directory')
            if not options.plan:
                cleanup.callback(signal.signal, signal.SIGTERM, signal.getsignal(signal.SIGTERM))
                signal.signal(signal.SIGTERM, signal.default_int_handler)
                fault = claim_target_cache(target_cache, shown_cache, head, started, cleanup)
                if fault:
                    return not_started(fault)
        except (OSError, RuntimeError):
            return not_started('the target-cache directory could not be opened')
        settings['target_cache'] = shown_cache
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
    if target_cache is not None:
        print(f'Target cache: {shown_cache}. The tree\'s own crates rebuild and registry crates are reused.')
    if derivation is not None:
        show_derivation(derivation, names_file, 'CONDUIT_PRIVATE_NAMES' in removed)
    elif private_names is not None:
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
    print('Environment: cargo runs with CARGO_NET_OFFLINE=true, RUSTUP_AUTO_INSTALL=0, '
          f'CARGO_TERM_COLOR=never and CARGO_BUILD_JOBS={options.jobs}, and cargo test with '
          f'RUST_TEST_THREADS={options.test_threads}. Removed from every command: '
          f"{', '.join(removed) or 'no named variable'}, "
          f'and {credentials} variables named for a credential.')

    record = dict(format=1, tool=TOOL, started=started, finished=None, result='incomplete',
                  base=dict(ref=shown_base, commit=base), merge_base=merge_base, head=head,
                  branch=shown_branch, contains_base=contains_base, behind=behind,
                  tree=dict(clean=not dirty, uncommitted_paths=dirty), changed_paths=len(changed),
                  platform=platform.system() + ' ' + platform.machine(), python=platform.python_version(),
                  python_packages=not missing,
                  settings=settings,
                  environment=dict(set=environment_set, removed=removed, removed_credentials=credentials,
                                   private_names_configured=False,
                                   private_names=dict(file=names_file, derived=derived_record)), gates=[])
    record_path = root / OUTPUT / (head + '.json')
    log_directory = root / OUTPUT / 'logs' / head
    if not options.plan:
        log_directory.mkdir(parents=True, exist_ok=True)
        for old in log_directory.glob('*.log'):
            old.unlink()
        write_record(record_path, record)
    current = 'rust discovery'

    def finish(row, output='', findings=()):
        shown = dict(row)
        for key in ('name', 'directory', 'command', 'reason', 'why', 'partial', 'log'):
            if isinstance(row[key], str):
                shown[key] = redact_output(row[key], private_names)
        record['gates'].append(shown)
        if not options.plan:
            write_record(record_path, record)
        show(shown, output, (redact_output(item, private_names) for item in findings),
             options.plan and row['name'] != 'rust discovery')

    def execute(args, log, minutes, env=None, display=None, directory='.', raw_output=None):
        def redact(text):
            if raw_output is not None:
                raw_output.append(text)
            if target_cache is not None:
                text = text.replace(str(target_cache), '<target-cache>')
            return redact_output(text, private_names)

        print(redact_output(f'running {current}: {display or command_text(args)}', private_names),
              file=sys.stderr)
        command_environment = dict(environment if env is None else env)
        if 'CONDUIT_PRIVATE_NAMES' in command_environment:
            record['environment']['private_names_configured'] = True
            write_record(record_path, record)
        return run(args, cwd=root / directory, env=command_environment,
                   timeout=minutes * 60, log=log, redact=redact)

    try:
        manifests = rust_manifests(root)
        discovery = gate_row(current,
                             'cargo locate-project and cargo metadata --no-deps --offline, '
                             f'for {len(manifests) + 1} manifests',
                             reason='the Rust gates are chosen from it')
        cargo = shutil.which('cargo')
        directories, dependencies = [], {}
        if cargo is None:
            discovery['why'] = 'cargo was not found on the PATH; install Rust through rustup'
        else:
            started = time.monotonic()
            discovery_output = []
            # Plans also discover, but write neither a persistent log nor a record.
            with tempfile.TemporaryDirectory() as temporary:
                log = Path(temporary) / 'discovery.log' if options.plan else log_directory / '01-rust-discovery.log'

                def query(args):
                    printed = command_text(['cargo', *args])
                    # Cargo prints absolute paths. Parse the original paths in memory.
                    raw_output = []
                    result = execute([cargo, *args], Path(temporary) / 'command.log', 5,
                                     display=printed, raw_output=raw_output)
                    discovery_output.append(redact_output(printed, private_names) + '\n' + result.output + '\n')
                    discovery['exit'] = result.exit
                    if result.timed_out:
                        raise DiscoveryError(timeout_words(5), from_command=True)
                    if result.exit != 0:
                        raise DiscoveryError(f'{printed}: exit {result.exit}', from_command=True)
                    return raw_output[-1] if raw_output else result.output

                try:
                    directories, dependencies = discover_rust(
                        root, manifests, query, target_cache=target_cache is not None)
                    files = git(root, environment, 'ls-files', '--cached', '--others',
                                '--exclude-standard', '-z', '--', 'engine')
                    if files.returncode:
                        raise DiscoveryError('git ls-files could not list the engine sources')
                    sources = {os.fsdecode(path) for path in files.stdout.split(b'\0')
                               if path.endswith(b'.rs')}
                    engine_prefixes = engine_paths(root, dependencies, sorted(sources, key=os.fsencode))
                    discovery.update(result='PASS', exit=0)
                except DiscoveryError as fault:
                    discovery.update(result='FAIL',
                                     why=redact_output(fault.record_reason, private_names))
                    if not fault.from_command:
                        discovery_output.append(redact_output(str(fault), private_names) + '\n')
                finally:
                    discovery['seconds'] = round(time.monotonic() - started, 1)
                    log.write_text(''.join(discovery_output), encoding='utf-8')
                    if not options.plan:
                        discovery['log'] = log.relative_to(root).as_posix()
        finish(discovery, ''.join(discovery_output) if cargo else '')

        current = 'identity'
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
                      ['python3', VECTOR, merge_base, 'WORKTREE', '--declared-changed']),
                     ('decisions', [sys.executable, 'tools/check_decisions.py', merge_base, 'WORKTREE'],
                      ['python3', 'tools/check_decisions.py', merge_base, 'WORKTREE'])]
        for current, args, printed in commands:
            row = gate_row(current, command_text(printed), reason=PYTHON_REASON)
            if current == 'tool tests' and not (options.all or tool_tests_reached(changed)):
                row.update(required=False, why=TOOL_TESTS_WHY)
                finish(row)
                continue
            if options.plan:
                row['result'] = 'PASS'  # Only show WOULD RUN; no check is claimed or recorded.
                finish(row)
                continue
            position = len(record['gates']) + 1
            log_name = redact_output(current, private_names).replace(' ', '-').replace('/', '-')
            log = log_directory / (f'{position:02d}-' + log_name + '.log')
            env = dict(environment)
            if current == 'private paths' and private_names is not None:
                env['CONDUIT_PRIVATE_NAMES'] = private_names
            started = time.monotonic()
            raw = []
            result = execute(args, log, 15, env, row['command'], raw_output=raw)
            row.update(result='FAIL' if result.timed_out or result.exit != 0 else 'PASS',
                       exit=result.exit, seconds=round(time.monotonic() - started, 1),
                       log=log.relative_to(root).as_posix())
            if row['result'] == 'FAIL':
                row['why'] = timeout_words(15) if result.timed_out else f'exit {result.exit}'
            if current in ('whitespace', 'private paths') and untracked:
                row['partial'] = f'{untracked} untracked path(s)'
            if current == 'docs':
                skipped = sum(line.startswith('Notice: skipped repository permalinks at')
                              for line in ''.join(raw).splitlines())
                if skipped:
                    row['partial'] = f'the permalinks at {skipped} commit(s) this clone lacks, which CI fetches'
            if current == 'tool tests':
                skipped = re.search(r'OK \(skipped=(\d+)\)\s*$', ''.join(raw))
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
                raw_output = []
                version = execute([executable, 'version'], temp / 'version.log', 10,
                                  display='gitleaks version', raw_output=raw_output)
                found = (raw_output[-1] if raw_output else version.output).strip().lstrip('v')
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
        if discovery['result'] != 'PASS':
            finish(gate_row('rust and engine', None, required=False, why=RUST_WHY))
        else:
            selected, skipped_own_paths, engine_reason = select_rust(
                directories, dependencies, changed, options.all, engine_prefixes)
            prepared = set()

            def rust_command(name, directory, arguments, reason):
                nonlocal current
                current = name
                env = dict(environment)
                printed = command_text(['cargo', *arguments])
                target = None
                if target_cache is not None and arguments[0] in ('clippy', 'test'):
                    cache_name = directory.replace('/', '-')
                    target = target_cache / cache_name
                    displayed_target = f'<target-cache>/{cache_name}'
                    if directory.startswith('packages/') and arguments[0] == 'test':
                        # Archive builders invoked by package tests use the same binaries.
                        # Engine tests build several packages and keep their separate targets.
                        env['CARGO_TARGET_DIR'] = str(target)
                        printed = f'CARGO_TARGET_DIR={shlex.quote(displayed_target)} {printed}'
                    else:
                        printed = command_text(['cargo', arguments[0], '--target-dir',
                                                displayed_target, *arguments[1:]])
                        arguments = [arguments[0], '--target-dir', str(target), *arguments[1:]]
                if arguments[0] == 'test':
                    env['RUST_TEST_THREADS'] = str(options.test_threads)
                    printed = f'RUST_TEST_THREADS={options.test_threads} ' + printed
                row = gate_row(name, printed, reason=reason)
                row['directory'] = directory
                if options.plan:
                    row['result'] = 'PASS'
                    finish(row)
                    return
                position = len(record['gates']) + 1
                log_name = redact_output(name, private_names).replace(' ', '-').replace('/', '-')
                log = log_directory / (f'{position:02d}-' + log_name + '.log')
                started = time.monotonic()
                if target is not None and directory not in prepared:
                    raw = []
                    with tempfile.TemporaryDirectory() as temporary:
                        stage = 'cached Rust host'
                        probe = execute(['rustc', '-vV'], Path(temporary) / 'host.log', 5,
                                        directory=directory, raw_output=raw)
                        hosts = re.findall(r'^host: (\S+)$', raw[-1] if raw else probe.output,
                                           re.MULTILINE)
                        if not probe.timed_out and probe.exit == 0 and len(hosts) == 1:
                            stage = 'cached path metadata'
                            raw.clear()
                            probe = execute([cargo, 'metadata', '--format-version', '1', '--offline',
                                             '--filter-platform', hosts[0]],
                                            Path(temporary) / 'metadata.log', 5,
                                            directory=directory, raw_output=raw)
                    if probe.timed_out or probe.exit != 0 or len(hosts) != 1:
                        failure = (timeout_words(5) if probe.timed_out else
                                   f'exit {probe.exit}' if probe.exit != 0 else
                                   'expected one host triple from rustc -vV')
                        log.write_text(probe.output, encoding='utf-8')
                        row.update(result='FAIL', why=f'{stage}: {failure}', exit=probe.exit,
                                   seconds=round(time.monotonic() - started, 1),
                                   log=log.relative_to(root).as_posix())
                        finish(row, probe.output)
                        return
                    try:
                        output = raw[-1] if raw else probe.output
                        # Cargo's JSON is one line. The runner also captures warnings from stderr.
                        payload = next((line for line in output.splitlines() if line.startswith('{')), '')
                        packages = json.loads(payload)['packages']
                        names = {package['name'] for package in packages if package['source'] is None}
                        invalidate_path_fingerprints(target, names)
                    except (ValueError, KeyError, TypeError, OSError) as fault:
                        why = (f'cannot invalidate path fingerprints: {fault.strerror}'
                               if isinstance(fault, OSError) else 'invalid cached path metadata')
                        row.update(result='FAIL', why=why, seconds=round(time.monotonic() - started, 1))
                        finish(row)
                        return
                    prepared.add(directory)
                result = execute([cargo, *arguments], log, 90, env, printed + ' in ' + directory,
                                 directory=directory)
                row.update(result='FAIL' if result.timed_out or result.exit != 0 else 'PASS',
                           exit=result.exit, seconds=round(time.monotonic() - started, 1),
                           log=log.relative_to(root).as_posix())
                if row['result'] == 'FAIL':
                    row['why'] = timeout_words(90) if result.timed_out else f'exit {result.exit}'
                finish(row, result.output)

            for entry in directories:
                if entry.path in skipped_own_paths:
                    finish(gate_row('rust ' + entry.path, None, required=False,
                                    why=skipped_own_paths[entry.path]))
                    continue
                if entry.path not in selected:
                    continue
                reason = selected[entry.path]
                if entry.lint:
                    rust_command('rust ' + entry.path + ' fmt', entry.path, ['fmt', '--check'], reason)
                if not entry.build:
                    finish(gate_row('rust ' + entry.path, None, required=False,
                                    reason=reason, why=NOT_BUILT_WHY))
                    continue
                if entry.lint:
                    rust_command('rust ' + entry.path + ' clippy', entry.path,
                                 ['clippy', '--all-targets', '--', '-D', 'warnings'], reason)
                rust_command('rust ' + entry.path + ' test', entry.path, ['test', '--release'], reason)
            unreached = [entry.path for entry in directories
                         if entry.path not in selected and entry.path not in skipped_own_paths]
            if unreached:
                finish(gate_row('rust unreached', None, required=False,
                                why=UNREACHED_WHY + ', '.join(unreached)))
            if engine_reason:
                for suffix, arguments in (('fmt', ['fmt', '--check']),
                                          ('clippy', ['clippy', '--all-targets', '--', '-D', 'warnings']),
                                          ('test', ['test', '--release'])):
                    rust_command('engine ' + suffix, 'engine', arguments, engine_reason)
            else:
                finish(gate_row('engine', None, required=False, why=engine_why(engine_prefixes)))
    except KeyboardInterrupt:
        print(redact_output(f'Gate: interrupted during {current}; its process group was ended, '
                            'and the record says incomplete.', private_names))
        return 130

    if options.plan:
        # Discovery finishes first and is the only gate the plan executes.
        would = sum(row['result'] != 'NOT RUN' for row in record['gates'][1:])
        state = {'PASS': 'passed', 'FAIL': 'failed'}.get(discovery['result'], 'was not run')
        print(f"Gate plan: rust discovery {state}; {would} other gates would run, "
              f"{len(record['gates']) - 1 - would} would not. "
              'No other gate was checked, and no record was written.')
        return 0 if discovery['result'] == 'PASS' else 1
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
