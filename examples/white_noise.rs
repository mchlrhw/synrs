use synrs::{stream_setup_for, WhiteNoise};

fn main() -> anyhow::Result<()> {
    let (_, samples) = stream_setup_for()?;
    let noise = WhiteNoise;

    for sample in noise {
        samples.send(sample)?;
    }

    Ok(())
}
