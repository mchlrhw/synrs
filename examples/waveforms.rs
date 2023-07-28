use synrs::{stream_setup_for, Oscillator, Waveform};

fn main() -> anyhow::Result<()> {
    let (sample_rate, samples) = stream_setup_for()?;
    let mut oscillator = Oscillator {
        sample_rate: sample_rate as f32,
        waveform: Waveform::Sine,
        current_sample_index: 0.0,
        frequency_hz: 440.0,
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
        samples.send(oscillator.tick())?;

        // Every second, change the waveform
        let elapsed = start_time.elapsed().as_secs();
        if elapsed > previous_elapsed {
            oscillator.set_waveform(*waveforms.next().unwrap());
            previous_elapsed = elapsed
        }
    }
}
