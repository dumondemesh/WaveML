use serde_json::{json, Value};
use waveforge::strict_nf::{strict_nf, strict_nf_hex};

fn nodes(v: &Value) -> &Vec<Value> {
    v.get("graph")
        .and_then(|g| g.get("nodes"))
        .and_then(|n| n.as_array())
        .expect("graph.nodes[]")
}

#[test]
fn strict_nf_produces_canonical_form() {
    let src = json!({
        "graph": { "nodes": [
            { "op":"W", "n_fft":1024, "hop":512, "window":"Hann", "center": true,  "pad_mode":"reflect" },
            { "op":"W", "n_fft":2048, "hop":1024,"window":"Hann", "center": true,  "pad_mode":"reflect" }
        ]}
    });

    let nf = strict_nf(&src).expect("strict_nf");
    let ns = nodes(&nf);

    // Проверим ключевые канонические поля, как договорились в strict-NF:
    for n in ns {
        assert_eq!(n.get("op").and_then(|s| s.as_str()), Some("W"));
        assert_eq!(n.get("center").and_then(|b| b.as_bool()), Some(false));
        assert_eq!(n.get("pad_mode").and_then(|s| s.as_str()), Some("toeplitz"));
        // базовые поля должны сохраниться
        assert!(n.get("n_fft").is_some());
        assert!(n.get("hop").is_some());
        assert_eq!(n.get("window").and_then(|s| s.as_str()), Some("Hann"));
    }
}

#[test]
fn strict_nf_hex_matches_self() {
    let src = json!({
        "graph": { "nodes": [
            { "op":"W", "n_fft":1024, "hop":512, "window":"Hann" }
        ]}
    });

    let id1 = strict_nf_hex(&src).expect("id1");
    let id2 = strict_nf_hex(&src).expect("id2");
    assert_eq!(id1, id2, "ID должен быть детерминированным");
}
