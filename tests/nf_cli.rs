use std::process::Command;

fn bin() -> String {
    std::env::var("WAVECTL_BIN").unwrap_or_else(|_| "target/debug/wavectl".into())
}

#[test]
fn forge_check_roundtrip() {
    // forge --check should fail on non-canonical and pass on canonical
    let bin = bin();
    let src = "examples/graph/forge_eq_A.json";
    let output = Command::new(&bin)
        .args(["forge", "--check", "--input", src])
        .output()
        .expect("run forge --check on source");
    assert!(!output.status.success());

    let canon = "/tmp/_nf_cli_canon.json";
    let out = Command::new(&bin)
        .args(["forge", "--input", src, "--print-nf"])
        .output()
        .expect("run forge --print-nf");
    std::fs::write(canon, &out.stdout).unwrap();

    let output2 = Command::new(&bin)
        .args(["forge", "--check", "--input", canon])
        .output()
        .expect("run forge --check on canon");
    assert!(output2.status.success());
}

#[test]
fn nf_diff_equivalent_and_different() {
    let bin = bin();
    // A ~ B : equal
    let out = Command::new(&bin)
        .args(["nf-diff", "--left", "examples/graph/forge_eq_A.json", "--right", "examples/graph/forge_eq_B.json"])
        .output()
        .expect("run nf-diff A~B");
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("Canonical NF are identical"));

    // reflect != toeplitz : different -> non-zero with --fail-on-diff
    let out2 = Command::new(&bin)
        .args([
            "nf-diff",
            "--left", "examples/graph/forge_diff_pad_reflect.json",
            "--right", "examples/graph/forge_diff_pad_toeplitz.json",
            "--fail-on-diff",
        ])
        .output()
        .expect("run nf-diff reflect vs toeplitz");
    assert!(!out2.status.success());
}

#[test]
fn nf_diff_id_only_two_lines() {
    let bin = bin();
    let out = Command::new(&bin)
        .args([
            "nf-diff",
            "--left", "examples/graph/forge_eq_A.json",
            "--right", "examples/graph/forge_eq_B.json",
            "--id-only",
        ])
        .output()
        .expect("run nf-diff --id-only");
    let s = String::from_utf8_lossy(&out.stdout);
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(!lines[0].is_empty());
    assert_eq!(lines[0], lines[1]);
}
