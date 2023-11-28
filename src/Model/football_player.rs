//For formatting the struct for printing
use std::fmt::Display;

//For reading from xml
use std::fs::File;
use std::io::BufReader;
use rand::Rng;
use xml::reader::{EventReader, XmlEvent};



//Model and traits
use crate::model::{person::Person, height::Height};
use crate::traits::{table_member_traits::TableMemberTraits, table_data_traits::TableDataTraits};

//Struct for individual football players
#[derive(Debug)]
pub struct FootballPlayer {
    person: Person,
    number: u16,
    position: String,
}
#[allow(dead_code)]
//Allowing dead code so the compiler doesn't complain about the getters and setters not being used.
impl FootballPlayer {
    pub fn new(person: Person, number: u16, position: String) -> FootballPlayer {
        FootballPlayer {
            person,
            number,
            position,
        }
    }
    pub fn new_empty() -> FootballPlayer {
        FootballPlayer {
            person: Person::new_empty(),
            number: 0,
            position: "no position".to_string(),
        }
    }

    //Getters and Setters
    pub fn person(&self) -> &Person {
        &self.person
    }
    pub fn number(&self) -> u16 {
        self.number
    }
    pub fn position(&self) -> &str {
        &self.position
    }

    pub fn set_person(&mut self, person: Person) {
        self.person = person;
    }
    pub fn set_number(&mut self, number: u16) {
        self.number = number;
    }
    pub fn set_position(&mut self, position: String) {
        self.position = position;
    }
}

impl Display for FootballPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{} {} {}", self.person, self.number, self.position))
    }
}
impl TableMemberTraits for FootballPlayer{
    fn get_attribute(&self, index: u16) -> String {
        match index {
            0 => self.person().first_name().to_string(),
            1 => self.person().last_name().to_string(),
            2 => self.person().age().to_string(),
            3 => self.person().height().to_string(),
            4 => self.person().weight().to_string(),
            5 => self.person().hometown().to_string(),
            6 => self.person().high_school().to_string(),
            7 => self.number().to_string(),
            8 => self.position().to_string(),
            _ => "Invalid index".to_string(),
        }
    }

    fn get_attributes(&self) -> Vec<String> {
        let mut attributes: Vec<String> = Vec::new();
        for i in 0..9 {
            attributes.push(self.get_attribute(i));
        }
        attributes
    }

    fn get_attribute_name(&self, index: u16) -> String {
        match index {
            0 => "First Name".to_string(),
            1 => "Last Name".to_string(),
            2 => "Age".to_string(),
            3 => "Height".to_string(),
            4 => "Weight".to_string(),
            5 => "Hometown".to_string(),
            6 => "High School".to_string(),
            7 => "Number".to_string(),
            8 => "Position".to_string(),
            _ => "Invalid index".to_string(),
        }
    }

    fn get_attribute_names(&self) -> Vec<String> {
        let mut attribute_names: Vec<String> = Vec::new();
        for i in 0..9 {
            attribute_names.push(self.get_attribute_name(i));
        }
        attribute_names
    }
}




//Struct for a collection of football players that'll be used to populate the table
#[derive(Debug)]
pub struct FootballPlayerData {
    players: Vec<FootballPlayer>,
}
impl FootballPlayerData {
    pub fn new(players: Vec<FootballPlayer>) -> FootballPlayerData {
        FootballPlayerData {
            players,
        }
    }
    pub fn new_empty() -> FootballPlayerData {
        FootballPlayerData {
            players: Vec::new(),
        }
    }
}

