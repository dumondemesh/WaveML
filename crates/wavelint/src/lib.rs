//! Линтеры R7/R8 для WaveML (v0.2, Unicode/multiline-safe)
use anyhow::{bail, Result};

#[derive(Debug, Default, Clone, Copy)]
pub struct LintConfig {
    /// Разрешать zero-pad только если явно allow_zero_pad=true (по умолчанию нельзя)
    pub allow_zero_pad: bool,
}

pub fn all(src: &str) -> Result<()> {
    let code = strip_line_comments(src);
    let cfg = LintConfig::default();
    check_r7(&code, cfg)?;
    check_r8(&code)?;
    Ok(())
}

/// Удаляем `//` комментарии (до конца строки).
fn strip_line_comments(src: &str) -> String {
    src.lines()
        .map(|line| line.split_once("//").map(|(l, _)| l).unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Находит участки вида `OP( … )` с учётом многострочности, кавычек и вложенных скобок.
/// Возвращает список **строк аргументов** внутри скобок.
fn find_sections(code: &str, op: &str) -> Vec<String> {
    let pat = format!("{op}(");
    let chars: Vec<char> = code.chars().collect();
    let mut i = 0usize;
    let mut out = Vec::new();

    while i < chars.len() {
        if i + pat.len() <= chars.len() && chars[i..i + pat.len()].iter().collect::<String>() == pat
        {
            let mut depth = 1i32;
            let mut j = i + pat.len();
            let mut in_s = false; // '
            let mut in_d = false; // "
            while j < chars.len() {
                let c = chars[j];
                match c {
                    '\'' if !in_d => in_s = !in_s,
                    '"' if !in_s => in_d = !in_d,
                    '(' if !in_s && !in_d => depth += 1,
                    ')' if !in_s && !in_d => {
                        depth -= 1;
                        if depth == 0 {
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
            i = i.saturating_add(1);
        } else {
            i += 1;
        }
    }
    out
}

/// R7: edge ∈ {reflect, Toeplitz}; zero-pad запрещён по умолчанию.
fn check_r7(src: &str, cfg: LintConfig) -> Result<()> {
    for args in find_sections(src, "W") {
        if let Some((s, e)) = find_value_span(&args, "edge", '=') {
            let val = unquote(args[s..e].trim());
            if val == "zero" && !cfg.allow_zero_pad {
                bail!("R7 violation: edge=\"zero\" запрещён. Используйте edge=\"reflect\" или edge=\"Toeplitz\".");
            }
            if !(val == "reflect" || val == "Toeplitz" || val == "zero") {
                bail!(format!(
                    "R7 violation: недопустимое значение edge=\"{}\" (разрешены: reflect | Toeplitz).",
                    val
                ));
            }
        }
    }
    Ok(())
}

/// R8: любой D(λ=...|lambda=...) ДОЛЖЕН иметь `aa=...` внутри скобок `D(...)`.
fn check_r8(src: &str) -> Result<()> {
    for args in find_sections(src, "D") {
        let has_lambda = find_value_span(&args, "λ", '=').is_some()
            || find_value_span(&args, "lambda", '=').is_some();
        let has_aa = find_value_span(&args, "aa", '=').is_some()
            || find_value_span(&args, "aa", ':').is_some();
        if has_lambda && !has_aa {
            bail!(format!(
                "R8 violation: Downsample без anti-alias. Укажите `aa=\"sinc\"` (или другой фильтр). Аргументы: `{}`",
                args.trim()
            ));
        }
    }
    Ok(())
}

/// Ищет `key <ws>* sep <value>` вне кавычек и возвращает **диапазон значения** `(val_start, val_end)`.
fn find_value_span(args: &str, key: &str, sep: char) -> Option<(usize, usize)> {
    let mut in_s = false;
    let mut in_d = false;
    let mut i = 0usize;
    let bytes = args.as_bytes();
    while i < bytes.len() {
        let (ch, ch_len) = next_char(&bytes[i..])?;
        match ch {
            '\'' if !in_d => in_s = !in_s,
            '"' if !in_s => in_d = !in_d,
            _ if !in_s && !in_d => {
                if args[i..].starts_with(key) {
                    let mut j = i + key.len();
                    while j < bytes.len() {
                        let (c2, l2) = next_char(&bytes[j..])?;
                        if c2.is_whitespace() {
                            j += l2;
                        } else {
                            break;
                        }
                    }
                    if j < bytes.len() && args[j..].starts_with(sep) {
                        let val_start = j + sep.len_utf8();
                        let val_end = find_value_end(args, val_start);
                        return Some((val_start, val_end));
                    }
                }
            }
            _ => {}
        }
        i += ch_len;
    }
    None
}

/// Находит конец значения: до первой запятой вне кавычек или до конца строки.
fn find_value_end(args: &str, mut i: usize) -> usize {
    let bytes = args.as_bytes();
    let mut in_s = false;
    let mut in_d = false;
    while i < bytes.len() {
        let (ch, ch_len) = match next_char(&bytes[i..]) {
            Some(t) => t,
            None => break,
        };
        match ch {
            '\'' if !in_d => in_s = !in_s,
            '"' if !in_s => in_d = !in_d,
            ',' if !in_s && !in_d => break,
            _ => {}
        }
        i += ch_len;
    }
    i
}

fn unquote(s: &str) -> &str {
    let t = s.trim();
    if (t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')) {
        &t[1..t.len() - 1]
    } else {
        t
    }
}

/// Читает следующий Unicode-символ и возвращает (char, длина_в_байтах).
fn next_char(bytes: &[u8]) -> Option<(char, usize)> {
    let s = std::str::from_utf8(bytes).ok()?;
    let mut it = s.chars();
    let ch = it.next()?;
    Some((ch, ch.len_utf8()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r7_ok_reflect() {
        let src = r#"
            x = W(bank="stft", edge="reflect")(x)
        "#;
        assert!(all(src).is_ok());
    }

    #[test]
    fn r7_ok_multiline_reflect() {
        let src = r#"
            x = W(
                bank="stft",
                edge="reflect"
            )(x)
        "#;
        assert!(all(src).is_ok());
    }

    #[test]
    fn r7_fail_zero() {
        let src = r#"
            x = W(bank="stft", edge="zero")(x)
        "#;
        assert!(all(src).is_err());
    }

    #[test]
    fn r8_ok_with_aa() {
        let src = r#"
            x = D(λ=2, aa="sinc")(x)
        "#;
        assert!(all(src).is_ok());
    }

    #[test]
    fn r8_fail_no_aa() {
        let src = r#"
            x = D(λ=2)(x)
        "#;
        assert!(all(src).is_err());
    }

    #[test]
    fn r8_comment_should_not_mask() {
        let src = r#"
            x = D(λ=2)(x) // aa="sinc"
        "#;
        assert!(all(src).is_err());
    }
}
