//! Линтеры: Safety / GB-R8 / Daʿat / MDL — быстрые проверки R7/R8 (v0.1)
use anyhow::{bail, Result};

#[derive(Debug, Default, Clone, Copy)]
pub struct LintConfig {
    /// Разрешать zero-pad только если явно allow_zero_pad=true (по умолчанию нельзя)
    pub allow_zero_pad: bool,
}

pub fn all(src: &str) -> Result<()> {
    // 1) убираем однострочные комментарии
    let code = strip_line_comments(src);
    // 2) проверки
    let cfg = LintConfig::default();
    check_r7(&code, cfg)?;
    check_r8(&code)?;
    Ok(())
}

/// Примитивное удаление `//` комментариев (до конца строки).
fn strip_line_comments(src: &str) -> String {
    src.lines()
        .map(|line| {
            if let Some(i) = line.find("//") {
                &line[..i]
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// R7: edge ∈ {reflect, Toeplitz}; zero-pad запрещён по умолчанию.
/// Проверяем только параметры внутри W(...), чтобы не ловить шум вне контекста.
fn check_r7(src: &str, cfg: LintConfig) -> Result<()> {
    for (i, line) in src.lines().enumerate() {
        let l = line;
        // ищем все вхождения W(
        let mut from = 0usize;
        while let Some(pos) = l[from..].find("W(") {
            let start = from + pos + 2; // после "W("
            // ищем закрывающую ')' на этой же строке (для v0.1 достаточно)
            if let Some(end_rel) = l[start..].find(')') {
                let end = start + end_rel;
                let args = &l[start..end];
                let has_edge = args.contains("edge=");
                if has_edge {
                    let is_zero = args.contains("edge=\"zero\"") || args.contains("edge='zero'");
                    if is_zero && !cfg.allow_zero_pad {
                        bail!(format!(
                            "R7 violation (строка {}): edge=\"zero\" запрещён. Используйте edge=\"reflect\" или edge=\"Toeplitz\".",
                            i + 1
                        ));
                    }
                    // если edge указан — он должен быть допустимым
                    let ok = args.contains("reflect") || args.contains("Toeplitz") || args.contains("zero");
                    if !ok {
                        bail!(format!(
                            "R7 violation (строка {}): edge задан, но не reflect/Toeplitz (аргументы: `{}`)",
                            i + 1, args.trim()
                        ));
                    }
                }
                from = end + 1;
            } else {
                bail!(format!("R7 violation (строка {}): незакрытая скобка для W(", i + 1));
            }
        }
    }
    Ok(())
}

/// R8: любой D(λ=...) ДОЛЖЕН иметь `aa=...` внутри скобок `D(...)`.
/// Не учитываем `aa=` вне аргументов и/или в комментариях.
fn check_r8(src: &str) -> Result<()> {
    for (i, line) in src.lines().enumerate() {
        let l = line;
        let mut from = 0usize;
        while let Some(pos) = l[from..].find("D(") {
            let start = from + pos + 2; // после "D("
            if let Some(end_rel) = l[start..].find(')') {
                let end = start + end_rel;
                let args = &l[start..end];
                let has_aa = args.contains("aa=") || args.contains("aa:");
                if !has_aa {
                    bail!(format!(
                        "R8 violation (строка {}): Downsample без anti-alias. Укажите `aa=\"sinc\"` (или другой фильтр). Аргументы: `{}`",
                        i + 1, args.trim()
                    ));
                }
                from = end + 1;
            } else {
                bail!(format!("R8 violation (строка {}): незакрытая скобка для D(", i + 1));
            }
        }
    }
    Ok(())
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


