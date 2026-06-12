use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

use crate::metronome::config::{
    MAX_BEATS, MAX_BPM, MAX_SUBDIVISIONS, MAX_VOLUME, MIN_BEATS, MIN_BPM, MIN_SUBDIVISIONS,
    MIN_VOLUME,
};
use clap::{Parser, value_parser};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct InputArgs {
    #[arg(short, long, default_value_t = 120, value_parser = |s: &str| parse_value(s, MIN_BPM..=MAX_BPM, "bpm"))]
    pub bpm: u16,

    #[arg(short, long, default_value_t = 100, value_parser = |s: &str| parse_value(s, MIN_VOLUME..=MAX_VOLUME, "volume"))]
    pub volume: u8,

    #[arg(short = 'm', long, default_value_t = 4, value_parser = |s: &str| parse_value(s, MIN_BEATS..=MAX_BEATS, "beats"))]
    pub beats: u8,

    #[arg(short, long, default_value_t = 1, value_parser = value_parser!(u8).range(MIN_SUBDIVISIONS as i64..=MAX_SUBDIVISIONS as i64))]
    pub subdivisions: u8,

    #[arg(short, long, default_value_t = false)]
    pub accent: bool,
}

fn parse_value<T>(s: &str, range: RangeInclusive<T>, name: &'static str) -> Result<T, String>
where
    T: FromStr + Display + PartialOrd<T>,
{
    let v = s
        .parse()
        .map_err(|_| format!("'{s}' isn't a valid number"))?;

    if !range.contains(&v) {
        return Err(format!(
            "{} must be in range {}..={}",
            name,
            range.start(),
            range.end()
        ));
    }

    Ok(v)
}
