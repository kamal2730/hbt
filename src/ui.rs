use ratatui::{
    Frame,
    widgets::{Block, Borders, LineGauge},
};
use crate::state::State;

pub fn render(frame: &mut Frame, state: &State) {
    let gauge = LineGauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .ratio(state.progress);

    frame.render_widget(gauge, frame.area());
}
