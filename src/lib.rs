use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, SizedSample,
};
use std::{
    sync::mpsc::{self, Receiver, SyncSender},
    thread,
};

#[derive(Clone, Copy)]
pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
}

impl Waveform {
    pub fn sample(&self, freq: f32, rate: f32, clock: f32) -> f32 {
        use Waveform::*;

        match self {
            Sine => Self::sine(freq, rate, clock),
            Square => Self::generative(2, 1.0, freq, rate, clock),
            Saw => Self::generative(1, 1.0, freq, rate, clock),
            Triangle => Self::generative(2, 2.0, freq, rate, clock),
        }
    }

    // Pure sinusoidal waveform.
    fn sine(freq: f32, rate: f32, clock: f32) -> f32 {
        use std::f32::consts::TAU;

        (clock * freq * TAU / rate).sin()
    }

    /// Generative waveform.
    fn generative(harmonic_inc: u32, gain_exp: f32, freq: f32, rate: f32, clock: f32) -> f32 {
        let mut output = 0.0;

        let mut i = 1;
        while !Self::is_multiple_of_freq_above_nyquist(i, freq, rate) {
            let gain = 1.0 / (i as f32).powf(gain_exp);
            output += gain * Self::sine(freq * i as f32, rate, clock);
            i += harmonic_inc;
        }

        output
    }

    fn is_multiple_of_freq_above_nyquist(multiple: u32, freq: f32, rate: f32) -> bool {
        freq * (multiple as f32) > rate / 2.0
    }
}

pub struct Oscillator {
    pub waveform: Waveform,
    pub freq: f32,
    pub rate: f32,
    pub clock: f32,
}

impl Oscillator {
    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    fn tick(&mut self) {
        self.clock = (self.clock + 1.0) % self.rate;
    }

    pub fn sample(&mut self) -> f32 {
        self.tick();

        self.waveform.sample(self.freq, self.rate, self.clock)
    }
}

impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.sample())
    }
}

pub fn stream_setup_for() -> anyhow::Result<(u32, SyncSender<f32>)>
where
{
    let (_host, device, config) = host_device_setup()?;

    match config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(device, config.into()),
        cpal::SampleFormat::I16 => make_stream::<i16>(device, config.into()),
        cpal::SampleFormat::I32 => make_stream::<i32>(device, config.into()),
        cpal::SampleFormat::I64 => make_stream::<i64>(device, config.into()),
        cpal::SampleFormat::U8 => make_stream::<u8>(device, config.into()),
        cpal::SampleFormat::U16 => make_stream::<u16>(device, config.into()),
        cpal::SampleFormat::U32 => make_stream::<u32>(device, config.into()),
        cpal::SampleFormat::U64 => make_stream::<u64>(device, config.into()),
        cpal::SampleFormat::F32 => make_stream::<f32>(device, config.into()),
        cpal::SampleFormat::F64 => make_stream::<f64>(device, config.into()),
        sample_format => Err(anyhow::Error::msg(format!(
            "Unsupported sample format '{sample_format}'"
        ))),
    }
}

pub fn host_device_setup(
) -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
    println!("Output device : {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config : {:?}", config);

    Ok((host, device, config))
}

pub fn make_stream<T>(
    device: cpal::Device,
    config: cpal::StreamConfig,
) -> anyhow::Result<(u32, SyncSender<f32>)>
where
    T: SizedSample + FromSample<f32>,
{
    const LATENCY: u32 = 250;
    let num_channels = config.channels as usize;
    let sample_rate = config.sample_rate.0;
    let buffer_size = (sample_rate / LATENCY) as usize;

    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    let (samples_snd, samples_rcv) = mpsc::sync_channel(buffer_size);

    thread::spawn(move || -> anyhow::Result<()> {
        let stream = device.build_output_stream(
            &config,
            move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
                process_frame(output, num_channels, &samples_rcv)
            },
            err_fn,
            None,
        )?;

        stream.play()?;
        thread::park();

        Ok(())
    });

    Ok((sample_rate, samples_snd))
}

fn process_frame<SampleType>(
    output: &mut [SampleType],
    num_channels: usize,
    samples: &Receiver<f32>,
) where
    SampleType: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(num_channels) {
        let value: SampleType = SampleType::from_sample(samples.recv().unwrap());

        // copy the same value to all channels
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
