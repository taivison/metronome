use crossterm::event::{KeyCode, poll, read};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    cmd::InputArgs,
    metronome::{Metronome, config::MetronomeConfig},
};

pub type AResult<T> = anyhow::Result<T>;
pub type IOResult<T> = std::io::Result<T>;

pub struct App {
    metronome: Metronome,
    exit: bool,
}

impl App {
    pub fn new(args: &InputArgs) -> AResult<Self> {
        Ok(Self {
            metronome: Metronome::new(args)?,
            exit: false,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> IOResult<()> {
        while !self.exit {
            self.metronome.sync();
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> IOResult<()> {
        if !poll(self.metronome.get_wait_time())? {
            return Ok(());
        }

        let Some(e) = read()?.as_key_event() else {
            return Ok(());
        };

        match e.code {
            KeyCode::Char(c) => match c.to_ascii_lowercase() {
                'q' => self.exit = true,
                '-' => self.metronome.dec_volume(),
                '+' => self.metronome.inc_volume(),
                ' ' => self.metronome.toggle_playing(),
                'a' => self.metronome.toggle_accent(),
                's' => self.metronome.change_subdivisions(),
                _ => {}
            },
            KeyCode::Down => self.metronome.dec_bpm(),
            KeyCode::Up => self.metronome.inc_bpm(),
            KeyCode::Left => self.metronome.dec_beats(),
            KeyCode::Right => self.metronome.inc_beats(),
            _ => {}
        };

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub(super) fn config(&self) -> MetronomeConfig {
        self.metronome.config()
    }

    pub(super) fn current_beat(&self) -> u8 {
        self.metronome.current_beat()
    }
}
