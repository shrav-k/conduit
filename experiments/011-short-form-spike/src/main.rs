use short_form_spike::evidence::{Harness, STAGES, STYLES, access, counts, stable_changes};

fn main() -> Result<(), String> {
    let harness = Harness::new()?;
    let (mut passed, mut failed) = (0, 0);
    let (mut probe_passed, mut probe_failed) = (0, 0);
    for style in STYLES {
        for task in STAGES {
            match harness.check(style, task) {
                Ok(c) => {
                    println!("PASS exact {style}/{task}: {} documents", c.documents.len());
                    passed += 1;
                }
                Err(e) => {
                    println!("FAIL exact {style}/{task}: {e}");
                    failed += 1;
                }
            }
            match harness.without_as(style, task).and_then(|c| {
                access(&c, task)?;
                Ok(c.documents.len())
            }) {
                Ok(n) => {
                    println!("PASS without-as probe {style}/{task}: {n} documents");
                    probe_passed += 1;
                }
                Err(e) => {
                    eprintln!("FAIL without-as probe {style}/{task}: {e}");
                    probe_failed += 1;
                }
            }
        }
        println!("{style}: task file-edits commands digest-edits total");
        for (i, [files, commands, digests]) in counts(style)?.iter().enumerate() {
            println!(
                "T{} {files} {commands} {digests} {}",
                i + 1,
                files + commands
            );
        }
        for (old, new) in [("t4", "t5"), ("t5", "t6")] {
            let changed = stable_changes(
                &harness.without_as(style, old)?,
                &harness.without_as(style, new)?,
                new,
            )?;
            println!("PASS {style}/{new} stable-name comparison (without-as probe): {changed:?}");
        }
    }
    println!("Without-as probes: {probe_passed} passed; {probe_failed} failed");
    println!("Exact translation states: {passed} passed; {failed} failed");
    if failed > 0 || probe_failed > 0 {
        Err("translation spike found failures (see diagnostics)".into())
    } else {
        Ok(())
    }
}
