# T6 timeout and recovery

Both native turns reached their deadline and returned `unknown`, with CLI exit status zero.
Agent inspections (`t6-diagnose-a/b.jsonl`) show the actual provider requests:
A and B read shared-new/readme.txt successfully, A wrote a-t6.txt successfully, and both
old shared/hello.txt reads failed. B's saved engine catalog has no directory write tool.
The write was checked on disk. These establish the requested directory behavior even though
final native turn replies were lost. Some nested requests finished after the outer turn.

The package documentation says a failed turn poisons its conversation. Recovery explicitly
retires and recreates both Agent instances and their Conduits, with the same names, types,
flags and intended access. This loses the private native conversation continuity; the shared
history Resource and its posts are retained. No timeout turn is implicitly restarted, no
write is replayed, and unknown outer outcomes are retained rather than guessed/reconciled.
