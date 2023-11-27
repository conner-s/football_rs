use std::fmt::Display;
use crate::model::person::Person;
use crate::traits::table_member_traits::TableMemberTraits;

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