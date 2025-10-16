//! WaveLint: aggregate lints for WaveML .wml sources.
//! Контракт: `all(src)` → Ok(()) если PASS, иначе Err(..) с кодом.

mod safety;

pub type Result<T> = anyhow::Result<T>;

/// Запуск всех линтеров поверх исходного текста .wml
pub fn all(src: &str) -> Result<()> {
    // R7: строго edge="reflect", запрет zero-pad
    safety::check_r7_edges(src)?;

    // R8: даунсэмплинг требует AA
    safety::check_r8_aa(src)?;

    // Safety: запрещено A ∘ Align
    safety::check_a_after_align(src)?;

    Ok(())
}
