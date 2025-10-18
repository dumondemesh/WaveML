use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
struct Plan {
    tests: Vec<TestCase>
}
#[derive(Deserialize)]
struct TestCase {
    id: String,
    kind: String,
    input: String,
}

pub fn run(plan_path: &str, outdir: &str) -> Result<()> {
    let raw = fs::read_to_string(plan_path).with_context(|| format!("read {plan_path}"))?;
    let plan: Plan = serde_yaml::from_str(&raw).context("parse yaml plan")?;
    fs::create_dir_all(outdir).ok();

    for t in plan.tests {
        let out = format!("{}/{}.wfr.json", outdir, t.id);
        crate::simulate_swaps::run(&t.input, &out, true)?;
        assert!(Path::new(&out).exists(), "missing wfr for {}", t.id);
    }
    Ok(())
}
