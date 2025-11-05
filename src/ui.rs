use crate::state::{State, Screen};

use ratatui::{
    Frame,
    widgets::{Block, Paragraph, Borders},
    style::{Style, Color, Stylize},
    layout::{Constraint, Direction, Layout},
};

pub fn render(frame: &mut Frame, state: &State) {
    match state.screen {
        Screen::Menu => draw_menu(frame),
        Screen::TodayEntry => draw_today(frame),
        Screen::ViewLog => draw_log(frame, state),
        Screen::HabitSettings => draw_settings(frame),
    }
}

fn draw_menu(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ])
        .split(frame.area());

    let text = "hbt\n
1: Enter Today
2: View Log
3: Habit Settings
q: Quit
Esc/m: Menu";

    let p = Paragraph::new(text)
        .centered()
        .style(Color::Cyan)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Menu ")
                .title_style(Style::default().fg(Color::Yellow).bold())
        );

    frame.render_widget(p, chunks[1]);
}

fn draw_today(frame: &mut Frame) {
    let p = Paragraph::new("Entering today's values...\nCheck terminal input.")
        .centered()
        .block(Block::default().borders(Borders::ALL).title(" Today "));

    frame.render_widget(p, frame.area());
}

fn draw_log(frame: &mut Frame, state: &State) {
    let mut text = String::new();

    for (date, habits) in &state.logs {
        text.push_str(&format!("{}\n", date));
        for (h, v) in habits {
            text.push_str(&format!("  {:<12} {}\n", h, v));
        }
        text.push('\n');
    }

    let p = Paragraph::new(text)
        .block(Block::default().title(" Logs ").borders(Borders::ALL));

    frame.render_widget(p, frame.area());
}

fn draw_settings(frame: &mut Frame) {
    let p = Paragraph::new("Adding habit...\nCheck terminal input.")
        .centered()
        .block(Block::default().borders(Borders::ALL).title(" Habit Settings "));

    frame.render_widget(p, frame.area());
}
