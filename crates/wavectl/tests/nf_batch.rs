use std::path::PathBuf;
use std::process::{Command, Stdio};
use serde_json::Value;

fn ws_root() -> PathBuf {
    // crates/wavectl -> workspace root
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    here.parent().unwrap().parent().unwrap().to_path_buf()
}

fn wavectl_bin() -> PathBuf {
    if let Ok(p) = std::env::var("WAVECTL_BIN") {
        return PathBuf::from(p);
    }
    ws_root().join("target").join("debug").join("wavectl")
}

fn run<I, S>(args: I) -> (i32, String, String)
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let bin = wavectl_bin();
    let out = Command::new(&bin)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("spawn wavectl");
    let code = out.status.code().unwrap_or(1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    (code, stdout, stderr)
}

fn ex(p: &str) -> String {
    ws_root().join(p).to_str().unwrap().to_string()
}

#[test]
fn nf_batch_equivalent_same_id_json() {
    let (code, stdout, stderr) = run([
        "nf-batch",
        "--input", &ex("examples/graph/forge_eq_A.json"),
        "--input", &ex("examples/graph/forge_eq_B.json"),
        "--input", &ex("examples/graph/forge_eq_synonyms.json"),
        "--json",
    ]);
    assert_eq!(code, 0, "nf-batch --json should exit 0, stderr:\n{stderr}");
    let v: Value = serde_json::from_str(&stdout).expect("json parse");
    let items = v.get("items").and_then(|x| x.as_array()).expect("items[]");
    assert_eq!(items.len(), 3, "expect 3 items");

    let ids: Vec<String> = items.iter()
        .map(|it| it.get("nf_id").and_then(|x| x.as_str()).unwrap().to_string())
        .collect();

    assert!(ids.windows(2).all(|w| w[0] == w[1]),
            "all nf_id must be equal, got: {:?}", ids);
}

#[test]
fn nf_batch_csv_has_header() {
    let (code, stdout, stderr) = run([
        "nf-batch",
        "--input", &ex("examples/graph/forge_eq_A.json"),
        "--input", &ex("examples/graph/forge_eq_B.json"),
        "--csv",
    ]);
    assert_eq!(code, 0, "nf-batch --csv should exit 0, stderr:\n{stderr}");
    let mut lines = stdout.lines();
    assert_eq!(lines.next(), Some("input,nf_id"), "must have CSV header");
    let rest: Vec<&str> = lines.filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(rest.len(), 2, "should have 2 data rows, got:\n{}", stdout);
}

#[test]
fn nf_batch_list_file_json() {
    // временный список в target/
    let list_path = ws_root().join("target").join("tmp_nf_list.txt");
    std::fs::create_dir_all(list_path.parent().unwrap()).ok();
    std::fs::write(
        &list_path,
        format!(
            "{}\n# comment\n{}\n{}\n",
            ex("examples/graph/forge_eq_A.json"),
            ex("examples/graph/forge_eq_B.json"),
            ex("examples/graph/forge_eq_synonyms.json"),
        ),
    ).expect("write list");

    let (code, stdout, stderr) = run([
        "nf-batch",
        "--list", list_path.to_str().unwrap(),
        "--json",
    ]);
    assert_eq!(code, 0, "nf-batch --list --json should exit 0, stderr:\n{stderr}");
    let v: Value = serde_json::from_str(&stdout).expect("json parse");
    let items = v.get("items").and_then(|x| x.as_array()).expect("items[]");
    assert_eq!(items.len(), 3, "expect 3 items from list");
}
