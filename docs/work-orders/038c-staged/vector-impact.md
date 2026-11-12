# Existing vector expectations

None. The search covered every case under conformance, including the Fleet cases already
in this tree, and every YAML or JSON fixture its files map names. No pre-existing fixture
states signIn. The new accepted fixtures therefore introduce no client-ID conflict into an
existing case and do not make an existing inspected object or installed package gain logins.
No existing request gives confirmClientIds.

A recursive search for signIn and install.v1, inspect.v1, register.v1 and replace.v1 found
292 candidate expectation steps in 185 existing cases. Inspection also covered schema and
tool-list expectations. Tool-schema assertions for the affected verbs are:

| Path | Zero-based step | Expectation and effect |
| --- | --- | --- |
| conformance/interface/schemas/e05-core-register/case.yaml | 0 | The tool assertion names use.register.v1; it does not pin the description or entire input/output schema. |
| conformance/interface/schemas/e05-core-replace/case.yaml | 0 | The tool assertion names use.replace.v1; it does not pin the description or entire input/output schema. |

The other candidates assert registration, installation, replacement, inspection or discovered
operation lists without a Type stating signIn. Optional logins outputs and confirmation inputs
leave those results and operation lists as asserted. The new schema is read directly by the
loader. No existing expectation or fixture is edited. The live case has 27 load steps;
rejections carry SUB-005 and the failing schema keyword location.

The verifier round adds zero-based steps 16 to 25 for empty HTTPS hosts, port-only
authorities and fragments in hosted redirect pages. Step 26 accepts query components in
both hosted return kinds. These exercise CTR-013 and SUB-005 without changing an existing
expectation. Authorization, page and issuer keep URI format validation and now require a
nonempty host; page also excludes fragments. No new live or staged contract is added.
