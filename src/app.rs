use crate::{events, state::State, ui};
use color_eyre::Result;
use ratatui::DefaultTerminal;

pub struct App {
    state: State,
    running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::default(),
            running: true,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|f| ui::render(f, &self.state))?;
            self.running = events::handle(&mut self.state)?;
        }
        self.state.save();
        Ok(())
    }
}
