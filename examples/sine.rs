use synrs::{stream_setup_for, Oscillator, Waveform};

fn main() -> anyhow::Result<()> {
    let (sample_rate, samples) = stream_setup_for()?;
    let mut oscillator = Oscillator {
        sample_rate: sample_rate as f32,
        waveform: Waveform::Sine,
        current_sample_index: 0.0,
        frequency_hz: 440.0,
    };

    loop {
        samples.send(oscillator.tick())?;
    }
}
