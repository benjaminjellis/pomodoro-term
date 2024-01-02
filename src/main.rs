mod app;
mod art;
mod state;
mod ui;

use crate::ui::ui;
use app::run_app;
use clap::Command;
use color_eyre::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use state::State;
use std::io::stdout;

#[tokio::main]
async fn main() -> Result<()> {
    let _matches = Command::new("CARGO_PKG_NAME")
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .about("Pomodoro in the terminal")
        .get_matches();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let state = State::new();

    run_app(&mut terminal, state)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
