# Metronomo

A command-line metronome written in Rust.

Metronomo provides accurate beat timing with real-time audio playback and a terminal-based user interface, making it easy to practice and keep tempo directly from the terminal.

## Features

- Adjustable BPM (beats per minute)
- Configurable time signature (beats per measure)
- Configurable subdivisions
- Optional accented first beat
- Real-time audio generation
- Interactive terminal user interface
- Command-line configuration
- Lightweight and efficient
- Cross-platform support

## Built With

- Rust
- CPAL — cross-platform audio output
- Ratatui — terminal user interface
- Clap — command-line argument parser

## Installation

### Clone the repository

```bash
git clone https://github.com/your-username/metronomo.git
cd metronomo
```

### Build

```bash
cargo build --release
```

The executable will be available at:

```text
target/release/metronomo
```

## Command Line Options

```text
Usage: metronome [OPTIONS]

Options:
  -b, --bpm <BPM>                    [default: 120]
  -v, --volume <VOLUME>              [default: 100]
  -m, --beats <BEATS>                [default: 4]
  -s, --subdivisions <SUBDIVISIONS>  [default: 1]
  -a, --accent
  -h, --help                         Print help
  -V, --version                      Print version
```

## User Interface

```text
╭────────────────── METRONOME ───────────────────╮
│                                                │
│  BPM:         120    STATUS: ⏹            VOL  │
│                                            █   │
│  BEATS:         4    ACCENT: ✗             █   │
│                                            █   │
│  SUBDIVISIONS:   1                         █   │
│                                            █   │
│                                            █   │
│                 ⬤  ◯  ◯  ◯                 █   │
│                 ▲                          █   │
│                                            █   │
│                                           100  │
│                                                │
╰────────────────────────────────────────────────╯

╭────────────────── SHORTCUTS ───────────────────╮
│                                                │
│  +/-  INCREASE/DECREASE VOLUME                 │
│  ↑/↓  INCREASE/DECREASE BPM                    │
│  ←/→  INCREASE/DECREASE BEATS                  │
│  S    CHANGE SUBDIVISIONS                      │
│  ␣    PLAY/STOP                                │
│  A    TOGGLE ACCENT                            │
│  Q    QUIT                                     │
│                                                │
│                                                │
│                                                │
│                                                │
╰────────────────────────────────────────────────╯
```

## Usage

Run with default settings:

```bash
metronome
```

Start at 90 BPM with an accented first beat:

```bash
metronome --bpm 90 --accent
```

Use a 7-beat measure with eighth-note subdivisions:

```bash
metronome --beats 7 --subdivisions 2
```

Show available options:

```bash
metronome --help
```

## License

This project is licensed under the MIT License.