use crate::state::{State, Screen};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

pub fn handle(state: &mut State) -> Result<bool> {
    match state.screen {
        Screen::TodayEntry => {
            state.enter_today_values();
            return Ok(true); // go back to menu
        }
        Screen::HabitSettings => {
            state.add_habit();
            return Ok(true);
        }
        _ => {}
    }

    // Normal input handling
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match (key.modifiers, key.code) {
            (_, KeyCode::Char('1')) => state.screen = Screen::TodayEntry,
            (_, KeyCode::Char('2')) => state.screen = Screen::ViewLog,
            (_, KeyCode::Char('3')) => state.screen = Screen::HabitSettings,

            (_, KeyCode::Char('m')) => state.screen = Screen::Menu,
            (_, KeyCode::Esc) => state.screen = Screen::Menu,

            // Quit
            (_, KeyCode::Char('q')) => return Ok(false),
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => return Ok(false),

            _ => {}
        },
        _ => {}
    }

    Ok(true)
}
