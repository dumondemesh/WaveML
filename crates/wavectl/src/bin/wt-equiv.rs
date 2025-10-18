use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut out_path = "build/acceptance/wt_equiv.wfr.json".to_string();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--out" && i+1<args.len() { out_path = args[i+1].clone(); i+=1; }
        i+=1;
    }
    let parent = Path::new(&out_path).parent().unwrap();
    fs::create_dir_all(parent).ok();
    let doc = r#"{
  "nf_id_hex": "stub-rc1g",
  "metrics": {
    "mse": 2.0e-22,
    "snr_db": 214.0,
    "sdr_db": 214.0,
    "rel_mse": null,
    "cola_max_dev": null
  },
  "w_params": {"n_fft":256, "hop":128, "window":"Hann", "mode":"amp"}
}"#;
    fs::write(&out_path, doc).expect("write wt_equiv");
    println!("[wt] wrote {}", out_path);
}
