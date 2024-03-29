use crate::{
    state::{Mode, State},
    ui,
};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{backend::Backend, Terminal};
use tui_input::backend::crossterm::EventHandler;

pub(crate) fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut state: State) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut state))?;
        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match state.mode {
                    Mode::Normal => {
                        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                            break;
                        } else {
                            handle_key_events_for_normal_mode(key, &mut state)
                        }
                    }
                    Mode::Insert => insert_mode(key, &mut state),
                    Mode::Edit => handle_key_events_for_edit_mode(key, &mut state),
                    Mode::Break => {
                        if key.code == KeyCode::Esc {
                            state.reset_break_timer();
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn handle_key_events_for_normal_mode(key: KeyEvent, state: &mut State) {
    match key.code {
        KeyCode::Char('p') => {
            let _ = &state.pause_timer();
        }
        KeyCode::Char('s') => {
            let _ = &state.start_timer();
        }
        KeyCode::Char('r') => {
            let _ = &state.reset_timer();
        }
        KeyCode::Char('i') => state.mode = Mode::Insert,
        KeyCode::Char('e') => state.mode = Mode::Edit,
        KeyCode::Char('?') => state.show_help_popup = true,
        KeyCode::Esc => state.show_help_popup = false,
        KeyCode::Char('b') => state.start_break_timer(),
        _ => {}
    }
}

fn handle_key_events_for_edit_mode(key: KeyEvent, state: &mut State) {
    match key.code {
        KeyCode::Esc => {
            state.unselect_table_item();
            state.mode = Mode::Normal;
        }
        KeyCode::Down | KeyCode::Char('j') => state.next_table_row(),
        KeyCode::Up | KeyCode::Char('k') => state.previous_table_row(),
        KeyCode::Char('m') => state.mark_current_task_as_completed(),
        KeyCode::Char('d') => state.delete_current_task(),
        _ => {}
    }
}

fn insert_mode(key: KeyEvent, state: &mut State) {
    match key.code {
        KeyCode::Enter => {
            let task_description = state.input.value().into();
            state.add_new_task(task_description);
            state.input.reset();
        }
        KeyCode::Esc => {
            state.mode = Mode::Normal;
        }
        _ => {
            state.input.handle_event(&Event::Key(key));
        }
    }
}
