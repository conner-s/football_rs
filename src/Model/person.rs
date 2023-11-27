//Conner Smith 2023


// Module: Model.Person
// Helper module for the Person struct.
use std::fmt;

//import height
use super::height::Height;

pub struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    height: Height,
    weight: u16,
    hometown: String,
    high_school: String,
}

#[allow(dead_code)]
//Allowing dead code so the compiler doesn't complain about the getters and setters not being used.
impl Person {
    pub fn new_empty() -> Person {
        Person {
            first_name: "no".to_string(),
            last_name: "name".to_string(),
            age: 0,
            height: Height::new_empty(),
            weight: 0,
            hometown: "no".to_string(),
            high_school: "no".to_string(),
        }
    }
    pub fn new(first_name: String, last_name: String, age: u8, height: Height, weight: u16, hometown: String, high_school: String) -> Self {
        Self { first_name, last_name, age, height, weight, hometown, high_school }
    }

    // Getters and Setters

    pub fn first_name(&self) -> &str {
        &self.first_name
    }
    pub fn last_name(&self) -> &str {
        &self.last_name
    }
    pub fn age(&self) -> u8 {
        self.age
    }
    pub fn height(&self) -> &Height {
        &self.height
    }
    pub fn weight(&self) -> u16 {
        self.weight
    }
    pub fn hometown(&self) -> &str {
        &self.hometown
    }
    pub fn high_school(&self) -> &str {
        &self.high_school
    }


    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }
    pub fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }
    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }
    pub fn set_height(&mut self, height: Height) {
        self.height = height;
    }
    pub fn set_weight(&mut self, weight: u16) {
        self.weight = weight;
    }
    pub fn set_hometown(&mut self, hometown: String) {
        self.hometown = hometown;
    }
    pub fn set_high_school(&mut self, high_school: String) {
        self.high_school = high_school;
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{} {} is {} years old, {} tall, and weighs {} pounds. They are from {} and went to {}.", self.first_name, self.last_name, self.age, self.height, self.weight, self.hometown, self.high_school);
        write!(f, "{}", s)
    }
}