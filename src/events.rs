use crate::state::State;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

pub fn handle(state: &mut State) -> Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) => return Ok(false),
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => return Ok(false),
            (_, KeyCode::Right) => state.progress += 0.01,
            _ => {}
        },
        _ => {}
    }
    Ok(true)
}

