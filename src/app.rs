use crate::{state::State, ui};
use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};

pub(crate) fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut state: State) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut state))?;
        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('p') {
                    let _ = &state.pause_timer();
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('s') {
                    let _ = &state.start_timer();
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('r') {
                    let _ = &state.reset_timer();
                }
            }
        }
    }
    Ok(())
}
