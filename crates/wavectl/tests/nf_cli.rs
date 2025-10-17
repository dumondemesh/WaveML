use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

/// Корень workspace: <repo>/
fn ws_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // crates/wavectl
    manifest
        .parent()
        .and_then(|p| p.parent())
        .unwrap_or(&manifest)
        .to_path_buf()
}

/// Путь к примеру из examples/graph/
fn example(name: &str) -> String {
    ws_root()
        .join("examples")
        .join("graph")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

/// Находим бинарник wavectl:
/// 1) WAVECTL_BIN (absolute / relative / от корня workspace)
/// 2) CARGO_TARGET_DIR или <ws_root>/target/debug/wavectl
fn wavectl_bin() -> PathBuf {
    if let Ok(val) = env::var("WAVECTL_BIN") {
        let p = PathBuf::from(&val);
        if p.is_absolute() && p.exists() {
            return p;
        }
        if p.exists() {
            return p;
        }
        let ws_try = ws_root().join(&p);
        if ws_try.exists() {
            return ws_try;
        }
        panic!(
            "WAVECTL_BIN='{}' не найден (пробовали: как есть, как абсолютный, и от ws-root: {})",
            val,
            ws_try.display()
        );
    }

    let target_dir = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| ws_root().join("target"));
    target_dir.join("debug").join(if cfg!(windows) {
        "wavectl.exe"
    } else {
        "wavectl"
    })
}

/// Запуск wavectl c аргументами, возвращаем (код, stdout, stderr)
fn run(args: &[&str]) -> (i32, String, String) {
    let output = Command::new(wavectl_bin())
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("spawn wavectl");
    let code = output.status.code().unwrap_or(-1);
    let out = String::from_utf8_lossy(&output.stdout).to_string();
    let err = String::from_utf8_lossy(&output.stderr).to_string();
    (code, out, err)
}

/// Достаём 64-символьный hex из текста (NF-ID)
fn extract_hex64(s: &str) -> Option<String> {
    let mut last: Option<String> = None;
    for tok in s.split_whitespace() {
        if tok.len() == 64 && tok.chars().all(|c| c.is_ascii_hexdigit()) {
            last = Some(tok.to_lowercase());
        }
    }
    last
}

/// Уникальное имя файла в /tmp (без сторонних зависимостей)
fn tmp_path(name: &str, ext: &str) -> PathBuf {
    let ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    env::temp_dir().join(format!("{name}_{ns}.{ext}"))
}

#[test]
fn forge_explain_works() {
    let (code, out, err) = run(&["forge-explain", "--input", &example("forge_eq_A.json")]);
    assert_eq!(
        code, 0,
        "forge-explain должен завершаться 0\nstdout:\n{out}\nstderr:\n{err}"
    );
    assert!(
        out.contains("NF-ID"),
        "ожидали увидеть 'NF-ID' в выводе forge-explain\nstdout:\n{out}"
    );
}

#[test]
fn forge_is_deterministic() {
    let (_c1, o1, _e1) = run(&["forge", "--input", &example("forge_eq_A.json"), "--print-id"]);
    let (_c2, o2, _e2) = run(&["forge", "--input", &example("forge_eq_A.json"), "--print-id"]);
    let id1 = extract_hex64(&o1).expect("id1");
    let id2 = extract_hex64(&o2).expect("id2");
    assert_eq!(id1, id2, "NF-ID должны совпасть при повторном прогоне");
}

#[test]
fn nf_diff_equivalent_are_identical() {
    let (code, out, err) = run(&[
        "nf-diff",
        "--left",
        &example("forge_eq_A.json"),
        "--right",
        &example("forge_eq_B.json"),
    ]);
    assert_eq!(
        code, 0,
        "nf-diff для эквивалентных должен быть 0\nstdout:\n{out}\nstderr:\n{err}"
    );
    assert!(
        out.contains("identical"),
        "ожидали 'identical' в выводе nf-diff эквивалентных\nstdout:\n{out}"
    );
}

#[test]
fn nf_diff_detects_difference() {
    let (code, out, err) = run(&[
        "nf-diff",
        "--left",
        &example("forge_diff_pad_reflect.json"),
        "--right",
        &example("forge_diff_pad_toeplitz.json"),
        "--fail-on-diff",
        "--show-source-diff",
    ]);
    assert_ne!(
        code, 0,
        "nf-diff должен возвращать ошибку для различий\nstdout:\n{out}\nstderr:\n{err}"
    );
    assert!(
        out.contains("NF-ID differ") || out.contains("Differences"),
        "ожидали сообщение о различиях\nstdout:\n{out}"
    );
}

#[test]
fn forge_check_non_canonical_fails() {
    let (code, out, err) = run(&["forge", "--check", "--input", &example("forge_eq_A.json")]);
    assert_ne!(code, 0, "forge --check на неканоничном должен падать");
    let combined = format!("{out}\n{err}");
    assert!(
        combined.contains("not canonical") || combined.contains("differs from canonical"),
        "ожидали подсказку, что вход не каноничен\nstdout:\n{out}\nstderr:\n{err}"
    );
}

#[test]
fn forge_check_on_canonical_passes() {
    // Получаем каноническую NF в stdout и сохраняем во временный файл
    let (c1, nf_json, err1) = run(&["forge", "--input", &example("forge_eq_A.json"), "--print-nf"]);
    assert_eq!(c1, 0, "forge --print-nf должен завершаться 0\nstderr:\n{err1}");
    let canon_path = tmp_path("canon_nf", "json");
    fs::write(&canon_path, nf_json).expect("write canon nf");

    // Проверяем, что --check проходит
    let (c2, out2, err2) = run(&["forge", "--check", "--input", canon_path.to_str().unwrap()]);
    assert_eq!(
        c2, 0,
        "forge --check должен пройти на каноническом\nstdout:\n{out2}\nstderr:\n{err2}"
    );
    assert!(
        out2.contains("already canonical"),
        "ожидали сообщение 'already canonical'\nstdout:\n{out2}"
    );
}
