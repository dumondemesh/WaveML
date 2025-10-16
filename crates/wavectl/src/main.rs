use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "wavectl", version, about = "WaveML CLI (proto)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile WML → WMLB (JSON stub)
    Compile {
        src: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
        #[arg(long, default_value_t = false)]
        strict: bool,
    },
    /// Run IR on WaveForm
    Run {
        ir: PathBuf,
        #[arg(long, value_name = "INPUT")]
        r#in: PathBuf,
        #[arg(long, value_name = "OUTPUT")]
        out: PathBuf,
    },
    /// Generate report .wfr.json (читает IR и строит сводку/сертификат)
    Report {
        ir: PathBuf,
        #[arg(long, default_value = "build/reports")]
        emit: PathBuf,
        #[arg(long, default_value = "I")]
        cert: String,
    },
    /// Acceptance runner: читает YAML-план и гоняет PASS/FAIL (+ optional run)
    Acceptance {
        /// План тестов (YAML)
        #[arg(long, default_value = "acceptance/tests.yaml")]
        plan: PathBuf,
        /// Директория для артефактов (index.md, .wfr, .err, .out)
        #[arg(long, default_value = "build/acceptance")]
        outdir: PathBuf,
        /// Строгий режим компиляции (включает линтеры)
        #[arg(long, default_value_t = true)]
        strict: bool,
    },
    /// Package module (stub)
    Pack {
        dir: PathBuf,
        #[arg(short, long, default_value = "build/pkg")]
        out: PathBuf,
    },
}

