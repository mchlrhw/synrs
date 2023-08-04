use synrs::{stream_setup_for, Oscillator, Waveform};

fn main() -> anyhow::Result<()> {
    let (sample_rate, samples) = stream_setup_for()?;

    let mut lfo = Oscillator {
        waveform: Waveform::Sine,
        freq: 0.1,
        rate: sample_rate as f32,
        clock: 0.0,
    };

    let mut oscillator = Oscillator {
        waveform: Waveform::Sine,
        freq: 440.0,
        rate: sample_rate as f32,
        clock: 0.0,
    };

    loop {
        let lfo_sample = (lfo.sample() + 1.0) / 2.0;

        let freq_min = 16.35;
        let freq_max = 7902.13;
        let freq = ((freq_max - freq_min) * lfo_sample) + freq_min;

        oscillator.freq = freq;

        samples.send(oscillator.sample())?;
    }
}
