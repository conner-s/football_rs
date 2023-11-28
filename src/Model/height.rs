use std::fmt;

#[derive(Debug)]
pub(crate) struct Height {
    feet: u8,
    inches: u8,
}
#[allow(dead_code)]
impl Height {
    // Constructors
    pub(crate) fn new(feet: u8, inches: u8) -> Height {
        Height {
            feet,
            inches,
        }
    }
    pub fn new_empty() -> Height {
        Height {
            feet: 6,
            inches: 0,
        }
    }
    pub fn new_from_feet(feet: &u8) -> Height {
        Height {
            feet: *feet,
            inches: 0,
        }
    }

    // Getters and Setters
    pub fn feet(&self) -> u8 {
        self.feet
    }
    pub fn inches(&self) -> u8 {
        self.inches
    }
    pub fn set_feet(&mut self, feet: u8) {
        self.feet = feet;
    }
    pub fn set_inches(&mut self, inches: u8) {
        self.inches = inches;
    }

    //Functions
    fn to_string(&self) -> String {
        format!("{}\'{}\"", self.feet, self.inches)
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\'{}\"", self.feet, self.inches)
    }
}
