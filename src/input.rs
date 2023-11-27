use std::io;
use crossterm::event::{KeyCode, KeyEvent};
use tui::Terminal;

use crate::app::App;

pub fn handle_input<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    event: KeyEvent,
    app: &mut App,
) -> io::Result<()> {
    match event.code {
        KeyCode::Char('q') => return Ok(()),
        KeyCode::Right => app.next(),
        KeyCode::Left => app.previous(),
        _ => {}
    }
    Ok(())
}
