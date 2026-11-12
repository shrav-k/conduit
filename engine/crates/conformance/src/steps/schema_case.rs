use super::*;

pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    let base = step.base;
    let case = step.case;
    let input = step.input;
    let kind = step.kind;
    let context = step.context;
    let spec = step.spec;
    let symbols = step.symbols;
    let next = step.state.clone();
    let actual = {
        let alias = input["file"].as_str().ok_or("missing schema fixture")?;
        let file = case.files.get(alias).ok_or("undeclared schema fixture")?;
        let fixture = expand(&read_value(&base.join(file))?, symbols, spec)?;
        let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        schemas::run(&repository, &fixture, kind, spec, context)?
    };
    Ok((actual, next))
}