impl TableDataTraits<FootballPlayer> for FootballPlayerData {
    fn load_table_data(&self) -> Vec<FootballPlayer> {
        let file = File::open("src/FootballPlayerTable.xml").unwrap();
        let file = BufReader::new(file);

        let parser = EventReader::new(file);
        let mut players = Vec::new();

        let mut rng = rand::thread_rng();

        let mut current_player = None;
        let mut current_person = None;
        let mut current_property = None;
        let mut current_height = None;

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    match name.local_name.as_str() {
                        "object" => {
                            let class = attributes.iter().find(|attr| attr.name.local_name == "class");
                            match class {
                                Some(class) if class.value == "Model.FootballPlayer" => {
                                    current_person = Some(Person::new_empty());
                                    current_player = Some(FootballPlayer::new_empty());
                                }
                                Some(class) if class.value == "Model.Height" => {
                                    current_height = Some(Height::new_empty());
                                }
                                _ => {}
                            }
                        }
                        "void" => {
                            current_property = Some(attributes[0].value.clone());
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::Characters(s)) => {
                    if let Some(property) = &current_property {
                        match property.as_str() {
                            "feet" => current_height.as_mut().unwrap().set_feet(s.parse().unwrap()),
                            "inches" => current_height.as_mut().unwrap().set_inches(s.parse().unwrap()),
                            "highSchool" => current_person.as_mut().unwrap().set_high_school(s),
                            "hometown" => current_person.as_mut().unwrap().set_hometown(s),
                            "name" => {
                                let names: Vec<&str> = s.split_whitespace().collect();
                                current_person.as_mut().unwrap().set_first_name(names[0].to_string());
                                current_person.as_mut().unwrap().set_last_name(names[1].to_string());
                            }
                            "number" => current_player.as_mut().unwrap().set_number(s.parse().unwrap()),
                            "position" => current_player.as_mut().unwrap().set_position(s),
                            "weight" => current_person.as_mut().unwrap().set_weight(s.parse().unwrap()),
                            _ => {}
                        }

                        // match property.as_str() {
                        //     "feet" => {
                        //         println!("Parsing Feet");
                        //         println!("{}", s);
                        //         current_height.as_mut().unwrap().set_feet(9);
                        //     },
                        //     "inches" => {
                        //         println!("Parsing Inches");
                        //         println!("{}", s);
                        //         current_height.as_mut().unwrap().set_inches(s.parse().unwrap());
                        //     },
                        //     "highSchool" => {
                        //         println!("Parsing High School");
                        //         println!("{}", s);
                        //         current_person.as_mut().unwrap().set_high_school(s);
                        //     },
                        //     "hometown" => {
                        //         println!("Parsing Hometown");
                        //         println!("{}", s);
                        //         current_person.as_mut().unwrap().set_hometown(s);
                        //     },
                        //     "name" => {
                        //         println!("Parsing Name");
                        //         println!("{}", s);
                        //         let names: Vec<&str> = s.split_whitespace().collect();
                        //         current_person.as_mut().unwrap().set_first_name(names[0].to_string());
                        //         current_person.as_mut().unwrap().set_last_name(names[1].to_string());
                        //     },
                        //     "number" => {
                        //         println!("Parsing Number");
                        //         println!("{}", s);
                        //         current_player.as_mut().unwrap().set_number(s.parse().unwrap());
                        //     },
                        //     "position" => {
                        //         println!("Parsing Position");
                        //         println!("{}", s);
                        //         current_player.as_mut().unwrap().set_position(s);
                        //     },
                        //     "weight" => {
                        //         println!("Parsing Weight");
                        //         println!("{}", s);
                        //         current_person.as_mut().unwrap().set_weight(s.parse().unwrap());
                        //     },
                        //     "e" => {
                        //         println!("Parsing e");
                        //         println!("{}", s);
                        //     },
                        //     _ => {println!("Invalid property");
                        //     println!("{}", property.as_str());
                        //         println!("{}", property);
                        //         println!("{:?}", s);
                        //     }
                        // }
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    if name.local_name == "object" {
                        if let Some(height) = current_height.take() {
                            current_person.as_mut().unwrap().set_height(height);
                        } else {
                            let age = rng.gen_range(18..=47);
                            current_person.as_mut().unwrap().set_age(age);
                            current_player.as_mut().unwrap().set_person(current_person.take().unwrap());
                            players.push(current_player.take().unwrap());
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        players
    }

    fn get_table_headers(&self) -> Vec<String> {
        let mut headers: Vec<String> = Vec::new();
        for i in 0..9 {
            headers.push(self.players[0].get_attribute_name(i));
        }
        headers
    }

    fn get_line(&self, index: usize) -> FootballPlayer {
        let player = FootballPlayer::new_empty();
        player
    }

    fn get_line_as_string(&self, index: usize) -> String {
        self.players[index].to_string()
    }

    fn get_lines(&self, first_line: usize, last_line: usize) -> Vec<FootballPlayer> {
        let mut lines: Vec<FootballPlayer> = Vec::new();
        for i in first_line..last_line {
            //lines.push(self.players[i].clone());
        }
        lines
    }
}