fn main() {
    env_logger::init();
    if let Err(err) = try_main() {
        eprintln!("Error: {err}");
        for cause in err.chain().skip(1) {
            eprintln!("  caused by: {cause}");
        }
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Compile { src, out, strict } => cmd_compile(src, out, strict),
        Commands::Run { ir, r#in, out } => cmd_run(ir, r#in, out),
        Commands::Report { ir, emit, cert } => cmd_report(ir, emit, cert),
        Commands::Acceptance { plan, outdir, strict } => cmd_acceptance(plan, outdir, strict),
        Commands::Pack { dir, out } => cmd_pack(dir, out),
    }
}

fn cmd_compile(src: PathBuf, out: PathBuf, strict: bool) -> Result<()> {
    let code = fs::read_to_string(&src)
        .with_context(|| format!("failed to read WML: {}", src.display()))?;
    let g = waveforge::compile(&code, strict)?;
    fs::create_dir_all(out.parent().unwrap_or(Path::new(".")))?;
    fs::write(&out, serde_json::to_string_pretty(&g)?)
        .with_context(|| format!("failed to write IR: {}", out.display()))?;
    println!("IR → {}", out.display());
    Ok(())
}

fn cmd_run(ir: PathBuf, input: PathBuf, out: PathBuf) -> Result<()> {
    let g: wmlb::Graph = serde_json::from_str(
        &fs::read_to_string(&ir).with_context(|| format!("failed to read IR: {}", ir.display()))?,
    )?;
    let wf = waveform::WaveForm::load_json(&input)
        .with_context(|| format!("failed to read WaveForm: {}", input.display()))?;
    let out_wf = waverunner::run(&g, &wf)?;
    out_wf
        .save_json(&out)
        .with_context(|| format!("failed to write WaveForm: {}", out.display()))?;
    println!("WaveForm → {}", out.display());
    Ok(())
}

fn cmd_report(ir: PathBuf, emit: PathBuf, cert: String) -> Result<()> {
    fs::create_dir_all(&emit)?;
    let g: wmlb::Graph = serde_json::from_str(
        &fs::read_to_string(&ir).with_context(|| format!("failed to read IR: {}", ir.display()))?,
    )?;
    let rep = wavereport::from_ir(&g);
    let mut path = emit.clone();
    path.push(format!("cert-{}.wfr.json", cert));
    wavereport::save_report_json(&rep, &path)
        .with_context(|| format!("failed to write report: {}", path.display()))?;
    println!("Report → {}", path.display());
    Ok(())
}

#[derive(Debug, Deserialize)]
struct TestCase {
    name: String,
    src: PathBuf,
    #[serde(default = "default_expect")]
    expect: Expect,
    #[serde(default)]
    run: Option<RunSpec>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Expect {
    PASS,
    FAIL,
}
fn default_expect() -> Expect { Expect::PASS }

#[derive(Debug, Deserialize)]
struct RunSpec {
    input: PathBuf,
    #[serde(default)]
    expect: Option<RunExpect>,
}

#[derive(Debug, Deserialize, Default)]
struct RunExpect {
    #[serde(default)]
    rate_div: Option<f64>,
    #[serde(default)]
    len_div: Option<f64>,
    #[serde(default)]
    len_eq: Option<bool>,
    #[serde(default)]
    mse_max: Option<f64>,
}

fn cmd_acceptance(plan: PathBuf, outdir: PathBuf, strict: bool) -> Result<()> {
    fs::create_dir_all(&outdir)?;
    let plan_s = fs::read_to_string(&plan)
        .with_context(|| format!("failed to read plan: {}", plan.display()))?;
    let tests: Vec<TestCase> = serde_yaml::from_str(&plan_s)
        .with_context(|| format!("failed to parse YAML plan: {}", plan.display()))?;

    // Пред-проверка файлов
    let missing: Vec<_> = tests
        .iter()
        .filter(|t| !t.src.exists())
        .map(|t| format!("{} -> {}", t.name, t.src.display()))
        .collect();
    if !missing.is_empty() {
        eprintln!("Plan has missing sources:");
        for m in &missing { eprintln!("  - {m}"); }
        anyhow::bail!("acceptance plan invalid: {} missing input files", missing.len());
    }

    let mut rows: Vec<String> = Vec::new();
    rows.push("| Тест | Expect | Result | Заметка |".into());
    rows.push("|------|--------|--------|---------|".into());

    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut mismatches: Vec<String> = Vec::new();

    for t in tests {
        let code = match fs::read_to_string(&t.src) {
            Ok(s) => s,
            Err(e) => {
                failed += 1;
                rows.push(format!("| {} | {:?} | ❌ | cannot read: {} |", t.name, t.expect, e));
                mismatches.push(format!("{}: expected {:?}, got IO error ({})", t.name, t.expect, e));
                continue;
            }
        };

        // Компиляция
        let res = waveforge::compile(&code, strict);
        match res {
            Ok(ir) => {
                let mut test_ok = true;
                let mut note = String::new();

                if let Some(run_spec) = t.run.as_ref() {
                    let in_wf = match waveform::WaveForm::load_json(&run_spec.input) {
                        Ok(w) => w,
                        Err(e) => {
                            note = format!("run: cannot read input: {}", e);
                            mismatches.push(format!("{}: run input error ({})", t.name, e));
                            rows.push(format!("| {} | {:?} | ❌ | {} |", t.name, t.expect, note));
                            if t.expect == Expect::PASS { failed += 1; } else { passed += 1; }
                            continue;
                        }
                    };

                    let out_wf = match waverunner::run(&ir, &in_wf) {
                        Ok(w) => w,
                        Err(e) => {
                            if t.expect == Expect::PASS {
                                failed += 1;
                                mismatches.push(format!("{}: expected PASS, got RUN FAIL ({})", t.name, e));
                                rows.push(format!("| {} | {:?} | ❌ | run failed: {} |", t.name, t.expect, e));
                            } else {
                                passed += 1;
                                rows.push(format!("| {} | {:?} | ✅ | run failed as expected |", t.name, t.expect));
                            }
                            continue;
                        }
                    };

                    if let Some(exp) = run_spec.expect.as_ref() {
                        let mut checks: Vec<String> = Vec::new();
                        // rate
                        if let (Some(in_rate), Some(rate_div)) = (in_wf.header.rate, exp.rate_div) {
                            let want = ((in_rate as f64) / rate_div).round() as u32;
                            let got = out_wf.header.rate.unwrap_or(0);
                            if got != want { test_ok = false; checks.push(format!("rate got={}, want={}", got, want)); }
                        }
                        // len div → ceil, как в D()
                        let in_len = mono_len(&in_wf);
                        let out_len = mono_len(&out_wf);
                        if let Some(len_div) = exp.len_div {
                            let want = ((in_len as f64) / len_div).ceil() as usize;
                            if out_len != want { test_ok = false; checks.push(format!("len got={}, want={}", out_len, want)); }
                        }
                        // len equality
                        if let Some(true) = exp.len_eq {
                            if out_len != in_len { test_ok = false; checks.push(format!("len_eq failed: in={}, out={}", in_len, out_len)); }
                        }
                        // MSE
                        if let Some(mse_thr) = exp.mse_max {
                            let mse = mse_mono(&in_wf, &out_wf);
                            if !mse.is_finite() || mse > mse_thr {
                                test_ok = false;
                                checks.push(format!("mse {:.6e} > {:.6e}", mse, mse_thr));
                            } else {
                                note = format!("mse={:.6e}", mse);
                            }
                        }
                        if checks.is_empty() && note.is_empty() {
                            note = format!("run ok: rate={}, len={}", out_wf.header.rate.unwrap_or(0), out_len);
                        } else if !checks.is_empty() {
                            note = format!("run mismatch: {}", checks.join(", "));
                        }
                    }

                    // Сохраняем аутпут
                    let mut outp = outdir.clone();
                    outp.push(format!("{}.out.wfm.json", t.name));
                    let _ = out_wf.save_json(&outp);
                }

                let got = if test_ok { Expect::PASS } else { Expect::FAIL };
                if got == t.expect { passed += 1; } else { failed += 1; mismatches.push(format!("{}: expected {:?}, got {:?}", t.name, t.expect, got)); }

                let rep = wavereport::from_ir(&ir);
                let mut p = outdir.clone();
                p.push(format!("{}.wfr.json", t.name));
                let _ = wavereport::save_report_json(&rep, &p);

                rows.push(format!(
                    "| {} | {:?} | {} | {} I1={} |",
                    t.name,
                    t.expect,
                    if got == t.expect { "✅" } else { "❌" },
                    if note.is_empty() { String::from("") } else { format!("{}; ", note) },
                    rep.certificate.i1,
                ));
            }
            Err(err) => {
                let got = Expect::FAIL;
                if got == t.expect { passed += 1; } else { failed += 1; mismatches.push(format!("{}: expected {:?}, got FAIL ({})", t.name, t.expect, err)); }
                let mut p = outdir.clone();
                p.push(format!("{}.err.txt", t.name));
                let _ = fs::write(&p, format!("{err}\n"));
                rows.push(format!("| {} | {:?} | {} | err saved: {} |", t.name, t.expect, if got == t.expect { "✅" } else { "❌" }, p.display()));
            }
        }
    }

    let mut md = String::new();
    md.push_str("# Acceptance — summary\n\n");
    md.push_str(&format!("**Всего:** {}  |  **Пройдено:** {}  |  **Провалено:** {}\n\n", passed + failed, passed, failed));
    md.push_str(&rows.join("\n"));
    md.push('\n');

    let mut idx = outdir.clone();
    idx.push("index.md");
    fs::write(&idx, md).with_context(|| format!("failed to write {}", idx.display()))?;
    println!("Acceptance → {}", idx.display());

    if failed > 0 {
        eprintln!("Acceptance mismatches:");
        for m in &mismatches { eprintln!("  - {m}"); }
        anyhow::bail!("acceptance failed: {} tests failed", failed);
    }

    Ok(())
}

fn mono_len(wf: &waveform::WaveForm) -> usize {
    wf.tracks
        .as_object()
        .and_then(|o| o.get("mono"))
        .and_then(|a| a.as_array())
        .map(|a| a.len())
        .unwrap_or(0)
}

fn mse_mono(a: &waveform::WaveForm, b: &waveform::WaveForm) -> f64 {
    let av = a.tracks.as_object().and_then(|o| o.get("mono")).and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let bv = b.tracks.as_object().and_then(|o| o.get("mono")).and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let n = av.len().min(bv.len());
    if n == 0 { return f64::NAN; }
    let mut se = 0.0;
    for i in 0..n {
        let ai = av[i].as_f64().unwrap_or(0.0);
        let bi = bv[i].as_f64().unwrap_or(0.0);
        let d = ai - bi;
        se += d * d;
    }
    se / (n as f64)
}

fn cmd_pack(dir: PathBuf, out: PathBuf) -> Result<()> {
    fs::create_dir_all(&out)?;
    println!("(stub) Pack {} → {}", dir.display(), out.display());
    Ok(())
}
