mod model;
use model::person::Person;
use model::height::Height;

fn main() {
    let person = Person::new("John".to_string(), "Doe".to_string(), 32, Height::new(6, 4), 0, "New York".to_string(), "New York".to_string());
    println!("Hello, world!");
    println!("Person: {}", person.to_string());
}
