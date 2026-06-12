use core::f32;
use std::time::{Duration, Instant};

use cpal::{OutputStreamTimestamp, StreamConfig};

use crate::metronome::{MetronomeShared, config::MAX_VOLUME};
pub struct MetronomeRunner {
    data: MetronomeShared,
    next_beat: u8,
    ticks_next_beat: usize,
    click_remaining: usize,
    tick_phase: f32,
    stream_config: StreamConfig,
    sound_generator: Harmonics,
}

impl MetronomeRunner {
    pub(super) fn new(data: MetronomeShared, stream_config: StreamConfig) -> Self {
        Self {
            data,
            next_beat: 0,
            ticks_next_beat: 0,
            click_remaining: 0,
            tick_phase: 0.0,
            stream_config,
            sound_generator: NORMAL,
        }
    }

    pub(super) fn process_audio(&mut self, buffer: &mut [f32], timestamp: &OutputStreamTimestamp) {
        let playback_instant =
            Instant::now() + timestamp.playback.duration_since(timestamp.callback);

        let configs = self.data.config();
        let sample_rate = self.stream_config.sample_rate as f32;

        if !configs.playing {
            self.reset();
            buffer.fill(0.0);
            return;
        }

        let samples_per_tick =
            ((sample_rate * 60.0) / configs.bpm as f32 / configs.subdivisions as f32) as usize;

        for (i, frame) in buffer
            .chunks_mut(self.stream_config.channels as usize)
            .enumerate()
        {
            if self.ticks_next_beat == 0 {
                self.sound_generator = match self.next_beat {
                    0 if configs.accent => ACCENTED,
                    r if r % configs.subdivisions == 0 => NORMAL,
                    _ => SUBBEAT,
                };

                // Calculate the time for the next main beat
                if self.next_beat % (configs.subdivisions) == 0 {
                    let time_per_sample = (sample_rate as f64).recip();
                    let playback_instant = playback_instant
                        + Duration::from_secs_f64(time_per_sample * i as f64)
                        + Duration::from_secs_f64(
                            time_per_sample * samples_per_tick as f64 * configs.subdivisions as f64,
                        );

                    let micros = playback_instant - self.data.start_time;
                    self.data.change_timestamp(
                        ((self.next_beat / configs.subdivisions) + 1) % configs.beats,
                        micros.as_micros() as u64,
                    );
                }

                self.ticks_next_beat = samples_per_tick;

                self.next_beat = (self.next_beat + 1) % (configs.beats * configs.subdivisions);
                self.tick_phase = 0.0;
                self.click_remaining = (SOUND_DURATION * sample_rate) as usize;
            }

            let mut sample = if self.click_remaining > 0 {
                let sample = self.sound_generator.generate(self.tick_phase);
                self.tick_phase += (sample_rate as f32).recip();
                self.click_remaining -= 1;
                sample
            } else {
                0.0f32
            };

            sample *= configs.volume as f32 / MAX_VOLUME as f32;
            frame.fill(sample);

            self.ticks_next_beat -= 1;
        }
    }

    fn reset(&mut self) {
        self.data.change_timestamp(0xFF, 0x00);
        self.next_beat = 0;
        self.ticks_next_beat = 0;
        self.click_remaining = 0;
        self.tick_phase = 0.0;
        self.sound_generator = NORMAL;
    }
}

struct Harmonics {
    fundamental: f32,
    multipliers: &'static [Multipliers],
}

impl Harmonics {
    pub fn generate(&self, time: f32) -> f32 {
        let envelope = (1.0 - time / SOUND_DURATION).max(0.0);
        let fundamental_phase = f32::consts::PI * 2.0 * self.fundamental * time;

        let sample = fundamental_phase.sin()
            + self
                .multipliers
                .iter()
                .map(|m| m.amplitude * (fundamental_phase * m.frequency).sin())
                .sum::<f32>();
        sample * envelope
    }
}

struct Multipliers {
    frequency: f32,
    amplitude: f32,
}

const SOUND_DURATION: f32 = 50e-3;

macro_rules! harmonic {
    ($name:ident, $fundamental:literal, $(($freq:literal, $amplitude:literal)),+ ) => {
       const $name: Harmonics = Harmonics {
           fundamental: $fundamental,
           multipliers: &[
            $(Multipliers {
                frequency: $freq,
                amplitude: $amplitude
            }),+
           ]
       };
    };
}

harmonic!(ACCENTED, 1800.0, (2.0, 0.5), (3.0, 0.25));
harmonic!(NORMAL, 1200.0, (2.0, 0.3));
harmonic!(SUBBEAT, 700.0, (2.0, 0.2));
