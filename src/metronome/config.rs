use std::sync::atomic::Ordering;

use crate::{cmd::InputArgs, metronome::Metronome};

pub const MAX_VOLUME: u8 = 100;
pub const MIN_VOLUME: u8 = 0;
pub const MAX_BPM: u16 = 500;
pub const MIN_BPM: u16 = 1;
pub const MAX_BEATS: u8 = 12;
pub const MIN_BEATS: u8 = 2;
pub const MAX_SUBDIVISIONS: u8 = 4;
pub const MIN_SUBDIVISIONS: u8 = 1;

impl Metronome {
    fn update(&self, mut f: impl FnMut(&mut MetronomeConfig)) {
        self.data
            .config
            .update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                let mut v = v.into();
                f(&mut v);
                v.into()
            });
    }

    pub fn inc_bpm(&self) {
        self.update(|cfg| cfg.bpm = cfg.bpm.saturating_add(1).min(MAX_BPM));
    }

    pub fn dec_bpm(&self) {
        self.update(|cfg| cfg.bpm = cfg.bpm.saturating_sub(1).max(MIN_BPM));
    }

    pub fn inc_volume(&self) {
        self.update(|cfg| cfg.volume = cfg.volume.saturating_add(1).min(MAX_VOLUME));
    }

    pub fn dec_volume(&self) {
        self.update(|cfg| cfg.volume = cfg.volume.saturating_sub(1).max(MIN_VOLUME));
    }

    pub fn inc_beats(&self) {
        self.update(|cfg| {
            cfg.beats = cfg.beats.saturating_add(1).min(MAX_BEATS);
            cfg.playing = false;
        });
    }

    pub fn dec_beats(&self) {
        self.update(|cfg| {
            cfg.beats = cfg.beats.saturating_sub(1).max(MIN_BEATS);
            cfg.playing = false;
        });
    }

    pub fn change_subdivisions(&self) {
        self.update(|cfg| {
            cfg.subdivisions = (cfg.subdivisions % (MAX_SUBDIVISIONS)) + 1;
            cfg.playing = false;
        });
    }

    pub fn toggle_accent(&self) {
        self.update(|cfg| cfg.accent = !cfg.accent);
    }

    pub fn toggle_playing(&self) {
        self.update(|cfg| cfg.playing = !cfg.playing);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetronomeConfig {
    pub bpm: u16,
    pub volume: u8,
    pub beats: u8,
    pub subdivisions: u8,
    pub accent: bool,
    pub playing: bool,
}

impl From<u64> for MetronomeConfig {
    fn from(value: u64) -> Self {
        let bytes = value.to_le_bytes();
        Self {
            bpm: u16::from_le_bytes([bytes[0], bytes[1]]),
            volume: bytes[2],
            beats: bytes[3],
            subdivisions: bytes[4],
            accent: bytes[5] != 0,
            playing: bytes[6] != 0,
        }
    }
}

impl From<MetronomeConfig> for u64 {
    fn from(value: MetronomeConfig) -> Self {
        let bpm = value.bpm.to_le_bytes();
        u64::from_le_bytes([
            bpm[0],
            bpm[1],
            value.volume,
            value.beats,
            value.subdivisions,
            value.accent as u8,
            value.playing as u8,
            0x0,
        ])
    }
}

impl From<&InputArgs> for MetronomeConfig {
    fn from(value: &InputArgs) -> Self {
        Self {
            bpm: value.bpm,
            volume: value.volume,
            beats: value.beats,
            subdivisions: value.subdivisions,
            accent: value.accent,
            playing: false,
        }
    }
}
