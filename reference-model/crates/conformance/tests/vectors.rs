#[test]
fn all_portable_cases_pass() {
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let reports = conduit_conformance::run_suite(&repository).unwrap_or_else(|e| panic!("{e}"));
    let ids: std::collections::BTreeSet<_> = reports.iter().map(|r| r.id.as_str()).collect();
    for required in [
        "e05-control-local-targets",
        "schema-pattern-property-ref",
        "schema-applicator-defaults",
        "schema-conflicting-defaults",
        "schema-equal-defaults",
        "e05-local",
        "e05-two-contexts",
        "e05-recursive",
        "e05-siblings",
        "e05-escaping",
        "e05-core-register",
        "e05-core-replace",
        "e05-handle",
        "e05-control",
        "e05-control-absent-template",
        "e05-control-absent-type",
        "e05-root-key",
        "schema-pattern-properties",
        "schema-defaults",
        "schema-invalid-default",
        "schema-root-ref-default",
        "schema-map-default",
        "schema-array-defaults",
        "schema-conditional-default",
        "schema-nested-key-allowed",
        "schema-formats",
        "first-start",
        "first-start-repeat",
        "first-start-protected",
        "first-start-protected-authorized",
        "first-start-lifecycle-protected",
        "first-start-lifecycle-pool-target",
        "loader-schema-attribution",
        "loader-depth-limit",
        "loader-string-values",
        "loader-format-letter-case",
        "loader-leading-bom",
    ] {
        assert!(
            ids.contains(required),
            "required work-order case absent: {required}"
        );
    }
}
