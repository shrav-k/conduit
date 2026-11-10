# Pinned agent-claude-code contracts

The two YAML files are byte-for-byte copies of
[the package](../../../../packages/agent-claude-code/README.md) at version 0.1.1 (work order 024). The accompanying
`contracts.tar.gz` pins just those files as deterministic gzip/USTAR evidence for the
registration vector; it contains no host executable and cannot run an Agent. The package's
own `cargo run --release --bin package` produces the runnable host archive with its digest.
After changing a package contract, refresh its two copies and this metadata archive together.
The live S1 registration vector uses these copies; scripted shell tests install real host
archives and explicitly substitute scripted clients.
