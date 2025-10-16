//! WaveForge — компилятор: WML → WMLB (v0.2: многострочный парсинг аргументов W/D, Unicode-safe)
use anyhow::*;
use serde_json::{Map, Value};
use wmlb::{Graph, Node};

/// Главная функция компиляции
pub fn compile(src: &str, strict: bool) -> Result<Graph> {
    if strict {
        wavelint::all(src)?;
    }
    let code = strip_line_comments(src);

    let mut g = Graph::new();
    let mut id = 0usize;

    // 1) W(...)
    for args in find_sections(&code, "W") {
        let params = Value::Object(parse_args(&args)?);
        id += 1;
        g.nodes.push(Node {
            id: format!("w{}", id),
            op: "W".into(),
            params,
            inputs: vec![],
            outputs: vec![format!("w{}", id)],
        });
    }

    // 2) D(...)
    for args in find_sections(&code, "D") {
        let params = Value::Object(parse_args(&args)?);
        id += 1;
        // подключаемся к последнему выходу, если он есть
        let input = g
            .nodes
            .last()
            .and_then(|n| n.outputs.last().cloned())
            .unwrap_or_else(|| "x".into());
        g.nodes.push(Node {
            id: format!("d{}", id),
            op: "D".into(),
            params,
            inputs: vec![input],
            outputs: vec![format!("d{}", id)],
        });
    }

    // 3) T() — без аргументов (v0.2 просто обнаруживаем присутствие)
    if code.contains("T(") {
        id += 1;
        let input = g
            .nodes
            .last()
            .and_then(|n| n.outputs.last().cloned())
            .unwrap_or_else(|| "x".into());
        g.nodes.push(Node {
            id: format!("t{}", id),
            op: "T".into(),
            params: Value::Object(Map::new()),
            inputs: vec![input],
            outputs: vec![format!("t{}", id)],
        });
    }

    Ok(g)
}

