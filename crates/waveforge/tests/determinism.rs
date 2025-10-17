use serde_json::json;
use waveforge::strict_nf::strict_nf_hex;

#[test]
fn nf_id_is_deterministic_and_order_invariant() {
    // Один и тот же узел с разным порядком ключей — ID должен совпадать
    let a = json!({
        "graph": { "nodes": [
            { "op":"W", "n_fft":1024, "hop":512, "window":"Hann" }
        ]}
    });

    let b = json!({
        "graph": { "nodes": [
            { "window":"Hann", "hop":512, "n_fft":1024, "op":"W" }
        ]}
    });

    let id1 = strict_nf_hex(&a).expect("id a");
    let id2 = strict_nf_hex(&b).expect("id b");
    assert_eq!(id1, id2, "NF-ID должен быть детерминированным и инвариантным к порядку ключей");
}
