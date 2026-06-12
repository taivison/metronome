mod app;
mod cmd;
mod metronome;
mod ui;
use clap::Parser;

use crate::{
    app::{AResult, App},
    cmd::InputArgs,
};

fn main() -> AResult<()> {
    let args = InputArgs::parse();

    let mut app = App::new(&args)?;
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
