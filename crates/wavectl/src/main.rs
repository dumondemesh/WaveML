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
  /// Acceptance runner: читает YAML-план и гоняет PASS/FAIL
  Acceptance {
    /// План тестов (YAML)
    #[arg(long, default_value = "acceptance/tests.yaml")]
    plan: PathBuf,
    /// Директория для артефактов (index.md, .wfr, .err)
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
}
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Expect {
  PASS,
  FAIL,
}
fn default_expect() -> Expect { Expect::PASS }

fn cmd_acceptance(plan: PathBuf, outdir: PathBuf, strict: bool) -> Result<()> {
  // сразу после парсинга YAML:
  

  fs::create_dir_all(&outdir)?;
  let plan_s = fs::read_to_string(&plan)
      .with_context(|| format!("failed to read plan: {}", plan.display()))?;
  let tests: Vec<TestCase> = serde_yaml::from_str(&plan_s)
      .with_context(|| format!("failed to parse YAML plan: {}", plan.display()))?;

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
        rows.push(format!(
          "| {} | {:?} | ❌ | cannot read: {} |",
          t.name, t.expect, e
        ));
        mismatches.push(format!("{}: expected {:?}, got IO error ({})", t.name, t.expect, e));
        continue;
      }
    };

    let res = waveforge::compile(&code, strict);
    match res {
      Ok(ir) => {
        let got = Expect::PASS;
        if got == t.expect {
          passed += 1;
        } else {
          failed += 1;
          mismatches.push(format!("{}: expected {:?}, got PASS", t.name, t.expect));
        }
        let rep = wavereport::from_ir(&ir);
        let mut p = outdir.clone();
        p.push(format!("{}.wfr.json", t.name));
        let _ = wavereport::save_report_json(&rep, &p);
        rows.push(format!(
          "| {} | {:?} | {} | I1={} ops={:?} |",
          t.name,
          t.expect,
          if got == t.expect { "✅" } else { "❌" },
          rep.certificate.i1,
          rep.ops.get("counts").unwrap_or(&serde_json::json!({}))
        ));
      }
      Err(err) => {
        let got = Expect::FAIL;
        if got == t.expect {
          passed += 1;
        } else {
          failed += 1;
          mismatches.push(format!("{}: expected {:?}, got FAIL ({})", t.name, t.expect, err));
        }
        let mut p = outdir.clone();
        p.push(format!("{}.err.txt", t.name));
        let _ = fs::write(&p, format!("{err}\n"));
        rows.push(format!(
          "| {} | {:?} | {} | err saved: {} |",
          t.name,
          t.expect,
          if got == t.expect { "✅" } else { "❌" },
          p.display()
        ));
      }
    }
  }

  // Итоговый index.md
  let mut md = String::new();
  md.push_str("# Acceptance — summary\n\n");
  md.push_str(&format!(
    "**Всего:** {}  |  **Пройдено:** {}  |  **Провалено:** {}\n\n",
    passed + failed,
    passed,
    failed
  ));
  md.push_str(&rows.join("\n"));
  md.push('\n');

  let mut idx = outdir.clone();
  idx.push("index.md");
  fs::write(&idx, md).with_context(|| format!("failed to write {}", idx.display()))?;
  println!("Acceptance → {}", idx.display());

  if failed > 0 {
    eprintln!("Acceptance mismatches:");
    for m in &mismatches {
      eprintln!("  - {m}");
    }
    anyhow::bail!("acceptance failed: {} tests failed", failed);
  }

  Ok(())
}

fn cmd_pack(dir: PathBuf, out: PathBuf) -> Result<()> {
  fs::create_dir_all(&out)?;
  println!("(stub) Pack {} → {}", dir.display(), out.display());
  Ok(())
}
