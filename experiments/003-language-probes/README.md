# Offline language probes

Supports the [language/protocol research](../../docs/research/language-protocol-foundations/README.md).
This is a research fixture, not a new runtime milestone or a selected implementation dependency.

Executed 2026-11-02: **20 cases passed**. [results.json](results.json) records the observations,
Python version and direct library versions. [requirements.txt](requirements.txt) records the
installed dependency set. The script validates only the behavior it describes; it is not a
Conduit parser, SDK, runtime, or comprehensive JSON Schema/YAML conformance suite.

The probes are useful because the accepted strict language profile cannot be obtained simply
by using a library's “safe” loading mode and validating the resulting mapping.

| Area | Observed behavior | Implication for future Conduit implementation |
| --- | --- | --- |
| Defaults | Validation leaves an omitted optional value absent. | Default materialization needs a separately specified pass. |
| Unknown semantics | A stock validator ignores the invented `conduitMustBeEven` keyword. | Publish/recognize semantic contracts; arbitrary keywords cannot promise enforcement. |
| Annotations | `readOnly` does not decide whether an operation may write. | Check authority and control contracts separately. |
| Formats | An invalid IPv4 string passes without an explicitly enabled format checker. | Specify which formats are asserted and test validators consistently. |
| Composition | `allOf` retains a closed base's restrictions; closure after composition behaves differently. | Choose composition/closure rules deliberately. |
| Scalars | A string number is not coerced; an integral JSON number satisfies `integer`. | Distinguish semantic numeric types from lexical spelling and wire precision limits. |
| Absence | Missing optional value, explicit null and empty array have different results. | Preserve their declared meanings. |
| YAML loader | YAML 1.2 `yes` stays a string; this loader rejects duplicate keys. | Useful evidence for those cases, not proof of the entire strict profile. |
| Extra YAML features | Anchors, merge keys, infinity and implicit date construction are accepted by this safe loader. | Reject forbidden surface features before normalization; implement the chosen scalar profile. A date-looking scalar is a string under the selected Core profile. |

These are not bugs alleged against the libraries. Their supported language is broader or their
application responsibilities differ from Conduit's requirements.

To reproduce in an isolated environment from this directory:

```sh
python3 -m venv /tmp/conduit-language-probes
/tmp/conduit-language-probes/bin/python -m pip install -r requirements.txt
/tmp/conduit-language-probes/bin/python probe.py
```

Installation requires package access; the probe itself performs no network, account or provider
operations and does not write results unless the caller redirects its JSON output. Exact Python
and platform differences should be reported when comparing runs. Do not silently replace the
recorded observations with new results from an unrecorded toolchain.

No parser is recommended solely on these probes. The later Rust parser/validator work still needs
its own strict-surface corpus, numeric and reference-resolution checks, resource limits and
cross-client conformance evidence.
