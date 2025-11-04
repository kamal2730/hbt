use crate::state::State;
use ratatui::{
    Frame,
    widgets::{Block, Borders, LineGauge},
};

pub fn render(frame: &mut Frame, state: &State) {
    let gauge = LineGauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .ratio(state.progress);

    frame.render_widget(gauge, frame.area());
}
