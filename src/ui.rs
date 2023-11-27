use std::io;

use tui::{backend::{Backend}, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, text::{Span, Spans}, widgets::{Block, Borders, Tabs}, Frame, Terminal};
use crossterm::{
    event::Event,
};

use crate::app::App;
use crate::input::handle_input;
use crate::ui::render_ui::render_ui;

pub fn run_ui<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| render_ui(f, &app))?;

        if let Event::Key(key) = crossterm::event::read()? {
            handle_input(terminal, key, &mut app)?;
        } else if let Event::Mouse(mouse_event) = crossterm::event::read()? {
            // Ignore mouse events
            match mouse_event {
                _ => {}
            }
        }
    }
}

pub mod render_ui {
    use super::*;

    pub fn render_ui<B: Backend>(f: &mut Frame<B>, app: &App) {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);

        let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::White));
        f.render_widget(block, size);

        let titles = app
            .titles
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(first, Style::default().fg(Color::Yellow)),
                    Span::styled(rest, Style::default().fg(Color::Green)),
                ])
            })
            .collect();

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(app.index)
            .style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Gray),
            );

        f.render_widget(tabs, chunks[0]);

        let inner_vec = vec![
            Block::default().title("Inner 0").borders(Borders::ALL),
            Block::default().title("Inner 1").borders(Borders::ALL),
            Block::default().title("Inner 2").borders(Borders::ALL),
            Block::default().title("Inner 3").borders(Borders::ALL),
        ];

        let inner = match app.index {
            0 => inner_vec[0].clone(),
            1 => inner_vec[1].clone(),
            2 => inner_vec[2].clone(),
            3 => inner_vec[3].clone(),
            _ => unreachable!(),
        };

        f.render_widget(inner, chunks[1]);
    }
}
