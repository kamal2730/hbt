use crate::state::State;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph},
};

pub fn render(frame: &mut Frame, state: &State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());
    let _title = format!("hbt : {:.1}%", state.progress * 100.0);
    let header = Paragraph::new("hbt").block(Block::default()).centered();

    frame.render_widget(header, chunks[0]);
}
