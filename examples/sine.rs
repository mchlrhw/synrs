use synrs::{stream_setup_for, Oscillator, Waveform};

fn main() -> anyhow::Result<()> {
    let (sample_rate, samples) = stream_setup_for()?;
    let oscillator = Oscillator {
        waveform: Waveform::Sine,
        freq: 440.0,
        rate: sample_rate as f32,
        clock: 0.0,
    };

    for sample in oscillator {
        samples.send(sample)?;
    }

    Ok(())
}
