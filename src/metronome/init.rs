use anyhow::anyhow;
use cpal::{
    Stream, default_host,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use crate::{
    app::AResult,
    metronome::{MetronomeShared, runner::MetronomeRunner},
};

pub fn initialize(data: MetronomeShared) -> AResult<Stream> {
    let host = default_host();

    let device = host
        .default_output_device()
        .ok_or(anyhow!("No output device founded!"))?;

    let supported_config = device.default_output_config()?;

    let config = supported_config.into();

    let err_fn = |err| eprintln!("erro no stream: {err}");

    let mut runner = MetronomeRunner::new(data, config);

    let stream = device.build_output_stream(
        config,
        move |buffer, timestamp| {
            runner.process_audio(buffer, &timestamp.timestamp());
        },
        err_fn,
        None,
    )?;

    stream.play()?;

    Ok(stream)
}
