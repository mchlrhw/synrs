use synrs::{stream_setup_for, Oscillator, Waveform};

fn main() -> anyhow::Result<()> {
    let (sample_rate, samples) = stream_setup_for()?;
    let mut oscillator = Oscillator {
        waveform: Waveform::Sine,
        freq: 440.0,
        rate: sample_rate as f32,
        clock: 0.0,
    };

    let mut waveforms = [
        Waveform::Square,
        Waveform::Saw,
        Waveform::Triangle,
        Waveform::Sine,
    ]
    .iter()
    .cycle();

    let start_time = std::time::Instant::now();
    let mut previous_elapsed = 0;

    loop {
        samples.send(oscillator.sample())?;

        // Every second, change the waveform
        let elapsed = start_time.elapsed().as_secs();
        if elapsed > previous_elapsed {
            oscillator.waveform = *waveforms.next().unwrap();
            previous_elapsed = elapsed
        }
    }
}
