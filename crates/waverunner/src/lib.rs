use anyhow::*; use wmlb::Graph; use waveform::WaveForm;
pub fn run(_g:&Graph, input:&WaveForm)->Result<WaveForm>{ Ok(input.clone()) }