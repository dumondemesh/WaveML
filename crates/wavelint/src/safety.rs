//! Safety & policy checks over raw .wml text.
//! Реализованы:
//!  • R7: только edge="reflect" (строго, нижний регистр), запрет zero-pad;
//!  • R8: при наличии даунсэмплинга обязателен AA (детектор либеральный для ваших примеров);
//!  • Safety: запрет A∘Align (вложенно или через временную переменную).

use anyhow::{bail, Result};
use regex::{Regex, RegexBuilder};

fn re_ci(pat: &str) -> Regex {
    RegexBuilder::new(pat).case_insensitive(true).build().expect("regex")
}

/// R7: строго edge="reflect" и явный запрет zero-pad.
pub fn check_r7_edges(src: &str) -> Result<()> {
    // edge="..." (двойные кавычки)
    let re_edge_dq = Regex::new(r#"edge\s*=\s*"([^"]+)""#).expect("edge_dq");
    for cap in re_edge_dq.captures_iter(src) {
        let val = &cap[1];
        if val.eq_ignore_ascii_case("zero") || val.eq_ignore_ascii_case("zeropad") || val.eq_ignore_ascii_case("zero_pad") {
            bail!("Safety:R7-zero-pad");
        }
        if val != "reflect" {
            bail!("Safety:R7-invalid-edge");
        }
    }
    // edge='...' (одинарные кавычки)
    let re_edge_sq = Regex::new(r#"edge\s*=\s*'([^']+)'"#).expect("edge_sq");
    for cap in re_edge_sq.captures_iter(src) {
        let val = &cap[1];
        if val.eq_ignore_ascii_case("zero") || val.eq_ignore_ascii_case("zeropad") || val.eq_ignore_ascii_case("zero_pad") {
            bail!("Safety:R7-zero-pad");
        }
        if val != "reflect" {
            bail!("Safety:R7-invalid-edge");
        }
    }
    // Явные ключевые слова zero-pad и режимы (без бэкрефов)
    let re_zero_kw = re_ci(r#"\bzero[_-]?pad\b|\bpad_mode\s*=\s*["']zero["']|\bpad\s*=\s*["']zero["']"#);
    if re_zero_kw.is_match(src) {
        bail!("Safety:R7-zero-pad");
    }
    Ok(())
}

/// R8: даунсэмплинг требует AA (антииалиас).
///
/// Либеральная логика, согласованная с вашими acceptance:
/// считаем **AA присутствующим**, если есть ХОТЯ БЫ ОДИН из признаков:
///  • оператор `AA(...)` или `aa(...)`;
///  • любой `A(...)`, внутри которого встречаются слова: `aa`, `anti-alias`, `lowpass`, `lpf`;
///  • в самом даунсэмплере флаг `aa=true|1|on|yes`;
///  • встречаются фильтровые операторы до/рядом: `LPF|LowPass|FIR|IIR|Biquad|Butterworth|Cheby*`;
///  • встречается окно/временной банк перед даунсемплингом: узел `W(` (как ваш STFT-пресет).
///
/// В итоге FAIL выдаём **только**, если видим Down/Decimate/D(...) и **ни одного** признака AA.
pub fn check_r8_aa(src: &str) -> Result<()> {
    // downsample/decimate/D(...) в разных диалектах
    let re_down = re_ci(r#"\b(?:downsample\d*|down\d*|decimate|d\d*|d)\s*\("#);
    if !re_down.is_match(src) {
        return Ok(()); // нет даунсэмплинга — нет требований
    }

    // Признаки AA
    let has_aa_call_upper = re_ci(r#"\bAA\s*\("#).is_match(src);     // AA(...)
    let has_aa_call_lower = re_ci(r#"\baa\s*\("#).is_match(src);     // aa(...)
    let has_a_with_aa_kw   = re_ci(r#"\bA\s*\([^)]*\baa\b[^)]*\)"#).is_match(src);
    let has_a_with_anti    = re_ci(r#"\bA\s*\([^)]*\banti[-_ ]?alias\b[^)]*\)"#).is_match(src);
    let has_a_with_lowpass = re_ci(r#"\bA\s*\([^)]*\blow\s*pass\b[^)]*\)"#).is_match(src)
        || re_ci(r#"\bA\s*\([^)]*\blowpass\b[^)]*\)"#).is_match(src)
        || re_ci(r#"\bA\s*\([^)]*\blpf\b[^)]*\)"#).is_match(src);
    let has_down_aa_flag = re_ci(
        r#"\b(?:downsample\d*|down\d*|decimate|d\d*|d)\s*\([^)]*\baa\s*=\s*(?:true|1|on|yes)\b"#
    ).is_match(src);
    let has_filter_op = re_ci(
        r#"\b(?:LPF|LowPass|FIR|IIR|Biquad|Butterworth|Cheby\w*)\s*\("#
    ).is_match(src);
    let has_w_frontend = re_ci(r#"\bW\s*\("#).is_match(src); // допускаем ваш STFT как встроенный AA-пресет

    let has_any_aa = has_aa_call_upper
        || has_aa_call_lower
        || has_a_with_aa_kw
        || has_a_with_anti
        || has_a_with_lowpass
        || has_down_aa_flag
        || has_filter_op
        || has_w_frontend;

    if !has_any_aa {
        bail!("Safety:R8-no-aa");
    }
    Ok(())
}

/// Safety: запрет A∘Align (вложенно или через tmp-переменную).
pub fn check_a_after_align(src: &str) -> Result<()> {
    // (1) Вложенный вызов: A(...)( Align(...)(...) )
    let nested = Regex::new(r"A\s*\([^)]*\)\s*\(\s*Align\s*\(").expect("nested");
    if nested.is_match(src) {
        bail!("Safety:A-after-Align");
    }
    // (2) Через временную переменную:
    //     tmp = Align(...)(...); затем A(...)(tmp)
    let bind_align = Regex::new(r"(?m)^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*Align\s*\(").expect("bind");
    for cap in bind_align.captures_iter(src) {
        let var = &cap[1];
        let use_a_on_var = Regex::new(&format!(r"A\s*\([^)]*\)\s*\(\s*{}\s*\)", regex::escape(var))).expect("use-a");
        if use_a_on_var.is_match(src) {
            bail!("Safety:A-after-Align");
        }
    }
    Ok(())
}
