use crate::form::Contract;
use serde::Serialize;
#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    Passed,
    Failed,
    Untested,
}
#[derive(Debug, Serialize)]
pub struct CaseReport {
    pub id: String,
    pub digest: String,
    pub verdict: Verdict,
    pub detail: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct Tool {
    pub name: String,
    pub version: String,
}
#[derive(Debug, Serialize)]
pub struct Report {
    pub repository_revision: String,
    pub contract: Contract,
    pub suite_digest: String,
    pub archive_digest: Option<String>,
    pub descriptor_digest: String,
    pub parameter_names: Vec<String>,
    pub tool: Option<Tool>,
    pub cases: Vec<CaseReport>,
    pub totals: Totals,
}
#[derive(Debug, Default, Serialize)]
pub struct Totals {
    pub passed: usize,
    pub failed: usize,
    pub untested: usize,
}
impl Report {
    /// Apply the complete suite's known sensitive values before either renderer sees them.
    pub fn redact(&mut self, sensitive: &std::collections::BTreeSet<String>) {
        let overlaps = |s: &str| sensitive.iter().any(|v| !v.is_empty() && s.contains(v));
        let mut strings = vec![
            &mut self.repository_revision,
            &mut self.suite_digest,
            &mut self.descriptor_digest,
        ];
        strings.extend(self.archive_digest.iter_mut());
        strings.extend(self.parameter_names.iter_mut());
        if let Some(tool) = &mut self.tool {
            strings.extend([&mut tool.name, &mut tool.version]);
        }
        for (n, case) in self.cases.iter_mut().enumerate() {
            if overlaps(&case.id) {
                case.id = format!("case-{}", n + 1);
            }
            strings.extend([&mut case.id, &mut case.digest]);
            strings.extend(case.detail.iter_mut());
        }
        for s in strings {
            if overlaps(s) {
                *s = "<redacted>".into();
            }
        }
    }
    pub fn finish(&mut self) {
        self.totals = Totals::default();
        for c in &self.cases {
            match c.verdict {
                Verdict::Passed => self.totals.passed += 1,
                Verdict::Failed => self.totals.failed += 1,
                Verdict::Untested => self.totals.untested += 1,
            }
        }
    }
    pub fn markdown(&self) -> String {
        let contract = serde_json::to_value(self.contract).unwrap();
        let mut s = format!(
            "# Provider contract report\n\nRevision: `{}`. Contract: `{}`.\n\nSuite SHA-256: `{}`.\n\nArchive SHA-256: `{}`.\n\nDescriptor SHA-256: `{}`.\n",
            self.repository_revision,
            contract.as_str().unwrap(),
            self.suite_digest,
            self.archive_digest.as_deref().unwrap_or("unavailable"),
            self.descriptor_digest
        );
        if let Some(tool) = &self.tool {
            s += &format!("\nTool: `{}` version `{}`.\n", tool.name, tool.version);
        }
        s += &format!(
            "\nDescriptor parameter names: `{}`. Local locations: `<lab>`.\n\n| Case | SHA-256 | Verdict | First difference or why |\n| --- | --- | --- | --- |\n",
            self.parameter_names.join(", ")
        );
        for c in &self.cases {
            let verdict = match c.verdict {
                Verdict::Passed => "passed",
                Verdict::Failed => "failed",
                Verdict::Untested => "untested",
            };
            s += &format!(
                "| {} | `{}` | {} | {} |\n",
                c.id,
                c.digest,
                verdict,
                c.detail.as_deref().unwrap_or("—")
            );
        }
        s += &format!(
            "\nPassed: {}; failed: {}; untested: {}.\n",
            self.totals.passed, self.totals.failed, self.totals.untested
        );
        s
    }
}