/// Удаляем `//` комментарии (до конца строки)
fn strip_line_comments(src: &str) -> String {
    src.lines()
        .map(|line| line.split_once("//").map(|(l, _)| l).unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Находит участки вида `OP( … )` с учётом многострочности, кавычек и вложенных скобок
fn find_sections(code: &str, op: &str) -> Vec<String> {
    let pat = format!("{op}(");
    let chars: Vec<char> = code.chars().collect();
    let mut i = 0usize;
    let mut out = Vec::new();

    while i < chars.len() {
        // ищем начало OP(
        if i + pat.len() <= chars.len() && chars[i..i + pat.len()].iter().collect::<String>() == pat
        {
            let mut depth = 1i32;
            let mut j = i + pat.len(); // позиция после '('
            let mut in_s = false; // в одинарных кавычках
            let mut in_d = false; // в двойных кавычках
            while j < chars.len() {
                let c = chars[j];
                match c {
                    '\'' if !in_d => {
                        in_s = !in_s;
                    }
                    '"' if !in_s => {
                        in_d = !in_d;
                    }
                    '(' if !in_s && !in_d => {
                        depth += 1;
                    }
                    ')' if !in_s && !in_d => {
                        depth -= 1;
                        if depth == 0 {
                            // args между i+pat.len() и j
                            let args = chars[i + pat.len()..j].iter().collect::<String>();
                            out.push(args);
                            i = j + 1;
                            break;
                        }
                    }
                    _ => {}
                }
                j += 1;
            }
            // если вышли из while без break — незакрытая скобка, пропускаем
        } else {
            i += 1;
        }
    }
    out
}

/// Разбивает строку по запятым, игнорируя запятые внутри кавычек
fn split_commas_outside_quotes(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let (mut in_s, mut in_d) = (false, false);
    for c in s.chars() {
        match c {
            '\'' if !in_d => {
                in_s = !in_s;
                cur.push(c);
            }
            '"' if !in_s => {
                in_d = !in_d;
                cur.push(c);
            }
            ',' if !in_s && !in_d => {
                if !cur.trim().is_empty() {
                    out.push(cur.trim().to_string());
                }
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

/// Делит строку по первому разделителю вне кавычек (байтовый оффсет, Unicode-safe).
fn split_once_outside_quotes(s: &str, sep: char) -> Option<(&str, &str)> {
    let (mut in_s, mut in_d) = (false, false);
    for (i, c) in s.char_indices() {
        match c {
            '\'' if !in_d => in_s = !in_s,
            '"' if !in_s => in_d = !in_d,
            _ if c == sep && !in_s && !in_d => {
                // i — БАЙТОВЫЙ индекс начала `sep`
                let after = i + sep.len_utf8();
                return Some((&s[..i], &s[after..]));
            }
            _ => {}
        }
    }
    None
}

/// Разбираем аргументы вида `key=value, key="str", 'ключ'=123` → JSON-Map
fn parse_args(args: &str) -> Result<Map<String, Value>> {
    // разбиваем по запятым вне кавычек
    let parts = split_commas_outside_quotes(args);
    let mut map = Map::new();

    for raw in parts {
        // поддерживаем и '=' и ':' как разделители ключ/значение
        let kv = split_once_outside_quotes(&raw, '=')
            .or_else(|| split_once_outside_quotes(&raw, ':'))
            .ok_or_else(|| anyhow!("некорректный аргумент: `{}`", raw))?;

        let (k, v) = kv;
        let key = normalize_key(k.trim().trim_matches(&['"', '\''][..]));
        let val_str = v.trim();
        let val = parse_value(val_str);

        map.insert(key, val);
    }
    Ok(map)
}

fn normalize_key(k: &str) -> String {
    if k == "λ" || k.eq_ignore_ascii_case("lambda") {
        "lambda".into()
    } else if k == "Φ" || k.eq_ignore_ascii_case("phi") {
        "phi".into()
    } else {
        k.to_string()
    }
}

fn parse_value(s: &str) -> Value {
    let t = s.trim();
    // строки в кавычках
    if (t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')) {
        return Value::String(t[1..t.len() - 1].to_string());
    }
    // bool
    if t.eq_ignore_ascii_case("true") {
        return Value::Bool(true);
    }
    if t.eq_ignore_ascii_case("false") {
        return Value::Bool(false);
    }
    // null
    if t.eq_ignore_ascii_case("null") {
        return Value::Null;
    }
    // число
    if let std::result::Result::Ok(num) = t.parse::<f64>() {
        return Value::from(num);
    }
    // fallback — как строка без кавычек
    Value::String(t.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn find_sections_multiline() {
        let s = r#"
        x = W(
            bank="stft",
            edge="reflect"
        )(x)
        "#;
        let v = find_sections(s, "W");
        assert_eq!(v.len(), 1);
        assert!(v[0].contains("bank=\"stft\""));
        assert!(v[0].contains("edge=\"reflect\""));
    }

    #[test]
    fn parse_args_lambda_aa() {
        let args = r#"λ=2, aa="sinc""#;
        let kv = parse_args(args).unwrap();
        assert_eq!(kv.get("lambda").unwrap(), &json!(2.0)); // λ → lambda
        assert_eq!(kv.get("aa").unwrap(), &json!("sinc"));
    }

    #[test]
    fn compile_parses_w_d_t() {
        let s = r#"
            input x: WaveForm(domain="audio")
            x = W(bank="stft", edge="Toeplitz")(x)
            x = D(λ=2, aa="sinc")(x)
            y = T()(x)
        "#;
        // strict включает линтеры R7/R8 — кейс корректный
        let g = compile(s, true).unwrap();
        assert!(g.nodes.iter().any(|n| n.op == "W"));
        assert!(g.nodes.iter().any(|n| n.op == "D"));
        assert!(g.nodes.iter().any(|n| n.op == "T"));
    }
}
