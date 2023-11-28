//Conner Smith 2023
//Porting of IST 242 Project to Rust
//If Penn State gets mad about this, uhhh I'll take it down I guess.
//

//Data structures and traits
mod model;
mod traits;

use model::person::Person;
use model::height::Height;
use model::football_player::FootballPlayer;
use crate::traits::table_member_traits::TableMemberTraits;


//TUI stuff
use std::{error::Error, time::Duration};

use argh::FromArgs;
use crate::model::football_player::FootballPlayerData;
use crate::traits::table_data_traits::TableDataTraits;

mod app;
mod crossterm;
mod ui;

/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);

    crossterm::run(tick_rate, cli.enhanced_graphics)?;

    debug_objects();
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


    let players = FootballPlayerData::new_empty();
    let players = players.load_table_data();
    println!("Football Player Data: {}", players[0]);
}