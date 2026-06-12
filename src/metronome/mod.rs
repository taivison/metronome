use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

use cpal::Stream;

use crate::{
    cmd::InputArgs,
    metronome::{
        config::{MAX_BPM, MetronomeConfig},
        init::initialize,
    },
};

pub(super) mod config;
mod init;
mod runner;

type AResult<T> = anyhow::Result<T>;

pub struct Metronome {
    data: MetronomeShared,
    current_beat: u8,
    next_beat_instant: Option<(u8, Instant)>,
    _stream: Stream,
}

impl Metronome {
    pub fn new(args: &InputArgs) -> AResult<Self> {
        let config: MetronomeConfig = args.into();
        let data = MetronomeData {
            config: AtomicU64::new(config.into()),
            next_beat_timestamp: AtomicU64::new(0),
            start_time: Instant::now(),
        };

        let data = MetronomeShared::new(data);

        let _stream = initialize(MetronomeShared::clone(&data))?;

        Ok(Self {
            data,
            _stream,
            current_beat: 0,
            next_beat_instant: None,
        })
    }

    pub fn config(&self) -> MetronomeConfig {
        self.data.config()
    }

    pub fn current_beat(&self) -> u8 {
        self.current_beat + 1
    }

    pub fn sync(&mut self) {
        if !self.config().playing {
            self.current_beat = 0;
            self.next_beat_instant = None;
            return;
        }

        if let Some((beat, instant)) = self.next_beat_instant {
            let now = Instant::now();
            if now >= instant {
                self.current_beat = beat;
                self.next_beat_instant = self.get_next_beat_instant();
            }
        } else {
            self.next_beat_instant = self.get_next_beat_instant();
        }
    }

    pub fn get_wait_time(&self) -> Duration {
        if !self.config().playing {
            return Duration::MAX;
        }

        let Some((_, instant)) = self.next_beat_instant else {
            return Duration::from_secs_f64(60.0 / MAX_BPM as f64 / 2.0);
        };

        instant.duration_since(Instant::now())
    }

    fn get_next_beat_instant(&self) -> Option<(u8, Instant)> {
        if !self.config().playing {
            return None;
        }

        let timestamp = self.data.next_beat_timestamp.load(Ordering::Relaxed);

        let mut bytes = timestamp.to_le_bytes();
        let beat = bytes[7];

        if beat >= self.config().beats {
            return None;
        }

        bytes[7] = 0x00;

        let micros = u64::from_le_bytes(bytes);

        let instant = self.data.start_time + Duration::from_micros(micros);

        Some((beat, instant))
    }
}

type MetronomeShared = Arc<MetronomeData>;

#[derive(Debug)]
pub struct MetronomeData {
    config: AtomicU64,
    next_beat_timestamp: AtomicU64,
    start_time: Instant,
}

impl MetronomeData {
    pub(super) fn config(&self) -> MetronomeConfig {
        self.config.load(Ordering::Relaxed).into()
    }

    fn change_timestamp(&self, beat: u8, micros: u64) {
        let mut timestamp = micros.to_le_bytes();
        timestamp[7] = beat;
        let beat_timestamp = u64::from_le_bytes(timestamp);

        self.next_beat_timestamp
            .store(beat_timestamp, Ordering::Relaxed);
    }
}
