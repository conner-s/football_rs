mod model;
mod traits;

use model::person::Person;
use model::height::Height;
use model::football_player::FootballPlayer;
use crate::traits::table_member_traits::TableMemberTraits;

use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

mod app;
mod ui;
mod input;

use app::App;
use ui::run_ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_ui(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

//Debugging function
#[allow(dead_code)]
fn debug_objects() {
    let person = Person::new("John".to_string(), "Doe".to_string(), 32, Height::new(6, 4), 0, "New York".to_string(), "New York".to_string());
    println!("Hello, world!");
    println!("Person: {}", person.to_string());
    let football_player = FootballPlayer::new(person, 0, "QB".to_string());

    let test = football_player.get_attributes();
    println!("Football Player: {}", football_player.to_string());
    println!("Football Player Attributes: {:?}", test);
}