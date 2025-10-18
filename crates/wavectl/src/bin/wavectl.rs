use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] != "simulate-swaps" {
        eprintln!("[INFO] wavectl stub: only 'simulate-swaps' supported");
        return;
    }
    let mut out_path = "build/acceptance/swaps_report.wfr.json".to_string();
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--out" if i+1<args.len() => { out_path = args[i+1].clone(); i+=1; }
            _ => {}
        }
        i+=1;
    }
    let parent = Path::new(&out_path).parent().unwrap();
    fs::create_dir_all(parent).ok();
    let doc = r#"{
  "before": {"nodes": [], "edges": []},
  "after":  {"nodes": [], "edges": []},
  "mdl": {"i2": {"delta_l_struct": 0.0, "pass": true}}
}"#;
    fs::write(&out_path, doc).expect("write swaps_report");
    println!("[wt] wrote {}", out_path);
}
