#[test]
fn all_portable_cases_pass() {
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let reports = conduit_conformance::run_suite(&repository).unwrap_or_else(|e| panic!("{e}"));
    let ids: std::collections::BTreeSet<_> = reports.iter().map(|r| r.id.as_str()).collect();
    for required in [
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
