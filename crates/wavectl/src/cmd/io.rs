use std::fs::File;
use std::io::{self, Read};
use glob::glob;

pub enum InputItem {
    Stdin(String),
    FilePath(String, String),
}

pub fn read_inputs(inputs: &[String]) -> io::Result<Vec<InputItem>> {
    let mut out = Vec::new();
    if inputs.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "no --input provided"));
    }
    for s in inputs {
        if s == "-" {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            out.push(InputItem::Stdin(buf));
            continue;
        }
        if s.contains('*') || s.contains('?') || s.contains('[') {
            for entry in glob(s).expect("invalid glob pattern") {
                if let Ok(path) = entry {
                    if path.is_file() {
                        let mut buf = String::new();
                        File::open(&path)?.read_to_string(&mut buf)?;
                        out.push(InputItem::FilePath(path.to_string_lossy().into(), buf));
                    }
                }
            }
            continue;
        }
        let mut buf = String::new();
        File::open(s)?.read_to_string(&mut buf)?;
        out.push(InputItem::FilePath(s.clone(), buf));
    }
    Ok(out)
}
