use crate::state::State;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn handle(state: &mut State) -> Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => return Ok(false),
            KeyCode::Right => state.progress += 0.01,
            _ => {}
        },
        _ => {}
    }
    Ok(true)
}
